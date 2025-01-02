use bpaf::*;
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use std::process;
use std::str::FromStr;
use crate::crypt::encrypt;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Options {
    #[bpaf(short, long, argument("ssh passwd"))]
    /// magic crypt
    pub encrypt: Option<String>,
    #[bpaf(short, long, argument("file"))]
    /// toml file
    pub file: Option<PathBuf>,
}

#[derive(Debug)]
pub struct TomlPath(PathBuf);

impl FromStr for TomlPath {
    type Err = TomlPathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = PathBuf::from(s);
        if !path.exists() {
            return Err(TomlPathError::IOError);
        }
        Ok(TomlPath(path))
    }
}

#[derive(Debug)]
pub enum TomlPathError {
    IOError
}

impl Display for TomlPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_os_str().to_str().unwrap())
    }
}

impl Options {
    pub fn init() -> TomlPath {
        let default_toml = "target.toml";
        let help = options().run_inner(&["--help"])
            .unwrap_err()
            .unwrap_stdout();

        let arg = options().run();
        match arg.encrypt {
            None => {}
            Some(a) => {
                println!("{}", encrypt(&a));
                process::exit(0);
            }
        }

        let ret = |a: &str | {
            println!("file not found {}", a);
            println!("{}", help);
            process::exit(-1);
        };

        return match arg.file {
            None => {
                match TomlPath::from_str(default_toml) {
                    Ok(path) => {path}
                    Err(_e) => ret(default_toml),
                }
            },
            Some(a) => {
                if !a.exists() {
                    ret(a.as_os_str().to_str().unwrap());
                };
                TomlPath(a)
            }
        }
    }
}
