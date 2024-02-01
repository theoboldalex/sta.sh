use clap::Parser;
use std::{env, path::PathBuf};

#[derive(Parser)]
struct Args {
    cmd: String,
    tag: Option<String>,
}

fn list_bookmarks() {
    println!("Printing all your current bookmarks...");
}

fn add_bookmark(tag: Option<String>, dir: PathBuf) {
    println!("Creating tag: {:?}, to location: {:?}", tag, dir);
}

fn remove_bookmark(tag: Option<String>) {
    println!("Removing tag: {:?}", tag);
}

fn rename_bookmark(tag: Option<String>) {
    println!("Renaming tag: {:?}", tag);
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let cwd = env::current_dir()?;

    match args.cmd.as_str() {
        "a" | "add" => add_bookmark(args.tag, cwd),
        "r" | "remove" => remove_bookmark(args.tag),
        "l" | "list" => list_bookmarks(),
        _ => println!("Command not yet implemented...")
    }

    Ok(())
}
