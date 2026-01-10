use reqwest::Client;
use scihub_scraper::{Scraper, fetch_base_urls};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short, long)]
    paper: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let client = Client::new();

    println!("Fetching base urls...");
    let base_urls = match fetch_base_urls(&client).await {
        Ok(base_urls) => {
            println!("Found base urls:");
            for base_url in &base_urls {
                println!("  - {}", base_url);
            }
            base_urls
        }
        Err(error) => {
            println!("Failed to fetch base urls: {}", error);
            return;
        }
    };

    let mut scraper = Scraper::with_base_urls_and_client(base_urls, client);
    println!("Fetching paper...");
    match scraper.fetch_paper_by_doi(&opt.paper).await {
        Ok(paper) => {
            println!("Found paper:");
            println!("Title:    {}", paper.title);
            println!("DOI:      {}", paper.doi);
            println!("Download: {}", paper.download_url);
            println!("Mirror:   {}", paper.scihub_url);
            println!("Version:  {}", paper.version);
        }
        Err(error) => {
            println!("Failed to fetch paper: {}", error);
        }
    }
}
