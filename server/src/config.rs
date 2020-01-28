use structopt::StructOpt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Mode{
    Drop,Parse,Pass
}

impl FromStr for Mode{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "Drop" => Ok(Mode::Drop),
            "Parse" => Ok(Mode::Parse),
            "Pass" => Ok(Mode::Pass),
            _ => Err("Kaputt".to_string())
        }
    }
}

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "basic")]
pub struct Opt {
    #[structopt(short, long, default_value = "127.0.0.1:31234")]
    pub bind_addr: String,
    #[structopt(short, long, default_value = "Drop")]
    pub mode: Mode,
}

