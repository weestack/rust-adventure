pub enum Cmd {}

impl Cmd {
    pub fn enter_command<'a>() -> String {
        use std::io::{self, Write};

        let mut input = String::new();

        print!("Enter cmd: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string()
    }
}