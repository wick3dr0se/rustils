// https://man.archlinux.org/man/mkdir.1

use std::fs;

fn make_dir(dst: &str, is_parent: bool) -> bool {
    let res = if is_parent {
        fs::create_dir_all(dst)
    } else {
        fs::create_dir(dst)
    };
    
    if let Err(e) = res {
        eprintln!("Error creating directory '{}': {}", dst, e);
        return false
    }

    println!("Created directory '{}'", dst);
    return true
}

fn main() {
    let mut opts = clop::get_opts();
    
    if opts.scrap.len() < 1 {
        panic!("Usage: mkdir [OPTION]... <DIRECTORY>...");
    }

    let has_parent = opts.has(&["p", "parents"], false).is_ok();

    for arg in &opts.scrap {
        make_dir(arg, has_parent);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_dir() {
        assert!(make_dir("a", false));

        let _ = fs::remove_dir("a");
    }

    #[test]
    fn test_make_dir_parent() {
        assert!(make_dir("b/c", true));

        let _ = fs::remove_dir_all("b");
    }
}