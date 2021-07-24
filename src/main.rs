#![feature(async_closure)]

use futures::stream;
use futures::stream::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
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

    let pb = ProgressBar::new(domains.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
            )
            // .with_key("eta", |state| format!("{:.1}s", state.eta().as_secs_f64()))
            .progress_chars("#>-"),
    );
    let pb = Arc::new(Mutex::new((0, pb)));
    let pb_clone = pb.clone();

    stream::iter(domains)
        .map(move |domain| {
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
        .map(|x| x.is_ok())
        .collect::<Vec<bool>>()
        .await;

    let pb = pb_clone
        .lock()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Could not lock"))?;
    pb.1.finish_with_message("done");

    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Set the number of concurrent requests to perform.
    #[structopt(short = "c", long = "concurrency", default_value = "1000")]
    concurrency: usize,

    /// Filename of the CSV containing the domains to crawl
    ///
    /// The format of the file should look like this:
    /// 1,example.org
    /// 2,example.org
    ///
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

    tokio::fs::create_dir(&opt.output).await?;

    run_requests(opt.domains_fname, opt.concurrency, opt.output).await?;
    Ok(())
}
