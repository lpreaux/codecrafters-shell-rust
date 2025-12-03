use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub fn find_executable_in_path(cmd_name: &str) -> Option<PathBuf> {
    if let Ok(path_var) = std::env::var("PATH") {
        for path in std::env::split_paths(&path_var) {
            let executable_path = path.join(cmd_name);
            if executable_path.is_file() {
                if let Ok(metadata) = executable_path.metadata() {
                    let permissions = metadata.permissions();
                    if permissions.mode() & 0o111 != 0 {
                        return Some(executable_path);
                    }
                }
            }
        }
    }
    None
}

pub fn find_executables_with_prefix(prefix: &str) -> Vec<String> {
    let mut executables = Vec::new();

    if let Ok(path_var) = std::env::var("PATH") {
        for path in std::env::split_paths(&path_var) {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        if file_name.starts_with(prefix) {
                            if let Ok(metadata) = entry.metadata() {
                                if metadata.is_file() {
                                    let permissions = metadata.permissions();
                                    if permissions.mode() & 0o111 != 0 {
                                        executables.push(file_name);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Trier et dédupliquer (plusieurs dossiers PATH peuvent avoir le même exe)
    executables.sort();
    executables.dedup();

    executables
}
