use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub command: String,
    #[arg(short, long, default_value_t = false)]
    pub launch: bool,
    #[arg(short, long, default_value_t = false)]
    pub focus: bool,
}
