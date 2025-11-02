use std::io::{self, Write};

pub fn read_choice() -> Result<String, String> {
    print!("👉 Enter choice: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
    Ok(input.trim().to_string())
}
