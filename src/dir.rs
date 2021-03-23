fn get_relative_path(path: &str) -> String {
    path.to_string()
        .replace("\\", "/")
        .split("/")
        .last()
        .unwrap()
        .to_string()
}

pub fn file_list(path: &str) -> Vec<String> {
    let mut files: Vec<String> = vec![];

    match std::fs::read_dir(&format!("{}", path)) {
        Ok(r) => {
            for f in r {
                match f {
                    Ok(dir_entry) => match dir_entry.file_type() {
                        Ok(x) => {
                            if x.is_file() {
                                match dir_entry.path().to_str() {
                                    Some(path) => {
                                        let path = get_relative_path(path);
                                        files.push(path);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        _ => {}
    }

    files
}

pub fn list(path: &str) -> Vec<String> {
    let mut files: Vec<String> = vec![];

    match std::fs::read_dir(&format!("{}", path)) {
        Ok(r) => {
            for f in r {
                match f {
                    Ok(dir_entry) => match dir_entry.path().to_str() {
                        Some(path) => {
                            let path = get_relative_path(path);
                            files.push(path);
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        _ => {}
    }

    files
}
