mod config;
mod crypt;
mod inventory;
mod request;
mod ssh;
mod args;

use std::fmt;
use std::process::ExitCode;
use serde_derive::{Deserialize, Serialize};
use crate::args::Options;
use crate::config::read_config;
use crate::request::post_request;
use crate::ssh::do_ssh;

#[derive(Debug)]
pub enum Rc {
    Normal,
    Warn,
    Error,
}

#[tokio::main]
async fn main() -> ExitCode {
    let toml = Options::init();
    let mut inventory = read_config(&toml.to_string()).unwrap();

    let a = do_ssh(&mut inventory)
        .await
        .unwrap_or_else(|e| panic!("ssh failed with {}", e));

    let url = inventory.webhook.url;

    match a.1 {
        Rc::Warn | Rc::Error => {
            post_request(&url, &OutgoingWebhook {
                text: format!("{}\n\n{}", a.1, a.0)
            }).await.expect("error: webhook");
        },
        _ => {}, //normal end
    }

    ExitCode::from(a.1 as u8)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutgoingWebhook {
    pub text: String,
}

impl fmt::Display for Rc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}