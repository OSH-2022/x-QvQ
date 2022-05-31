use crate::timer::get_time;
#[allow(dead_code)]
pub fn sys_get_time()->isize   //in milliseconds
{
    get_time() as isize
}
//exec

//fork