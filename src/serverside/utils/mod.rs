use std::{fs::File, io::Read};

pub fn load_file(path: &str) -> Result<String, String> {
    let file = File::open(path);
    match file {
        Ok(mut opened_file) => {
            let mut contents = String::new();
            if let Err(e) = opened_file.read_to_string(&mut contents) {
                return Err(e.to_string());
            };
            Ok(contents)
        }
        Err(e) => Err(e.to_string()),
    }
}
