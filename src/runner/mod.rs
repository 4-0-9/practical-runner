use std::time::Duration;

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use sdl2::{
    event::Event,
    keyboard::{Keycode, Mod},
    rect::Rect,
    render::{Canvas, WindowCanvas},
    ttf,
    video::Window,
    Sdl,
};

use crate::{
    config::{RunnerMenuSettings, PADDING},
    utils::{color_from_hex, get_font_path},
};

pub struct Runner {
    prompt: String,
    executables: Vec<String>,
    context: Sdl,
    canvas: Canvas<Window>,
    ttf: ttf::Sdl2TtfContext,
    font_path: String,
    input: String,
    window_size: (u32, u32),
    settings: RunnerMenuSettings,
    target_display_index: Option<i32>,
}

impl Runner {
    pub fn new(prompt: String, executables: Vec<String>, settings: RunnerMenuSettings) -> Self {
        let context = sdl2::init().expect("Error creating SDL context");

        let ttf = ttf::init().expect("Error creating SDL TTF context");

        let font_path: String;
        let (window_width, window_height): (u32, u32);
        window_width = 480 + (settings.border_size * 2) as u32;

        {
            font_path = get_font_path(match settings.font {
                Some(ref font_p) => font_p.clone(),
                None => String::from("Monospace"),
            })
            .expect("Could not load fonts");

            let font = ttf
                .load_font(&font_path, settings.font_size)
                .expect(&format!("Error loading font {}", &font_path));

            window_height = (PADDING
                + ((font.height() as u16 + settings.line_spacing) * (1 + settings.rows))
                - settings.line_spacing.div_euclid(2)
                + PADDING
                + (settings.border_size * 2) as u16)
                .into();
        }

        let video = context.video().expect("Error initializing SDL video");

        let (mut window_x, mut window_y): (i32, i32) = (0, 0);

        match settings.display_index {
            Some(display) => {
                if (display as i32).lt(&video
                    .num_video_displays()
                    .expect("Error getting number of displays"))
                {
                    let bounds = video
                        .display_bounds(display.into())
                        .expect(&format!("Error getting bounds for display {}", display));

                    window_x = bounds.x() + (bounds.width().div_euclid(2) as i32)
                        - window_width.div_euclid(2) as i32;
                    window_y = bounds.y() + (bounds.height().div_euclid(2) as i32)
                        - window_height.div_euclid(2) as i32;
                }
            }
            None => {}
        }

        let window = video
            .window("Practical runner", window_width, window_height)
            .borderless()
            .position_centered()
            .always_on_top()
            .build()
            .expect("Error creating window");

        let mut canvas = window.into_canvas().build().expect("Error creating canvas");

        canvas.present();

        // If we don't call this before window.display_index() it always returns 0
        let _ = context
            .event_pump()
            .expect("Error getting SDL event pump")
            .poll_iter()
            .count();

        let target_display_index = if settings.display_index.is_some() {
            Some(
                canvas
                    .window()
                    .display_index()
                    .expect("Error getting window display index"),
            )
        } else {
            None
        };

        if window_x.ne(&0) || window_y.ne(&0) {
            canvas.window_mut().set_position(
                sdl2::video::WindowPos::Positioned(window_x),
                sdl2::video::WindowPos::Positioned(window_y),
            );
        }

        canvas.window_mut().raise();

        let mut cloned_executables = executables.clone();
        cloned_executables.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        Self {
            prompt,
            executables: cloned_executables,
            context,
            canvas,
            input: String::from(""),
            ttf,
            window_size: (window_width, window_height),
            settings,
            font_path,
            target_display_index,
        }
    }

