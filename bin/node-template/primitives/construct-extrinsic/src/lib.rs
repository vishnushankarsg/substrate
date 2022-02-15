// This file is part of Substrate.

// Copyright (C) 2019-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

use sp_core::H256;

pub use frame_support::{
	traits::ConstU32, BoundedVec
};

sp_api::decl_runtime_apis! {
	pub trait ConstructExtrinsicApi {
		/// For submission of the processing result of the OE by the EWF Logic Provider library
		fn submit_processing_result_hash(
			hash: H256
		) -> Result<(), ()>;

		// For getting the resulting hash. This shold be called by the aggregator node
		fn get_processing_result_hash() -> Option<H256>;
	}
}
