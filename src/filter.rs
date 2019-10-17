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

use futures::stream::Stream;
use futures::{future, Future};
use hyper::header::{
    HeaderValue, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
};
use hyper::service::Service;
use hyper::{Body, Client, Method, Request, Response, StatusCode};

use super::Error;
use crate::bisect_set::BisectSet;

pub struct Filter {
    forward: hyper::Uri,
    allowed_rpcs: BisectSet<String>,
}

impl Filter {
    pub fn new(forward: hyper::Uri, allowed_rpcs: BisectSet<String>) -> Self {
        Filter {
            forward,
            allowed_rpcs,
        }
    }
}

impl Service for Filter {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Error;
    type Future = Box<dyn Future<Item = Response<Self::ResBody>, Error = Self::Error> + Send>;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let (header, body) = req.into_parts();
        if Method::POST != header.method && Method::OPTIONS != header.method {
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

        let forward = self.forward.clone();
        let allowed_rpcs = self.allowed_rpcs.clone();
        Box::new(
            body.collect()
                .map(|body| {
                    let mut buffer: Vec<u8> = Vec::new();
                    for i in body {
                        buffer.append(&mut i.to_vec());
                    }
                    buffer
                })
                .map_err(From::from)
                .and_then(move |buffer| filter_allowed_request(buffer, &allowed_rpcs))
                .and_then(|buffer| {
                    let mut req = Request::from_parts(header, Body::from(buffer));
                    *req.uri_mut() = forward;

                    Client::new().request(req).map_err(From::from).map(|mut response| {
                        response.headers_mut().insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
                        response
                    })
                })
                .map_err(|err| {
                    info!("Request is filtered: {}", err);
                    err
                }),
        )
    }
}

fn filter_allowed_request(buffer: Vec<u8>, allowed_rpcs: &BisectSet<String>) -> Result<Vec<u8>, Error> {
    let request = serde_json::from_slice::<serde_json::Value>(&buffer)?;
    let method = request.get("method").ok_or(Error::MethodIsNotDefined)?;
    let method = method.as_str().ok_or(Error::MethodIsNotString)?;
    if allowed_rpcs.contains(method) {
        Ok(buffer)
    } else {
        Err(Error::NotAllowedMethod(method.to_string()))
    }
}
