use std::error;
use std::fmt::Display;

#[derive(Debug)]
pub struct ProgramConfig {
    pub server: bool,
    pub watch: bool,
    pub directory: String,
}

#[derive(Debug)]
struct ProgramError {
    msg: String,
}
impl error::Error for ProgramError {}
impl Display for ProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl ProgramConfig {
    pub fn new() -> Self {
        Self {
            server: false,
            watch: false,
            directory: String::from("blogs"),
        }
    }

    pub fn check(&self) -> Result<(), Box<dyn error::Error>> {
        // if !self.server && self.watch {
        //     eprintln!("[ERROR] `-w` cannot be set when `-s` not present!");
        //     return Err(Box::new(ProgramError {
        //         msg: String::from("Options not acceptable!"),
        //     }));
        // }

        Ok(())
    }
}

pub fn parse_options(ops: &Vec<String>) -> ProgramConfig {
    let mut config = ProgramConfig::new();

    let mut i = 0;
    while i < ops.len() {
        match ops[i].as_str() {
            "--server" | "-s" => config.server = true,
            "--watch" | "-w" => config.watch = true,
            "--dir" | "-d" => {
                if i < ops.len() - 1 {
                    config.directory = ops[i + 1].clone();
                    i += 1;
                } else {
                    panic!("[ERROR] `--dir|-d` expect a relative directory from working directory");
                }
            }
            _ => (),
        }
        i += 1;
    }

    config
}

#[macro_export]
macro_rules! label_red {
    ($str:expr) => {
        $str.red()
    };
}
#[macro_export]
macro_rules! label_yellow {
    ($str:expr) => {
        $str.yellow()
    };
}
#[macro_export]
macro_rules! label_green {
    ($str:expr) => {
        $str.green()
    };
}

#[allow(unused_imports)]
pub(crate) use label_red;
#[allow(unused_imports)]
pub(crate) use label_yellow;
