//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0-rc6

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

impl crate::WeightInfo for () {
	fn propose_spend() -> Weight {
		(79604000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn reject_proposal() -> Weight {
		(61001000 as Weight)
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn approve_proposal() -> Weight {
		(17835000 as Weight)
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn report_awesome(r: u32) -> Weight {
		(101602000 as Weight)
			.saturating_add((2000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn retract_tip() -> Weight {
		(82970000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn tip_new(r: u32, t: u32) -> Weight {
		(63995000 as Weight)
			.saturating_add((2000 as Weight).saturating_mul(r as Weight))
			.saturating_add((153000 as Weight).saturating_mul(t as Weight))
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn tip(t: u32) -> Weight {
		(46765000 as Weight)
			.saturating_add((711000 as Weight).saturating_mul(t as Weight))
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn close_tip(t: u32) -> Weight {
		(160874000 as Weight)
			.saturating_add((379000 as Weight).saturating_mul(t as Weight))
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	fn propose_bounty(d: u32) -> Weight {
		(86198000 as Weight)
			.saturating_add((1000 as Weight).saturating_mul(d as Weight))
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(4 as Weight))
	}
	fn approve_bounty() -> Weight {
		(23063000 as Weight)
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn propose_curator() -> Weight {
		(18890000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn unassign_curator() -> Weight {
		(66768000 as Weight)
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn accept_curator() -> Weight {
		(69131000 as Weight)
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn award_bounty() -> Weight {
		(48184000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn claim_bounty() -> Weight {
		(243104000 as Weight)
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().writes(5 as Weight))
	}
	fn close_bounty_proposed() -> Weight {
		(65917000 as Weight)
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	fn close_bounty_active() -> Weight {
		(157232000 as Weight)
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(4 as Weight))
	}
	fn extend_bounty_expiry() -> Weight {
		(46216000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn on_initialize_proposals(p: u32) -> Weight {
		(119765000 as Weight)
			.saturating_add((108368000 as Weight).saturating_mul(p as Weight))
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().reads((3 as Weight).saturating_mul(p as Weight)))
			.saturating_add(DbWeight::get().writes(2 as Weight))
			.saturating_add(DbWeight::get().writes((3 as Weight).saturating_mul(p as Weight)))
	}
	fn on_initialize_bounties(b: u32) -> Weight {
		(112536000 as Weight)
			.saturating_add((107132000 as Weight).saturating_mul(b as Weight))
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().reads((3 as Weight).saturating_mul(b as Weight)))
			.saturating_add(DbWeight::get().writes(2 as Weight))
			.saturating_add(DbWeight::get().writes((3 as Weight).saturating_mul(b as Weight)))
	}
}
