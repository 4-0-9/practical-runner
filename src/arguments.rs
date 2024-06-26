use clap::Parser;

use crate::config::{
    BACKGROUND_COLOR, BACKGROUND_COLOR_ACTIVE, BORDER_COLOR, BORDER_SIZE, FONT_COLOR,
    FONT_COLOR_ACTIVE, FONT_POINT_SIZE, LINE_SPACING, MAX_ITEM_DISPLAY_COUNT,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[arg(short, long, help = "The menu's prompt message", default_value_t = String::from(""))]
    pub prompt: String,

    #[arg(long, help = "The font to use for the menu.")]
    pub font: Option<String>,

    #[arg(long, help = "The default font color", default_value_t = String::from(FONT_COLOR))]
    pub font_color: String,

    #[arg(long, help = "The font color of the active item", default_value_t = String::from(FONT_COLOR_ACTIVE))]
    pub font_color_active: String,

    #[arg(long, help = "The window border color", default_value_t = String::from(BORDER_COLOR))]
    pub border_color: String,

    #[arg(long, help = "The window border size in pixels", default_value_t = BORDER_SIZE)]
    pub border_size: u8,

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

    #[arg(short, long, help = "The index of the target display")]
    pub display: Option<u8>,
}
