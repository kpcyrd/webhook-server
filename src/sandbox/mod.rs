pub mod ipc;
#[cfg(target_os="linux")]
pub mod seccomp;
#[cfg(target_os="linux")]
pub mod syscalls;

use errors::Result;


#[inline]
pub fn activate_stage1() -> Result<()> {
    #[cfg(target_os="linux")]
    seccomp::activate_stage1()?;
    Ok(())
}

#[inline]
pub fn activate_stage2() -> Result<()> {
    #[cfg(target_os="linux")]
    seccomp::activate_stage2()?;
    Ok(())
}
