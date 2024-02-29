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
