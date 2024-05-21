use clap::Parser;

use crate::config::{
    BACKGROUND_COLOR, BACKGROUND_COLOR_ACTIVE, FONT_COLOR, FONT_COLOR_ACTIVE, FONT_POINT_SIZE,
    LINE_SPACING, MAX_ITEM_DISPLAY_COUNT,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[arg(short, long, help = "The menu's prompt message", default_value_t = String::from(""))]
    pub prompt: String,

    #[arg(long, help = "The default font color", default_value_t = String::from(FONT_COLOR))]
    pub font_color: String,

    #[arg(long, help = "The font color of the active item", default_value_t = String::from(FONT_COLOR_ACTIVE))]
    pub font_color_active: String,

    #[arg(long, help = "The default background color", default_value_t = String::from(BACKGROUND_COLOR))]
    pub background_color: String,

    #[arg(long, help = "The background color of the active item", default_value_t = String::from(BACKGROUND_COLOR_ACTIVE))]
    pub background_color_active: String,

    #[arg(short, long, help = "The amount of items to display at once", default_value_t = MAX_ITEM_DISPLAY_COUNT)]
    pub rows: u16,

    #[arg(long, help = "The menu's font size", default_value_t = FONT_POINT_SIZE)]
    pub font_size: u16,

    #[arg(long, help = "The spacing between items", default_value_t = LINE_SPACING)]
    pub line_spacing: u16,
}
