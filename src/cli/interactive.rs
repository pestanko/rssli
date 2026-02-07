use rustyline::{error::ReadlineError, DefaultEditor};

use crate::{parser::Value, Runtime};

/**
 * Process the interactive mode
 * This will read the input from the user and evaluate it
 * It will return the result of the evaluation
 */
pub(crate) fn process_interactive() -> anyhow::Result<Value> {
    // 1. Create the editor
    let mut rl = DefaultEditor::new()?;

    // 2. Load history from a hidden file in the home directory or local folder
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let mut runtime = Runtime::new_default();

    loop {
        // 3. Use readline instead of stdin().read_line()
        let readline = rl.readline("rssli> ");

        match readline {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                // 4. Add the line to history
                let _ = rl.add_history_entry(line);

                // handle special commands (starts with /)
                if line.starts_with("/") {
                    let command = line.split_whitespace().next().unwrap_or("");
                    match command {
                        "/exit" => {
                            std::process::exit(0);
                        }
                        "/list" => {
                            println!("Listing functions and variables");
                            println!("Functions:");
                            for k in runtime.env().funcs().keys() {
                                println!("{}", k);
                            }
                            println!("Variables:");
                            for v in runtime.env().vars().keys() {
                                println!("{}", v);
                            }
                        }
                        _ => {
                            println!("Unknown command: {}", command);
                        }
                    }
                    continue;
                }

                // 5. Evaluate
                let normal_line = normalize_line_to_list(line);
                match runtime.eval_string(&normal_line) {
                    Ok(result) => {
                        println!("=> {:?}", result);
                    }
                    Err(err) => {
                        eprintln!("Error: {:?}", err);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                log::debug!("CTRL-C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                log::debug!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    // 6. Save history before exiting
    rl.save_history("history.txt")?;

    Ok(Value::Nil)
}


/**
 * Cases to cover:
 * - line is already a list - do nothing
 * - line starts with ; - do nothing
 * - line is empty - do nothing (this should be handled before)
 * - otherwise, add a ( at the beginning and a ) at the end
 */
fn normalize_line_to_list(line: &str) -> String {
    if line.is_empty() {
        return "".to_string();
    }

    if line.starts_with(";") {
        return line.to_string();
    }

    if line.starts_with("(") && line.ends_with(")") {
        return line.to_string();
    }

    format!("({})", line)
}
