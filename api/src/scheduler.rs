use tokio::time::{interval, Duration};
use store::store::Store;

use crate::auth::health::check_website_health;

pub fn start_scheduler(){
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(60));

        loop {
            ticker.tick().await;

            println!("[scheduler] running health checks");
            
            let websites = tokio::task::spawn_blocking(|| {
                let mut s = Store::default().expect("DB connection not acheived");
                s.list_websites()
            })
            .await
            .unwrap_or_else(|_| Ok(vec![]));

            let websites = match websites{
                Ok(w) => w,
                Err(e) => {
                    eprintln!("DB error: {}", e); // the output goes to io::stderr instead of io::stdout
                    continue;
                }
            };

            for website in websites{
                let url = website.url.clone();
                let website_id = website.id.clone();

                match check_website_health(&url).await {
                    Ok(result) => {
                        println!("[check] {} up={} latency={}ms", url, result.up,
                        result.response_time);

                        // Save the results and update the website status
                        // in the log itself locally as of now.
                    }
                    Err(e) => {
                        eprintln!("[check] {} ERROR: {}, website_id: {}", url, e, website_id)
                    }
                }
            }
        }
    });
}
