use clap::Parser;
use std::env;

#[derive(Parser)]
struct Args {
    cmd: String,
    bookmark: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let cwd = env::current_dir()?;

    println!(
        "cmd: {:?}, bookmark: {:?}, cwd: {:?}",
        args.cmd,
        args.bookmark,
        cwd.display()
    );

    Ok(())
}
