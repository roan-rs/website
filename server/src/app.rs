use crate::var::{required_var, var};
use anyhow::Result;
use axum::extract::{FromRef, FromRequestParts, State};
use cookie::Key;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, TokenUrl};
use std::{net::IpAddr, ops::Deref, sync::Arc};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::AsyncPgConnection;
use crate::db::create_db_connection;

pub struct App {
    pub port: u16,
    pub ip: IpAddr,
    pub github_auth: BasicClient,
    pub session_key: Key,
    pub db: Pool<AsyncPgConnection>,
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

        let gh_client_id = ClientId::new(required_var("GH_CLIENT_ID")?);
        let gh_client_secret = ClientSecret::new(required_var("GH_CLIENT_SECRET")?);
        let session_key = required_var("SESSION_KEY")?;

        Ok(Self {
            port,
            ip,
            github_auth: BasicClient::new(
                gh_client_id.clone(),
                Some(gh_client_secret.clone()),
                AuthUrl::new(String::from("https://github.com/login/oauth/authorize"))?,
                Some(TokenUrl::new(String::from(
                    "https://github.com/login/oauth/access_token",
                ))?),
            ),
            session_key: Key::derive_from(session_key.as_bytes()),
            db: create_db_connection()?,
        })
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

impl FromRef<AppState> for Key {
    fn from_ref(app: &AppState) -> Self {
        app.session_key.clone()
    }
}
