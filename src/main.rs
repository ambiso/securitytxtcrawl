use futures::stream;
use futures::stream::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use log::{error, info};
use simple_logger::SimpleLogger;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use structopt::StructOpt;

mod util;

/// Performs requests to obtain the `/.well-known/security.txt` of the domains in the `domains_fname` CSV file.
/// Stores each `security.txt` with the domain name as a filename in the given `output_path` directory.
/// Pretty prints a progress bar to the console.
async fn run_requests(
    domains_fname: PathBuf,
    concurrency: usize,
    output_path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let domains = util::get_domains(&domains_fname).await?;
    let domains_len = domains.len();

    let pb = ProgressBar::new(domains_len as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            .progress_chars("#>-"),
    );
    let pb = Arc::new(Mutex::new((0, pb)));
    let pb_clone = pb.clone();

    let successes = stream::iter(domains)
        .map(|domain| {
            let output_path = output_path.clone();
            let pb = pb.clone();
            async move {
                let data = util::get_security_txt(domain.clone()).await?;
                util::write_to_file(output_path, Path::new(&domain), &data).await?;

                let mut pb = pb.lock().map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::Other, "Could not lock")
                })?;
                pb.0 += 1; // increment progress
                pb.1.set_position(pb.0);
                Ok::<(), Box<dyn Error>>(())
            }
        })
        .buffer_unordered(concurrency)
        .fold(0, |a, x| async move { a + x.is_ok() as u64 })
        .await;

    let pb = pb_clone
        .lock()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Could not lock"))?;
    pb.1.finish();

    info!(
        "done. succeeded with {}/{} ({:.02}%)",
        successes,
        domains_len,
        successes as f64 / domains_len as f64 * 100.
    );
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "securitytxtcrawl",
    about = "Crawl the security.txt of many websites"
)]
struct Opt {
    /// Set the number of concurrent requests to perform.
    #[structopt(short = "c", long = "concurrency", default_value = "200")]
    concurrency: usize,

    /// Filename of the CSV containing the domains to crawl
    /// The CSV should contain a numeric ID and the domain.
    /// There is no header.
    /// If the file does not exist, a top 1 million domains list is downloaded.
    #[structopt(
        short = "d",
        long = "domains",
        default_value = "1m.csv",
        parse(from_os_str)
    )]
    domains_fname: PathBuf,

    /// Output directory for security.txts.
    /// A file containing the contents of the security.txt is created for each domain.
    #[structopt(
        short = "o",
        long = "outdir",
        default_value = "output",
        parse(from_os_str)
    )]
    output: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let opt = Opt::from_args();

    match tokio::fs::create_dir(&opt.output).await {
        Ok(_) => {},
        Err(e) => {
            error!("Failed to create output directory {}: {:?}", &opt.output.display(), e.to_string());
            Err(e)?;
        }
    };

    run_requests(opt.domains_fname, opt.concurrency, opt.output).await?;
    Ok(())
}
