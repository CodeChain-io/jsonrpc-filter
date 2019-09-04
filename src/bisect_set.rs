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

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::iter::{FromIterator, IntoIterator};

#[derive(Clone)]
pub struct BisectSet<T> {
    items: Vec<T>,
}

impl<T: Ord> FromIterator<T> for BisectSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut items: Vec<_> = iter.into_iter().collect();
        items.sort_unstable();
        BisectSet { items }
    }
}

impl<T: std::cmp::Ord> BisectSet<T> {
    pub fn contains<Q>(&self, target: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut left = 0;
        let mut right = self.items.len();
        while left != right {
            let pivot = (left + right) / 2;
            match Borrow::<Q>::borrow(&self.items[pivot]).cmp(target) {
                Ordering::Less => left = pivot + 1,
                Ordering::Greater => right = pivot,
                Ordering::Equal => return true,
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> BisectSet<&'static str> {
        BisectSet::from_iter(
            [
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
            .cloned(),
        )
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn bisect_test() {
        let allowed_list = fixture();
        assert!(allowed_list.contains("chain_containsTransaction"));
        assert!(allowed_list.contains("chain_executeTransaction"));
        assert!(allowed_list.contains("chain_executeVM"));
        assert!(allowed_list.contains("chain_getAsset"));
        assert!(allowed_list.contains("chain_getAssetSchemeByTracker"));
        assert!(allowed_list.contains("chain_getAssetSchemeByType"));
        assert!(allowed_list.contains("chain_getBalance"));
        assert!(allowed_list.contains("chain_getBestBlockId"));
        assert!(allowed_list.contains("chain_getBestBlockNumber"));
        assert!(allowed_list.contains("chain_getBlockByHash"));
        assert!(allowed_list.contains("chain_getBlockByNumber"));
        assert!(allowed_list.contains("chain_getBlockHash"));
        assert!(allowed_list.contains("chain_getBlockTransactionCountByHash"));
        assert!(allowed_list.contains("chain_getGenesisAccounts"));
        assert!(allowed_list.contains("chain_getMiningReward"));
        assert!(allowed_list.contains("chain_getNetworkId"));
        assert!(allowed_list.contains("chain_getNumberOfShards"));
        assert!(allowed_list.contains("chain_getRegularKey"));
        assert!(allowed_list.contains("chain_getRegularKeyOwner"));
        assert!(allowed_list.contains("chain_getSeq"));
        assert!(allowed_list.contains("chain_getShardIdByHash"));
        assert!(allowed_list.contains("chain_getShardOwners"));
        assert!(allowed_list.contains("chain_getShardRoot"));
        assert!(allowed_list.contains("chain_getShardUsers"));
        assert!(allowed_list.contains("chain_getText"));
        assert!(allowed_list.contains("chain_getTransaction"));
        assert!(allowed_list.contains("chain_getTransactionByTracker"));
        assert!(allowed_list.contains("chain_isAssetSpent"));
        assert!(allowed_list.contains("commitHash"));
        assert!(allowed_list.contains("engine_getBlockReward"));
        assert!(allowed_list.contains("engine_getCoinbase"));
        assert!(allowed_list.contains("engine_getCustomActionData"));
        assert!(allowed_list.contains("engine_getRecommendedConfimation"));
        assert!(allowed_list.contains("mempool_getErrorHint"));
        assert!(allowed_list.contains("mempool_getPendingTransactions"));
        assert!(allowed_list.contains("mempool_getPendingTransactionsCount"));
        assert!(allowed_list.contains("mempool_getTransactionResultsByTracker"));
        assert!(allowed_list.contains("mempool_sendSignedTransaction"));
        assert!(allowed_list.contains("ping"));
        assert!(allowed_list.contains("version"));
        assert!(!allowed_list.contains("non exist"));
    }
}
