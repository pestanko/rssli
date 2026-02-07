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

                // 5. Evaluate
                match runtime.eval_string(line) {
                    Ok(result) => {
                        println!("=> {:?}", result);
                    }
                    Err(err) => {
                        eprintln!("Error: {:?}", err);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
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
