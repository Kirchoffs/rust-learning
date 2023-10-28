#[cfg(test)]
mod test {
    use std::path::PathBuf;

    #[test]
    fn pathbuf_demo() {
        let mut path = PathBuf::new();
        path.push("dir");
        path.push("file");
        println!("Path: {:?}", path);
    
        let parent = path.parent();
        match parent {
            Some(parent_path) => println!("Parent: {:?}", parent_path),
            None => println!("No parent directory"),
        }
    
        let file_name = path.file_name();
        match file_name {
            Some(name) => println!("File name: {:?}", name),
            None => println!("No file name"),
        }
    
        if path.exists() {
            println!("Path exists");
        } else {
            println!("Path does not exist");
        }
    }
}
