use std::mem::size_of;

const BYTE_TO_BIT: usize = 8;

pub struct BlockParam {
    pub block_size_bits: usize,
    pub fingerprint_size_bits: usize,
}

#[derive(Debug)]
pub struct Block {
    atom_size_bits: usize,
    atom_per_block: usize,
    fingerprint_per_atom: usize,
    fingerprint_size_bits: usize,
    read_mask: u64,
    _block_store: Vec<u64>,
}

impl Block {
    pub fn new(params: BlockParam) -> Block {
        let atom_size_bits = BYTE_TO_BIT * (size_of::<u64>() as usize);
        let atom_per_block = params.block_size_bits / atom_size_bits;
        let fingerprint_per_atom = atom_size_bits / params.fingerprint_size_bits;

        return Block {
            atom_size_bits,
            atom_per_block,
            fingerprint_per_atom,
            fingerprint_size_bits: params.fingerprint_size_bits,
            read_mask: (1 as u64) << params.fingerprint_size_bits - 1,
            _block_store: Vec::with_capacity(atom_per_block as usize),
        };
    }

    pub fn at(&self, atom_index: usize) -> u64 {
        return self._block_store[atom_index];
    }

    pub fn add(&mut self, index: usize, item: u64) {
        let atom_index: usize = index / self.fingerprint_per_atom;
        let atom_bit_offset = (index % self.fingerprint_per_atom) * self.fingerprint_size_bits;

        self._block_store[atom_index] &= !(self.read_mask << atom_bit_offset);
        self._block_store[atom_index] |= item << atom_bit_offset;
    }
}
