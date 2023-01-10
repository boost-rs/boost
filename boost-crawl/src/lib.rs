use lazy_static::lazy_static;
use reqwest::Response;
// use scraper::Html;
use regex::Regex;

pub async fn extract_links(res: Response) -> Vec<String> {
    lazy_static! {
        static ref URL_REGEX: Regex = Regex::new(r"https?://(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&/=]*)").unwrap();
    }
    // let html = Html::parse_document(&res.text().await.unwrap());
    URL_REGEX
        .find_iter(&res.text().await.unwrap())
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use reqwest::get;

    use crate::extract_links;
    #[tokio::test]
    async fn it_works() {
        let res = get("https://example.com").await.unwrap();
        assert_eq!(
            extract_links(res).await,
            vec!["https://www.iana.org/domains/example".to_owned()]
        )
    }
}
