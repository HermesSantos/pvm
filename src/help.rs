pub mod help_handler {
    use crate::print_logo;

    pub fn show_help(version: &str) {
        println!("PVM running version v{}\n", version);
        print_logo::print_full_logo();
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
            "-i <php_version>", "Install the specified PHP version"
        );
        println!(
            "{:<20} : {}",
            "--install <php_version>", "Install the specified PHP version"
        );
        println!(
            "{:<20} : {}",
            "--remove <php_version>", "Remove the specified PHP version"
        );
        println!(
            "{:<20} : {}",
            "-r <php_version>", "Remove the specified PHP version"
        );
    }
}
