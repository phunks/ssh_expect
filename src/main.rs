mod config;
mod crypt;
mod inventory;
mod request;
mod ssh;
mod args;

use std::fmt;
use std::process::ExitCode;
use serde_derive::{Deserialize, Serialize};
use strum::EnumMessage;
use crate::args::Options;
use crate::config::read_config;
use crate::request::post_request;
use crate::ssh::do_ssh;


const EXT_NORMAL_MESSAGES: &str = "aあああああ";
#[derive(Debug, EnumMessage)]
pub enum Rc {
    /// aaaa [`EXT_NORMAL_MESSAGES`].
    Normal,
    /// warn bbb
    Warn,
    #[doc = "Returns a new ああああ `"]
    Error,
}

#[tokio::main]
async fn main() -> ExitCode {
    let opts = Options::init();
    let mut inventory = match read_config(&opts.to_string()) {
        Ok(toml) => {toml}
        Err(e) => {
            panic!("toml error: {}", e)
        }
    };

    let a = do_ssh(&mut inventory)
        .await
        .unwrap_or_else(|e| panic!("ssh failed with {}", e));

    let url = inventory.webhook.url;

    if opts.debug {
        println!("{}\n\n{}", a.1.get_documentation().unwrap(), a.0);
    }
    match a.1 {
        Rc::Warn | Rc::Error => {
            if !opts.debug {
                post_request(&url, &OutgoingWebhook {
                    text: format!("{}\n\n{}", a.1, a.0)
                }, opts.insecure).await.expect("error: webhook");
            }
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