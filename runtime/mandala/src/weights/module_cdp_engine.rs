// This file is part of Acala.

// Copyright (C) 2020-2021 Acala Foundation.
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

//! Autogenerated weights for module_cdp_engine
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-16, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/acala
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=module_cdp_engine
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/mandala/src/weights/


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_cdp_engine.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_cdp_engine::WeightInfo for WeightInfo<T> {
	fn on_initialize(c: u32, ) -> Weight {
		(48_807_000 as Weight)
			// Standard Error: 1_035_000
			.saturating_add((46_350_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(c as Weight)))
	}
	fn set_collateral_params() -> Weight {
		(56_882_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_global_params() -> Weight {
		(19_257_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn liquidate_by_auction(b: u32, ) -> Weight {
		(233_927_000 as Weight)
			// Standard Error: 161_000
			.saturating_add((30_664_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(23 as Weight))
			.saturating_add(T::DbWeight::get().writes(15 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(b as Weight)))
	}
	fn liquidate_by_dex() -> Weight {
		(381_589_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(27 as Weight))
			.saturating_add(T::DbWeight::get().writes(15 as Weight))
	}
	fn settle() -> Weight {
		(162_567_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
}
