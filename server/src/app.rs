use std::net::IpAddr;
use crate::var::var;
use anyhow::Result;

pub struct App {
    pub port: u16,
    pub ip: IpAddr,
}

impl App {
    pub fn from_env() -> Result<Self> {
        let deployed = var("DEPLOYED")?.unwrap_or("false".to_string()) == "true";
        
        let ip = if deployed {
            [0, 0, 0, 0].into()
        } else {
            [127, 0, 0, 1].into()
        };
        
        let port = var("PORT")?.unwrap_or("3000".to_string()).parse()?;
        
        Ok(Self { port, ip })
    }
}