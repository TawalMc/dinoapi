use select::node::Node;

#[derive(Debug)]
pub struct DinoData {
    name: String,
    desc: String,
    img_url: String,
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

pub fn p_with_link(paragraph: &Node, parent: &Node) -> DinoData {
    let mut dino = DinoData::new(
        paragraph.children().next().unwrap().text(),
        parent.last_child().unwrap().text(),
        "".to_string(),
    );

    match paragraph.attr("href") {
        None => {}
        Some(img_url) => dino.set_img_url(img_url.to_string()),
    }

    return dino;
}

pub fn p_without_link(paragraph: &Node) -> DinoData {
    let name_and_desc: Vec<_> = paragraph
        .children()
        .next()
        .unwrap()
        .text()
        .split('-')
        .collect();

    return DinoData::new(
        name_and_desc[0].to_string(),
        name_and_desc[1].to_string(),
        "".to_string(),
    );
}
