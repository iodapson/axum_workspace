#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub hmac_key: String,
}

impl Config {
    pub fn new() -> Config {
        // -------- DATABASE SETUP START
        // load up '.env'
        dotenv::dotenv().ok(); // 'dotenv' is a third-party crate

        // extract `DATABASE_URL` value inside now loaded '.env' file
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found.");

        // extract `HMAC_KEY` value inside now loaded '.env' file
        let hmac_key = std::env::var("HMAC_KEY").expect("HMAC_KEY not found!");

        Config {
            database_url,
            hmac_key,
        }
    }
}
