use serde::Deserialize;
use sqlx::ConnectOptions;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub debug: bool,
    pub database: DatabaseSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
    pub base_url: String,
    pub protocol: String,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn connect_to_db(&self) -> sqlx::postgres::PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            sqlx::postgres::PgSslMode::Require
        } else {
            sqlx::postgres::PgSslMode::Prefer
        };
        let options = sqlx::postgres::PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(ssl_mode)
            .database(&self.database_name);

        let op = options.log_statements(tracing::log::LevelFilter::Trace);
        op
    }
}

pub enum Environment {
    Development,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "development" => Ok(Environment::Development),
            "production" => Ok(Environment::Production),
            other => Err(format!(
                "{} is not a supported environment. Use development or production.",
                other
            )),
        }
    }
}

/// Multipurpose function that helps detect the current environment the application
/// is running using the `APP_ENVIRONMENT` environment variable.
///
/// \`\`\`
/// APP_ENVIRONMENT = development | production.
/// \`\`\`
///
/// After detection, it loads the appropriate .yaml file
/// then it loads the environment variable that overrides whatever is set in the .yaml file.
/// For this to work, you the environment variable MUST be in uppercase and starts with `APP`,
/// a `_` separator then the category of settings,
/// followed by `__` separator,  and then the variable, e.g.
/// `APP__APPLICATION_PORT=5001` for `port` to be set as `5001`

pub fn get_settings() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine current directory");
    let settings_dir = base_path.join("settings");

    //detect running environment
    //default to development if unspecified

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "development".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");
    let environment_filename = format!("{}.yaml", environment.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(settings_dir.join("base.yaml")))
        .add_source(config::File::from(settings_dir.join(environment_filename)))

    // Add in settings from environment variables (with a prefix of APP and '__' as separator)
    // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
        .add_source(config::Environment::with_prefix("APP").prefix_separator("_").separator("__"))
        .build()?;

    settings.try_deserialize::<Settings>()
}
