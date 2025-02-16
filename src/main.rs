use std::io::{self, Write};

mod file_manager;

// basic REPL loop to start.
// just for my future self, REPL stands for Read-Eval-Print-Loop.
fn main() {
    println!("welcome to chrisql!");
    println!("type your SQL commands into the cli, or type 'exit' to quit.");

    loop {
        print!("db> ");

        // this line ensure the prompt is displayed immediately.
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("goodbye!");
            break;
        }

        println!("you entered: {}", input);
    }
}
