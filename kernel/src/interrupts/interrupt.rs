use crate::interrupts::pics::PIC_1_OFFSET;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    #[inline]
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    #[inline]
    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}
