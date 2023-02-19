/// ## PSTree-Rust
/// 实现一个GNU/Linux pstree的rust建议仿造版
/// 命令行参数风格使用clap.rs, Derive风格
/// 实现功能:
///
/// pstree -V version
/// 
use clap::Parser;
use std::error::Error;
use std::io::Result;

#[derive(Parser, Debug)]
#[command(about="Display a tree of processed")]
#[command(author="Egg Shaw", version)]
struct Args {
    #[arg(short = 'a', 
            long, 
            about = "show command line arguments",
            default_value = false,
            )]
    arguments: bool,
    #[arg(short = 'H', long = "highlight-pid", about = "highlight this process and its ancestors")]
    hpid: Option<usize>,
    #[arg(short = 'p', long, 
            about = "show PIDs; implies -c",
            default_value = false, )]
    shot_pids: bool, 
}


impl Args {
    pub fn run(self, arg: Args) -> Result<()> {
        Ok(())
    }
}

pub fn parse_arg() -> Result<Args> {
    let args = Args::parse();   
    Ok(args)
}
