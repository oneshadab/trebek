use std::io::stdin;

use trebek::{repl::repl, runner::Runner};


fn main() {
  println!("Trebek shell v0.1");
  repl();
}
