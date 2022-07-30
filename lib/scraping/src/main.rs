mod extraction;
mod tests;

use crate::extraction::{p_with_link, p_without_link};
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

const THOUGHCO_DINO_WEBSITE: &'static str =
    "https://www.thoughtco.com/dinosaurs-a-to-z-1093748";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::get(THOUGHCO_DINO_WEBSITE).await?.text().await?;
    let doc = Document::from(res.as_str());

    // "mntl-sc-block_1-0-50"
    let node = doc.find(Attr("id", "mntl-sc-block_1-0-28")).take(1).next();
    match node {
        None => println!("Doesn't contain this id"),
        Some(paragraph) => match paragraph.find(Name("a")).take(1).next() {
            None => println!("p: {:#?}", p_without_link(&paragraph)),
            Some(p_link) => {
                println!("p: {:#?}", p_with_link(&p_link, &paragraph));
            }
        },
    }

    Ok(())
}

// let picture_source = para_link.attr("href");
// let dino_name = para_link.children().next().unwrap();
// let description = node.last_child().unwrap();
// println!("name: {} \n img_url: {:#?} \n description: {}", dino_name.text(), picture_source, description.text());
