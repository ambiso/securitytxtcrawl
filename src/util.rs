use bytes::Bytes;
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
    bytes: &Bytes,
) -> Result<(), Box<dyn Error>> {
    dir.push(name);
    let mut f = File::create(dir).await?;
    f.write_all(&bytes[..]).await?;
    Ok(())
}

/// Reads the domains from the given filepath or downloads a CSV of top 1 million domains
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

#[cfg(test)]
mod test {
    use super::*;

    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use std::io::Read;

    #[test]
    fn test_write_file() -> Result<(), Box<dyn Error>> {
        let mut temp_dir = std::env::temp_dir();
        temp_dir.push(
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .map(char::from)
                .collect::<String>(),
        );
        std::fs::create_dir(&temp_dir)?;
        let fname = Path::new("test");
        let data: &'static [u8] = &[1, 2, 3];
        tokio::runtime::Runtime::new()?.block_on(write_to_file(
            temp_dir.to_path_buf(),
            fname,
            &Bytes::from(data),
        ))?;

        let mut temp_file = temp_dir.clone();
        temp_file.push(fname);
        let mut f = std::fs::File::open(temp_file)?;
        let mut read_data = Vec::new();
        f.read_to_end(&mut read_data)?;
        assert_eq!(read_data, data);
        Ok(())
    }
}
