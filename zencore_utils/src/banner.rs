use emojis;
use figlet_rs::FIGfont;

pub fn display_banner() {
    let motd = "Zencore Rust Edition";
    let emoji = emojis::get("🎶").unwrap();
    // Load the standard font and give error if standard font cannot be found
    let std_font = FIGfont::standard().expect("Font not found, and cannot be loaded");
    // Combine the string and emoji
    let msg = format!("{}, {}", motd, emoji.as_str());
    // Convert text to banner
    let banner_figure = std_font.convert(msg.clone().as_str());
    assert!(banner_figure.is_some());
    println!("{}", banner_figure.unwrap());
}
