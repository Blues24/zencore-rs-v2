use figlet_rs::FIGfont;
use colored::Colorize;
use emojis;

pub fn display_banner(){

    let motd = String::from("Zencore Rust Edition").blue();
    let emoji = emojis::get("🎶").unwrap().expect("Emoji canot be found!")?;
    // Load the standard font and give error if standard font cannot be found
    let std_font = FIGfont::standard().expect("Font not found, and cannot be loaded");
    // Convert text to banner
    let bannerFigure = stdFont.convert(msg);

    println!("{}", bannerFigure);
}

