use std::env;

use git_state;

fn main() {
    let path = match env::current_dir() {
        Ok(path) => path,
        Err(_) => env::args()
            .skip(1)
            .next()
            .expect("could not determine git directory")
            .into(),
    };

    println!("{}", git_state::git_state(&path).unwrap());
}
