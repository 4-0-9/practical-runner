pub const MAX_ITEM_DISPLAY_COUNT: u16 = 9;
pub const PADDING: u16 = 8;
pub const LINE_SPACING: u16 = 2;
pub const FONT_POINT_SIZE: u16 = 16;

pub const FONT_COLOR: &str = "#cdd6f4";
pub const FONT_COLOR_ACTIVE: &str = "#1e1e2e";

pub const BACKGROUND_COLOR: &str = "#1e1e2e";
pub const BACKGROUND_COLOR_ACTIVE: &str = "#89b4fa";

pub struct RunnerMenuSettings {
    pub font_color: String,
    pub font_color_active: String,
    pub background_color: String,
    pub background_color_active: String,
    pub rows: u16,
    pub font_size: u16,
    pub line_spacing: u16,
}
