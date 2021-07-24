use bytes::{Buf, Bytes};
use log::info;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::error::Error;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn get_security_txt(domain: String) -> Result<Bytes, Box<dyn Error>> {
    let client = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(30))
        .build()?;
    let url = String::from("http://") + &domain + "/.well-known/security.txt";
    Ok(client.get(&url).send().await?.bytes().await?)
}

pub async fn write_to_file(
    mut dir: PathBuf,
    name: &Path,
    mut bytes: Bytes,
) -> Result<(), Box<dyn Error>> {
    dir.push(name);
    let mut f = File::create(dir).await?;
    while bytes.remaining() > 0 {
        let chunk = bytes.chunk();
        let len = chunk.len();
        f.write_all(chunk).await?;
        bytes.advance(len);
    }
    Ok(())
}

pub async fn get_domains(fname: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let data = match File::open(fname).await {
        Ok(mut f) => {
            info!("Reading {}", fname.display());
            let mut data = Vec::new();
            f.read_to_end(&mut data).await?;
            data
        }
        Err(_) => {
            info!("Downloading zip of top 1 million sites");
            let data = reqwest::get("http://s3.amazonaws.com/alexa-static/top-1m.csv.zip")
                .await?
                .bytes()
                .await?;
            let data = std::io::Cursor::new(data);
            let mut zip = zip::ZipArchive::new(data)?;
            let file = zip.by_index(0).unwrap();
            let data: Vec<u8> = file.bytes().map(|x| x.expect("expected byte")).collect();
            {
                info!("Writing to {}", fname.display());
                let mut f = File::create(fname).await?;
                f.write_all(&data).await?;
            }
            data
        }
    };
    let mut reader = csv::Reader::from_reader(std::io::Cursor::new(data));
    let mut retval: Vec<String> = Vec::new();
    for result in reader.records() {
        let record = result?;
        retval.push(record.get(1).expect("Expected data").into());
    }
    // Avoid visiting the same sites in the same order every time we run
    retval.shuffle(&mut thread_rng());
    Ok(retval)
}
