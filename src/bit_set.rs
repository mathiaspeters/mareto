#[derive(Debug, Clone)]
pub struct BitSet {
    pub size: usize,
    pub chunks: Vec<u8>,
}

impl BitSet {
    pub fn new() -> Self {
        Self {
            chunks: Vec::new(),
            size: 0,
        }
    }

    pub fn resize(&mut self, new_size: usize) {
        self.size = new_size;
        self.chunks.resize(new_size.div_ceil(8), 0);
    }

    pub fn set_bit(&mut self, bit_index: usize) {
        if bit_index >= self.size {
            return;
        }
        let chunk = bit_index / 8;
        let bit = bit_index % 8;
        let mask = 1_u8 << bit;
        self.chunks[chunk] |= mask;
    }

    pub fn unset_bit(&mut self, bit_index: usize) {
        if bit_index >= self.size {
            return;
        }
        let chunk = bit_index / 8;
        let bit = bit_index % 8;
        let mask = 1_u8 << bit;
        let mask = !mask;
        self.chunks[chunk] &= mask;
    }

    pub fn is_bit_set(&self, bit_index: usize) -> bool {
        if bit_index >= self.size {
            return false;
        }
        let chunk = bit_index / 8;
        let bit = bit_index % 8;
        (self.chunks[chunk] & (1 << bit)) > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(1, 1)]
    #[case(8, 1)]
    #[case(9, 2)]
    #[case(16, 2)]
    #[case(17, 3)]
    fn test_resize(#[case] bits: usize, #[case] bytes: usize) {
        let mut bit_set = BitSet::new();
        bit_set.resize(bits);
        assert_eq!(bit_set.size, bits);
        assert_eq!(bit_set.chunks.len(), bytes);
    }

    #[rstest]
    #[case(0, vec![], vec![])]
    #[case(8, vec![], vec![0])]
    #[case(16, vec![], vec![0, 0])]
    #[case(24, vec![], vec![0, 0, 0])]
    #[case(6, vec![(7, true), (8, true)], vec![0])]
    #[case(8, vec![(1, true)], vec![0b00000010])]
    #[case(16, vec![(9, true), (11, true)], vec![0, 0b00001010])]
    #[case(24, vec![(1, true), (23, true) ], vec![0b00000010, 0, 0b10000000])]
    #[case(8, vec![(1, true), (2, true), (3, true), (2, false)], vec![0b00001010])]
    fn test_set_bit(
        #[case] size: usize,
        #[case] bits_to_set: Vec<(usize, bool)>,
        #[case] expected: Vec<u8>,
    ) {
        let mut bit_set = BitSet::new();
        bit_set.resize(size);
        for (bit, value) in bits_to_set {
            if value {
                bit_set.set_bit(bit);
            } else {
                bit_set.unset_bit(bit);
            }
        }
        assert_eq!(bit_set.chunks, expected);
    }

    #[rstest]
    #[case(vec![], 0, vec![])]
    #[case(vec![0], 8, vec![])]
    #[case(vec![0, 0], 16, vec![])]
    #[case(vec![0, 0, 0], 24, vec![])]
    #[case(vec![0b10000000], 8, vec![7])]
    #[case(vec![0, 0b00010000], 16, vec![12])]
    #[case(vec![0, 0, 0b00000010], 24, vec![17])]
    #[case(vec![0b00001110], 8, vec![1,2,3])]
    #[case(vec![0, 0b11100000], 16, vec![13,14,15])]
    #[case(vec![0b10000001, 0b10000001, 0b10000001], 24, vec![0, 7, 8, 15, 16, 23])]
    #[case(vec![0b10000010], 7, vec![1])]
    #[case(vec![0b00100001, 0b11111111], 9, vec![0, 5, 8])]
    #[case(vec![0b00000001, 0, 0b11111111], 16, vec![0])]
    fn test_is_bit_set(#[case] chunks: Vec<u8>, #[case] size: usize, #[case] expected: Vec<usize>) {
        let total_bits = chunks.len() * 8;
        let bit_set = BitSet { chunks, size };
        let actual = (0..total_bits)
            .filter(|i| bit_set.is_bit_set(*i))
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }
}
