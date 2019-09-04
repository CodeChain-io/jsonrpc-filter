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

#![feature(test)]

extern crate test;

use std::collections::{BTreeSet, HashSet};

use test::{black_box, Bencher};

use jsonrpc_filter::bisect_set::BisectSet;

static ALLOWED_LIST: [&'static str; 40] = [
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
];

fn contains_linear(list: &[String], method: &str) -> bool {
    list.iter().any(|allowed| allowed == method)
}

fn contains_hash_set(list: &HashSet<String>, method: &str) -> bool {
    list.contains(method)
}

fn contains_btree_set(list: &BTreeSet<String>, method: &str) -> bool {
    list.contains(method)
}

fn contains_bisect(list: &BisectSet<String>, method: &str) -> bool {
    list.contains(method)
}

mod first_item {
    use super::*;
    use std::iter::FromIterator;

    #[bench]
    fn with_vec_iter(b: &mut Bencher) {
        let list: Vec<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();

        let method = ALLOWED_LIST.first().unwrap().to_string();

        b.iter(|| {
            black_box(contains_linear(&list, &method));
        })
    }

    #[bench]
    fn with_vec_bisect(b: &mut Bencher) {
        let list = BisectSet::from_iter(ALLOWED_LIST.iter().map(ToString::to_string));

        let method = ALLOWED_LIST.first().unwrap().to_string();

        b.iter(|| {
            black_box(contains_bisect(&list, &method));
        })
    }

    #[bench]
    fn with_hash_set(b: &mut Bencher) {
        let list: HashSet<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();

        let method = ALLOWED_LIST.first().unwrap().to_string();

        b.iter(|| {
            black_box(contains_hash_set(&list, &method));
        })
    }

    #[bench]
    fn with_btree_set(b: &mut Bencher) {
        let list: BTreeSet<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();

        let method = ALLOWED_LIST.first().unwrap().to_string();

        b.iter(|| {
            black_box(contains_btree_set(&list, &method));
        })
    }
}

mod second_item {
    use super::*;
    use std::iter::FromIterator;

    #[bench]
    fn with_vec_iter(b: &mut Bencher) {
        let list: Vec<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();

        let method = ALLOWED_LIST.get(1).unwrap().to_string();

        b.iter(|| {
            black_box(contains_linear(&list, &method));
        })
    }

    #[bench]
    fn with_vec_bisect(b: &mut Bencher) {
        let list = BisectSet::from_iter(ALLOWED_LIST.iter().map(ToString::to_string));

        let method = ALLOWED_LIST.get(1).unwrap().to_string();

        b.iter(|| {
            black_box(contains_bisect(&list, &method));
        })
    }

    #[bench]
    fn with_hash_set(b: &mut Bencher) {
        let list: HashSet<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();

        let method = ALLOWED_LIST.get(1).unwrap().to_string();

        b.iter(|| {
            black_box(contains_hash_set(&list, &method));
        })
    }

    #[bench]
    fn with_btree_set(b: &mut Bencher) {
        let list: BTreeSet<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();

        let method = ALLOWED_LIST.get(1).unwrap().to_string();

        b.iter(|| {
            black_box(contains_btree_set(&list, &method));
        })
    }
}

mod third_item {
    use super::*;
    use std::iter::FromIterator;

    #[bench]
    fn with_vec_iter(b: &mut Bencher) {
        let list: Vec<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();

        let method = ALLOWED_LIST.get(2).unwrap().to_string();

        b.iter(|| {
            black_box(contains_linear(&list, &method));
        })
    }

    #[bench]
    fn with_vec_bisect(b: &mut Bencher) {
        let list = BisectSet::from_iter(ALLOWED_LIST.iter().map(ToString::to_string));

        let method = ALLOWED_LIST.get(2).unwrap().to_string();

        b.iter(|| {
            black_box(contains_bisect(&list, &method));
        })
    }

    #[bench]
    fn with_hash_set(b: &mut Bencher) {
        let list: HashSet<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();

        let method = ALLOWED_LIST.get(2).unwrap().to_string();

        b.iter(|| {
            black_box(contains_hash_set(&list, &method));
        })
    }

    #[bench]
    fn with_btree_set(b: &mut Bencher) {
        let list: BTreeSet<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();

        let method = ALLOWED_LIST.get(2).unwrap().to_string();

        b.iter(|| {
            black_box(contains_btree_set(&list, &method));
        })
    }
}

mod q1_item {
    use super::*;
    use std::iter::FromIterator;

    #[bench]
    fn with_vec_iter(b: &mut Bencher) {
        let list: Vec<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();
        let method = ALLOWED_LIST
            .get(ALLOWED_LIST.len() / 4)
            .unwrap()
            .to_string();

        b.iter(|| {
            black_box(contains_linear(&list, &method));
        })
    }

    #[bench]
    fn with_vec_bisect(b: &mut Bencher) {
        let list = BisectSet::from_iter(ALLOWED_LIST.iter().map(ToString::to_string));

        let method = ALLOWED_LIST
            .get(ALLOWED_LIST.len() / 4)
            .unwrap()
            .to_string();

        b.iter(|| {
            black_box(contains_bisect(&list, &method));
        })
    }

    #[bench]
    fn with_hash_set(b: &mut Bencher) {
        let list: HashSet<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();
        let method = ALLOWED_LIST
            .get(ALLOWED_LIST.len() / 4)
            .unwrap()
            .to_string();

        b.iter(|| {
            black_box(contains_hash_set(&list, &method));
        })
    }

