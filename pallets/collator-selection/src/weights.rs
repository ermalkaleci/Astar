
// This file is part of Astar.

// Copyright (C) Stake Technologies Pte.Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// Astar is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Astar is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Astar. If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_collator_selection
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-06-26, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `gh-runner-01-ovh`, CPU: `Intel(R) Xeon(R) E-2236 CPU @ 3.40GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("shiden-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/astar-collator
// benchmark
// pallet
// --chain=shiden-dev
// --steps=50
// --repeat=20
// --pallet=pallet-collator-selection
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./benchmark-results/shiden-dev/pallet-collator-selection_weights.rs
// --template=./scripts/templates/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_collator_selection.
pub trait WeightInfo {
	fn set_invulnerables(b: u32, ) -> Weight;
	fn set_desired_candidates() -> Weight;
	fn set_candidacy_bond() -> Weight;
	fn register_as_candidate(c: u32, ) -> Weight;
	fn leave_intent(c: u32, ) -> Weight;
	fn withdraw_bond() -> Weight;
	fn note_author() -> Weight;
	fn new_session(r: u32, c: u32, ) -> Weight;
}

/// Weights for pallet_collator_selection using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Session::NextKeys` (r:48 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::Invulnerables` (r:0 w:1)
	/// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[1, 48]`.
	fn set_invulnerables(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `204 + b * (79 ±0)`
		//  Estimated: `1194 + b * (2554 ±0)`
		// Minimum execution time: 11_169_000 picoseconds.
		Weight::from_parts(11_917_783, 1194)
			// Standard Error: 4_757
			.saturating_add(Weight::from_parts(2_433_455, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 2554).saturating_mul(b.into()))
	}
	/// Storage: `CollatorSelection::DesiredCandidates` (r:0 w:1)
	/// Proof: `CollatorSelection::DesiredCandidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_desired_candidates() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_229_000 picoseconds.
		Weight::from_parts(5_400_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `CollatorSelection::CandidacyBond` (r:0 w:1)
	/// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_candidacy_bond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_266_000 picoseconds.
		Weight::from_parts(5_521_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `CollatorSelection::Candidates` (r:1 w:1)
	/// Proof: `CollatorSelection::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::DesiredCandidates` (r:1 w:0)
	/// Proof: `CollatorSelection::DesiredCandidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	/// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `DappStaking::Ledger` (r:1 w:0)
	/// Proof: `DappStaking::Ledger` (`max_values`: None, `max_size`: Some(310), added: 2785, mode: `MaxEncodedLen`)
	/// Storage: `Session::NextKeys` (r:1 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::NonCandidates` (r:1 w:1)
	/// Proof: `CollatorSelection::NonCandidates` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::CandidacyBond` (r:1 w:0)
	/// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:1)
	/// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[1, 148]`.
	fn register_as_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `962 + c * (50 ±0)`
		//  Estimated: `4307 + c * (51 ±0)`
		// Minimum execution time: 38_433_000 picoseconds.
		Weight::from_parts(40_514_505, 4307)
			// Standard Error: 539
			.saturating_add(Weight::from_parts(48_079, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 51).saturating_mul(c.into()))
	}
	/// Storage: `CollatorSelection::Candidates` (r:1 w:1)
	/// Proof: `CollatorSelection::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::NonCandidates` (r:0 w:1)
	/// Proof: `CollatorSelection::NonCandidates` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[6, 148]`.
	fn leave_intent(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `734 + c * (48 ±0)`
		//  Estimated: `2178 + c * (49 ±0)`
		// Minimum execution time: 16_910_000 picoseconds.
		Weight::from_parts(17_595_269, 2178)
			// Standard Error: 442
			.saturating_add(Weight::from_parts(38_279, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 49).saturating_mul(c.into()))
	}
	/// Storage: `CollatorSelection::NonCandidates` (r:1 w:1)
	/// Proof: `CollatorSelection::NonCandidates` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:1)
	/// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn withdraw_bond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `725`
		//  Estimated: `4190`
		// Minimum execution time: 28_804_000 picoseconds.
		Weight::from_parts(29_066_000, 4190)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:1)
	/// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn note_author() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `226`
		//  Estimated: `6196`
		// Minimum execution time: 35_307_000 picoseconds.
		Weight::from_parts(35_803_000, 6196)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `CollatorSelection::Candidates` (r:1 w:0)
	/// Proof: `CollatorSelection::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::LastAuthoredBlock` (r:149 w:0)
	/// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	/// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:143 w:143)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::SlashDestination` (r:1 w:0)
	/// Proof: `CollatorSelection::SlashDestination` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::NonCandidates` (r:0 w:143)
	/// Proof: `CollatorSelection::NonCandidates` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `r` is `[1, 148]`.
	/// The range of component `c` is `[1, 148]`.
	fn new_session(r: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4554 + c * (97 ±0) + r * (112 ±0)`
		//  Estimated: `6296 + c * (2637 ±0) + r * (2599 ±0)`
		// Minimum execution time: 20_506_000 picoseconds.
		Weight::from_parts(20_755_000, 6296)
			// Standard Error: 503_081
			.saturating_add(Weight::from_parts(16_868_215, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2637).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 2599).saturating_mul(r.into()))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `Session::NextKeys` (r:48 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::Invulnerables` (r:0 w:1)
	/// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[1, 48]`.
	fn set_invulnerables(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `204 + b * (79 ±0)`
		//  Estimated: `1194 + b * (2554 ±0)`
		// Minimum execution time: 11_169_000 picoseconds.
		Weight::from_parts(11_917_783, 1194)
			// Standard Error: 4_757
			.saturating_add(Weight::from_parts(2_433_455, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(b.into())))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 2554).saturating_mul(b.into()))
	}
	/// Storage: `CollatorSelection::DesiredCandidates` (r:0 w:1)
	/// Proof: `CollatorSelection::DesiredCandidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_desired_candidates() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_229_000 picoseconds.
		Weight::from_parts(5_400_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `CollatorSelection::CandidacyBond` (r:0 w:1)
	/// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_candidacy_bond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_266_000 picoseconds.
		Weight::from_parts(5_521_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `CollatorSelection::Candidates` (r:1 w:1)
	/// Proof: `CollatorSelection::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::DesiredCandidates` (r:1 w:0)
	/// Proof: `CollatorSelection::DesiredCandidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	/// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `DappStaking::Ledger` (r:1 w:0)
	/// Proof: `DappStaking::Ledger` (`max_values`: None, `max_size`: Some(310), added: 2785, mode: `MaxEncodedLen`)
	/// Storage: `Session::NextKeys` (r:1 w:0)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::NonCandidates` (r:1 w:1)
	/// Proof: `CollatorSelection::NonCandidates` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::CandidacyBond` (r:1 w:0)
	/// Proof: `CollatorSelection::CandidacyBond` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:1)
	/// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[1, 148]`.
	fn register_as_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `962 + c * (50 ±0)`
		//  Estimated: `4307 + c * (51 ±0)`
		// Minimum execution time: 38_433_000 picoseconds.
		Weight::from_parts(40_514_505, 4307)
			// Standard Error: 539
			.saturating_add(Weight::from_parts(48_079, 0).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(Weight::from_parts(0, 51).saturating_mul(c.into()))
	}
	/// Storage: `CollatorSelection::Candidates` (r:1 w:1)
	/// Proof: `CollatorSelection::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::NonCandidates` (r:0 w:1)
	/// Proof: `CollatorSelection::NonCandidates` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[6, 148]`.
	fn leave_intent(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `734 + c * (48 ±0)`
		//  Estimated: `2178 + c * (49 ±0)`
		// Minimum execution time: 16_910_000 picoseconds.
		Weight::from_parts(17_595_269, 2178)
			// Standard Error: 442
			.saturating_add(Weight::from_parts(38_279, 0).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 49).saturating_mul(c.into()))
	}
	/// Storage: `CollatorSelection::NonCandidates` (r:1 w:1)
	/// Proof: `CollatorSelection::NonCandidates` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:1)
	/// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn withdraw_bond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `725`
		//  Estimated: `4190`
		// Minimum execution time: 28_804_000 picoseconds.
		Weight::from_parts(29_066_000, 4190)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::LastAuthoredBlock` (r:0 w:1)
	/// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn note_author() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `226`
		//  Estimated: `6196`
		// Minimum execution time: 35_307_000 picoseconds.
		Weight::from_parts(35_803_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `CollatorSelection::Candidates` (r:1 w:0)
	/// Proof: `CollatorSelection::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::LastAuthoredBlock` (r:149 w:0)
	/// Proof: `CollatorSelection::LastAuthoredBlock` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::Invulnerables` (r:1 w:0)
	/// Proof: `CollatorSelection::Invulnerables` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:143 w:143)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `CollatorSelection::SlashDestination` (r:1 w:0)
	/// Proof: `CollatorSelection::SlashDestination` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorSelection::NonCandidates` (r:0 w:143)
	/// Proof: `CollatorSelection::NonCandidates` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `r` is `[1, 148]`.
	/// The range of component `c` is `[1, 148]`.
	fn new_session(r: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4554 + c * (97 ±0) + r * (112 ±0)`
		//  Estimated: `6296 + c * (2637 ±0) + r * (2599 ±0)`
		// Minimum execution time: 20_506_000 picoseconds.
		Weight::from_parts(20_755_000, 6296)
			// Standard Error: 503_081
			.saturating_add(Weight::from_parts(16_868_215, 0).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2637).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 2599).saturating_mul(r.into()))
	}
}
