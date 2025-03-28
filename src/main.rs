use std::process::{ExitCode, exit};

use config::{default_config, init_config};
use indicatif::{ProgressBar, ProgressStyle};

pub mod cli;
pub mod config;

pub mod install;

fn main() -> ExitCode {
    let config = config::init_config();
    let cli = cli::parse_cli();

    match &cli.command {
        Some(cli::Commands::Install { target }) => {
            install::install(&config, target);
        }
        Some(cli::Commands::Update { target }) => {
            todo!()
        }
        Some(cli::Commands::Info { target }) => {
            todo!()
        }
        Some(cli::Commands::List {}) => {
            for package in config.packages.iter() {
                println!("{} from {} by {} version {}", package.name, package.source, package.account, package.version);
            }
        }
        None => {
            exit(1)
        }
    };

    // Récupérer les versions
    /*let client = Client::new();
    let response = client.get(&api_url)
        .header("User-Agent", "rust-downloader")
        .send()
        .await();

    if !response.status().is_success() {
        eprintln!("Erreur lors de la récupération des versions");
        exit(1);
    }

    let releases: Value = response.json().await?;
    let versions: Vec<&str> = match releases_url {
        "GitHub" => releases.as_array().unwrap().iter().map(|r| r["tag_name"].as_str().unwrap()).collect(),
        "GitLab" => releases.as_array().unwrap().iter().map(|r| r["tag_name"].as_str().unwrap()).collect(),
        _ => vec![]
    };

    // Sélection de la version
    let selection = Select::new()
        .with_prompt("Choisissez une version")
        .items(&versions)
        .default(0)
        .interact();

    let selected_version = versions[selection];

    // Téléchargement du binaire (adaptez selon la structure de votre dépôt)
    let download_url = match releases_url {
        "GitHub" => releases[selection]["assets"][0]["browser_download_url"].as_str().unwrap(),
        "GitLab" => releases[selection]["assets"]["links"][0]["direct_asset_url"].as_str().unwrap(),
        _ => ""
    };

    // Confirmation
    println!("Téléchargement de la version {} depuis: {}", selected_version, download_url);

    // Téléchargement avec barre de progression
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));

    let response = client.get(download_url)
        .send()
        .await?;

    let total_size = response.content_length().unwrap_or(0);
    pb.set_length(total_size);

    let mut file = std::fs::File::create(format!("binary-{}", selected_version))?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        pb.inc(chunk.len() as u64);
        std::io::copy(&mut chunk.as_ref(), &mut file)?;
    }

    //pb.finish_with_message("Téléchargement terminé");*/
    ExitCode::from(0)
}
