use std::io::{self, Write};

use trebek::repl::Repl;

fn main() {
    println!("Trebek shell v0.1");
    let mut repl = Repl::new();

    loop {
        print!("> ");
        io::stdout().flush().ok().expect("Could not flush stdout");

        repl.next();
    }
}
