use std::fs;

fn main() {
    for file in fs::read_dir("target/deploy")
        .unwrap()
        .filter_map(Result::ok)
    {
        let path = file.path();
        fs::copy(&path, path.file_name().unwrap()).unwrap();
    }
}
