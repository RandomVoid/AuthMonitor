#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use std::process::{Command, ExitCode};
use std::time::Duration;
use std::{env, thread};

use signal_hook::consts::{SIGABRT, SIGINT};
use signal_hook::iterator::Signals;

use crate::auth_monitor::AuthMonitor;
use crate::auth_monitor_params::AuthMonitorParams;

mod auth_file_reader;
mod auth_file_watcher;
mod auth_message_parser;
mod auth_monitor;
mod auth_monitor_options;
mod auth_monitor_params;
mod file_event_filter;
mod file_path;

#[cfg(test)]
mod test_utils;

const SLEEP_DURATION: Duration = Duration::from_millis(500);

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let params = match AuthMonitorParams::from_arguments(&arguments) {
        Ok(params) => params,
        Err(error) => {
            eprintln!("Invalid arguments: {}", error);
            return ExitCode::FAILURE;
        }
    };
    println!("Monitoring process started with parameters {}", params);
    let exit_code = start_monitoring(params);
    println!("Monitoring process stopped");
    return exit_code;
}

fn start_monitoring(params: AuthMonitorParams) -> ExitCode {
    let mut auth_monitor = match AuthMonitor::new(params) {
        Ok(auth_monitor) => auth_monitor,
        Err(error) => {
            eprintln!("Error creating AuthMonitor: {}", error);
            return ExitCode::FAILURE;
        }
    };
    let mut signals = match Signals::new([SIGABRT, SIGINT]) {
        Ok(signals) => signals,
        Err(error) => {
            eprintln!("Error creating signals: {}", error);
            return ExitCode::FAILURE;
        }
    };
    loop {
        auth_monitor.update(shutdown);
        match signals.pending().next() {
            Some(signal) => {
                println!("Received signal {}", signal);
                return ExitCode::SUCCESS;
            }
            None => thread::sleep(SLEEP_DURATION),
        }
    }
}

const SUDO_COMMAND: &str = "sudo";
const SYSTEMCTL_POWER_OFF_ARGS: [&str; 2] = ["systemctl", "poweroff"];

fn shutdown() {
    let output = match Command::new(SUDO_COMMAND)
        .args(SYSTEMCTL_POWER_OFF_ARGS)
        .output()
    {
        Ok(output) => output,
        Err(error) => {
            eprintln!("Unable to shutdown: {}", error);
            return;
        }
    };
    let output_data = if output.status.success() {
        output.stdout
    } else {
        output.stderr
    };
    if output_data.is_empty() {
        return;
    }
    match String::from_utf8(output_data) {
        Ok(output_message) => println!("Shutdown output: {}", output_message),
        Err(error) => eprintln!("Error converting command output to string: {}", error),
    };
}
