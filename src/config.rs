use clap::Parser;
use dotenv::dotenv;

#[derive(Parser)]
#[clap(author="Kike Fontán (@CosasDePuma)",version,name="sharingan",about="",long_about=None)]
pub struct Config {
    #[clap(short,long,env("SHARINGAN_ADDRESS"),default_value("127.0.0.1"))]
    pub address: String,
    #[clap(short,long,env("DATABASE_URL"),default_value("postgres://user:pass@localhost:5432/sharingan"))]
    pub database: String,
    #[clap(short,long,env("SHARINGAN_SECRET"),default_value("ch4ng3meAndUs3aVeryR0bu2T&r4nD0mS3cr3T!!"))]
    pub secret: String,
}
impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        Self::parse()
    }
}