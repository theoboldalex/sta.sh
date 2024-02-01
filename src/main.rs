use clap::Parser;
use std::{env, path::PathBuf};

#[derive(Parser)]
struct Args {
    cmd: String,
    tag: String,
}

fn list_bookmarks() {
}

fn add_bookmark(tag: String, dir: PathBuf) {
    println!("Creating tag: {}, to location: {:?}", tag, dir);
}

fn remove_bookmark(tag: String) {
    println!("Removing tag: {}", tag);
}

fn rename_bookmark(tag: String) {
    println!("Renaming tag: {}", tag);
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let cwd = env::current_dir()?;

    match args.cmd.as_str() {
        "add" => add_bookmark(args.tag, cwd),
        _ => println!("Command not yet implemented...")
    }

    Ok(())
}
