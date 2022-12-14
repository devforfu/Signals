use std::env;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use libc::{raise, kill, c_int, pid_t, SIGUSR1, SIGTERM};

enum Signal {
    Term,
    Usr1,
}

impl Display for Signal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Signal::Term => "SIGTERM",
            Signal::Usr1 => "SIGUSR1",
        })
    }
}

impl FromStr for Signal {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "SIGTERM" | "TERM" => Signal::Term,
            "SIGUSR1" | "USR1" => Signal::Usr1,
            _ => return Err(())
        })
    }
}

impl Signal {
    pub fn raise(&self) { unsafe { raise(self.to_int_c()); } }

    pub fn kill(&self, pid: u32) { unsafe { kill(pid as pid_t, self.to_int_c()); } }

    fn to_int_c(&self) -> c_int {
        match self {
            Signal::Term => SIGTERM,
            Signal::Usr1 => SIGUSR1,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(value) = env::args().nth(1) {
        if let Ok(signal) = Signal::from_str(&value) {
            if let Some(value) = env::args().nth(2) {
                println!("sending {signal} to pid={value}");
                let pid: u32 = str::parse(&value)?;
                signal.kill(pid);
            } else {
                println!("raising");
                signal.raise();
            }
        }
    }
    Ok(())
}