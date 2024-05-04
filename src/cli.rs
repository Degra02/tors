use clap::Parser;


#[derive(Parser)]
pub struct Cli {
    #[arg(short, long, default_value="false")]
    pub create: bool
}
