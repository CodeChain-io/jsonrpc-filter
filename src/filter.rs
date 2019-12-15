// Copyright 2019 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Poll};

use futures::TryStreamExt;
use hyper::header::{
    HeaderValue, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
};
use hyper::service::Service;
use hyper::{Body, Client, Method, Request, Response, StatusCode};

use super::Error;
use crate::bisect_set::BisectSet;
use crate::config::Config;
use std::pin::Pin;

pub struct Filter {
    config: Arc<Config>,
    seq: u64,
}

impl Filter {
    pub fn new(config: Arc<Config>, seq: u64) -> Self {
        Filter {
            config,
            seq,
        }
    }
}

type DynFuture<Output> = Pin<Box<dyn Future<Output = Output> + Send>>;

impl Service<Request<Body>> for Filter {
    type Response = Response<Body>;
    type Error = Error;
    type Future = DynFuture<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        async fn run(config: Arc<Config>, seq: u64, req: Request<Body>) -> Result<Response<Body>, Error> {
            let (header, body) = req.into_parts();
            if Method::POST != header.method && Method::OPTIONS != header.method {
                info!("seq: {}, Invalid method: {}", seq, header.method);
                return Ok(Response::builder()
                    .status(StatusCode::METHOD_NOT_ALLOWED)
                    .header(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"))
                    .header(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("POST, OPTIONS"))
                    .header(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("Content-Type"))
                    .body(Body::from("Used HTTP Method is not allowed. POST or OPTIONS is required"))?)
            }

            if Method::OPTIONS == header.method {
                info!("seq: {}, CORS preflight", seq);
                return Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"))
                    .header(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("POST, OPTIONS"))
                    .header(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("Content-Type"))
                    .body(Body::from(""))?)
            }

            let buffer = collect_body(body).await?;
            trace!("seq: {}, bytes: {}", seq, String::from_utf8_lossy(&buffer));
            filter_allowed_request(&buffer, &config.allowed_rpcs, seq)?;

            let mut req = Request::from_parts(header, Body::from(buffer));
            *req.uri_mut() = config.forward.clone();

            let mut response = Client::new().request(req).await?;
            response.headers_mut().insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
            let (parts, body) = response.into_parts();
            let buffer = collect_body(body).await?;
            trace!("seq: {}, forward, bytes: {}", seq, String::from_utf8_lossy(&buffer));
            Ok(Response::from_parts(parts, Body::from(buffer)))
        }

        Box::pin(run(Arc::clone(&self.config), self.seq, req))
    }
}

fn filter_allowed_request(buffer: &[u8], allowed_rpcs: &BisectSet<String>, seq: u64) -> Result<(), Error> {
    let request = serde_json::from_slice::<serde_json::Value>(&buffer)?;
    let method = request.get("method").ok_or(Error::MethodIsNotDefined)?;
    let method = method.as_str().ok_or(Error::MethodIsNotString)?;
    debug!("seq: {}, method: {}", seq, method);
    if allowed_rpcs.contains(method) {
        Ok(())
    } else {
        info!("seq: {}, blocked", seq);
        Err(Error::NotAllowedMethod(method.to_string()))
    }
}

async fn collect_body(body: Body) -> Result<Vec<u8>, hyper::Error> {
    body.map_ok(|bytes| bytes.to_vec()).try_concat().await
}
