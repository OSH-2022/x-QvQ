use crate::arch::{Context, Exception, PAGE_SIZE, VIRT_PAGE_MANAGE};
use crate::mmu::{Addr, MemoryType, VirtAddr};
use alloc::collections::VecDeque;
use spin::Mutex;

pub static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::empty());

struct InitContext {
    pc: VirtAddr,
}

impl InitContext {
    fn new(pc: VirtAddr) -> Self {
        Self { pc }
    }
}

enum ContextOpt {
    Init(InitContext),
    Normal(&'static Context),
}

pub struct Thread {
    stack_start: VirtAddr,
    context: ContextOpt,
}

impl Thread {
    pub fn new(pc: VirtAddr) -> Self {
        let mut vm = VIRT_PAGE_MANAGE.lock();
        let va = vm.new_page(MemoryType::Normal);
        Self {
            stack_start: va,
            context: ContextOpt::Init(InitContext::new(pc)),
        }
    }
}

pub struct Scheduler {
    queue: Option<VecDeque<Thread>>,
    current: Option<Thread>,
}

impl Scheduler {
    const fn empty() -> Self {
        Self {
            queue: None,
            current: None,
        }
    }

    pub fn init(&mut self) {
        self.queue = Some(VecDeque::new());
    }

    pub fn insert(&mut self, th: Thread) {
        let queue = self.queue.as_mut().expect("un-init scheduler");
        queue.push_back(th);
    }

    pub fn remove_self(&mut self) {
        let _cur = self.current.take().expect("no current thread to remove");
    }

    /* only for irq_handler */
    pub fn schedule(&mut self, old_cont: &'static Context) {
        let queue = self.queue.as_mut().expect("un-init scheduler");

        if let Some(mut cur) = self.current.take() {
            cur.context = ContextOpt::Normal(old_cont);
            queue.push_back(cur);
        }

        let first = queue.pop_front().expect("no thread to schedule");
        unsafe {
            self.current = Some(core::mem::transmute_copy(&first));
        }
        match first.context {
            ContextOpt::Init(cont) => unsafe {
                /* we've got the lock */
                Exception::unmask_irq();
                Self::schedule_init_context(first.stack_start.add_off(PAGE_SIZE), cont.pc);
            },
            ContextOpt::Normal(cont) => unsafe {
                Exception::set_sp_and_exit(cont.to_sp());
            },
        }
    }

    unsafe fn schedule_init_context(sp: VirtAddr, pc: VirtAddr) {
        core::arch::asm!(
            "mov sp, {sp}",
            "br {pc}",
            sp = in(reg) sp.to_usize(),
            pc = in(reg) pc.to_usize(),
        )
    }
}
