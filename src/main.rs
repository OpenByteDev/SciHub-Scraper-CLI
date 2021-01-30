use scihub_scraper::SciHubScraper;
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

    println!("Fetching paper: {}", opt.paper);
    let mut scraper = SciHubScraper::new();

    println!("Fetching base urls...");
    match scraper.fetch_base_urls().await {
        Ok(base_urls) => {
            println!(
                "Found base urls: {}",
                base_urls
                    .iter()
                    .map(|e| e.url.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        Err(error) => {
            println!("Failed to fetch base urls: {}", error);
            return;
        }
    }

    println!("Fetching paper...");
    match scraper.fetch_paper_by_doi(&opt.paper).await {
        Ok(paper) => {
            println!("Found paper:");
            println!("{:#?}", paper);
        }
        Err(error) => {
            println!("Failed to fetch paper: {}", error);
        }
    }
}
