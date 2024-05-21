use std::time::Duration;

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    rect::Rect,
    render::Canvas,
    ttf::{self},
    video::Window,
    Sdl,
};

use crate::{
    config::{
        BACKGROUND_COLOR, BACKGROUND_COLOR_SELECTED, FONT_COLOR, FONT_COLOR_SELECTED,
        FONT_POINT_SIZE, LINE_SPACING, MAX_ITEM_DISPLAY_COUNT, PADDING,
    },
    utils::color_from_hex,
};

pub struct Runner {
    prompt: String,
    executables: Vec<String>,
    context: Sdl,
    canvas: Canvas<Window>,
    ttf: ttf::Sdl2TtfContext,
    input: String,
    window_size: (u32, u32),
}

impl Runner {
    pub fn new(prompt: String, executables: Vec<String>) -> Self {
        let context = sdl2::init().expect("Error creating SDL context");

        let ttf = ttf::init().expect("Error creating SDL TTF context");

        let window_height: u32;
        {
            let font_path = String::from("/usr/share/fonts/OTF/GeistMonoNerdFontMono-Regular.otf");

            let font = ttf
                .load_font(font_path.clone(), FONT_POINT_SIZE)
                .expect(&format!("Error loading font {}", font_path));

            window_height = (PADDING
                + ((font.height() as u16 + LINE_SPACING) * (1 + MAX_ITEM_DISPLAY_COUNT)))
                .into();
        }

        let video = context.video().expect("Error initializing SDL video");

        let mut window = video
            .window("Practical runner", 480, window_height)
            .position_centered()
            .borderless()
            .build()
            .expect("Error creating window");

        window.set_opacity(0.0).unwrap();
        let window_size = window.size();

        let canvas = window.into_canvas().build().expect("Error creating canvas");

        let mut cloned_executables = executables.clone();
        cloned_executables.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        Self {
            prompt,
            executables: cloned_executables,
            context,
            canvas,
            input: String::from(""),
            ttf,
            window_size,
        }
    }

    pub fn run(&mut self) -> Option<String> {
        let matcher = SkimMatcherV2::default();

        let mut selection_index: u16 = 0;
        let mut filtered_executables = self.executables.clone();

        let background_color = color_from_hex(BACKGROUND_COLOR).unwrap();
        let background_color_selected = color_from_hex(BACKGROUND_COLOR_SELECTED).unwrap();

        let font_path = String::from("/usr/share/fonts/OTF/GeistMonoNerdFontMono-Regular.otf");

        let font = self
            .ttf
            .load_font(font_path.clone(), FONT_POINT_SIZE)
            .expect(&format!("Error loading font {}", font_path));

        let font_color = color_from_hex(FONT_COLOR).expect("Error loading FONT_COLOR");
        let font_color_selected =
            color_from_hex(FONT_COLOR_SELECTED).expect("Error loading FONT_COLOR_SELECTED");

        let creator = self.canvas.texture_creator();

        let mut event_pump = self.context.event_pump().unwrap();

        'run: loop {
            self.canvas.set_draw_color(background_color);
            self.canvas.clear();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
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

            if !self.input.is_empty() || !self.prompt.is_empty() {
                let surface = font
                    .render(&format!("{}{}", &self.prompt, &self.input))
                    .blended(font_color)
                    .expect("Error rendering text");

                let rect = Rect::new(
                    PADDING.into(),
                    (PADDING - 3).into(),
                    surface.width(),
                    surface.height(),
                );

                let texture = creator
                    .create_texture_from_surface(surface)
                    .expect("Error creating texture");

                let _ = self.canvas.copy(&texture, None, Some(rect));
            }

            let item_count = MAX_ITEM_DISPLAY_COUNT.min(filtered_executables.len() as u16);
            for i in 0..item_count {
                let offset = PADDING + (font.height() as u16 + LINE_SPACING) * (i + 1);

                let surface = font
                    .render(&filtered_executables[i as usize])
                    .blended(if i != selection_index {
                        font_color
                    } else {
                        font_color_selected
                    })
                    .expect("Error rendering text");

                let rect = Rect::new(
                    PADDING.into(),
                    offset.into(),
                    surface.width(),
                    surface.height(),
                );

                let background_rect =
                    Rect::new(0, offset.into(), self.window_size.0, surface.height());

                self.canvas.set_draw_color(if i != selection_index {
                    background_color
                } else {
                    background_color_selected
                });

                let _ = self.canvas.fill_rect(background_rect);

                let texture = creator
                    .create_texture_from_surface(surface)
                    .expect("Error creating texture");

                let _ = self.canvas.copy(&texture, None, Some(rect));
            }

            self.canvas.present();

            std::thread::sleep(Duration::from_millis(8))
        }

        if self.input.is_empty() {
            None
        } else {
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
