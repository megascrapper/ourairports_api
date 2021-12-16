pub mod ourairports;

fn csv_web_request_blocking(url: &str) -> reqwest::Result<String> {
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
