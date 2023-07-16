mod scanner;

use scanner::Scanner;

use std::{
    io::{self, Write},
    path::PathBuf,
};

const DATA_ERR_EXIT_ERROR: i32 = 65;

pub fn run_file(path: PathBuf) -> anyhow::Result<()> {
    let bytes = std::fs::read_to_string(path)?;

    if run(bytes).is_err() {
        std::process::exit(DATA_ERR_EXIT_ERROR);
    };

    Ok(())
}

pub fn run_prompt() -> anyhow::Result<()> {
    let stdin = io::stdin();

    loop {
        let mut buffer = String::new();

        print!("> ");
        io::stdout().flush().unwrap();

        if stdin.read_line(&mut buffer)? == 0 {
            break;
        };

        let _ = run(buffer);
    }

    Ok(())
}

pub fn run(source: String) -> anyhow::Result<()> {
    let scanner = Scanner::from(source);
    let tokens = scanner.scan_tokens();

    dbg!(tokens);

    Ok(())
}
