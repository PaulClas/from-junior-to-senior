#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Tag {
    pub title: String,
    pub children: Vec<Tag>
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct TagsList(pub Vec<Tag>);