    #[bench]
    fn with_btree_set(b: &mut Bencher) {
        let list: BTreeSet<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();
        let method = ALLOWED_LIST
            .get(ALLOWED_LIST.len() / 4)
            .unwrap()
            .to_string();

        b.iter(|| {
            black_box(contains_btree_set(&list, &method));
        })
    }
}

mod middle_item {
    use super::*;
    use std::iter::FromIterator;

    #[bench]
    fn with_vec_iter(b: &mut Bencher) {
        let list: Vec<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();
        let method = ALLOWED_LIST
            .get(ALLOWED_LIST.len() / 2)
            .unwrap()
            .to_string();

        b.iter(|| {
            black_box(contains_linear(&list, &method));
        })
    }

    #[bench]
    fn with_vec_bisect(b: &mut Bencher) {
        let list = BisectSet::from_iter(ALLOWED_LIST.iter().map(ToString::to_string));

        let method = ALLOWED_LIST
            .get(ALLOWED_LIST.len() / 2)
            .unwrap()
            .to_string();

        b.iter(|| {
            black_box(contains_bisect(&list, &method));
        })
    }

    #[bench]
    fn with_hash_set(b: &mut Bencher) {
        let list: HashSet<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();
        let method = ALLOWED_LIST
            .get(ALLOWED_LIST.len() / 2)
            .unwrap()
            .to_string();

        b.iter(|| {
            black_box(contains_hash_set(&list, &method));
        })
    }

    #[bench]
    fn with_btree_set(b: &mut Bencher) {
        let list: BTreeSet<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();
        let method = ALLOWED_LIST
            .get(ALLOWED_LIST.len() / 2)
            .unwrap()
            .to_string();

        b.iter(|| {
            black_box(contains_btree_set(&list, &method));
        })
    }
}

mod q3_item {
    use super::*;
    use std::iter::FromIterator;

    #[bench]
    fn with_vec_iter(b: &mut Bencher) {
        let vec: Vec<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();
        let method = ALLOWED_LIST
            .get(3 * ALLOWED_LIST.len() / 4)
            .unwrap()
            .to_string();

        b.iter(|| {
            black_box(contains_linear(&vec, &method));
        })
    }

    #[bench]
    fn with_vec_bisect(b: &mut Bencher) {
        let list = BisectSet::from_iter(ALLOWED_LIST.iter().map(ToString::to_string));

        let method = ALLOWED_LIST
            .get(3 * ALLOWED_LIST.len() / 4)
            .unwrap()
            .to_string();

        b.iter(|| {
            black_box(contains_bisect(&list, &method));
        })
    }

    #[bench]
    fn with_hash_set(b: &mut Bencher) {
        let vec: HashSet<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();
        let method = ALLOWED_LIST
            .get(3 * ALLOWED_LIST.len() / 4)
            .unwrap()
            .to_string();

        b.iter(|| {
            black_box(contains_hash_set(&vec, &method));
        })
    }

    #[bench]
    fn with_btree_set(b: &mut Bencher) {
        let vec: BTreeSet<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();
        let method = ALLOWED_LIST
            .get(3 * ALLOWED_LIST.len() / 4)
            .unwrap()
            .to_string();

        b.iter(|| {
            black_box(contains_btree_set(&vec, &method));
        })
    }
}

mod last_item {
    use super::*;
    use std::iter::FromIterator;

    #[bench]
    fn with_vec_iter(b: &mut Bencher) {
        let vec: Vec<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();
        let method = ALLOWED_LIST.last().unwrap().to_string();

        b.iter(|| {
            black_box(contains_linear(&vec, &method));
        })
    }

    #[bench]
    fn with_vec_bisect(b: &mut Bencher) {
        let list = BisectSet::from_iter(ALLOWED_LIST.iter().map(ToString::to_string));

        let method = ALLOWED_LIST.last().unwrap().to_string();

        b.iter(|| {
            black_box(contains_bisect(&list, &method));
        })
    }

    #[bench]
    fn with_hash_set(b: &mut Bencher) {
        let vec: HashSet<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();
        let method = ALLOWED_LIST.last().unwrap().to_string();

        b.iter(|| {
            black_box(contains_hash_set(&vec, &method));
        })
    }

    #[bench]
    fn with_btree_set(b: &mut Bencher) {
        let vec: BTreeSet<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();
        let method = ALLOWED_LIST.last().unwrap().to_string();

        b.iter(|| {
            black_box(contains_btree_set(&vec, &method));
        })
    }
}

mod non_exist {
    use super::*;
    use std::iter::FromIterator;

    #[bench]
    fn with_vec_iter(b: &mut Bencher) {
        let vec: Vec<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();
        let method = "chain_getNonExist".to_string();

        b.iter(|| {
            black_box(contains_linear(&vec, &method));
        })
    }

    #[bench]
    fn with_vec_bisect(b: &mut Bencher) {
        let list = BisectSet::from_iter(ALLOWED_LIST.iter().map(ToString::to_string));

        let method = "chain_getNonExist".to_string();

        b.iter(|| {
            black_box(contains_bisect(&list, &method));
        })
    }

    #[bench]
    fn with_hash_set(b: &mut Bencher) {
        let vec: HashSet<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();
        let method = "chain_getNonExist".to_string();

        b.iter(|| {
            black_box(contains_hash_set(&vec, &method));
        })
    }

    #[bench]
    fn with_btree_set(b: &mut Bencher) {
        let vec: BTreeSet<String> = ALLOWED_LIST.iter().map(ToString::to_string).collect();
        let method = "chain_getNonExist".to_string();

        b.iter(|| {
            black_box(contains_btree_set(&vec, &method));
        })
    }
}
