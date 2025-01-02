mod config;
mod crypt;
mod inventory;
mod request;
mod ssh;
mod args;

use std::error::Error;
use std::process::ExitCode;
use crate::args::Options;
use crate::ssh::do_ssh;

#[tokio::main]
async fn main() -> ExitCode {
    let toml = Options::init();

    let a = do_ssh(toml)
        .await
        .unwrap_or_else(|e| panic!("ssh failed with {}", e));

    if a.1 > 0 {
        match a.1 {
            1 => println!("warn\n{}", a.0),
            2 | 3 => println!("error\n{}", a.0),
            _ => {},
        }
    }

    ExitCode::from(a.1)
}
