use std::time::Duration;
use tokop::time::{interval, Duration};    

fn start_scheduler(){
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(60));

        loop {
            ticker.tick().await;

            println!("[scheduler] running health checks");
            
        }
    })
}