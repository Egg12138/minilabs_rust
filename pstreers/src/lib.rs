use std::io::{ BufRead, BufReader, Result};
use clap::{Parser, Arg, builder::Str};

const SIMPLE_INFO: &str = " pstree shows running processes as a tree.  The tree is rooted  at  ei‐
ther  pid or init if pid is omitted.  If a user name is specified, all
process trees rooted at processes owned by that user are shown.";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = SIMPLE_INFO)]
pub(crate) struct Config {
  #[arg(short = 'p', long)]
  show_pids: Option<bool>,
  #[arg(short = 'a', long)]
  ascii: Option<bool>,
}

pub(crate) fn get_args() -> Result<Config>{
  let cli = Config::parse();
  cli
}

pub(crate) fn run(cli: Config) -> Result<()> {

}
}

fn print_node(
      name: &str, 
    ) {
      todo!()
}