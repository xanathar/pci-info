#![allow(clippy::precedence)]

use crate::PciInfoError;

pub(super) struct PciConfigBuffer<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> PciConfigBuffer<'a> {
    const REGISTER_SIZE: usize = std::mem::size_of::<u32>();

    pub fn new(bytes: &'a [u8], offset: usize) -> Self {
        assert!(offset % Self::REGISTER_SIZE == 0);

        Self { bytes, offset }
    }

    #[inline(always)]
    fn calc_base(&self, register: usize) -> usize {
        register * Self::REGISTER_SIZE - self.offset
    }

    #[inline(always)]
    pub fn read_u32(&self, register: usize) -> u32 {
        let base = self.calc_base(register);
        (self.bytes[base] as u32)
            | (self.bytes[base + 1] as u32) << 8
            | (self.bytes[base + 2] as u32) << 16
            | (self.bytes[base + 3] as u32) << 24
    }

    #[inline(always)]
    pub fn read_u16_lo(&self, register: usize) -> u16 {
        let base = self.calc_base(register);
        (self.bytes[base] as u16) | (self.bytes[base + 1] as u16) << 8
    }

    #[inline(always)]
    pub fn read_u16_hi(&self, register: usize) -> u16 {
        let base = self.calc_base(register);
        (self.bytes[base + 2] as u16) | (self.bytes[base + 3] as u16) << 8
    }

    #[inline(always)]
    pub fn read_u8(&self, register: usize, offset_in_register: usize) -> u8 {
        assert!(offset_in_register <= 3);
        self.bytes[self.calc_base(register) + offset_in_register]
    }

    pub fn assert_registers_available(
        &self,
        first_register: usize,
        last_register_including: usize,
    ) -> Result<(), PciInfoError> {
        if self.first_register() > first_register {
            return Err(PciInfoError::ParseError(
                "not enough bytes at start of header".into(),
            ));
        }

        if last_register_including >= self.last_register() {
            return Err(PciInfoError::ParseError(
                "not enough bytes at end of header".into(),
            ));
        }

        Ok(())
    }

    pub fn first_register(&self) -> usize {
        self.offset / Self::REGISTER_SIZE
    }

    pub fn last_register(&self) -> usize {
        self.offset + self.bytes.len() / Self::REGISTER_SIZE
    }
}
