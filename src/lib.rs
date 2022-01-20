use log::debug;
use reqwest::blocking::Client;
use std::time::Duration;

pub mod ourairports;

// time limit for downloading one file
const REQUEST_TIMEOUT: u64 = 300;

fn web_request_blocking(url: &str) -> Result<String, reqwest::Error> {
    debug!("requesting data from {}", url);
    //reqwest::blocking::get(url)?.text()
    let client = Client::builder()
        .timeout(Duration::from_secs(REQUEST_TIMEOUT))
        .build()?;
    client.get(url).send()?.text()


}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
