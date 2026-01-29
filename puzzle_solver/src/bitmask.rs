use ndarray::Array2;
use std::ops::{BitAnd, BitOr, BitXor, Index};

/// Must be the same as the bits in the primitive type used in the bitmask array.
const BITS_IN_PRIMITIVE: usize = 128;
/// Number of elements in the bitmask array. Adjust this to change the size of the bitmask.
/// This should be kept small to allow for optimized operations. It may be increased, if
/// more puzzles are added.
const BITMASK_ARRAY_LENGTH: usize = 3;
/// A mask with all bits set in a single primitive type.
const FULL_PRIMITIVE_MASK: u128 = u128::MAX;

/// Total number of bits in the Bitmask. This is not necessarily the number of relevant bits.
const TOTAL_BITS: usize = BITMASK_ARRAY_LENGTH * BITS_IN_PRIMITIVE;

/// A Bitmask represents a set of bits, allowing for efficient bitwise operations.
/// The size is limited to `BITMASK_ARRAY_LENGTH * BITS_IN_PRIMITIVE` bits to hopefully
/// allow for optimized operations.
#[derive(Debug, Eq, PartialEq, Hash)]
pub(crate) struct Bitmask {
    /// Number of relevant bits in the bitmask.
    /// Those are the bits that are of interest, e.g., corresponding to the board size.
    /// The rest of the bits are still used in some operations.
    /// See their documentation for details.
    relevant_bits: usize,
    /// The actual bits of the bitmask.
    /// This is an array to allow for bitmasks larger than the primitive type.
    /// Operations should use `BITMASK_ARRAY_LENGTH` to determine the number of elements to process
    /// and allow for compiler optimizations.
    bits: [u128; BITMASK_ARRAY_LENGTH],
}

impl Bitmask {
    /// Constructs a new Bitmask with the given amount of relevant bits.
    /// Relevant bits start at the index 0 and go up to `length - 1`.
    /// All bits are initialized to zero.
    ///
    /// # Panics
    ///
    /// Panics if the length exceeds the maximum number of bits supported by the Bitmask.
    ///
    /// # Arguments
    ///
    /// * `length`: Number of relevant bits in the bitmask.
    ///
    /// returns: Bitmask
    pub(crate) fn new(length: usize) -> Self {
        if length > TOTAL_BITS {
            panic!(
                "Bitmask length {} exceeds maximum of {}",
                length, TOTAL_BITS
            );
        }
        Bitmask {
            relevant_bits: length,
            bits: [0; BITMASK_ARRAY_LENGTH],
        }
    }

    /// Sets the bit at the given index to 1.
    ///
    /// # Arguments
    ///
    /// * `index`: Index of the bit to set.
    ///
    /// returns: ()
    pub(crate) fn set_bit(&mut self, index: usize) {
        let array_index = index / BITS_IN_PRIMITIVE;
        let bit_index = index % BITS_IN_PRIMITIVE;
        self.bits[array_index] |= 1 << bit_index;
    }

    /// Clears the bit at the given index (sets it to 0).
    ///
    /// # Arguments
    ///
    /// * `index`: Index of the bit to clear.
    ///
    /// returns: ()
    #[allow(dead_code)]
    pub(crate) fn clear_bit(&mut self, index: usize) {
        let array_index = index / BITS_IN_PRIMITIVE;
        let bit_index = index % BITS_IN_PRIMITIVE;
        self.bits[array_index] &= !(1 << bit_index);
    }

    /// Returns the number of relevant bits in the bitmask.
    ///
    /// # Arguments
    ///
    /// returns: usize
    pub(crate) fn relevant_bits(&self) -> usize {
        self.relevant_bits
    }

    /// Checks if all relevant bits are set to 1.
    ///
    /// # Arguments
    ///
    /// returns: bool
    pub(crate) fn all_relevant_bits_set(&self) -> bool {
        match BITMASK_ARRAY_LENGTH {
            1 => {
                let mask = (1 << self.relevant_bits) - 1;
                if self.bits[0] & mask != mask {
                    false
                } else {
                    true
                }
            }
            _ => {
                let full_words = self.relevant_bits / BITS_IN_PRIMITIVE;
                let remaining_bits = self.relevant_bits % BITS_IN_PRIMITIVE;

                for i in 0..full_words {
                    if self.bits[i] != FULL_PRIMITIVE_MASK {
                        return false;
                    }
                }

                if remaining_bits > 0 {
                    let mask = (1 << remaining_bits) - 1;
                    if self.bits[full_words] & mask != mask {
                        return false;
                    }
                }

                true
            }
        }
    }

