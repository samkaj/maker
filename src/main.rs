use std::fs;

fn main() {
    let repo = walk(".".to_string());
    for file in repo {
        println!("{}", file);
    }
}

/// Performs a depth first search and returns the paths to all files
/// TODO: include ignore directives
pub fn walk(path: String) -> Vec<String> {
    let contents =
        fs::read_dir(&path).expect("FIXME: Error handling: Could not read the directory");
    let mut repo: Vec<String> = vec![];
    for file in contents {
        match file {
            Ok(f) => {
                let md = f.metadata().unwrap();
                let name = f.path().to_str().unwrap().to_string();
                if md.is_dir() {
                    repo.extend(walk(name.clone()));
                } else {
                    repo.push(name.clone());
                }
            }
            Err(error) => panic!("uh oh {:?}", error),
        }
    }

    return repo;
}
