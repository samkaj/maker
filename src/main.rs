use walker::Walker;

pub mod walker;

fn main() {
    let root = String::from(".");
    let ignore_dirs = vec![String::from("target"), String::from(".git")];
    let walker = Walker::new(root, ignore_dirs);

    let files = walker.walk();
    for file in files {
        println!("{}", file);
    }
}
