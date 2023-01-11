use std::collections::HashMap;

use lazy_static::lazy_static;
use maplit::hashmap;
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
        .map(|digits| defang(digits.as_str()))
        .collect()
}

fn defang(text: &str) -> String {
    let defangs: HashMap<&str, &str> = hashmap! {
        " (dot) " => ".",
        "(dot)" => ".",
        " (at) " => "@",
        "(at)" => "@",
        "[.]" => ".",
        "[&]" => "&",
        "[@]" => "@",
        "[/]" => "/",
        "hxxps" => "https",
        "hxxp" => "http",
        "fxps" => "ftps",
        "fxp" => "ftp"
    };
    let mut ans = text.to_owned();
    for (df, f) in defangs.into_iter() {
        ans = ans.replace(df, f);
    }
    ans
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
