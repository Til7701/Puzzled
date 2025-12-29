use std::ops::{BitAnd, BitOr, BitXor};

const BITS_IN_PRIMITIVE: usize = 128;
const BITMASK_ARRAY_LENGTH: usize = 1;

pub struct Bitmask {
    bits: [u128; BITMASK_ARRAY_LENGTH],
}

impl Bitmask {
    pub fn new() -> Self {
        Bitmask {
            bits: [0; BITMASK_ARRAY_LENGTH],
        }
    }

    fn or(a: &Bitmask, b: &Bitmask, output: &mut Bitmask) {
        match BITMASK_ARRAY_LENGTH {
            1 => {
                output.bits[0] = a.bits[0] | b.bits[0];
                return;
            }
            _ => {
                for i in 0..BITMASK_ARRAY_LENGTH {
                    output.bits[i] = a.bits[i] | b.bits[i];
                }
            }
        }
    }

    fn xor(a: &Bitmask, b: &Bitmask, output: &mut Bitmask) {
        match BITMASK_ARRAY_LENGTH {
            1 => {
                output.bits[0] = a.bits[0] ^ b.bits[0];
                return;
            }
            _ => {
                for i in 0..BITMASK_ARRAY_LENGTH {
                    output.bits[i] = a.bits[i] ^ b.bits[i];
                }
            }
        }
    }

    fn and(a: &Bitmask, b: &Bitmask, output: &mut Bitmask) {
        match BITMASK_ARRAY_LENGTH {
            1 => {
                output.bits[0] = a.bits[0] & b.bits[0];
                return;
            }
            _ => {
                for i in 0..BITMASK_ARRAY_LENGTH {
                    output.bits[i] = a.bits[i] & b.bits[i];
                }
            }
        }
    }

    fn set_bit(&mut self, index: usize) {
        let array_index = index / BITS_IN_PRIMITIVE;
        let bit_index = index % BITS_IN_PRIMITIVE;
        self.bits[array_index] |= 1 << bit_index;
    }

    fn clear_bit(&mut self, index: usize) {
        let array_index = index / BITS_IN_PRIMITIVE;
        let bit_index = index % BITS_IN_PRIMITIVE;
        self.bits[array_index] &= !(1 << bit_index);
    }

    fn and_is_zero(&self, other: &Bitmask) -> bool {
        match BITMASK_ARRAY_LENGTH {
            1 => (self.bits[0] & other.bits[0]) == 0,
            _ => {
                for i in 0..BITMASK_ARRAY_LENGTH {
                    if (self.bits[i] & other.bits[i]) != 0 {
                        return false;
                    }
                }
                true
            }
        }
    }

    fn count_ones(&self) -> u32 {
        let mut count = 0;
        for word in &self.bits {
            count += word.count_ones();
        }
        count
    }

    fn and_xor_count_ones(a: &Bitmask, b: &Bitmask, c: &Bitmask) -> u32 {
        match BITMASK_ARRAY_LENGTH {
            1 => ((a.bits[0] & b.bits[0]) ^ c.bits[0]).count_ones(),
            _ => {
                let mut count = 0;
                for i in 0..BITMASK_ARRAY_LENGTH {
                    count += (a.bits[i] & b.bits[i] ^ c.bits[i]).count_ones();
                }
                count
            }
        }
    }

    fn and_equals(a: &Bitmask, b: &Bitmask, c: &Bitmask) -> bool {
        match BITMASK_ARRAY_LENGTH {
            1 => (a.bits[0] & b.bits[0]) == c.bits[0],
            _ => {
                for i in 0..BITMASK_ARRAY_LENGTH {
                    if (a.bits[i] & b.bits[i]) != c.bits[i] {
                        return false;
                    }
                }
                true
            }
        }
    }
}

impl BitOr for Bitmask {
    type Output = Bitmask;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut output = Bitmask::new();
        Bitmask::or(&self, &rhs, &mut output);
        output
    }
}

impl BitXor for Bitmask {
    type Output = Bitmask;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut output = Bitmask::new();
        Bitmask::xor(&self, &rhs, &mut output);
        output
    }
}

impl BitAnd for Bitmask {
    type Output = Bitmask;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut output = Bitmask::new();
        Bitmask::and(&self, &rhs, &mut output);
        output
    }
}

impl Default for Bitmask {
    fn default() -> Self {
        Self::new()
    }
}

