#[track_caller]
pub fn var(key: &str) -> anyhow::Result<Option<String>> {
    match dotenvy::var(key) {
        Ok(content) => Ok(Some(content)),
        Err(dotenvy::Error::EnvVar(std::env::VarError::NotPresent)) => Ok(None),
        Err(error) => Err(error.into()),
    }
}

#[track_caller]
pub fn required_var(key: &str) -> anyhow::Result<String> {
    match var(key)? {
        Some(value) => Ok(value),
        None => Err(anyhow::anyhow!("Missing required environment variable: {key}")),
    }
}