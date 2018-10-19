use std::io;

fn main() {
    loop {
        // Read line from standard input
        let line = read_line();

        // "parse" line into executable command
        // execute the command in a sep process
        // show output
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}
