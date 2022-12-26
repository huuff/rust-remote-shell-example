use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None,
    disable_help_flag = true
)]
pub struct Args {
    #[arg(short, long, default_value_t = 3333)]
    pub port: u16,

    #[arg(short, long, default_value_t = String::from("localhost"))]
    pub host: String,

    #[arg(long, default_value_t = false)]
    pub tls: bool,
    
}
