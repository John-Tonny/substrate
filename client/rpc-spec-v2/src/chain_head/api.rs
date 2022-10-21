// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! API trait of the chain head.
use crate::chain_head::event::{ChainHeadEvent, FollowEvent};
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use sc_client_api::StorageKey;
use sp_core::Bytes;

#[rpc(client, server)]
pub trait ChainHeadApi<Hash> {
	/// Track the state of the head of the chain: the finalized, non-finalized, and best blocks.
	#[subscription(
		name = "chainHead_unstable_follow" => "chainHead_unstable_followBlock",
		unsubscribe = "chainHead_unstable_unfollow",
		item = FollowEvent<Hash>,
	)]
	fn chain_head_unstable_follow(&self, runtime_updates: bool);

	/// Retrieves the body (list of transactions) of a pinned block.
	///
	/// This method should be seen as a complement to `chainHead_unstable_follow`,
	/// allowing the JSON-RPC client to retrieve more information about a block
	/// that has been reported.
	///
	/// Use `archive_unstable_body` if instead you want to retrieve the body of an arbitrary block.
	#[subscription(
		name = "chainHead_unstable_body" => "chainHead_unstable_getBody",
		unsubscribe = "chainHead_unstable_stopBody",
		item = ChainHeadEvent<String>,
	)]
	fn chain_head_unstable_body(
		&self,
		follow_subscription: String,
		hash: Hash,
		network_config: Option<()>,
	);

	/// Retrieves the header of a pinned block.
	///
	/// This method should be seen as a complement to `chainHead_unstable_follow`,
	/// allowing the JSON-RPC client to retrieve more information about a block
	/// that has been reported.
	///
	/// Use `archive_unstable_header` if instead you want to retrieve the header of an arbitrary
	/// block.
	#[method(name = "chainHead_unstable_header", aliases = ["chainHead_unstable_getHeader"], blocking)]
	fn chain_head_unstable_header(
		&self,
		follow_subscription: String,
		hash: Hash,
	) -> RpcResult<Option<String>>;

	/// Return a storage entry at a specific block's state.
	#[subscription(
		name = "chainHead_unstable_storage" => "chainHead_unstable_queryStorage",
		unsubscribe = "chainHead_unstable_stopStorage",
		item = ChainHeadEvent<String>,
	)]
	fn chain_head_unstable_storage(
		&self,
		follow_subscription: String,
		hash: Hash,
		key: StorageKey,
		child_key: Option<StorageKey>,
		network_config: Option<()>,
	);

	/// Call into the Runtime API at a specified block's state.
	#[subscription(
		name = "chainHead_unstable_call" => "chainHead_unstable_runtimeCall",
		unsubscribe = "chainHead_unstable_stopCall",
		item = ChainHeadEvent<String>,
	)]
	fn chain_head_unstable_call(
		&self,
		follow_subscription: String,
		hash: Hash,
		function: String,
		call_parameters: Bytes,
		network_config: Option<()>,
	);

	/// Unpin a block reported by the `follow` method.
	///
	/// Ongoing operations that require the provided block
	/// will continue normally.
	#[method(name = "chainHead_unstable_unpin", aliases = ["chainHead_unstable_unpinBlock"], blocking)]
	fn chain_head_unstable_unpin(&self, follow_subscription: String, hash: Hash) -> RpcResult<()>;
}
