use error_handler::error_handler::invalid_argument_given;
use help::help_handler::show_help;
use php_handler::path_handler::switch_php_version;
use std::env;

mod error_handler;
mod help;
mod php_handler;
mod print_logo;
mod verify_server;

fn main() {
    let php_dir: &str = "C:\\wamp64\\bin\\php";
    let valid_args: [&str; 10] = [
        "--list",
        "--help",
        "--current",
        "-h",
        "--use",
        "-i",
        "--install <version>",
        "--remove",
        "--testes",
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
            "-cv" | "--current" => php_handler::path_handler::current_php_v(),
            "--list" => php_handler::path_handler::list_php_versions(&php_dir),
            "--use" => switch_php_version(&args[2]),
            "--testes" => verify_server::verify_server(),
            _ => invalid_argument_given(),
        }
    } else {
        invalid_argument_given()
    }
}
