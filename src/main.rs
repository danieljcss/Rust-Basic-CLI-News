use std::error::Error;  // Standard errors library
use serde::Deserialize; // Used to parse JSON
use colour::{dark_green, yellow}; // Colours to add CLI
use dotenv;
use std::env;

// The attribute derive allows to implement some traits to the items below. If a struct implenets some traits all its contents must implement the same traits
#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String
}

// Result<T,E> is an enum object that retrives a result T or an error E. The result should typically be Ok()
// Box<dyn Error> provides a pointer to an object that implements the Error trait
// A trait is a set of methods that can be implemented by any data type
fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>>{
    let response = ureq::get(url).call()?.into_string()?;
    // The ? sign allows us to retreive the success value of the operaation or "raise an error" if it fails
    let articles: Articles = serde_json::from_str(&response)?;
    Ok(articles)
}

fn display_articles(articles_list: &Articles) {
    for article in &articles_list.articles {
        dark_green!("> {}\n", article.title);
        yellow!("- {}\n\n", article.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().expect("Failed to read .env file");
    let api_key: &str = &env::var("API_KEY").expect("API_KEY not found");
    let mut url: String = "https://newsapi.org/v2/everything?q=keyword&apiKey=".to_owned();
    url.push_str(api_key);
    let articles: Articles = get_articles(&url)?;
    display_articles(&articles);

    Ok(())
}
