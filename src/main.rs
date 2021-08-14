use std::error::Error;

struct Articles {
    articles: Vec<Article>
}

struct Article {
    title: String,
    url: String
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;

    dbg!(response);

    todo!()
}

fn main() {
    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=d529f926002e4ed19607bc0e04202c21";
    let articles = get_articles(url);
}
