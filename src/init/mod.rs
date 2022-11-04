use dotenv::dotenv;
pub fn init() {
    // Initialize logger
    env_logger::init();

    // Initialize env variables
    dotenv().ok();
}
