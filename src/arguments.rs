use clap::Parser;

use crate::config::{FONT_COLOR, FONT_COLOR_ACTIVE, BACKGROUND_COLOR, BACKGROUND_COLOR_ACTIVE};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[arg(short, long, help = "The menu's prompt message", default_value_t = String::from(""))]
    pub prompt: String,

    #[arg(long, help = "The default font color", default_value_t = String::from(FONT_COLOR))]
    pub font_color: String,

    #[arg(long, help = "The font color of the selected element", default_value_t = String::from(FONT_COLOR_ACTIVE))]
    pub font_color_active: String,

    #[arg(long, help = "The default font color", default_value_t = String::from(BACKGROUND_COLOR))]
    pub background_color: String,

    #[arg(long, help = "The font color of the selected element", default_value_t = String::from(BACKGROUND_COLOR_ACTIVE))]
    pub background_color_active: String,
}
