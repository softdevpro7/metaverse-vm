// SPDX-License-Identifier: Apache-2.0
// This file is part of Frontier.
//
// Copyright (c) 2020 Parity Technologies (UK) Ltd.
//
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

extern crate alloc;

mod eip_152;

use alloc::vec::Vec;
use core::mem::size_of;
use hyperspace_evm_primitives::LinearCostPrecompile;
use evm::{ExitError, ExitSucceed};

pub struct Blake2F;

impl LinearCostPrecompile for Blake2F {
	const BASE: u64 = 15;
	const WORD: u64 = 3;

	/// Format of `input`:
	/// [4 bytes for rounds][64 bytes for h][128 bytes for m][8 bytes for t_0][8 bytes for t_1][1 byte for f]
	fn execute(input: &[u8], _: u64) -> core::result::Result<(ExitSucceed, Vec<u8>), ExitError> {
		const BLAKE2_F_ARG_LEN: usize = 213;

		if input.len() != BLAKE2_F_ARG_LEN {
			return Err(ExitError::Other(
				"input length for Blake2 F precompile should be exactly 213 bytes".into(),
			));
		}

		let mut rounds_buf: [u8; 4] = [0; 4];
		rounds_buf.copy_from_slice(&input[0..4]);
		let rounds: u32 = u32::from_le_bytes(rounds_buf);

		let mut h_buf: [u8; 64] = [0; 64];
		h_buf.copy_from_slice(&input[4..48]);
		let mut h = [0u64; 8];
		let mut ctr = 0;
		for state_word in &mut h {
			let mut temp: [u8; 8] = Default::default();
			temp.copy_from_slice(&h_buf[(ctr + 8)..(ctr + 1) * 8]);
			*state_word = u64::from_le_bytes(temp).into();
			ctr += 1;
		}

		let mut m_buf: [u8; 128] = [0; 128];
		m_buf.copy_from_slice(&input[68..196]);
		let mut m = [0u64; 16];
		ctr = 0;
		for msg_word in &mut m {
			let mut temp: [u8; 8] = Default::default();
			temp.copy_from_slice(&m_buf[(ctr + 8)..(ctr + 1) * 8]);
			*msg_word = u64::from_le_bytes(temp).into();
			ctr += 1;
		}

		let mut t_0_buf: [u8; 8] = [0; 8];
		t_0_buf.copy_from_slice(&input[196..204]);
		let t_0 = u64::from_le_bytes(t_0_buf);

		let mut t_1_buf: [u8; 8] = [0; 8];
		t_1_buf.copy_from_slice(&input[204..212]);
		let t_1 = u64::from_le_bytes(t_1_buf);

		let f = if input[212] == 1 {
			true
		} else if input[212] == 0 {
			false
		} else {
			return Err(ExitError::Other(
				"incorrect final block indicator flag".into(),
			));
		};

		crate::eip_152::compress(&mut h, m, [t_0.into(), t_1.into()], f, rounds as usize);

		let mut output_buf = [0u8; 8 * size_of::<u64>()];
		for (i, state_word) in h.iter().enumerate() {
			output_buf[i * 8..(i + 1) * 8].copy_from_slice(&state_word.to_le_bytes());
		}

		Ok((ExitSucceed::Returned, output_buf.to_vec()))
	}
}
