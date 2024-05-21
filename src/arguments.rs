use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[arg(short, long, help = "The menu prompt message", default_value_t = String::from(""))]
    pub prompt: String,
}
