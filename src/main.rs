use trebek::repl::Repl;

fn main() {
    println!("Trebek v0.1");
    println!("type (help) for help, (exit) or CTRL-C to exit");

    let mut repl = Repl::new();
    loop {
        repl.prompt();
    }
}
