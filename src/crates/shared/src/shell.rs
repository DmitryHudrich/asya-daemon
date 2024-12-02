use std::process::Command;

use clap::Parser;

/// create a proccess and returns stdout.
/// args: command and args
pub fn execute_command(args: Vec<&str>) -> Option<String> {
    assert!(!args.is_empty(), "Empty command to execute in shell");

    Command::new(args[0])
        .args(&args[1..])
        .output()
        .map(|out| {
            String::from_utf8(out.stdout)
                .expect("Any shell command output must be valid UTF-8 string")
        })
        .ok()
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
