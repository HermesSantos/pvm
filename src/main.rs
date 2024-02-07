use regex::Regex;
use std::env;

fn main() {
    let valid_args: [&str; 10] = [
        "--list",
        "--help",
        "--current",
        "-cv",
        "-h",
        "use <php_version>",
        "-i",
        "--install",
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
    println!(
        "PVM running version v{}\nSupported arguments: \n --list, --help, -h, use <php_version>, -i, --install, -r, --remove",
        version
    );
}
fn current_php_v() {
    if let Ok(path) = env::var("path") {
        let parts: Vec<&str> = path.split(';').collect();
        for part in parts {
            if part.contains("wamp64\\bin\\php\\") {
                let re = Regex::new(r"php(\d+\.\d+\.\d+)$").unwrap();

                // Verifica se há correspondência na string
                if let Some(capturas) = re.captures(part) {
                    // Extrai a parte correspondente à versão
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
