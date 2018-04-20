
use std::ops::Deref;
use std::collections::BTreeMap;

use ethcore::encoded::Header as EthHeader;

use serde::{Serialize, Serializer};
use serde::ser::Error;
use v1::types::{Bytes, Transaction, H160, H256, H2048, U256, Receipt, LocalizedTrace};

#[derive(Debug, Serialize)]
pub struct TransactionWithReceipt {
	pub transaction: Transaction,
	pub receipt: Receipt,
	pub traces: Vec<LocalizedTrace>
}

/// Block representation
#[derive(Debug, Serialize)]
pub struct BlockWithTransactions {
	/// Hash of the block
	pub hash: Option<H256>,
	/// Hash of the parent
	#[serde(rename="parentHash")]
	pub parent_hash: H256,
	/// Hash of the uncles
	#[serde(rename="sha3Uncles")]
	pub uncles_hash: H256,
	/// Authors address
	pub author: H160,
	// TODO: get rid of this one
	/// ?
	pub miner: H160,
	/// State root hash
	#[serde(rename="stateRoot")]
	pub state_root: H256,
	/// Transactions root hash
	#[serde(rename="transactionsRoot")]
	pub transactions_root: H256,
	/// Transactions receipts root hash
	#[serde(rename="receiptsRoot")]
	pub receipts_root: H256,
	/// Block number
	pub number: Option<U256>,
	/// Gas Used
	#[serde(rename="gasUsed")]
	pub gas_used: U256,
	/// Gas Limit
	#[serde(rename="gasLimit")]
	pub gas_limit: U256,
	/// Extra data
	#[serde(rename="extraData")]
	pub extra_data: Bytes,
	/// Logs bloom
	#[serde(rename="logsBloom")]
	pub logs_bloom: H2048,
	/// Timestamp
	pub timestamp: U256,
	/// Difficulty
	pub difficulty: U256,
	/// Total difficulty
	#[serde(rename="totalDifficulty")]
	pub total_difficulty: Option<U256>,
	/// Seal fields
	#[serde(rename="sealFields")]
	pub seal_fields: Vec<Bytes>,
	/// Uncles' hashes
	pub uncles: Vec<H256>,
	/// Transactions
	pub transactions: Vec<TransactionWithReceipt>,
	/// Size in bytes
	pub size: Option<U256>,
}
