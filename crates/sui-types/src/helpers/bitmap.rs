// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// 
// Bitmap Helper functions
// 

pub fn bitmap_set_one<const BYTE_LENGTH: usize>(bitmap: &mut [u8; BYTE_LENGTH], index: usize) {
    let byte = index / 8;
    let bit = index - (byte * 8);
    bitmap[byte] |= 0b10000000 >> bit
}

pub fn bitmap_set_zero<const BYTE_LENGTH: usize>(bitmap: &mut [u8; BYTE_LENGTH], index: usize) {
    let byte: usize = index / 8;
    let bit: usize = index - (byte * 8);
    bitmap[byte] &= !(0b10000000 >> bit)
}

pub fn bitmap_get<const BYTE_LENGTH: usize>(bitmap: &[u8; BYTE_LENGTH], index: usize) -> bool {
    let byte: usize = index / 8;
    let bit: usize = index - (byte * 8);
    (bitmap[byte] & (0b10000000u8 >> bit as u8)) != 0
}

pub fn empty_bitmap<const BYTE_LENGTH: usize>() -> [u8; BYTE_LENGTH]{
    [0; BYTE_LENGTH]
}

pub fn bitmap_len<const BYTE_LENGTH: usize>(bitmap: &[u8; BYTE_LENGTH]) -> usize {
    return BYTE_LENGTH * 8;
}