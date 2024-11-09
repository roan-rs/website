use axum::extract::State;
use std::net::IpAddr;
use std::ops::Deref;
use std::sync::Arc;
use crate::var::var;
use anyhow::Result;
use axum::extract::FromRequestParts;

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


#[derive(Clone, FromRequestParts)]
#[from_request(via(State))]
pub struct AppState(pub Arc<App>);

impl Deref for AppState {
    type Target = App;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}