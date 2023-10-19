use std::mem::size_of;

use block::BlockParam;

use crate::block::Block;

pub mod block;

fn main() {
    let params = BlockParam {
        block_size_bits: 32,
        fingerprint_size_bits: 8,
    };
    let block: Block = Block::new(params);

    println!("{:?}", block);
    println!("size of u8 {}", size_of::<u8>());
}
