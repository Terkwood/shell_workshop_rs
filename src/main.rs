use std::io;
use std::process::Command;

fn main() {
    loop {
        // Read line from standard input
        let line = read_line();

        // "parse" line into executable command
        // ... TODO ...

        // execute the command in a sep process
        let output = sys_call(line);

        // show output
        match output.status.success() {
            true => println!("{}", std::str::from_utf8(&output.stdout[..]).unwrap()),
            _ => panic!(),
        }
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn sys_call(command: String) -> std::process::Output {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process")
}
