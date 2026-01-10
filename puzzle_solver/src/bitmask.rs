use ndarray::Array2;
use std::fmt::{Display, Formatter};
use std::ops::{BitAnd, BitOr, BitXor, Index};

const BITS_IN_PRIMITIVE: usize = 128;
const BITMASK_ARRAY_LENGTH: usize = 1;

const TOTAL_BITS: usize = BITMASK_ARRAY_LENGTH * BITS_IN_PRIMITIVE;

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

    pub fn to_string(&self, board_width: i32) -> String {
        let mut output = String::new();
        for bit_index in 0..TOTAL_BITS {
            if bit_index as i32 % board_width == 0 && bit_index != 0 {
                output.push('_');
            }
            let bit_set = self[bit_index];
            let symbol = if bit_set { '1' } else { '0' };
            output.push(symbol);
        }
        output

        // let mut output = String::new();
        // let bits_per_row = board_width as usize;
        // for row_start in (0..TOTAL_BITS).step_by(bits_per_row) {
        //     for col in 0..bits_per_row {
        //         let index = row_start + col;
        //         let bit_set = self[index];
        //         let symbol = if bit_set { '1' } else { '0' };
        //         output.push(symbol);
        //     }
        //     output.push('\n');
        // }
        // output
    }

    pub fn fmt(&self, f: &mut Formatter<'_>, board_width: i32) -> std::fmt::Result {
        write!(f, "{}", self.to_string(board_width))
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
        let (xs, ys) = value.dim();
        for x in 0..ys {
            for y in 0..xs {
                if value[[y, x]] {
                    let index = x * xs + y;
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

impl Index<usize> for Bitmask {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        let array_index = index / BITS_IN_PRIMITIVE;
        let bit_index = index % BITS_IN_PRIMITIVE;
        if self.bits[array_index] & (1 << bit_index) != 0 {
            &true
        } else {
            &false
        }
    }
}
