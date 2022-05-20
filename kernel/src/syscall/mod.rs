// unfinished
// system call module
// need to add basic system call
// e.g. exit,read,write,exec
mod fs;
use self::fs::*;
const SYSCALL_EXIT: usize = 1;
pub fn syscall(syscall_id: usize,args:[usize;3])->isize{
    match syscall_id
    {
        // to be filled
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        _=> 
        {
            panic!("Unsupported system call:{}",syscall_id);
        }
    }
}