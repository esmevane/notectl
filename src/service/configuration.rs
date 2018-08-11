pub struct Configuration {
    pub port: String,
    pub host: String,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            port: String::from("8000"),
            host: String::from("0.0.0.0"),
        }
    }
}
