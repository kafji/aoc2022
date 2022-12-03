use std::{env, fs::File, path::PathBuf};

fn get_input_path() -> PathBuf {
    let prog_name = {
        let arg0 = env::args().next().unwrap();
        arg0.split('/')
            .last()
            .expect("failed to extract program name")
            .to_owned()
    };
    let file_name = format!("{}_input", prog_name);
    let root_path = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(root_path).join("input").join(file_name)
}

pub fn get_input_file() -> File {
    let path = get_input_path();
    let file = File::open(&path);
    file.unwrap_or_else(|_| panic!("failed to open input file at {}", path.display()))
}
