use clap::{Arg, ArgAction, Command};
use std::fs;
use std::path::Path;
use std::io::{self, Write};
use colored::*;

fn main() {
    let matches = Command::new("cortex")
        .version("0.1.0")
        .author("Neo Mannskär <neo.mannskar@gmail.com>")
        .about("File and Directory Management")
        .subcommand(
            Command::new("create")
                .about("Creates a file or directory")
                .arg(
                    Arg::new("path")
                        .help("The path to the file or directory to create")
                        .required(true),
                )
                .arg(
                    Arg::new("directory")
                        .help("Create a directory instead of a file")
                        .short('d')
                        .long("directory")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("copy")
                .about("Copies a file or directory")
                .arg(
                    Arg::new("source")
                        .help("The source file or directory")
                        .required(true),
                )
                .arg(
                    Arg::new("destination")
                        .help("The destination file or directory")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("paste")
                .about("Pastes a file or directory")
                .arg(
                    Arg::new("source")
                        .help("The source file or directory")
                        .required(true),
                )
                .arg(
                    Arg::new("destination")
                        .help("The destination file or directory")
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("create") {
        let path = matches.get_one::<String>("path").unwrap();
        let is_dir = matches.get_flag("directory");
        if is_dir {
            if Path::new(path).exists() && !ask_for_overwrite(path) {
                print_warning("Skipped", &format!("Directory not overwritten: {}", path));
                return;
            }
            if let Err(e) = fs::create_dir(path) {
                print_error("Error", &format!("Failed to create directory: {}", e));
            } else {
                print_info("Success", &format!("Directory created: {}", path));
            }
        } else {
            if Path::new(path).exists() && !ask_for_overwrite(path) {
                print_warning("Skipped", &format!("File not overwritten: {}", path));
                return;
            }
            if let Err(e) = fs::File::create(path) {
                print_error("Error", &format!("Failed to create file: {}", e));
            } else {
                print_info("Success", &format!("File created: {}", path));
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("copy") {
        let source = matches.get_one::<String>("source").unwrap();
        let destination = matches.get_one::<String>("destination").unwrap();
        copy_item(source, destination);
    } else if let Some(matches) = matches.subcommand_matches("paste") {
        let source = matches.get_one::<String>("source").unwrap();
        let destination = matches.get_one::<String>("destination").unwrap();
        paste_item(source, destination);
    }
}

fn ask_for_overwrite(path: &str) -> bool {
    println!(
        "{}{}{}{} already exists. Overwrite? (y/n): ",
        "Warning".bold().magenta(),
        ": ".bold().white(),
        path.bold().green(),
        ": ".bold().white()
    );
    io::stdout().flush().unwrap();
    let mut response = String::new();
    io::stdin().read_line(&mut response).unwrap();
    matches!(response.trim(), "y" | "Y")
}

fn copy_item(source: &str, destination: &str) {
    let src_path = Path::new(source);
    let dest_path = Path::new(destination);

    if dest_path.exists() && !ask_for_overwrite(destination) {
        print_warning("Skipped", &format!("Item not overwritten: {}", destination));
        return;
    }

    if src_path.is_dir() {
        match fs::copy(source, destination) {
            Ok(_) => print_info("Success", &format!("Directory copied to: {}", destination)),
            Err(e) => print_error("Error", &format!("Failed to copy directory: {}", e)),
        }
    } else {
        match fs::copy(source, destination) {
            Ok(_) => print_info("Success", &format!("File copied to: {}", destination)),
            Err(e) => print_error("Error", &format!("Failed to copy file: {}", e)),
        }
    }
}

fn paste_item(source: &str, destination: &str) {
    let src_path = Path::new(source);
    let dest_path = Path::new(destination);

    if dest_path.exists() && !ask_for_overwrite(destination) {
        print_warning("Skipped", &format!("Item not overwritten: {}", destination));
        return;
    }

    if src_path.is_dir() {
        match fs::copy(source, destination) {
            Ok(_) => print_info("Success", &format!("Directory pasted to: {}", destination)),
            Err(e) => print_error("Error", &format!("Failed to paste directory: {}", e)),
        }
    } else {
        match fs::copy(source, destination) {
            Ok(_) => print_info("Success", &format!("File pasted to: {}", destination)),
            Err(e) => print_error("Error", &format!("Failed to paste file: {}", e)),
        }
    }
}

fn print_error(title: &str, message: &str) {
    eprintln!(
        "{}{}{}",
        title.bold().red(),
        ": ".bold().white(),
        message.bold().white()
    );
}

fn print_info(title: &str, message: &str) {
    eprintln!(
        "{}{}{}",
        title.bold().green(),
        ": ".bold().white(),
        message.bold().white()
    );
}

fn print_warning(title: &str, message: &str) {
    eprintln!(
        "{}{}{}",
        title.bold().yellow(),
        ": ".bold().white(),
        message.bold().white()
    );
}
