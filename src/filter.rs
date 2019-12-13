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

use std::sync::Arc;

use futures::stream::Stream;
use futures::{future, Future};
use hyper::header::{
    HeaderValue, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
};
use hyper::service::Service;
use hyper::{Body, Client, Method, Request, Response, StatusCode};

use super::Error;
use crate::bisect_set::BisectSet;
use crate::config::Config;

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

impl Service for Filter {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Error;
    type Future = Box<dyn Future<Item = Response<Self::ResBody>, Error = Self::Error> + Send>;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let seq = self.seq;
        let (header, body) = req.into_parts();
        if Method::POST != header.method && Method::OPTIONS != header.method {
            info!("seq: {}, Invalid method: {}", seq, header.method);
            return Box::new(future::result(
                Response::builder()
                    .status(StatusCode::METHOD_NOT_ALLOWED)
                    .header(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"))
                    .header(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("POST, OPTIONS"))
                    .header(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("Content-Type"))
                    .body(Body::from("Used HTTP Method is not allowed. POST or OPTIONS is required"))
                    .map_err(From::from),
            ))
        }

        if Method::OPTIONS == header.method {
            info!("seq: {}, CORS preflight", seq);
            return Box::new(future::result(
                Response::builder()
                    .status(StatusCode::OK)
                    .header(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"))
                    .header(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("POST, OPTIONS"))
                    .header(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("Content-Type"))
                    .body(Body::from(""))
                    .map_err(From::from),
            ))
        }

        let forward = self.config.forward.clone();
        let allowed_rpcs = self.config.allowed_rpcs.clone();
        Box::new(
            collect_body(body)
                .map_err(From::from)
                .inspect(move |buffer| trace!("seq: {}, bytes: {}", seq, String::from_utf8_lossy(&buffer)))
                .and_then(move |buffer| filter_allowed_request(buffer, &allowed_rpcs, seq))
                .and_then(move |buffer| {
                    let mut req = Request::from_parts(header, Body::from(buffer));
                    *req.uri_mut() = forward.clone();

                    Client::new().request(req).map_err(Error::from).and_then(move |mut response| {
                        response.headers_mut().insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
                        let (parts, body) = response.into_parts();
                        collect_body(body)
                            .map_err(From::from)
                            .inspect(move |buffer| {
                                trace!("seq: {}, forward, bytes: {}", seq, String::from_utf8_lossy(&buffer))
                            })
                            .map(|buffer| Response::from_parts(parts, Body::from(buffer)))
                    })
                })
                .map_err(move |err| {
                    error!("seq: {}, forward, error: {}", seq, err);
                    err
                }),
        )
    }
}

fn filter_allowed_request(buffer: Vec<u8>, allowed_rpcs: &BisectSet<String>, seq: u64) -> Result<Vec<u8>, Error> {
    let request = serde_json::from_slice::<serde_json::Value>(&buffer)?;
    let method = request.get("method").ok_or(Error::MethodIsNotDefined)?;
    let method = method.as_str().ok_or(Error::MethodIsNotString)?;
    debug!("seq: {}, method: {}", seq, method);
    if allowed_rpcs.contains(method) {
        Ok(buffer)
    } else {
        info!("seq: {}, blocked", seq);
        Err(Error::NotAllowedMethod(method.to_string()))
    }
}

fn collect_body(body: Body) -> impl Future<Item = Vec<u8>, Error = hyper::Error> {
    body.collect().map(|body| {
        let mut buffer: Vec<u8> = Vec::new();
        for i in body {
            buffer.append(&mut i.to_vec());
        }
        buffer
    })
}
