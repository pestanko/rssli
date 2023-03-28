use std::fs;

use rssli::Runtime;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in &args {
        log::info!("Executing program {}", arg);
        let content: String = fs::read_to_string(arg)?;

        let mut runtime = Runtime::new_default();

        let result = runtime.eval_string(&content);
        println!("result for '{}': {:?}", arg, result);
    }

    Ok(())
}
