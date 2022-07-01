use crate::mmu::{Addr, VirtAddr};
use cortex_a::registers::{CNTP_CTL_EL0, CNTP_TVAL_EL0, CNTFRQ_EL0};
use spin::Mutex;
use tock_registers::interfaces::{Writeable, Readable};
use tock_registers::registers::ReadWrite;
use tock_registers::{register_bitfields, register_structs};

use super::RegisterWrapper;

pub static CORE_TIMER: Mutex<CoreTimer> = Mutex::new(CoreTimer::empty());
pub struct CoreTimer {
    start: Option<VirtAddr>,
}

impl CoreTimer {
    const CRYSTAL_FREQ: usize = 19.2e6 as usize;
    const TIMER_FREQ: usize = 1e6 as usize;

    const fn empty() -> Self {
        Self { start: None }
    }

    pub fn init(&mut self, start: VirtAddr) {
        match self.start {
            Some(_) => panic!("re-init core timer"),
            None => {
                self.start = Some(start);
                CNTP_CTL_EL0.write(CNTP_CTL_EL0::ENABLE::SET);
                Self::set_interval(10);

                let reg: RegisterWrapper<CoreTimerRegs> = RegisterWrapper::new(start.to_usize());
                reg.ctrl.write(CTRL::INC::Inc1 + CTRL::CLK::Crystal);
                reg.core0_int_ctrl.write(INT_CTRL::CNTPNSIRQ.val(1));
                reg.prescaler.set(Self::prescaler_cal(Self::CRYSTAL_FREQ, Self::TIMER_FREQ));
            }
        }
    }

    pub fn set_interval(ms: usize) {
        CNTP_TVAL_EL0.set((CNTFRQ_EL0.get() * ms as u64) / 1000);
    }

    const fn prescaler_cal(freq_in: usize, freq_out: usize) -> u32 {
        // freq_out = (1 << 31) / prescaler * freq_in
        ((freq_in * (1 << 31)) / freq_out) as u32
    }
}

register_structs! {
    CoreTimerRegs {
        (0x00 => ctrl: ReadWrite<u32, CTRL::Register>),
        (0x04 => _reserved1),
        (0x08 => prescaler: ReadWrite<u32>),
        (0x0c => _reserved2),
        (0x40 => core0_int_ctrl: ReadWrite<u32, INT_CTRL::Register>),
        (0x44 => @END),
    }
}

register_bitfields! {
    u32,

    CTRL [
        INC OFFSET(9) NUMBITS(1) [
            Inc1 = 0,
            Inc2 = 1,
        ],
        CLK OFFSET(8) NUMBITS(1) [
            Crystal = 0,
            Apb = 1,
        ],
    ],

    INT_CTRL [
        CNTVIRQ_FIQ     7,
        CNTHPIRQ_FIQ    6,
        CNTPNSIRQ_FIQ   5,
        CNTPSIRQ_FIQ    4,
        CNTVIRQ         3,
        CNTHPIRQ        2,
        CNTPNSIRQ       1,
        CNTPSIRQ        0,
    ],
}
