pub mod file_utils {
    use core::panic;
    use std::{fs::File, io::Read};

    use clap::Parser;

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
    pub fn load_files(pathes: Vec<&'static str>) -> Result<(String, String), String> {
        for path in pathes {
            if let Ok(data) = load_file(path) {
                return Ok((path.to_owned(), data));
            };
        }
        panic!("еблан чтоли");
    }

    pub fn shell_args() -> Args {
        Args::parse()
    }

    #[derive(Parser)]
    #[command(version, about, long_about = None)]
    pub struct Args {
        #[arg(short, long, default_value = "AskaConfig.toml")]
        pub config: String,
    }
}

pub mod shell_utils {
    use core::panic;
    use std::process::Command;

    /// create a proccess and returns stdout.
    /// args: command and args
    pub fn execute_command(args: Vec<&str>) -> Option<String> {
        if args.is_empty() {
            panic!("Empty command for execute in shell.")
        }
        let output_res = Command::new(args[0]).args(&args[1..]).output();
        let output = match output_res {
            Ok(v) => v.stdout,
            Err(_) => return None,
        };
        let from_utf8 = String::from_utf8(output).expect("да я заебался это анврапать уже");
        Some(from_utf8)
    }
}
