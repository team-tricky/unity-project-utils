use std::fs::{self, DirEntry};
use std::path::Path;
use std::collections::HashSet;


/// Returns True if dir was removed
fn remove_inner_meta(dir_entry: &DirEntry, ignore_paths: &HashSet<&str>) -> bool {
    
    let path = dir_entry.path();
    let file_name = dir_entry.file_name().into_string().unwrap();

    if ignore_paths.contains(&*file_name) {
        return false;
    }

    if dir_entry.metadata().unwrap().is_dir() {
        return clean_meta(&path, ignore_paths);
    } 
    
    if file_name.ends_with(".meta") {
        let dirname = path.parent().unwrap().join(path.file_stem().unwrap());

        if !dirname.exists() {
            println!("Remove meta file: {:?}, {:?}", path, dirname);
            fs::remove_file(path).unwrap();
            return true;
        }
    }

    return false;
}


/// Retutns true if path was removed
fn clean_meta(path: &Path, ignore_paths: &HashSet<&str>) -> bool {

    let mut should_remove = true;

    for child_entry in fs::read_dir(path).unwrap() {
        
        let removed = remove_inner_meta(&child_entry.unwrap(), ignore_paths);
        should_remove &= removed;
    }

    if should_remove {
        println!("Removing {}", path.to_str().unwrap());
        std::fs::remove_dir(path).unwrap();
    }
    return should_remove
}

fn main() {
    let path = std::env::args().nth(1).expect("Specify path");

    let ignore_paths = vec![
        ".git",
        ".hg",
        ".hgcheck",
        "Library",
        "obj",
    ].into_iter().collect();

    let path = Path::new(&path);
    clean_meta(path, &ignore_paths);
}
