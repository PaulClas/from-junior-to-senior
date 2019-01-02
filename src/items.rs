pub struct Link {
    pub title: String,
    pub url: String,
}

impl Link {
    pub fn to_markdown(self) -> String {
        format!("[{0}]({1})", self.title, self.url)
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum ListItemType {
    Article,
    Book,
    Cheatsheet,
    Course,
    Paper,
    Slides,
    Video,
}

impl ListItemType {
    #[allow(unused)]
    pub fn to_string(self) -> String {
        match self {
            ListItemType::Article => "Article",
            ListItemType::Book => "Book",
            ListItemType::Cheatsheet => "Cheatsheet",
            ListItemType::Course => "Course",
            ListItemType::Paper => "Paper",
            ListItemType::Slides => "Slides",
            ListItemType::Video => "Video",
        }.to_owned()
    }

    pub fn to_plural(self) -> String {
        match self {
            ListItemType::Article => "Articles",
            ListItemType::Book => "Books",
            ListItemType::Cheatsheet => "Cheatsheets",
            ListItemType::Course => "Courses",
            ListItemType::Paper => "Papers",
            ListItemType::Slides => "Slides",
            ListItemType::Video => "Videos",
        }.to_owned()
    }

    pub fn to_emoji(self) -> String {
        match self {
            ListItemType::Article => ":pencil:",
            ListItemType::Book => ":book:",
            ListItemType::Cheatsheet => ":paperclip:",
            ListItemType::Course => ":mortar_board:",
            ListItemType::Paper => ":page_facing_up:",
            ListItemType::Slides => ":stars:",
            ListItemType::Video => ":movie_camera:",
        }.to_owned()
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub enum ListItemLanguage {
    English,
    Russian,
}

impl ListItemLanguage {
    pub fn to_emoji(&self) -> String {
        match *self {
            ListItemLanguage::English => ":us:",
            ListItemLanguage::Russian => ":ru:",
        }.to_owned()
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct ListItemAuthor {
    name: String,
    link: Option<String>,
}

impl ListItemAuthor {
    fn to_markdown(&self) -> String {
        self.link.clone()
            .map(|l| { Link { title: self.name.clone(), url: l } }.to_markdown())
            .unwrap_or(self.name.clone())
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct ListItem {
    pub title: String,
    pub link: String,
    pub language: ListItemLanguage,
    pub item_type: ListItemType,
    pub authors: Vec<ListItemAuthor>,
    pub tags: Vec<String>,
    pub done: Option<bool>,
}

impl ListItem {
    pub fn to_markdown(&self) -> String {
        let mut parts = vec!["-".to_owned()];

        match self.done {
            Some(true) => parts.push("[x]".to_owned()),
            Some(false) => parts.push("[ ]".to_owned()),
            None => {}
        };

        let ref language = self.language;
        language.to_emoji();
        parts.push(self.language.to_emoji());
        parts.push(Link { title: self.title.clone(), url: self.link.clone() }.to_markdown());

        if self.authors.len() != 0 {
            parts.push("by".to_string());

            for (i, author) in self.authors.iter().enumerate() {
                parts.push(
                    if i == self.authors.len() - 1 {
                        author.to_markdown()
                    } else {
                        author.to_markdown() + ","
                    }
                );
            }
        }

        parts.join(" ")
    }
}

#[derive(Serialize, Deserialize)]
pub struct ItemsList(pub Vec<ListItem>);