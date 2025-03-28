use dirs;
use serde::{Serialize, Deserialize};
use std::{fs::{self, File}, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_os")]
    pub os: String,

    #[serde(default = "default_arch")]
    pub arch: String,

    #[serde(default = "default_config_path")]
    pub config_path: PathBuf,

    #[serde(default = "default_install_path")]
    pub install_path: PathBuf,

    #[serde(default = "default_download_path")]
    pub download_path: PathBuf,

    #[serde(default = "default_packages")]
    pub packages: Vec<Package>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub source: String,
    pub account: String,
    pub repo: String,
    pub version: String,
}

fn default_os() -> String {
    std::env::consts::OS.to_string()
}

fn default_arch() -> String {
    std::env::consts::ARCH.to_string()
}

fn default_config_path() -> PathBuf {
    let config_path_directory = match dirs::config_local_dir() {
        Some(config_path_directory) => {
            config_path_directory.join("sbm").join("config.yml")
        },
        None => panic!("no user configuration path defined."),
    };

    config_path_directory
}

fn default_install_path() -> PathBuf {
    let install_path_directory = match dirs::home_dir() {
        Some(install_path_directory) => {
            install_path_directory.join(".local").join("lib")
        },
        None => panic!("no user home directory path defined."),
    };

    install_path_directory
}

fn default_download_path() -> PathBuf {
    let download_path_directory = match dirs::download_dir() {
        Some(download_path_directory) => {
            download_path_directory
        },
        None => panic!("no user download directory path defined."),
    };

    download_path_directory
}

fn default_packages() -> Vec<Package> {
    Vec::new()
}

pub fn default_config() -> Config {
    serde_yaml::from_str(&"").unwrap()
}

pub fn load_config(config_path: &PathBuf) -> Config {
    if !config_path.exists() {
        fs::create_dir_all(config_path.parent().unwrap());
        File::create(config_path);
    }
    let content = fs::read_to_string(config_path).expect("Should have been able to read the file");
    let config : Config = serde_yaml::from_str(&content).unwrap();

    config
}

pub fn init_config() -> Config {
    let config = default_config();
    load_config(&config.config_path)
}
