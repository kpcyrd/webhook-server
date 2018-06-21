use syscallz::{Context, Syscall};

use errors::Result;


pub fn activate_stage1() -> Result<()> {
	let mut ctx = Context::init()?;
	ctx.allow_syscall(Syscall::read)?;
	ctx.allow_syscall(Syscall::write)?;
	ctx.allow_syscall(Syscall::socketpair)?;
	ctx.allow_syscall(Syscall::socket)?;
	ctx.allow_syscall(Syscall::connect)?;
	ctx.allow_syscall(Syscall::close)?;
	ctx.allow_syscall(Syscall::sigaltstack)?;
	ctx.allow_syscall(Syscall::munmap)?;
	ctx.allow_syscall(Syscall::exit_group)?;
	ctx.allow_syscall(Syscall::getsockopt)?;
	ctx.allow_syscall(Syscall::sendmsg)?;
	ctx.allow_syscall(Syscall::futex)?;
	ctx.allow_syscall(Syscall::epoll_create1)?;
	ctx.allow_syscall(Syscall::pipe2)?;
	ctx.allow_syscall(Syscall::epoll_ctl)?;
	ctx.allow_syscall(Syscall::setsockopt)?;
	ctx.allow_syscall(Syscall::bind)?;
	ctx.allow_syscall(Syscall::listen)?;
	ctx.allow_syscall(Syscall::fcntl)?;
	ctx.allow_syscall(Syscall::epoll_pwait)?;
	ctx.allow_syscall(Syscall::accept4)?;
	ctx.allow_syscall(Syscall::ioctl)?;
	ctx.allow_syscall(Syscall::recvfrom)?;
	ctx.allow_syscall(Syscall::sendto)?;
	ctx.allow_syscall(Syscall::prctl)?; // needed for stage2
	ctx.allow_syscall(Syscall::seccomp)?; // needed for stage2

	ctx.load()?;

	info!("stage 1/2 is active");
	Ok(())
}

pub fn activate_stage2() -> Result<()> {
	let mut ctx = Context::init()?;
	ctx.allow_syscall(Syscall::read)?;
	ctx.allow_syscall(Syscall::write)?;
	// ctx.allow_syscall(Syscall::socketpair)?;
	// ctx.allow_syscall(Syscall::socket)?;
	// ctx.allow_syscall(Syscall::connect)?;
	// ctx.allow_syscall(Syscall::close)?;
	// ctx.allow_syscall(Syscall::sigaltstack)?;
	// ctx.allow_syscall(Syscall::munmap)?;
	// ctx.allow_syscall(Syscall::exit_group)?;
	// ctx.allow_syscall(Syscall::getsockopt)?;
	ctx.allow_syscall(Syscall::sendmsg)?;
	// ctx.allow_syscall(Syscall::futex)?;
	// ctx.allow_syscall(Syscall::epoll_create1)?;
	// ctx.allow_syscall(Syscall::pipe2)?;
	ctx.allow_syscall(Syscall::epoll_ctl)?;
	// ctx.allow_syscall(Syscall::setsockopt)?;
	// ctx.allow_syscall(Syscall::bind)?;
	// ctx.allow_syscall(Syscall::listen)?;
	// ctx.allow_syscall(Syscall::fcntl)?;
	ctx.allow_syscall(Syscall::epoll_pwait)?;
	ctx.allow_syscall(Syscall::accept4)?;
	ctx.allow_syscall(Syscall::ioctl)?;
	ctx.allow_syscall(Syscall::recvfrom)?;
	ctx.allow_syscall(Syscall::sendto)?;

	ctx.load()?;

	info!("stage 2/2 is active");
	Ok(())
}
