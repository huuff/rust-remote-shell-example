use clap::Parser;
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = 3333)]
    pub port: u16,

    #[arg(short, long, default_value_t = String::from("0.0.0.0"))]
    pub addr: String,

    #[arg(long, default_value_t = Uuid::new_v4().to_string())]
    pub password: String,

}
