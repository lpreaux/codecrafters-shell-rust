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
