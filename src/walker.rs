use std::fs;

/// A type used for getting file paths contained in the root file tree.
pub struct Walker {
    root: String,
    ignore_dirs: Vec<String>,
}

impl Walker {
    /// Create a new Walker.
    pub fn new(root: String, ignore_dirs: Vec<String>) -> Walker {
        return Walker { root, ignore_dirs };
    }

    /// Walk the directory from the `path` field and return all files not within an ignored
    /// directory.
    pub fn walk(&self) -> Vec<String> {
        self.walk_recursive(self.root.clone())
    }

    /// Depth first search where the root is the path.
    fn walk_recursive(&self, path: String) -> Vec<String> {
        let dir = fs::read_dir(path).expect("FIXME: Error handling: Failed to read directory");
        let mut repo: Vec<String> = vec![];
        for file in dir {
            match file {
                Ok(f) => {
                    let md = f.metadata().unwrap();
                    let path = f.path().to_str().unwrap().to_string();
                    if md.is_dir() {
                        if !self.dir_is_ignored(f.file_name().to_str().unwrap().to_string()) {
                            repo.extend(self.walk_recursive(path.clone()));
                        }
                    } else {
                        repo.push(path.clone());
                    }
                }
                Err(error) => panic!("uh oh {:?}", error),
            }
        }

        return repo;
    }

    /// Determine if a directory name should be ignored.
    fn dir_is_ignored(&self, dir_name: String) -> bool {
        return self.ignore_dirs.contains(&dir_name);
    }
}
