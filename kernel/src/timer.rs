
use cortex_a::registers::{CNTFRQ_EL0, CNTPCT_EL0, CNTP_CTL_EL0, CNTP_TVAL_EL0};
use tock_registers::interfaces::{Readable,Writeable};
use crate::gicv2::irq_set_mask;
static mut CLOCK_FREQ : u64=0;
use crate::config::TICKS_PER_SECOND;
#[allow(unused_imports)]
const MSEC_PER_SEC:u64=1000;
pub fn get_time()->u64{// as ms
    unsafe{CNTPCT_EL0.get() * MSEC_PER_SEC / CLOCK_FREQ}
}
pub fn set_next_trigger(){
    unsafe{CNTP_TVAL_EL0.set(CLOCK_FREQ/TICKS_PER_SECOND);}
}
pub fn init()
{
    unsafe{CLOCK_FREQ=CNTFRQ_EL0.get();}
    CNTP_CTL_EL0.write(CNTP_CTL_EL0::ENABLE::SET);
    set_next_trigger();
    irq_set_mask(30,false);
}
