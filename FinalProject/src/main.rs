use serde::{Deserialize};
use ureq;
use std::{fs, thread, time::{Duration, Instant}};
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
struct WebsiteStatus {
    url: String,
    status: Result<u16, String>,
    response_time: Duration,
    timestamp: DateTime<Utc>,
}

impl WebsiteStatus{
    fn new(url: String, status: Result<u16, String>, response_time: Duration) -> Self{
        WebsiteStatus{
            url,
            status,
            response_time,
            timestamp: Utc::now(),
        }
    }
}

fn check_website(url: String, timeout: u64, retries: u8) -> WebsiteStatus{
    let mut attempts = 0;
    let start_time = Instant::now();

    while attempts < retries{
        let response = ureq::get(&url)
        .timeout(Duration::from_secs(timeout))
        .call();

        let duration = start_time.elapsed();

        match response{
            Ok(resp) => {
                return WebsiteStatus::new(url, Ok(resp.status()), duration);
            }
            Err(err) => {
                attempts += 1;
                if attempts == retries{
                    return WebsiteStatus::new(url, Err(err.to_string()), duration);
                }
            }
        }
    }

    let duration = start_time.elapsed();
    WebsiteStatus::new(url, Err("All retries failed".to_string()), duration)
}

fn read_from_file(file_path: &str) -> Vec<String>{
    let content = fs::read_to_string(file_path).expect("Failed to read file");

    content
        .lines()
        .map(|line| line.trim().to_string())
        .collect()
}

fn monitor_websites(urls: Vec<String>, timeout: u64, retries: u8){
    let mut handles = vec![];

    for url in urls{
        let handle = thread::spawn(move || {
            let status = check_website(url, timeout, retries);
            println!(
                "URL: {}\nStatus: {:?}\nResponse Time: {:?}\nTimestamp: {}\n",
                status.url,
                status.status,
                status.response_time,
                status.timestamp
            );
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }
}

fn main(){
    let file_path = "website_urls.txt";
    let urls = read_from_file(file_path);

    let timeout = 5;
    let retries = 3;

    monitor_websites(urls, timeout, retries);
}