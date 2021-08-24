use std::error::Error;

use colour::{dark_green, yellow};

use newsapi::{get_articles, Articles};

fn render_articles(articles: &Articles) {
    for i in &articles.articles {
        dark_green!("> {}\n", i.title);
        yellow!("- {}\n\n", i.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let url =
        "https://newsapi.org/v2/top-headlines?country=us&apiKey=d529f926002e4ed19607bc0e04202c21";
    let articles = get_articles(url)?;

    render_articles(&articles);

    Ok(())
}
