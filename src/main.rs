#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use std::process::{Command, ExitCode};
use std::time::Duration;
use std::{env, process, thread};

use signal_hook::consts::{SIGABRT, SIGINT};
use signal_hook::iterator::Signals;

use crate::arguments::parse_arguments;
use crate::auth_monitor::{AuthMonitor, AuthMonitorParams};

mod arguments;
mod auth_file_reader;
mod auth_file_watcher;
mod auth_monitor;
mod file_event_filter;
mod file_path;
mod message_parser;

const SLEEP_DURATION: Duration = Duration::from_millis(500);

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let params = match parse_arguments(&arguments) {
        Ok(params) => params,
        Err(error) => {
            eprintln!("Invalid arguments: {}", error);
            return ExitCode::FAILURE;
        }
    };
    println!(
        "Monitoring process {} started with parameters {}",
        process::id(),
        params,
    );
    let exit_code = start_monitoring(params);
    println!("Monitoring process stopped");
    return exit_code;
}

fn start_monitoring(params: AuthMonitorParams) -> ExitCode {
    let mut auth_monitor = match AuthMonitor::new(params) {
        Ok(auth_monitor) => auth_monitor,
        Err(e) => {
            eprintln!("Error creating AuthMonitor: {}", e);
            return ExitCode::FAILURE;
        }
    };
    let mut signals = Signals::new([SIGABRT, SIGINT]).unwrap();
    loop {
        auth_monitor.update(shutdown);
        let signal = signals.pending().next();
        if signal.is_some() {
            println!("Received signal {}, stopping...", signal.unwrap());
            return ExitCode::SUCCESS;
        }
        thread::sleep(SLEEP_DURATION);
    }
}

const SYSTEMCTL_COMMAND: &str = "systemctl";
const POWER_OFF_ARGS: [&str; 1] = ["poweroff"];

fn shutdown() {
    let output = match Command::new(SYSTEMCTL_COMMAND)
        .args(POWER_OFF_ARGS)
        .output()
    {
        Ok(output) => output,
        Err(error) => {
            eprintln!("Unable to shutdown: {}", error);
            return;
        }
    };
    match String::from_utf8(output.stdout) {
        Ok(stdout) => println!("Shutdown output: {}", stdout),
        Err(error) => eprintln!("Error converting command output to string: {}", error),
    };
}
