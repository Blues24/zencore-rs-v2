use colored::*;

// take a single or more message to give information to user
pub fn info_console(msg: &str) {
    println!("[INFO]: {} {}", "[•]".cyan(), msg.bold());
}
// take a single message to give information about process has been successfully executed
pub fn success_info(msg: &str) {
    println!("[SUCCESS]: {} {}", "[✓]".green(), msg.bold());
}
// take a single or more message to warn the user
pub fn warn_info(msg: &str) {
    println!("[WARN]: {} {}", "[!]".yellow(), msg.bold());
}
// give error message to the user when debug is set to true or important error is occurring
pub fn error_info(msg: &str) {
    println!("[ERROR]: {} {}", "[X]".red(), msg.bold());
}
