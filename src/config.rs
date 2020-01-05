// Copyright 2019-2020 Kodebox, Inc.
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

use super::{Error, Filter};
use crate::bisect_set::BisectSet;
use futures::future;
use hyper::service::Service;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};

pub struct Config {
    pub forward: hyper::Uri,
    pub allowed_rpcs: BisectSet<String>,
}

impl Config {
    pub fn new(forward: hyper::Uri, allowed_rpcs: BisectSet<String>) -> Self {
        Config {
            forward,
            allowed_rpcs,
        }
    }
}

pub struct ServiceMaker {
    config: Arc<Config>,
    counter: AtomicU64,
}

impl ServiceMaker {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            counter: AtomicU64::new(0),
        }
    }
}

impl<T> Service<T> for ServiceMaker {
    type Response = Filter;
    type Error = Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let config = Arc::clone(&self.config);
        let seq = self.counter.fetch_add(1, Ordering::SeqCst);
        future::ok(Filter::new(config, seq))
    }
}
