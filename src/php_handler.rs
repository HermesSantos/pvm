pub mod path_handler {
    use regex::Regex;
    use std::env;
    use std::fs;

    pub fn list_php_versions(path: &str) {
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
    pub fn switch_php_version(arg: &str) {
        let php_dir: &str = "C:\\wamp64\\bin\\php";
        if arg.len() < 3 {
            println!(
                "PHP version was not given.\nYou can type pvm -h or --help to look for details."
            )
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

    pub fn current_php_v() {
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
}
