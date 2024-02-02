use clap::Parser;
use std::{env, path::PathBuf};

#[derive(Parser)]
struct Args {
    cmd: String,
    tag: Option<String>,
}

fn validate_bookamrk_name(name: Option<String>) -> bool {
    false
}

fn list_bookmarks() {
    println!("Printing all your current bookmarks...");
}

fn add_bookmark(tag: Option<String>, dir: PathBuf) {
    if !validate_bookamrk_name(tag.clone()) {
        println!("Invalid tag name");
        return;
    }
    println!("Creating tag: {:?}, to location: {:?}", tag, dir);
}

fn remove_bookmark(tag: Option<String>) {
    println!("Removing tag: {:?}", tag);
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let cwd = env::current_dir()?;

    match args.cmd.as_str() {
        "a" | "add" => add_bookmark(args.tag, cwd),
        "rm" | "remove" => remove_bookmark(args.tag),
        "l" | "list" => list_bookmarks(),
        _ => println!("Command not yet implemented...")
    }

    Ok(())
}
