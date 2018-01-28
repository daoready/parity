// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Bulk rpc implementation.

use std::thread;
use std::time::{Instant, Duration};
use std::sync::Arc;

use rlp::{self, UntrustedRlp};
use time::get_time;
use bigint::prelude::U256;
use bigint::hash::{H64, H160, H256};
use util::Address;
use parking_lot::Mutex;

use ethash::SeedHashCompute;
use ethcore::account_provider::{AccountProvider, DappId};
use ethcore::block::IsBlock;
use ethcore::client::{MiningBlockChainClient, BlockId, TransactionId, UncleId};
use ethcore::ethereum::Ethash;
use ethcore::filter::Filter as EthcoreFilter;
use ethcore::header::{Header as BlockHeader, BlockNumber as EthBlockNumber};
use ethcore::log_entry::LogEntry;
use ethcore::miner::{MinerService, ExternalMinerService};
use ethcore::transaction::SignedTransaction;
use ethcore::snapshot::SnapshotService;
use ethsync::{SyncProvider};

use jsonrpc_core::{BoxFuture, Error};
use jsonrpc_core::futures::future;
use jsonrpc_macros::Trailing;

use v1::helpers::{errors, limit_logs, fake_sign};
use v1::helpers::dispatch::{FullDispatcher, default_gas_price};
use v1::helpers::block_import::is_major_importing;
use v1::helpers::accounts::unwrap_provider;
use v1::traits::Bulk;
use v1::types::{
	RichBlock, Block, BlockTransactions, BlockNumber, Bytes, SyncStatus, SyncInfo, BlockWithTransactions,
	Transaction, CallRequest, Index, Filter, Log, Receipt, Work,
	H64 as RpcH64, H256 as RpcH256, H160 as RpcH160, U256 as RpcU256,
};
use v1::metadata::Metadata;


/// Bulk rpc implementation.
pub struct BulkClient<C> where
	C: MiningBlockChainClient {
	client: Arc<C>,
	eip86_transition: u64
}

impl<C> BulkClient<C> where
	C: MiningBlockChainClient {

	/// Creates new BulkClient.
	pub fn new(
		client: &Arc<C>
	) -> Self {
		BulkClient {
			client: client.clone(),
			eip86_transition: client.eip86_transition()
		}
	}


	fn block(&self, id: BlockId) -> Result<Option<BlockWithTransactions>, Error> {
		let client = &self.client;
		match (client.block(id.clone()), client.block_total_difficulty(id)) {
			(Some(block), Some(total_difficulty)) => {
				let view = block.header_view();
				Ok(Some(BlockWithTransactions {
						hash: Some(view.hash().into()),
						size: Some(block.rlp().as_raw().len().into()),
						parent_hash: view.parent_hash().into(),
						uncles_hash: view.uncles_hash().into(),
						author: view.author().into(),
						miner: view.author().into(),
						state_root: view.state_root().into(),
						transactions_root: view.transactions_root().into(),
						receipts_root: view.receipts_root().into(),
						number: Some(view.number().into()),
						gas_used: view.gas_used().into(),
						gas_limit: view.gas_limit().into(),
						logs_bloom: view.log_bloom().into(),
						timestamp: view.timestamp().into(),
						difficulty: view.difficulty().into(),
						total_difficulty: Some(total_difficulty.into()),
						seal_fields: view.seal().into_iter().map(Into::into).collect(),
						uncles: block.uncle_hashes().into_iter().map(Into::into).collect(),
						transactions: block.view().localized_transactions().into_iter().map(|t| Transaction::from_localized(t, self.eip86_transition)).collect(),
						extra_data: Bytes::new(view.extra_data()),
					}
				))
			},
			_ => Ok(None)
		}
	}

}

impl<C> Bulk for BulkClient<C> where
	C: MiningBlockChainClient + 'static
{
	type Metadata = Metadata;

	fn block_by_number(&self, num: BlockNumber) -> BoxFuture<Option<BlockWithTransactions>, Error> {
		Box::new(future::done(self.block(num.into())))
	}


}
