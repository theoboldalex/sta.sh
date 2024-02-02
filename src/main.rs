use clap::Parser;
use std::{env, path::PathBuf};

#[derive(Parser)]
struct Args {
    cmd: String,
    tag: Option<String>,
}

fn validate_tag_name(tag: &str) -> bool {
    let is_valid = tag.chars().all(|c| c.is_alphabetic() || c == '-' || c == '_');
    is_valid
}

fn list_bookmarks() {
    println!("Printing all your current bookmarks...");
}

fn add_bookmark(tag: Option<String>, dir: PathBuf) {
    if let Some(tag) = &tag {
        if !validate_tag_name(tag.clone().as_str()) {
            println!("Invalid tag name");
            return;
        }
    }
    println!("Creating tag: {:?}, to location: {:?}", tag, dir);
}

fn remove_bookmark(tag: Option<String>) {
    if let Some(tag) = &tag {
        println!("Removing tag: {:?}", tag);
    }
}

fn navigate_to_bookmark(tag: Option<String>) {
    if let Some(tag) = &tag {
        println!("Navigating to {:?}", tag);
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let cwd = env::current_dir()?;

    match args.cmd.as_str() {
        "a" | "add" => add_bookmark(args.tag, cwd),
        "rm" | "remove" => remove_bookmark(args.tag),
        "l" | "list" => list_bookmarks(),
        "go" => navigate_to_bookmark(args.tag),
        _ => println!("Command not yet implemented...")
    }

    Ok(())
}
