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

use futures::{future, Future};
use hyper::service::NewService;
use hyper::Body;

use super::{Error, Filter};

pub struct Config {
    forward: hyper::Uri,
    allowed_rpcs: Vec<String>,
}

impl Config {
    pub fn new(forward: hyper::Uri, allowed_rpcs: Vec<String>) -> Self {
        Config {
            forward,
            allowed_rpcs,
        }
    }
}

impl NewService for Config {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Error;
    type Service = Filter;
    type Future = Box<dyn Future<Item = Self::Service, Error = Self::InitError> + Send>;
    type InitError = Error;

    fn new_service(&self) -> Self::Future {
        Box::new(future::ok(Filter::new(
            self.forward.clone(),
            self.allowed_rpcs.clone(),
        )))
    }
}
