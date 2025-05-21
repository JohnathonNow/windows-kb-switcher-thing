use serde::Deserialize;

use figment::{Figment, providers::{Format, Toml, Json, Yaml}};

#[derive(Deserialize)]
pub(super) struct Command {
    pub(super) keyboard: String,
    pub(super) cmd: String,
    pub(super) args: Vec<String>,
}

#[derive(Deserialize)]
pub(super) struct Config {
    commands: Vec<Command>,
}

impl Config {
    pub(super) fn to_commands(self) -> Vec<Command> {
        self.commands
    }
}

pub(super) fn get_config() -> Config {
    Figment::new()
        .merge(Toml::file("config.toml"))
        .join(Yaml::file("config.yaml"))
        .join(Json::file("config.json"))
        .extract().unwrap()
}