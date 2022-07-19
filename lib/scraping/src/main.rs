use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

const THOUGHCO_DINO_WEBSITE: &'static str = "https://www.thoughtco.com/dinosaurs-a-to-z-1093748";

struct DinoData {
    name: String,
    img_url: String,
    description: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(THOUGHCO_DINO_WEBSITE).await?.text().await?;
    let document = Document::from(resp.as_str());

    let p_node = document.find(Attr("id", "mntl-sc-block_1-0-50")).take(1).next();
    match p_node {
        None => println!("Doesn't contain this id"),
        Some(node) => {
            let para_with_link = node.find(Name("a")).take(1).next();
            match para_with_link {
                None => println!("Doesn't contain a link"),
                Some(para_link) => {
                    let picture_source = para_link.attr("href");
                    let dino_name = para_link.children().next().unwrap();
                    let description = node.last_child().unwrap();
                    println!("name: {} \n img_url: {:#?} \n description: {}", dino_name.text(), picture_source, description.text());
                }
            }
        }
    }

    Ok(())
}
