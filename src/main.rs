use std::cmp::Ordering;

const BAD_USAGE_EXIT_ERROR: i32 = 64;

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();

    match args.len().cmp(&1) {
        Ordering::Greater => {
            println!("Usage: jlox [script]");
            std::process::exit(BAD_USAGE_EXIT_ERROR);
        }
        Ordering::Equal => {
            lox::run_file((&args[0]).into())?;
        }
        _ => {
            lox::run_prompt()?;
        }
    }

    Ok(())
}
