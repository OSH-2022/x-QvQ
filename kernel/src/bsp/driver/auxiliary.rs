use super::{Driver, RegisterWrapper};
use ::spin::Mutex;
use core::fmt::{Arguments, Result, Write};
use tock_registers::{
    interfaces::{ReadWriteable, Readable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

register_structs! {
    AuxReg {
        (0x00 => irq: ReadOnly<u32, ENABLES::Register>),
        (0x04 => enables: ReadWrite<u32, ENABLES::Register>),
        (0x08 => _reserved1),
        (0x40 => mu_io: ReadWrite<u32, MU_IO::Register>),           // io data
        (0x44 => mu_ier: ReadWrite<u32, MU_IER::Register>),         // interrupt enable
        (0x48 => mu_iir: ReadOnly<u32, MU_IIR::Register>),          // interrupt id / clear FIFO
        (0x4c => mu_lcr: ReadWrite<u32, MU_LCR::Register>),         // break, set data size
        (0x50 => mu_mcr: ReadWrite<u32>),                           // moderm signals
        (0x54 => mu_lsr: ReadWrite<u32, MU_LSR::Register>),         // data stauts
        (0x58 => mu_msr: ReadWrite<u32>),                           // moderm status
        (0x5c => mu_scratch: ReadWrite<u32>),                       // what's this?
        (0x60 => mu_cntl: ReadWrite<u32, MU_CNTL::Register>),       // extra internal control
        (0x64 => mu_stat: ReadWrite<u32>),                          // extra internal status
        (0x68 => mu_baud: ReadWrite<u32, MU_BAUD::Register>),       // easier way to set baudrate
        (0x6c => _reserved2),                                       // ignore spi1, spi2 as we don't care currently
        (0xd8 => @END),
    }
}

register_bitfields! {
    u32,

    ENABLES [
        MU   0,
        SPI1 1,
        SPI2 2,
    ],

    MU_IER [
        // some bits ignored here
        RX_INT_EN 1,
        TX_INT_EN 0,
    ],

    MU_IIR [
        // ignore writing for clearing FIFO
        INT_ID OFFSET(1) NUMBITS(2) [
            NoInt = 0,
            TXInt = 1,
            RXInt = 2,
        ],
    ],

    MU_LCR [
        BREAK OFFSET(6) NUMBITS(1) [],
        DATA_SIZE OFFSET(0) NUMBITS(1) [
            BIT7 = 0,
            BIT8 = 1,
        ],
    ],

    MU_IO [
        DATA OFFSET(0) NUMBITS(8),
    ],

    MU_LSR [
        TX_IDLE       6,
        TX_EMPTY      5,
        RX_OVERRUN    1,
        RX_DATA_READY 0,
    ],

    MU_CNTL [
        // some bits ignored here
        TX_EN 1,
        RX_EN 0,
    ],

    MU_BAUD [
        BAUD OFFSET(0) NUMBITS(16),
    ],
}

const VCLK: usize = 250_000_000;
const BAUDRATE: usize = 115200;

pub struct MiniUartInner {
    aux_reg: RegisterWrapper<AuxReg>,
}

impl MiniUartInner {
    const fn empty() -> Self {
        Self {
            aux_reg: RegisterWrapper::new(0),
        }
    }

    fn init(&mut self, va: usize) {
        self.aux_reg.start = va;
        self.aux_reg.enables.modify(ENABLES::MU.val(1));
        self.aux_reg.mu_cntl.modify(MU_CNTL::TX_EN.val(0));

        self.aux_reg.mu_lcr.modify(MU_LCR::DATA_SIZE::BIT8);
        let baud_reg: u32 = (VCLK / (8 * BAUDRATE) - 1) as u32;
        self.aux_reg
            .mu_baud
            .modify(MU_BAUD::BAUD.val(baud_reg as u32));

        self.aux_reg.mu_cntl.modify(MU_CNTL::TX_EN.val(1));
    }

    fn putc(&self, ch: u8) {
        while !self.aux_reg.mu_lsr.is_set(MU_LSR::TX_EMPTY) {}
        self.aux_reg.mu_io.modify(MU_IO::DATA.val(ch as u32));
    }

    fn _flush(&self) {
        while !self.aux_reg.mu_lsr.is_set(MU_LSR::TX_IDLE) {}
    }
}

impl Write for MiniUartInner {
    fn write_str(&mut self, s: &str) -> Result {
        for &c in s.as_bytes() {
            self.putc(c);
        }
        Ok(())
    }
}

pub struct MiniUart {
    inner: Mutex<MiniUartInner>,
}

impl MiniUart {
    const fn empty() -> Self {
        Self {
            inner: Mutex::new(MiniUartInner::empty()),
        }
    }

    pub fn lock_and_write(&self, args: Arguments) -> Result {
        let mut mini_uart = self.inner.lock();
        mini_uart.write_fmt(args)
    }
}

impl Driver for MiniUart {
    fn init(&self, va: usize) {
        let mut mini_uart = self.inner.lock();
        mini_uart.init(va);
    }
}

pub static MINI_UART: MiniUart = MiniUart::empty();
