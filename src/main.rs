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
        "use <php_version>",
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
            _ => invalid_argument_given(),
        }
    } else {
        invalid_argument_given()
    }
}
fn invalid_argument_given() {
    println!("Invalid argument given, for a list of valid arguments type pvm --help or pvm -h");
}
fn show_help(version: &str) {
    println!("PVM running version v{}", version);
    println!("{:<20} : {}", "--list", "List all installed PHP versions");
    println!("{:<20} : {}", "--help", "Show help");
    println!("{:<20} : {}", "-h", "Show help");
    println!("{:<20} : {}", "--current", "Show current PHP running");
    println!(
        "{:<20} : {}",
        "use <php_version>", "Switch to the specified PHP version"
    );
    println!(
        "{:<20} : {}",
        "-i <version>", "Switch to the specified PHP version"
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
    let paths = fs::read_dir(path).unwrap();
    let re = Regex::new(r"php(\d+\.\d+\.\d+)$").unwrap();
    let php_v = return_current_php_v();

    println!("Instaled PHP versions\n");

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
