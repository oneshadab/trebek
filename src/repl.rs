use std::{
    fs::create_dir_all,
    io::{stdout, Write},
    path::PathBuf,
    process,
};

use rustyline::{error::ReadlineError, Editor};

use crate::{
    misc::RuntimeResult,
    runtime::Runtime,
};

pub struct Repl {
    pub runtime: Runtime,
}

impl Repl {
    pub fn new() -> Repl {
        Repl {
            runtime: Runtime::new(),
        }
    }

    pub fn prompt(&mut self) {
        let program = self.read().unwrap();
        let output = match self.eval(program) {
            Ok(out) => out,
            Err(e) => {
                format!("Error: {}", e.to_string())
            }
        };
        println!("{}", output);
    }

    pub fn read(&self) -> Result<String, String> {
        let mut lines: Vec<String> = vec![];

        let mut depth = 0;

        let mut rl = Editor::<()>::new();
        let history_path = self.get_history_path()?;

        if rl.load_history(&history_path).is_err() {
            // No previous history
        }

        loop {
            stdout().flush().ok().expect("Could not flush stdout");

            let prompt = if lines.len() == 0 { ">>> " } else { "... " };

            let line = match rl.readline(prompt) {
                Ok(line) => line,
                Err(ReadlineError::Interrupted) => {
                    process::exit(0);
                }
                Err(ReadlineError::Eof) => {
                    process::exit(0);
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            };

            for ch in line.chars() {
                if ch == '(' {
                    depth += 1;
                }
                if ch == ')' {
                    depth -= 1;
                }
            }

            rl.add_history_entry(line.as_str());
            lines.push(line);

            if depth == 0 {
                break;
            }
        }

        if let Err(e) = rl.save_history(&history_path) {
            eprintln!("Error: Failed to save cmd history: {}", e);
        }

        Ok(lines.join("\n"))
    }

    pub fn eval(&mut self, program: String) -> RuntimeResult<String> {
        self.runtime.run(program)
    }

    fn get_history_path(&self) -> RuntimeResult<PathBuf> {
        let cache_dir = dirs::cache_dir().ok_or("Cache dir not found")?;

        let history_dir = cache_dir.join("trebek");
        create_dir_all(&history_dir)
            .ok()
            .ok_or("Failed to create dir for history")?;

        let history_path = history_dir.join("cmd_history");
        Ok(history_path)
    }
}
