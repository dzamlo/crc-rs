use crate::crc16::{finalize, init, update_slice16};
use crate::table::crc16_table_slice_16;
use crate::{Algorithm, Crc, Digest, Slice16};

impl Crc<Slice16<u16>> {
    pub const fn new(algorithm: Algorithm<u16>) -> Self {
        let table = crc16_table_slice_16(algorithm.width, algorithm.poly, algorithm.refin);
        Self { algorithm, table }
    }

    pub const fn checksum(&self, bytes: &[u8]) -> u16 {
        let mut crc = init(self.algorithm, self.algorithm.init);
        crc = self.update(crc, bytes);
        finalize(self.algorithm, crc)
    }

    const fn update(&self, crc: u16, bytes: &[u8]) -> u16 {
        update_slice16(crc, self.algorithm.refin, &self.table, bytes)
    }

    pub const fn digest(&self) -> Digest<Slice16<u16>> {
        self.digest_with_initial(self.algorithm.init)
    }

    /// Construct a `Digest` with a given initial value.
    ///
    /// This overrides the initial value specified by the algorithm.
    /// The effects of the algorithm's properties `refin` and `width`
    /// are applied to the custom initial value.
    pub const fn digest_with_initial(&self, initial: u16) -> Digest<Slice16<u16>> {
        let value = init(self.algorithm, initial);
        Digest::new(self, value)
    }
}

impl<'a> Digest<'a, Slice16<u16>> {
    const fn new(crc: &'a Crc<Slice16<u16>>, value: u16) -> Self {
        Digest { crc, value }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.value = self.crc.update(self.value, bytes);
    }

    pub const fn finalize(self) -> u16 {
        finalize(self.crc.algorithm, self.value)
    }
}
