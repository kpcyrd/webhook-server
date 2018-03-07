use libc;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Syscall {
    read                = libc::SYS_read                as isize,
    write               = libc::SYS_write               as isize,
    socketpair          = libc::SYS_socketpair          as isize,
    socket              = libc::SYS_socket              as isize,
    connect             = libc::SYS_connect             as isize,
    close               = libc::SYS_close               as isize,
    sigaltstack         = libc::SYS_sigaltstack         as isize,
    munmap              = libc::SYS_munmap              as isize,
    exit_group          = libc::SYS_exit_group          as isize,
    getsockopt          = libc::SYS_getsockopt          as isize,
    sendmsg             = libc::SYS_sendmsg             as isize,
    futex               = libc::SYS_futex               as isize,
    epoll_create1       = libc::SYS_epoll_create1       as isize,
    pipe2               = libc::SYS_pipe2               as isize,
    epoll_ctl           = libc::SYS_epoll_ctl           as isize,
    setsockopt          = libc::SYS_setsockopt          as isize,
    bind                = libc::SYS_bind                as isize,
    listen              = libc::SYS_listen              as isize,
    fcntl               = libc::SYS_fcntl               as isize,
    epoll_pwait         = libc::SYS_epoll_pwait         as isize,
    accept4             = libc::SYS_accept4             as isize,
    ioctl               = libc::SYS_ioctl               as isize,
    recvfrom            = libc::SYS_recvfrom            as isize,
    sendto              = libc::SYS_sendto              as isize,
    prctl               = libc::SYS_prctl               as isize,
    seccomp             = libc::SYS_seccomp             as isize,
}

impl Syscall {
    #[inline]
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}
