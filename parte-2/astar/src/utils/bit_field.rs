//! Implementation of a bit field

use std::rc::Rc;

/// Bit field of N bits, of arbitrary (dynamic) size
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct BitField(Rc<Vec<u8>>);

impl BitField {
    /// Creates a new `BitField` with the given size
    ///
    /// # Parameters
    ///
    /// * `size`: Amount of bits for which to allocate space
    ///
    /// # Examples
    ///
    /// ```
    /// use astar::utils::BitField;
    /// let field = BitField::new(10);
    /// ```
    #[must_use]
    #[allow(clippy::cast_possible_truncation)] // The result will always fit in a single byte
    pub fn new(size: usize) -> Self {
        // (x + 7) & !7 rounds x to the next multiple of 8, to make
        // sure enough bits are allocated for the requested size
        let reserved = (size + 7) & !7;
        let mut out = vec![0; reserved >> 3];
        // Uses the most significant N-3 bits to select the last byte
        out[(size - 1) >> 3] = !((1u16 << (8 - (reserved - size))) - 1) as u8;
        Self(Rc::new(out))
    }

    /// Checks whether the bit at the given index is set
    ///
    /// # Parameters
    ///
    /// * `i`: Index of the bit to check for
    ///
    /// # Panics
    ///
    /// Panics if an index higher than the amount of bits with which the field was created is given
    ///
    /// # Examples
    ///
    /// ```
    /// use astar::utils::BitField;
    /// let field = BitField::new(16usize);
    /// assert_eq!(field.get(8), false);
    /// let field = field.set(8);
    /// assert_eq!(field.get(8), true);
    /// ```
    #[must_use]
    pub fn get(&self, i: usize) -> bool {
        // Uses the most significant N-3 bits to select the byte
        // Uses the remaining 3 bits to select the bit inside
        // the byte, and checks whether the bit is set
        self.0[i >> 3] & (1 << (i & 0b111)) != 0
    }

    /// Sets the bit at the given index, modifying a copy of the field
    ///
    /// # Parameters
    ///
    /// * `i`: Index of the bit to set
    ///
    /// # Panics
    ///
    /// Panics if an index higher than the amount of bits with which the field was created is given
    ///
    /// # Examples
    ///
    /// ```
    /// use astar::utils::BitField;
    /// let field = BitField::new(16);
    /// assert_eq!(field.get(8), false);
    /// let modified = field.set(8);
    /// assert_eq!(field.get(8), false);
    /// assert_eq!(modified.get(8), true);
    /// ```
    pub fn set(&mut self, size: usize) {
        // Clones the field to not modify the copy stored by other fields
        let mut vec = (*self.0).clone();
        // Uses the most significant N-3 bits to select the byte
        // Uses the remaining 3 bits to select the bit
        // inside the byte, and sets it with a bitwise OR
        vec[size >> 3] |= 1 << (size & 0b111);
        *self = Self(Rc::new(vec));
    }

    /// Checks whether all bits are set
    ///
    /// # Examples
    ///
    /// ```
    /// use astar::utils::BitField;
    /// let field = BitField::new(2);
    /// assert_eq!(field.all(), false);
    /// let field = field.set(0);
    /// assert_eq!(field.all(), false);
    /// let field = field.set(1);
    /// assert_eq!(field.all(), true);
    /// ```
    #[must_use]
    pub fn all(&self) -> bool {
        self.0.iter().all(|&x| x == 0xFF)
    }
}
