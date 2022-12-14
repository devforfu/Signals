use libc::{SIGTERM, SIGUSR1};

static mut SHUT_DOWN: bool = false;

pub fn register_signal_handlers() {
    unsafe {
        libc::signal(SIGTERM, handle_sigterm as usize);
        libc::signal(SIGUSR1, handle_sigusr1 as usize);
    }
}

pub fn stopped() -> bool { unsafe { SHUT_DOWN } }

#[allow(dead_code)]
fn handle_sigterm(_signal: i32) {
    register_signal_handlers();
    println!("SIGTERM");
    unsafe { SHUT_DOWN = true; }
}

#[allow(dead_code)]
fn handle_sigusr1(_signal: i32) {
    register_signal_handlers();
    println!("SIGUSR1");
}

