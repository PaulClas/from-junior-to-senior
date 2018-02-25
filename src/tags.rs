#[derive(Serialize, Deserialize)]
#[derive(Clone)]
#[derive(Debug)]
pub struct Tag {
    pub title: String,
    pub children: Vec<Tag>
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct TagsList(pub Vec<Tag>);

pub fn get_tags_with_children(tags: &Vec<Tag>) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    for tag in tags {
        result.push(tag.title.clone());

        for tag in get_tags_with_children(&tag.children) {
            result.push(tag);
        }
    }

    result
}