use std::path::Path;

use ignore::gitignore::{Gitignore, GitignoreBuilder};

fn main() {
    let ignore_a = load_ignore(".ignore-a");
    let ignore_b = load_ignore(".ignore-b");

    let match_a = ignore_a.matched("/my_dir/my_file.txt", false);
    let match_b = ignore_b.matched("/my_dir/my_file.txt", false);

    println!("ignore /my_dir/my_file.txt?");
    println!("a: {}", match_a.is_ignore());
    println!("b: {}", match_b.is_ignore());
}

fn load_ignore(path: impl AsRef<Path>) -> Gitignore {
    println!("loading ignore file {:?}", path.as_ref());
    let mut ignore_builder = GitignoreBuilder::new("/");
    ignore_builder.add(path);
    ignore_builder.build().unwrap()
}
