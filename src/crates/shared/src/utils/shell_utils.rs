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
