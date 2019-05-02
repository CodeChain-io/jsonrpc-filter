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
use hyper::header::{HeaderValue, ACCESS_CONTROL_ALLOW_ORIGIN};
use hyper::service::Service;
use hyper::{Body, Client, Method, Request, Response, StatusCode};

use super::Error;
use std::cmp::Ordering;

pub struct Filter {
    forward: hyper::Uri,
    allowed_rpcs: Vec<String>,
}

impl Filter {
    pub fn new(forward: hyper::Uri, allowed_rpcs: Vec<String>) -> Self {
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
    type Future = Box<Future<Item = Response<Self::ResBody>, Error = Self::Error> + Send>;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let (header, body) = req.into_parts();
        if Method::POST != header.method && Method::OPTIONS != header.method {
            return Box::new(future::result(
                Response::builder()
                    .status(StatusCode::METHOD_NOT_ALLOWED)
                    .header(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"))
                    .body(Body::from(
                        "Used HTTP Method is not allowed. POST or OPTIONS is required",
                    ))
                    .map_err(From::from),
            ));
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

                    Client::new()
                        .request(req)
                        .map_err(From::from)
                        .map(|mut response| {
                            response
                                .headers_mut()
                                .insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
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

fn contains_bisect(list: &[String], method: &str) -> bool {
    let len = list.len();
    if len == 0 {
        return false;
    }

    let index = len / 2;
    match method.cmp(&list[index]) {
        Ordering::Less => contains_bisect(&list[0..index], method),
        Ordering::Equal => true,
        Ordering::Greater => contains_bisect(&list[index + 1..len], method),
    }
}

fn filter_allowed_request(buffer: Vec<u8>, allowed_rpcs: &[String]) -> Result<Vec<u8>, Error> {
    let request = serde_json::from_slice::<serde_json::Value>(&buffer)?;
    let method = request.get("method").ok_or(Error::MethodIsNotDefined)?;
    let method = method.as_str().ok_or(Error::MethodIsNotString)?;
    if contains_bisect(allowed_rpcs, method) {
        Ok(buffer)
    } else {
        Err(Error::NotAllowedMethod(method.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn allowed_list() -> Vec<String> {
        let mut allowed_list: Vec<String> = [
            "chain_containsTransaction",
            "chain_executeTransaction",
            "chain_executeVM",
            "chain_getAsset",
            "chain_getAssetSchemeByTracker",
            "chain_getAssetSchemeByType",
            "chain_getBalance",
            "chain_getBestBlockId",
            "chain_getBestBlockNumber",
            "chain_getBlockByHash",
            "chain_getBlockByNumber",
            "chain_getBlockHash",
            "chain_getBlockTransactionCountByHash",
            "chain_getGenesisAccounts",
            "chain_getMiningReward",
            "chain_getNetworkId",
            "chain_getNumberOfShards",
            "chain_getRegularKey",
            "chain_getRegularKeyOwner",
            "chain_getSeq",
            "chain_getShardIdByHash",
            "chain_getShardOwners",
            "chain_getShardRoot",
            "chain_getShardUsers",
            "chain_getText",
            "chain_getTransaction",
            "chain_getTransactionByTracker",
            "chain_isAssetSpent",
            "commitHash",
            "engine_getBlockReward",
            "engine_getCoinbase",
            "engine_getCustomActionData",
            "engine_getRecommendedConfimation",
            "mempool_getErrorHint",
            "mempool_getPendingTransactions",
            "mempool_getPendingTransactionsCount",
            "mempool_getTransactionResultsByTracker",
            "mempool_sendSignedTransaction",
            "ping",
            "version",
        ]
        .iter()
        .map(ToString::to_string)
        .collect();

        allowed_list.sort_unstable();
        allowed_list
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn bisect_test() {
        let allowed_list = allowed_list();
        assert!(contains_bisect(&allowed_list, "chain_containsTransaction"));
        assert!(contains_bisect(&allowed_list, "chain_executeTransaction"));
        assert!(contains_bisect(&allowed_list, "chain_executeVM"));
        assert!(contains_bisect(&allowed_list, "chain_getAsset"));
        assert!(contains_bisect(
            &allowed_list,
            "chain_getAssetSchemeByTracker"
        ));
        assert!(contains_bisect(&allowed_list, "chain_getAssetSchemeByType"));
        assert!(contains_bisect(&allowed_list, "chain_getBalance"));
        assert!(contains_bisect(&allowed_list, "chain_getBestBlockId"));
        assert!(contains_bisect(&allowed_list, "chain_getBestBlockNumber"));
        assert!(contains_bisect(&allowed_list, "chain_getBlockByHash"));
        assert!(contains_bisect(&allowed_list, "chain_getBlockByNumber"));
        assert!(contains_bisect(&allowed_list, "chain_getBlockHash"));
        assert!(contains_bisect(
            &allowed_list,
            "chain_getBlockTransactionCountByHash"
        ));
        assert!(contains_bisect(&allowed_list, "chain_getGenesisAccounts"));
        assert!(contains_bisect(&allowed_list, "chain_getMiningReward"));
        assert!(contains_bisect(&allowed_list, "chain_getNetworkId"));
        assert!(contains_bisect(&allowed_list, "chain_getNumberOfShards"));
        assert!(contains_bisect(&allowed_list, "chain_getRegularKey"));
        assert!(contains_bisect(&allowed_list, "chain_getRegularKeyOwner"));
        assert!(contains_bisect(&allowed_list, "chain_getSeq"));
        assert!(contains_bisect(&allowed_list, "chain_getShardIdByHash"));
        assert!(contains_bisect(&allowed_list, "chain_getShardOwners"));
        assert!(contains_bisect(&allowed_list, "chain_getShardRoot"));
        assert!(contains_bisect(&allowed_list, "chain_getShardUsers"));
        assert!(contains_bisect(&allowed_list, "chain_getText"));
        assert!(contains_bisect(&allowed_list, "chain_getTransaction"));
        assert!(contains_bisect(
            &allowed_list,
            "chain_getTransactionByTracker"
        ));
        assert!(contains_bisect(&allowed_list, "chain_isAssetSpent"));
        assert!(contains_bisect(&allowed_list, "commitHash"));
        assert!(contains_bisect(&allowed_list, "engine_getBlockReward"));
        assert!(contains_bisect(&allowed_list, "engine_getCoinbase"));
        assert!(contains_bisect(&allowed_list, "engine_getCustomActionData"));
        assert!(contains_bisect(
            &allowed_list,
            "engine_getRecommendedConfimation"
        ));
        assert!(contains_bisect(&allowed_list, "mempool_getErrorHint"));
        assert!(contains_bisect(
            &allowed_list,
            "mempool_getPendingTransactions"
        ));
        assert!(contains_bisect(
            &allowed_list,
            "mempool_getPendingTransactionsCount"
        ));
        assert!(contains_bisect(
            &allowed_list,
            "mempool_getTransactionResultsByTracker"
        ));
        assert!(contains_bisect(
            &allowed_list,
            "mempool_sendSignedTransaction"
        ));
        assert!(contains_bisect(&allowed_list, &"ping"));
        assert!(contains_bisect(&allowed_list, &"version"));
        assert!(!contains_bisect(&allowed_list, "non exist"));
    }
}
