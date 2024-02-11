use config::*;
use std::fs;
use std::error::Error;

pub fn read_config() -> Result<Configuration,Box<dyn Error>> {
    let builder = Config::builder()
    .add_source(File::new("settings", FileFormat::Toml));

match builder.build() {
    Ok(config) => {
        let token = config.get_string("discord_token")?;
        let channel = config.get::<u64>("discord_channel")?;
        let port = config.get::<u16>("port")?;
        return Ok(Configuration { discord_token: token, discord_channel: channel, port: port });
    },
    Err(e) => {
        match &e {
            ConfigError::Foreign(b) => {
                if b.is::<std::io::Error>() {
                    println!("IO Error encountered, attempting to create default config file. Run again after you have setup the configuration.");
                    let default_config = "discord_token=\"YOURDISCORDTOKENHERE\"\ndiscord_channel=1234567890\nport=21621";
                    fs::write("settings.toml",default_config)?;
                }
            },
            _ => println!("Not fileparse")
        }
        return Err(Box::new(e));
    }
}
}

#[derive(Clone)]
pub struct Configuration {
    pub discord_token: String,
    pub discord_channel: u64,
    pub port: u16,
}