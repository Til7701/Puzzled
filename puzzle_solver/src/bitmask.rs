use ndarray::Array2;
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

    pub fn or(&mut self, a: &Bitmask, b: &Bitmask) {
        match BITMASK_ARRAY_LENGTH {
            1 => {
                self.bits[0] = a.bits[0] | b.bits[0];
                return;
            }
            _ => {
                for i in 0..BITMASK_ARRAY_LENGTH {
                    self.bits[i] = a.bits[i] | b.bits[i];
                }
            }
        }
    }

    pub fn xor(&mut self, a: &Bitmask, b: &Bitmask) {
        match BITMASK_ARRAY_LENGTH {
            1 => {
                self.bits[0] = a.bits[0] ^ b.bits[0];
                return;
            }
            _ => {
                for i in 0..BITMASK_ARRAY_LENGTH {
                    self.bits[i] = a.bits[i] ^ b.bits[i];
                }
            }
        }
    }

    pub fn and(&mut self, a: &Bitmask, b: &Bitmask) {
        match BITMASK_ARRAY_LENGTH {
            1 => {
                self.bits[0] = a.bits[0] & b.bits[0];
                return;
            }
            _ => {
                for i in 0..BITMASK_ARRAY_LENGTH {
                    self.bits[i] = a.bits[i] & b.bits[i];
                }
            }
        }
    }

    pub fn set_bit(&mut self, index: usize) {
        let array_index = index / BITS_IN_PRIMITIVE;
        let bit_index = index % BITS_IN_PRIMITIVE;
        self.bits[array_index] |= 1 << bit_index;
    }

    pub fn clear_bit(&mut self, index: usize) {
        let array_index = index / BITS_IN_PRIMITIVE;
        let bit_index = index % BITS_IN_PRIMITIVE;
        self.bits[array_index] &= !(1 << bit_index);
    }

    pub fn and_is_zero(&self, other: &Bitmask) -> bool {
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

    pub fn count_ones(&self) -> u32 {
        let mut count = 0;
        for word in &self.bits {
            count += word.count_ones();
        }
        count
    }

    pub fn and_xor_count_ones(a: &Bitmask, b: &Bitmask, c: &Bitmask) -> u32 {
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

    pub fn and_equals(a: &Bitmask, b: &Bitmask, c: &Bitmask) -> bool {
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
        Bitmask::or(&mut output, &rhs, &self);
        output
    }
}

impl BitXor for Bitmask {
    type Output = Bitmask;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut output = Bitmask::new();
        Bitmask::xor(&mut output, &rhs, &self);
        output
    }
}

impl BitAnd for Bitmask {
    type Output = Bitmask;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut output = Bitmask::new();
        Bitmask::and(&mut output, &rhs, &self);
        output
    }
}

impl Default for Bitmask {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&Array2<bool>> for Bitmask {
    fn from(value: &Array2<bool>) -> Self {
        let mut bitmask = Bitmask::new();
        let (rows, cols) = value.dim();
        for r in 0..rows {
            for c in 0..cols {
                if value[[r, c]] {
                    let index = r * cols + c;
                    bitmask.set_bit(index);
                }
            }
        }
        bitmask
    }
}

impl Clone for Bitmask {
    fn clone(&self) -> Self {
        let mut new_bitmask = Bitmask::new();
        for i in 0..BITMASK_ARRAY_LENGTH {
            new_bitmask.bits[i] = self.bits[i];
        }
        new_bitmask
    }
}
