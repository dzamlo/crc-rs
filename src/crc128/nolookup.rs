use crate::{Algorithm, Crc, Digest, NoTable};

use super::{finalize, init, update_nolookup};

impl Crc<NoTable<u128>> {
    pub const fn new(algorithm: Algorithm<u128>) -> Self {
        Self {
            algorithm,
            table: (),
        }
    }

    pub const fn checksum(&self, bytes: &[u8]) -> u128 {
        let mut crc = init(self.algorithm, self.algorithm.init);
        crc = self.update(crc, bytes);
        finalize(self.algorithm, crc)
    }

    const fn update(&self, crc: u128, bytes: &[u8]) -> u128 {
        update_nolookup(crc, self.algorithm, bytes)
    }

    pub const fn digest(&self) -> Digest<NoTable<u128>> {
        self.digest_with_initial(self.algorithm.init)
    }

    /// Construct a `Digest` with a given initial value.
    ///
    /// This overrides the initial value specified by the algorithm.
    /// The effects of the algorithm's properties `refin` and `width`
    /// are applied to the custom initial value.
    pub const fn digest_with_initial(&self, initial: u128) -> Digest<NoTable<u128>> {
        let value = init(self.algorithm, initial);
        Digest::new(self, value)
    }
}

impl<'a> Digest<'a, NoTable<u128>> {
    const fn new(crc: &'a Crc<NoTable<u128>>, value: u128) -> Self {
        Digest { crc, value }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.value = self.crc.update(self.value, bytes);
    }

    pub const fn finalize(self) -> u128 {
        finalize(self.crc.algorithm, self.value)
    }
}
