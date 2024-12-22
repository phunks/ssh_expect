mod config;
mod crypt;
mod inventory;
mod request;
mod ssh;
mod args;

use crate::args::Options;
use crate::ssh::do_ssh;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let toml = Options::init();

    let a = do_ssh(toml)
        .await
        .unwrap_or_else(|e| panic!("ssh failed with {}", e));
    println!("{a}");
    Ok(())
}
