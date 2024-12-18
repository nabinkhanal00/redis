use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(long, default_value_t = String::from("."))]
    pub dir: String,
    #[arg(long, default_value_t=String::from("dump.rdb"))]
    pub dbfilename: String,
}
