use std::path::{Path, PathBuf};
use std::process::exit;
use std::fs::{self, copy, File};
use std::io::{self, BufReader, Write};
use bytes::Bytes;

use crate::config;

fn download(source: &String) -> Result<Bytes, reqwest::Error> {
    let body = match reqwest::blocking::get(source) {
        Ok(content) => content.bytes(),
        Err(_) => exit(1),
    };
    body
}

fn save_binary(binary: &Bytes, path: &PathBuf) {
    let mut file = File::create(&path).expect("couldn't save download files.");
    file.write_all(binary);
}

fn extract_zip(package_name: &String, zip_path: &PathBuf, extract_to: &PathBuf) -> std::io::Result<()> {
    let fname = std::path::Path::new(zip_path);
    let file = fs::File::open(fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let mut outpath = match file.enclosed_name() {
            Some(path) => path,
            None => continue,
        };
        println!("{:?}", outpath);
        let root = Path::new(package_name);
        outpath = extract_to.join(root.join(outpath));

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {i} comment: {comment}");
            }
        }

        if file.is_dir() {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    
    Ok(())
}

fn parse_source(target: &String) -> (String, String, String) {

    let (package_name, api_url, releases_url) = if target.contains("github.com") {
        let parts: Vec<&str> = target
            .trim_start_matches("https://")
            .split('/')
            .collect();
        println!("{:?}", parts);
        let api_url = format!(
            "https://api.github.com/repos/{}/{}/releases",
            parts[0], parts[1]
        );
        // let api_url = format!("https://api.github.com/repos/{}/{}/git/tags", parts[0], parts[1]);
        (parts[2].to_string(), api_url, parts[0].to_string())
    } else {
        eprintln!("URL not supported");
        exit(1);
    };

    (package_name, api_url, releases_url)
}

pub fn install(config: &config::Config, target: &String) {
    let repo_url = target;

    // Détecter la plateforme
    let (package_name, api_url, releases_url) = parse_source(target);

    println!("{:?}", api_url);
    let binary_path = config.download_path.join(format!("{}.{}", package_name, "zip".to_string()));
    let binary = download(repo_url).expect("FLOP");
    save_binary(&binary, &binary_path);

    extract_zip(&package_name, &binary_path, &config.install_path);
}
