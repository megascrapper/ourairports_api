use log::debug;

pub mod ourairports;

// async fn web_request(url: &str) -> Result<String, reqwest::Error> {
//     debug!("requesting data from {}", url);
//     reqwest::get(url).await?.text().await
// }

fn web_request_blocking(url: &str) -> Result<String, reqwest::Error> {
    debug!("requesting data from {}", url);
    reqwest::blocking::get(url)?.text()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
