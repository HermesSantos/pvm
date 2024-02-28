use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let php_dir: &str = "C:\\wamp64\\bin\\php";
    let valid_args: [&str; 9] = [
        "--list",
        "--help",
        "--current",
        "-h",
        "--use",
        "-i",
        "--install <version>",
        "--remove",
        "-r",
    ];
    let version: &str = "1.0.0";
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        invalid_argument_given();
        return;
    }
    let first_arg = &args[1];

    if valid_args.contains(&first_arg.as_str()) {
        match first_arg.as_str() {
            "--help" | "-h" => show_help(&version),
            "-cv" | "--current" => current_php_v(),
            "--list" => list_php_versions(&php_dir),
            "--use" => switch_php_version(&args[2]),
            _ => invalid_argument_given(),
        }
    } else {
        invalid_argument_given()
    }
}
fn switch_php_version(arg: &str) {
    let php_dir: &str = "C:\\wamp64\\bin\\php";
    if arg.len() < 3 {
        println!("PHP version was not given.\nYou can type pvm -h or --help to look for details.")
    } else {
        if verify_arg(&arg, &php_dir) == true {
            let full_path_env: String = env::var("path").unwrap();
            let re = Regex::new(r"php(\d+\.\d+\.\d+)$").unwrap();
            let path_to_be_modified = return_current_path();
            if let Some(php_number_version) = re.captures(&path_to_be_modified) {
                if let Some(php_last) = php_number_version.get(1) {
                    let new_path = path_to_be_modified.replace(php_last.as_str(), arg);
                    let new_full_path = full_path_env.replace(&path_to_be_modified, &new_path);
                    // here the PVM actually changes the full path
                    println!("{:?}", new_full_path);
                }
            }
        } else {
            println!("PHP version especified is not installed.\nRun pvm --i <php_version_desired> if you wish to install this php version")
        }
    }
}

fn verify_arg(arg_given: &str, path: &str) -> bool {
    let re = Regex::new(r"php(\d+\.\d+\.\d+)$").unwrap();

    if let Ok(paths) = fs::read_dir(path) {
        for path in paths {
            if let Ok(entry) = path {
                if let Some(file_name) = entry.file_name().into_string().ok() {
                    if let Some(captures) = re.captures(&file_name) {
                        if let Some(version) = captures.get(1) {
                            if version.as_str() == arg_given {
                                // Todo: create a struck to handle this
                                return true;
                            }
                        }
                    }
                }
            }
        }
        return false;
    }
    return false;
}

fn invalid_argument_given() {
    println!("Invalid argument given. For a list of valid arguments type pvm --help or pvm -h");
}
fn print_logo() {
    println!("      ___                       ___     ");
    println!("     /\\  \\        ___          /\\  \\    ");
    println!("    /::\\  \\      /\\  \\        |::\\  \\   ");
    println!("   /:/\\:\\__\\     \\:\\  \\       |:|:\\  \\  ");
    println!("  /:/ /:/  /      \\:\\  \\    __|:|:\\   \\ ");
    println!(" /:/_/:/  /   ___  \\:\\__\\  /::::|_\\:\\__\\");
    println!(" \\:\\/:/  /   /\\  \\ |:|  |  \\:\\--\\  \\/__/");
    println!("  \\::/__/    \\:\\  \\|:|  |   \\:\\  \\     ");
    println!("   \\:\\  \\     \\:\\__|:|__|    \\:\\  \\    ");
    println!("    \\:\\__\\     \\::::/__/      \\:\\__\\   ");
    println!("     \\/__/      \\____/         \\/__/   ");
    println!("\n");
}
fn show_help(version: &str) {
    println!("PVM running version v{}\n", version);
    print_logo();
    println!("Functions:");
    println!(
        "{:<20} : {}",
        "--list", "List all installed PHP versions (ok)"
    );
    println!("{:<20} : {}", "--help", "Show help (ok)");
    println!("{:<20} : {}", "-h", "Show help (ok)");
    println!("{:<20} : {}", "--current", "Show current PHP running (ok)");
    println!(
        "{:<20} : {}",
        "--use <php_version>",
        "Switch to the specified PHP version (ok, needs to be implemented to real path)"
    );
    println!(
        "{:<20} : {}",
        "-i <version>", "Install the specified PHP version"
    );
    println!(
        "{:<20} : {}",
        "--install <version>", "Install the specified PHP version"
    );
    println!(
        "{:<20} : {}",
        "--remove <version>", "Remove the specified PHP version"
    );
    println!(
        "{:<20} : {}",
        "-r <version>", "Remove the specified PHP version"
    );
}

fn return_current_path() -> String {
    if let Ok(path) = env::var("path") {
        let paths: Vec<&str> = path.split(';').collect();
        for sub_path in paths {
            if sub_path.contains("wamp64\\bin\\php") {
                return sub_path.to_string();
            }
        }
    }
    return "".to_string();
}

fn current_php_v() {
    if let Ok(path) = env::var("path") {
        let parts: Vec<&str> = path.split(';').collect();

        for part in parts {
            if part.contains("wamp64\\bin\\php\\") {
                let re = Regex::new(r"php(\d+\.\d+\.\d+)$").unwrap();

                if let Some(capturas) = re.captures(part) {
                    if let Some(versao) = capturas.get(1) {
                        println!("Current PHP running: {}", versao.as_str());
                    }
                } else {
                    println!("No PHP versions found on path");
                }
            }
        }
    } else {
        println!("erro")
    }
}

fn return_current_php_v() -> String {
    if let Ok(path) = env::var("path") {
        let parts: Vec<&str> = path.split(';').collect();

        for part in parts {
            if part.contains("wamp64\\bin\\php\\") {
                let re = Regex::new(r"php(\d+\.\d+\.\d+)$").unwrap();

                if let Some(capturas) = re.captures(part) {
                    if let Some(versao) = capturas.get(1) {
                        return versao.as_str().to_string();
                    }
                } else {
                    return "Error".to_string();
                }
            }
        }
        return "Error".to_string();
    } else {
        return "Error".to_string();
    }
}

fn list_php_versions(path: &str) {
    let re = Regex::new(r"php(\d+\.\d+\.\d+)$").unwrap();
    let php_v = return_current_php_v();

    println!("Instaled PHP versions\n");
    if let Ok(paths) = fs::read_dir(path) {
        for path in paths {
            if let Ok(entry) = path {
                if let Some(file_name) = entry.file_name().into_string().ok() {
                    if let Some(captures) = re.captures(&file_name) {
                        if let Some(version) = captures.get(1) {
                            if version.as_str() == php_v {
                                println!("{} * (Currently using)", version.as_str());
                            } else {
                                println!("{}", version.as_str());
                            }
                        }
                    }
                }
            }
        }
    }
}
