use std::env;

fn main() {
    let valid_args: [&str; 8] = [
        "--list",
        "--help",
        "-h",
        "use <php_version>",
        "-i",
        "--install",
        "--remove",
        "-r",
    ];
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        invalid_argument_given();
        return;
    }
    let first_arg = &args[1];

    if valid_args.contains(&first_arg.as_str()) {
        match first_arg.as_str() {
            "--help" | "-h" => show_help(),
            _ => invalid_argument_given(),
        }
    } else {
        invalid_argument_given()
    }
}
fn invalid_argument_given() {
    println!("Invalid argument given, for a list of valid arguments type pvm --help or pvm -h");
}
fn show_help() {
    println!(
        "PVM running version v1.0.0\nSupported arguments: \n --list, --help, -h, use <php_version>, -i, --install, -r, --remove"
    );
}
