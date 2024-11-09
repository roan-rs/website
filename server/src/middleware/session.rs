use std::{collections::HashMap, sync::Arc};
use axum::extract::{Extension, FromRequestParts, Request};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::SignedCookieJar;
use base64::{engine::general_purpose, Engine};
use cookie::time::Duration;
use cookie::{Cookie, SameSite};
use std::ops::Deref;
use parking_lot::RwLock;

#[derive(Clone, FromRequestParts)]
#[from_request(via(Extension))]
pub struct SessionExtension(Arc<RwLock<Session>>);

impl SessionExtension {
    fn new(session: Session) -> Self {
        Self(Arc::new(RwLock::new(session)))
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.0.read().data.get(key).cloned()
    }

    pub fn insert(&self, key: String, value: String) -> Option<String> {
        let mut session = self.0.write();
        session.dirty = true;
        session.data.insert(key, value)
    }

    pub fn remove(&self, key: &str) -> Option<String> {
        let mut session = self.0.write();
        session.dirty = true;
        session.data.remove(key)
    }
}

impl Deref for SessionExtension {
    type Target = RwLock<Session>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Session {
    data: HashMap<String, String>,
    dirty: bool,
}

impl Session {
    const COOKIE_NAME: &'static str = "session";
    const MAX_AGE_DAYS: i64 = 30;

    fn new(data: HashMap<String, String>) -> Self {
        Self { data, dirty: false }
    }
}

pub async fn handle_session(jar: SignedCookieJar, mut req: Request, next: Next) -> Response {
    let data = jar.get(Session::COOKIE_NAME).map(decode).unwrap_or_default();
    let session = SessionExtension::new(Session::new(data));
    req.extensions_mut().insert(session.clone());

    let response = next.run(req).await;

    if session.read().dirty {
        let cookie = Cookie::build((Session::COOKIE_NAME, encode(&session.read().data)))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Strict)
            .max_age(Duration::days(Session::MAX_AGE_DAYS))
            .path("/")
            .build();

        (jar.add(cookie), response).into_response()
    } else {
        response
    }
}

fn decode(cookie: Cookie<'_>) -> HashMap<String, String> {
    general_purpose::STANDARD.decode(cookie.value().as_bytes()).ok()
        .and_then(|bytes| {
            Some(bytes.split(|&b| b == 0xff)
                .collect::<Vec<_>>()
                .chunks(2)
                .filter_map(|pair| match pair {
                    [key, value] if !key.is_empty() => Some((String::from_utf8_lossy(key).into_owned(), String::from_utf8_lossy(value).into_owned())),
                    _ => None,
                })
                .collect::<HashMap<_, _>>())
        })
        .unwrap_or_default()
}

fn encode(data: &HashMap<String, String>) -> String {
    let mut bytes = Vec::new();
    for (k, v) in data {
        if !bytes.is_empty() {
            bytes.push(0xff);
        }
        bytes.extend(k.bytes());
        bytes.push(0xff);
        bytes.extend(v.bytes());
    }
    while bytes.len() * 8 % 6 != 0 {
        bytes.push(0xff);
    }
    general_purpose::STANDARD.encode(bytes)
}