    /// Performs a bitwise OR operation between two bitmasks and stores the result in self.
    /// The content of self is overwritten.
    ///
    /// # Arguments
    ///
    /// * `a`: First bitmask.
    /// * `b`: Second bitmask.
    ///
    /// returns: ()
    pub(crate) fn or(&mut self, a: &Bitmask, b: &Bitmask) {
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

    /// Performs a bitwise XOR operation between two bitmasks and stores the result in self.
    /// The content of self is overwritten.
    ///
    /// # Arguments
    ///
    /// * `a`: First bitmask.
    /// * `b`: Second bitmask.
    ///
    /// returns: ()
    pub(crate) fn xor(&mut self, a: &Bitmask, b: &Bitmask) {
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

    /// Performs a bitwise AND operation between two bitmasks and stores the result in self.
    /// The content of self is overwritten.
    ///
    /// # Arguments
    ///
    /// * `a`: First bitmask.
    /// * `b`: Second bitmask.
    ///
    /// returns: ()
    pub(crate) fn and(&mut self, a: &Bitmask, b: &Bitmask) {
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

    /// Checks if the bitwise AND between self and other results in zero.
    /// This is equivalent to checking if there are no common bits set in both bitmasks.
    ///
    /// Hopefully more performant shorthand for `(a & b) == 0`.
    ///
    /// # Arguments
    ///
    /// * `other`: Other bitmask to AND with.
    ///
    /// returns: bool
    pub(crate) fn and_is_zero(&self, other: &Bitmask) -> bool {
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

    /// Checks if the bitwise AND between two bitmasks equals a third bitmask.
    /// This is equivalent to checking if `(a & b) == c` and hopefully more performant shorthand.
    ///
    /// # Arguments
    ///
    /// * `a`: Fist bitmask for the AND operation.
    /// * `b`: Second bitmask for the AND operation.
    /// * `c`: Bitmask to compare the result against.
    ///
    /// returns: bool
    pub(crate) fn and_equals(a: &Bitmask, b: &Bitmask, c: &Bitmask) -> bool {
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

    /// Converts the bitmask to a string representation.
    /// Bits are represented as '1' for set bits and '0' for unset bits.
    /// An underscore '_' is added every `board_width` bits for better readability.
    /// Only the relevant bits are included in the output.
    ///
    /// # Arguments
    ///
    /// * `board_width`: Width of the board to format the output.
    ///
    /// returns: String
    pub(crate) fn to_string(&self, board_width: i32) -> String {
        let mut output = String::new();
        for bit_index in 0..self.relevant_bits {
            if bit_index as i32 % board_width == 0 && bit_index != 0 {
                output.push('_');
            }
            let bit_set = self[bit_index];
            let symbol = if bit_set { '1' } else { '0' };
            output.push(symbol);
        }
        output
    }
}

impl BitOr for Bitmask {
    type Output = Bitmask;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut output = Bitmask::new(self.relevant_bits);
        output.or(&self, &rhs);
        output
    }
}

impl BitXor for Bitmask {
    type Output = Bitmask;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut output = Bitmask::new(self.relevant_bits);
        output.xor(&self, &rhs);
        output
    }
}

impl BitAnd for Bitmask {
    type Output = Bitmask;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut output = Bitmask::new(self.relevant_bits);
        output.and(&self, &rhs);
        output
    }
}

impl Default for Bitmask {
    /// Creates a new Bitmask, where all bits are initialized to zero.
    /// The number of relevant bits is set to the maximum supported by the Bitmask.
    fn default() -> Self {
        Self::new(TOTAL_BITS)
    }
}

impl From<&Array2<bool>> for Bitmask {
    /// Creates a Bitmask from a 2D array of booleans.
    /// The relevant bits are determined by the number of elements in the array.
    /// Each cell in the array corresponds to a bit in the bitmask.
    fn from(value: &Array2<bool>) -> Self {
        let relevant_bits = value.iter().count();
        let mut bitmask = Bitmask::new(relevant_bits);
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
        let mut new_bitmask = Bitmask::new(self.relevant_bits);
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

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn test_new() {
        let bitmask = Bitmask::new(10);
        assert_eq!(bitmask.relevant_bits(), 10);
        for i in 0..10 {
            assert_eq!(bitmask[i], false);
        }
    }

    #[test]
    fn test_set_bit() {
        let mut bitmask = Bitmask::new(10);
        bitmask.set_bit(3);
        assert_eq!(bitmask[3], true);
        for i in 0..10 {
            if i != 3 {
                assert_eq!(bitmask[i], false);
            }
        }
    }

    #[test]
    fn test_clear_bit() {
        let mut bitmask = Bitmask::new(10);
        for i in 0..10 {
            bitmask.set_bit(i);
        }

        bitmask.clear_bit(3);

        assert_eq!(bitmask[3], false);
        for i in 0..10 {
            if i != 3 {
                assert_eq!(bitmask[i], true);
            }
        }
    }

    #[test]
    fn test_relevant_bits() {
        let bitmask = Bitmask::new(20);
        assert_eq!(bitmask.relevant_bits(), 20);
    }

    #[test]
    fn test_all_relevant_bits_set() {
        let mut bitmask = Bitmask::new(5);
        for i in 0..5 {
            bitmask.set_bit(i);
        }
        assert_eq!(bitmask.all_relevant_bits_set(), true);

        bitmask.clear_bit(2);
        assert_eq!(bitmask.all_relevant_bits_set(), false);
    }

    #[test]
    fn test_all_relevant_bits_set_other_set() {
        let mut bitmask = Bitmask::new(5);
        for i in 0..4 {
            bitmask.set_bit(i);
        }
        bitmask.set_bit(7); // Set a bit outside the relevant range

        assert_eq!(bitmask.all_relevant_bits_set(), false);
    }

    #[test]
    fn test_or() {
        let mut a = Bitmask::new(10);
        a.set_bit(2);
        a.set_bit(4);

        let mut b = Bitmask::new(10);
        b.set_bit(4);
        b.set_bit(6);

        let mut result = Bitmask::new(10);
        result.or(&a, &b);

        assert_eq!(result[2], true);
        assert_eq!(result[4], true);
        assert_eq!(result[6], true);
        assert_eq!(result[0], false);
        assert_eq!(result.relevant_bits(), 10);
    }

    #[test]
    fn test_or_different_lengths() {
        let mut a = Bitmask::new(15);
        a.set_bit(2);
        a.set_bit(4);

        let mut b = Bitmask::new(10);
        b.set_bit(4);
        b.set_bit(6);

        let mut result = Bitmask::new(9);
        result.or(&a, &b);

        assert_eq!(result[2], true);
        assert_eq!(result[4], true);
        assert_eq!(result[6], true);
        assert_eq!(result[0], false);
        assert_eq!(result.relevant_bits(), 9);
    }

    #[test]
    fn test_xor() {
        let mut a = Bitmask::new(10);
        a.set_bit(2);
        a.set_bit(4);

        let mut b = Bitmask::new(10);
        b.set_bit(4);
        b.set_bit(6);

        let mut result = Bitmask::new(10);
        result.xor(&a, &b);

        assert_eq!(result[2], true);
        assert_eq!(result[4], false);
        assert_eq!(result[6], true);
        assert_eq!(result[0], false);
        assert_eq!(result.relevant_bits(), 10);
    }

    #[test]
    fn test_and() {
        let mut a = Bitmask::new(10);
        a.set_bit(2);
        a.set_bit(4);

        let mut b = Bitmask::new(10);
        b.set_bit(4);
        b.set_bit(6);

        let mut result = Bitmask::new(10);
        result.and(&a, &b);

        assert_eq!(result[2], false);
        assert_eq!(result[4], true);
        assert_eq!(result[6], false);
        assert_eq!(result[0], false);
        assert_eq!(result.relevant_bits(), 10);
    }

    #[test]
    fn test_and_is_zero() {
        let mut a = Bitmask::new(10);
        a.set_bit(2);
        a.set_bit(4);

        let mut b = Bitmask::new(10);
        b.set_bit(6);
        b.set_bit(8);

        assert_eq!(a.and_is_zero(&b), true);

        b.set_bit(4);
        assert_eq!(a.and_is_zero(&b), false);
    }

    #[test]
    fn test_and_equals() {
        let mut a = Bitmask::new(10);
        a.set_bit(2);
        a.set_bit(4);

        let mut b = Bitmask::new(10);
        b.set_bit(4);
        b.set_bit(6);

        let mut c = Bitmask::new(10);
        c.set_bit(4);

        assert_eq!(Bitmask::and_equals(&a, &b, &c), true);

        c.clear_bit(4);
        assert_eq!(Bitmask::and_equals(&a, &b, &c), false);
    }

    #[test]
    fn test_to_string() {
        let mut bitmask = Bitmask::new(10);
        bitmask.set_bit(0);
        bitmask.set_bit(3);
        bitmask.set_bit(5);
        bitmask.set_bit(9);

        let board_width = 5;
        let expected = "10010_10001";
        let result = bitmask.to_string(board_width);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_bitor() {
        let mut a = Bitmask::new(10);
        a.set_bit(2);
        a.set_bit(4);

        let mut b = Bitmask::new(10);
        b.set_bit(4);
        b.set_bit(6);

        let result = a | b;

        assert_eq!(result[2], true);
        assert_eq!(result[4], true);
        assert_eq!(result[6], true);
        assert_eq!(result[0], false);
        assert_eq!(result.relevant_bits(), 10);
    }

    #[test]
    fn test_bitxor() {
        let mut a = Bitmask::new(10);
        a.set_bit(2);
        a.set_bit(4);

        let mut b = Bitmask::new(10);
        b.set_bit(4);
        b.set_bit(6);

        let result = a ^ b;

        assert_eq!(result[2], true);
        assert_eq!(result[4], false);
        assert_eq!(result[6], true);
        assert_eq!(result[0], false);
        assert_eq!(result.relevant_bits(), 10);
    }

    #[test]
    fn test_bitand() {
        let mut a = Bitmask::new(10);
        a.set_bit(2);
        a.set_bit(4);

        let mut b = Bitmask::new(10);
        b.set_bit(4);
        b.set_bit(6);

        let result = a & b;

        assert_eq!(result[2], false);
        assert_eq!(result[4], true);
        assert_eq!(result[6], false);
        assert_eq!(result[0], false);
        assert_eq!(result.relevant_bits(), 10);
    }

    #[test]
    fn test_default() {
        let bitmask = Bitmask::default();
        assert_eq!(bitmask.relevant_bits(), TOTAL_BITS);
        for i in 0..TOTAL_BITS {
            assert_eq!(bitmask[i], false);
        }
    }

    #[test]
    fn test_from_array2_bool() {
        let array = arr2(&[[true, false, true, true], [false, true, false, false]]);

        let bitmask = Bitmask::from(&array);

        assert_eq!(bitmask.relevant_bits(), 8);
        assert_eq!(bitmask[0], true);
        assert_eq!(bitmask[1], false);
        assert_eq!(bitmask[2], false);
        assert_eq!(bitmask[3], true);
        assert_eq!(bitmask[4], true);
        assert_eq!(bitmask[5], false);
        assert_eq!(bitmask[6], true);
        assert_eq!(bitmask[7], false);
    }

    #[test]
    fn test_from_array2_bool_empty() {
        let array = arr2(&[[]]);

        let bitmask = Bitmask::from(&array);

        assert_eq!(bitmask.relevant_bits(), 0);
    }

    #[test]
    fn test_from_array2_bool_one() {
        let array = arr2(&[[true]]);

        let bitmask = Bitmask::from(&array);

        assert_eq!(bitmask.relevant_bits(), 1);
        assert_eq!(bitmask[0], true);
    }

    #[test]
    fn test_clone() {
        let mut original = Bitmask::new(10);
        original.set_bit(2);
        original.set_bit(5);

        let cloned = original.clone();

        assert_eq!(cloned.relevant_bits(), original.relevant_bits());
        for i in 0..10 {
            assert_eq!(cloned[i], original[i]);
        }
    }

    #[test]
    fn test_index() {
        let mut bitmask = Bitmask::new(10);
        bitmask.set_bit(3);
        bitmask.set_bit(7);

        assert_eq!(bitmask[0], false);
        assert_eq!(bitmask[1], false);
        assert_eq!(bitmask[2], false);
        assert_eq!(bitmask[3], true);
        assert_eq!(bitmask[4], false);
        assert_eq!(bitmask[5], false);
        assert_eq!(bitmask[6], false);
        assert_eq!(bitmask[7], true);
        assert_eq!(bitmask[8], false);
        assert_eq!(bitmask[9], false);
    }
}
