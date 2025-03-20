use bpaf::*;
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use std::{process};
use std::str::FromStr;
use crate::crypt::encrypt;

#[derive(Debug, Bpaf)]
#[bpaf(options)]
pub struct Options {
    /// Activate debug mode
    #[bpaf(short, long)]
    debug: bool,
    /// Allow insecure server connect
    #[bpaf(short('k'), long)]
    insecure: bool,
    /// magic crypt
    #[bpaf(short, long, argument("ssh passwd"))]
    pub encrypt: Option<String>,
    /// toml file
    #[bpaf(short, long, argument("file"))]
    pub file: Option<PathBuf>,
}

#[derive(Debug, Default)]
pub struct Opts{
    toml: PathBuf,
    pub debug: bool,
    pub insecure: bool,
}

impl FromStr for Opts {
    type Err = TomlPathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = PathBuf::from(s);
        if !path.exists() {
            return Err(TomlPathError::IOError);
        }
        Ok(Opts{
            toml: path,
            ..Default::default()
        })
    }
}

#[derive(Debug)]
pub enum TomlPathError {
    IOError
}

impl Display for Opts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.toml.as_os_str().to_str().unwrap())
    }
}

impl Options {
    pub fn init() -> Opts {
        let default_toml = "target.toml";
        let help = options().run_inner(&["--help"])
            .unwrap_err()
            .unwrap_stdout();

        let ret = |a: &str | {
            println!("file not found: {}", a);
            println!("{}", help);
            process::exit(-1);
        };

        if !PathBuf::from(default_toml).exists() {
            ret(default_toml)
        }

        let arg = options().run();
        match arg.encrypt {
            None => {}
            Some(a) => {
                println!("{}", encrypt(&a));
                process::exit(0);
            }
        }

        let toml = match arg.file {
            None => {
                PathBuf::from(default_toml)
            },
            Some(toml) => {
                if !toml.exists() {
                    ret(toml.to_str().unwrap());
                };
                toml
            }
        };
        Opts{
            toml,
            debug: arg.debug,
            insecure: arg.insecure,
        }
    }
}