    pub fn run(&mut self) -> Option<String> {
        let matcher = SkimMatcherV2::default();

        let mut selection_index: u16 = 0;
        let mut filtered_executables = self.executables.clone();

        let background_color = color_from_hex(&self.settings.background_color).unwrap();
        let background_color_active =
            color_from_hex(&self.settings.background_color_active).unwrap();
        let font_color = color_from_hex(&self.settings.font_color).unwrap();
        let font_color_active = color_from_hex(&self.settings.font_color_active).unwrap();

        let border_color = color_from_hex(&self.settings.border_color).unwrap();

        let font = self
            .ttf
            .load_font(&self.font_path, self.settings.font_size)
            .expect(&format!("Error loading font {}", self.font_path));

        let creator = self.canvas.texture_creator();

        let mut event_pump = self.context.event_pump().unwrap();

        'run: loop {
            self.canvas.set_draw_color(background_color);
            self.canvas.clear();
            self.canvas.set_draw_color(border_color);
            draw_borders(
                self.settings.border_size,
                self.window_size,
                &mut self.canvas,
            );

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    }
                    | Event::KeyDown {
                        keycode: Some(Keycode::C),
                        keymod: Mod::LCTRLMOD,
                        ..
                    } => {
                        self.input = String::from("");
                        break 'run;
                    }
                    Event::KeyDown { keycode, .. } => {
                        if let Some(key) = keycode {
                            match key {
                                Keycode::Backspace => {
                                    if !self.input.is_empty() {
                                        self.input.pop();

                                        filter_executables(
                                            &self.input,
                                            &self.executables,
                                            &mut filtered_executables,
                                            &matcher,
                                        );
                                        selection_index = 0;
                                    }
                                }
                                Keycode::Return => {
                                    let executables_len = filtered_executables.len();
                                    if executables_len > 0 {
                                        self.input = filtered_executables
                                            [executables_len.min(selection_index.into()) as usize]
                                            .clone();
                                    }
                                    break 'run;
                                }
                                Keycode::Down => {
                                    if selection_index < (filtered_executables.len() - 1) as u16 {
                                        selection_index += 1;
                                    }
                                }
                                Keycode::Up => {
                                    if selection_index > 0 {
                                        selection_index -= 1;
                                    }
                                }
                                Keycode::Tab => {
                                    if filtered_executables.len() > 0 {
                                        self.input =
                                            filtered_executables[selection_index as usize].clone();

                                        filter_executables(
                                            &self.input,
                                            &self.executables,
                                            &mut filtered_executables,
                                            &matcher,
                                        );
                                        selection_index = 0;
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    Event::TextInput { text, .. } => {
                        self.input += &text;

                        filter_executables(
                            &self.input,
                            &self.executables,
                            &mut filtered_executables,
                            &matcher,
                        );

                        selection_index = 0;
                    }
                    _ => {}
                }
            }

            let mut cursor_offset_x = PADDING + self.settings.border_size as u16;
            let input_position_y: u16 = (PADDING + self.settings.border_size as u16 - 1
                + self.settings.line_spacing.div_ceil(4))
            .into();

            if !self.input.is_empty() || !self.prompt.is_empty() {
                let surface = font
                    .render(&format!("{}{}", &self.prompt, &self.input))
                    .blended(font_color)
                    .expect("Error rendering text");

                let rect = Rect::new(
                    (PADDING + self.settings.border_size as u16).into(),
                    input_position_y.into(),
                    surface.width(),
                    surface.height(),
                );

                cursor_offset_x += surface.width() as u16;

                let texture = creator
                    .create_texture_from_surface(surface)
                    .expect("Error creating texture");

                let _ = self.canvas.copy(&texture, None, Some(rect));
            }

            if self.canvas.window().has_input_focus() {
                let cursor_rect = Rect::new(
                    cursor_offset_x.into(),
                    input_position_y.into(),
                    3,
                    font.height() as u32,
                );

                self.canvas.set_draw_color(background_color_active);
                let _ = self.canvas.fill_rect(cursor_rect);
            }

            // try to keep the selection centered
            let executables_len: u16 = filtered_executables.len() as u16;
            let start: u16 = selection_index
                .saturating_sub(self.settings.rows.div_euclid(2))
                .min(executables_len.saturating_sub(self.settings.rows));
            let end: u16 = (start + self.settings.rows).min(executables_len);
            // ---

            let half_line_spacing = self.settings.line_spacing.div_euclid(2);

            let mut display_count: u16 = 0;

            for i in start..end {
                let offset = self.settings.border_size as u16
                    + PADDING * 2
                    + (font.height() as u16 + self.settings.line_spacing) * (display_count + 1);

                let surface = font
                    .render(&filtered_executables[i as usize])
                    .blended(if i != selection_index {
                        font_color
                    } else {
                        font_color_active
                    })
                    .expect("Error rendering text");

                let rect = Rect::new(
                    (self.settings.border_size as u16 + PADDING).into(),
                    offset.into(),
                    surface.width(),
                    surface.height(),
                );

                let background_rect = Rect::new(
                    self.settings.border_size.into(),
                    (offset - half_line_spacing).into(),
                    self.window_size.0 - (self.settings.border_size as u32) * 2,
                    (surface.height() + self.settings.line_spacing as u32).into(),
                );

                self.canvas.set_draw_color(if i != selection_index {
                    background_color
                } else {
                    background_color_active
                });

                let _ = self.canvas.fill_rect(background_rect);

                let texture = creator
                    .create_texture_from_surface(surface)
                    .expect("Error creating texture");

                let _ = self.canvas.copy(&texture, None, Some(rect));

                display_count += 1;
            }

            self.canvas.present();

            std::thread::sleep(Duration::from_millis(8));
        }

        if self.input.is_empty() {
            None
        } else {
            match self.target_display_index {
                Some(target_display_index) => {
                    let window = self.canvas.window_mut();
                    let target_display_bounds = self
                        .context
                        .video()
                        .expect("Error getting SDL video")
                        .display_bounds(target_display_index)
                        .expect("Error getting target display bounds");

                    window.set_position(
                        sdl2::video::WindowPos::Positioned(target_display_bounds.x()),
                        sdl2::video::WindowPos::Positioned(target_display_bounds.y()),
                    );

                    window.raise();
                    window.hide();
                }
                None => (),
            }

            Some(self.input.clone())
        }
    }
}

fn filter_executables(
    input: &String,
    executables: &Vec<String>,
    filtered_executables: &mut Vec<String>,
    matcher: &SkimMatcherV2,
) {
    *filtered_executables = executables
        .iter()
        .filter(|e| matcher.fuzzy_indices(*e, input).is_some())
        .map(|e| e.to_string())
        .collect();

    filtered_executables.sort_by(|a, b| b.starts_with(input).cmp(&a.starts_with(input)));
}

fn draw_borders(border_size: u8, window_size: (u32, u32), canvas: &mut WindowCanvas) {
    if border_size > 0 {
        let _ = canvas.fill_rect(Rect::new(0, 0, window_size.0, border_size.into()));
        let _ = canvas.fill_rect(Rect::new(
            (window_size.0 - border_size as u32) as i32,
            0,
            border_size.into(),
            window_size.1,
        ));
        let _ = canvas.fill_rect(Rect::new(
            0,
            (window_size.1 - border_size as u32) as i32,
            window_size.0,
            border_size.into(),
        ));
        let _ = canvas.fill_rect(Rect::new(0, 0, border_size.into(), window_size.1));
    }
}
