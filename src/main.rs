use std::{
    io::{self, BufRead, Write},
    process::{Command, Stdio},
};

fn handle_line(s: &str) {
    let process = match Command::new("jq").stdin(Stdio::piped()).spawn() {
        Err(why) => panic!("couldn't spawn jq: {}", why),
        Ok(process) => process,
    };

    match process.stdin.unwrap().write_all(s.as_bytes()) {
        Err(why) => panic!("couldn't write to jq stdin: {}", why),
        Ok(_) => return,
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    for line in lines {
        match line {
            Ok(l) => handle_line(&l),
            Err(error) => panic!("error while parsing line: {}", error),
        }
    }
}
