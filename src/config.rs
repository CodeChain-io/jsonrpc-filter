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

use std::sync::atomic::{AtomicU64, Ordering};

use futures::{future, Future};
use hyper::service::MakeService;
use hyper::Body;

use super::{Error, Filter};
use crate::bisect_set::BisectSet;

pub struct Config {
    forward: hyper::Uri,
    allowed_rpcs: BisectSet<String>,
    counter: AtomicU64,
}

impl Config {
    pub fn new(forward: hyper::Uri, allowed_rpcs: BisectSet<String>) -> Self {
        Config {
            forward,
            allowed_rpcs,
            counter: AtomicU64::new(0),
        }
    }
}

impl<Ctx> MakeService<Ctx> for Config {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = Error;
    type Service = Filter;
    type Future = Box<dyn Future<Item = Self::Service, Error = Self::MakeError> + Send>;
    type MakeError = Error;

    fn make_service(&mut self, _ctx: Ctx) -> Self::Future {
        let seq = self.counter.fetch_add(1, Ordering::SeqCst);
        Box::new(future::ok(Filter::new(self.forward.clone(), self.allowed_rpcs.clone(), seq)))
    }
}
