use clap::{Arg, ArgAction, Command};
use std::env;
use std::fs;
use std::path::Path;
use std::io::{self, Write};
use colored::*;
use fs_extra::dir::{copy as copy_dir, CopyOptions};
use fs_extra::file::{copy as copy_file, CopyOptions as FileCopyOptions};

#[cfg(target_os = "windows")]
use winapi::um::shellapi::ShellExecuteW;
#[cfg(target_os = "windows")]
use winapi::um::winnt::LPCWSTR;
#[cfg(target_os = "windows")]
use std::ptr::null_mut;
#[cfg(target_os = "windows")]
use std::ffi::OsString;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
#[cfg(target_os = "windows")]
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
#[cfg(target_os = "windows")]
use winapi::um::securitybaseapi::CheckTokenMembership;
#[cfg(target_os = "windows")]
use winapi::um::winnt::{HANDLE, TOKEN_QUERY, TOKEN_ELEVATION};
#[cfg(target_os = "windows")]
use winapi::shared::minwindef::BOOL;
#[cfg(target_os = "windows")]
use winapi::um::handleapi::CloseHandle;

fn main() {
    let matches = Command::new("xfetch")
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
            Command::new("import")
                .about("Imports a file or directory")
                .arg(
                    Arg::new("source")
                        .help("The source file or directory")
                        .required(true),
                )
                .arg(
                    Arg::new("destination")
                        .help("The destination directory")
                        .required(false),
                ),
        )
        .get_matches();

    if !is_elevated() {
        println!("Requesting elevated privileges...");
        run_as_elevated();
        return;
    }

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
    } else if let Some(matches) = matches.subcommand_matches("import") {
        let source = matches.get_one::<String>("source").unwrap();
        let destination = matches.get_one::<String>("destination").map_or(".", |s| s.as_str());
        import_path(source, destination);
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

fn import_path(src: &str, dest: &str) {
    let src_path = Path::new(src);
    let dest_path = Path::new(dest).join(src_path.file_name().unwrap());

    if dest_path.exists() && !ask_for_overwrite(dest) {
        print_warning("Skipped", &format!("Item not overwritten: {}", dest));
        return;
    }

    if src_path.is_file() {
        let mut options = FileCopyOptions::new();
        options.overwrite = true;
        match copy_file(src_path, &dest_path, &options) {
            Ok(_) => print_info("Success", &format!("File imported to: {}", dest)),
            Err(e) => print_error("Error", &format!("Failed to import file: {}", e)),
        }
    } else if src_path.is_dir() {
        let mut options = CopyOptions::new();
        options.copy_inside = true;
        options.overwrite = true;
        match copy_dir(src_path, dest, &options) {
            Ok(_) => print_info("Success", &format!("Directory imported to: {}", dest)),
            Err(e) => print_error("Error", &format!("Failed to import directory: {}", e)),
        }
    } else {
        print_error("Error", "The specified path does not exist.");
    }
}

#[cfg(target_os = "windows")]
fn is_elevated() -> bool {
    let mut is_elevated = false;
    unsafe {
        let mut token: HANDLE = null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) != 0 {
            let mut elevation = TOKEN_ELEVATION {
                TokenIsElevated: 0,
            };
            let mut is_member: BOOL = 0;
            if CheckTokenMembership(null_mut(), &mut elevation as *mut _ as *mut _, &mut is_member) != 0 {
                is_elevated = is_member != 0;
            }
            CloseHandle(token);
        }
    }
    is_elevated
}

#[cfg(target_os = "windows")]
fn run_as_elevated() {
    use winapi::um::winuser::SW_SHOWNORMAL;
    let args: Vec<String> = env::args().collect();
    let mut args_wide: Vec<Vec<u16>> = args.iter().map(|arg| OsString::from(arg).encode_wide().collect()).collect();
    for arg in &mut args_wide {
        arg.push(0);
    }

    unsafe {
        let lp_file: LPCWSTR = args_wide[0].as_ptr();
        let lp_parameters: LPCWSTR = args_wide[1..].iter().flat_map(|arg| arg.iter()).cloned().collect::<Vec<u16>>().as_ptr();

        ShellExecuteW(
            null_mut(),
            OsString::from("runas").encode_wide().chain(Some(0)).collect::<Vec<u16>>().as_ptr(),
            lp_file,
            lp_parameters,
            null_mut(),
            SW_SHOWNORMAL,
        );
    }
}

#[cfg(not(target_os = "windows"))]
fn is_elevated() -> bool {
    env::var("USER").map_or(false, |user| user == "root")
}

#[cfg(not(target_os = "windows"))]
fn run_as_elevated() {
    let args: Vec<String> = env::args().collect();
    let status = std::process::Command::new("sudo")
        .args(&args)
        .status()
        .expect("failed to execute process");

    if status.success() {
        println!("Elevated privileges granted.");
    } else {
        println!("Failed to obtain elevated privileges.");
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
