use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Name};

#[derive(Debug)]
pub struct DinoData {
    name: String,
    desc: String,
    img_url: String,
}

#[derive(Debug)]
pub enum ScrapingData {
    Nothing(String),
    DinoData(DinoData),
}

impl DinoData {
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_desc(&mut self, desc: String) {
        self.desc = desc;
    }
    pub fn set_img_url(&mut self, img_url: String) {
        self.img_url = img_url;
    }
}

impl DinoData {
    pub fn new(name: String, desc: String, img_url: String) -> Self {
        DinoData {
            name,
            desc,
            img_url,
        }
    }
}

pub fn scraping(doc: &Document, id: &str) -> ScrapingData {
    let node = doc.find(Attr("id", id)).take(1).next();
    match node {
        None => ScrapingData::Nothing(format!(
            "id = {} doesn't exist in the document",
            id
        )),
        Some(paragraph) => match paragraph.find(Name("a")).take(1).next() {
            None => ScrapingData::DinoData(p_without_link(&paragraph)),
            Some(p_link) => {
                ScrapingData::DinoData(p_with_link(&p_link, &paragraph))
            }
        },
    }
}

pub fn p_with_link(paragraph: &Node, parent: &Node) -> DinoData {
    let mut dino = DinoData::new(
        paragraph
            .children()
            .next()
            .unwrap()
            .text()
            .trim()
            .to_string(),
        parent.last_child().unwrap().text().trim().to_string(),
        "".to_string(),
    );

    match paragraph.attr("href") {
        None => {}
        Some(img_url) => dino.set_img_url(img_url.to_string()),
    }

    return dino;
}

pub fn p_without_link(paragraph: &Node) -> DinoData {
    let name_and_desc = paragraph.children().next().unwrap().text();

    let dino_infos: Vec<_> = name_and_desc.split('-').collect();

    return DinoData::new(
        dino_infos[0].trim().to_string(),
        dino_infos[1].trim().to_string(),
        "".to_string(),
    );
}
