use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};
use tokio::join;

// Get user request
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    // Print question in specific Color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    // Reset Color
    stdout.execute(ResetColor).unwrap();

    let mut user_response: String = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("failed to read response");

    // Trim whitespaces and return
    user_response.trim().to_string()
}
