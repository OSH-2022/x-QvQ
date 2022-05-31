#[allow(unused_imports)]
use tock_registers::register_structs;
use crate::config::{GIC_BASE,GIC_BORDER};
#[allow(dead_code)]
struct Gic{
    gicd_base:usize,
    gicc_base:usize,
    max_irqs:usize,
}

impl Gic {
     pub const fn new(gicd_base:usize,gicc_base:usize)->Self{
        
        //unfinished
        //change gicd_base,gicc_base to gicd,gicc
        Self { gicd_base: (gicd_base), gicc_base: (gicc_base), max_irqs: (0 as usize) }
    }
    // const  fn gicd(&self) ->&GicDistributorRegs{
    //     unsafe{ &*(self.gicd_base as *const _)}
    // }
    // const  fn gicc(&self) ->&GicCpuInterfaceRegs{
    //     unsafe{ &*(self.gicc_base as *const _)}
    // }
    #[allow(unused_variables)]
    pub fn set_enable(&self,vector:usize,enable:bool)
    {
        //unfinished
    }
}
static GIC:Gic=Gic::new(GIC_BASE,GIC_BASE+GIC_BORDER);

pub fn irq_set_mask(vector:usize,masked:bool)
{   
    GIC.set_enable(vector,!masked);
}