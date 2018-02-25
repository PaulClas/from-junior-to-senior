use tags::Tag;
use items::ListItem;
use items::ListItemType;
use tags::get_tags_with_children;

pub fn progress_bar_color(progress: f32) -> String {
    if progress == 0.0 {
        "ff0000".to_owned()
    } else if progress == 0.5 {
        "ffff00".to_owned()
    } else if progress == 1.0 {
        "00ff00".to_owned()
    } else if progress < 0.5 {
        let green = 255.0 * progress;
        format!("ff{0:02x}00", green as i32)
    } else {
        let red = 255.0 * (100.0 - progress) / 100.0;
        format!("{0:02x}ff00", red as i32)
    }
}

pub fn render_stats(items: &Vec<ListItem>) -> String {
    use items::ListItemType::*;

    let mut stat_items: Vec<String> = vec![];

    stat_items.push("| Resource type | Progress |".to_owned());
    stat_items.push("| --- | --- |".to_owned());

    let mut total = 0.0;
    let mut total_done = 0.0;

    for item_type in vec![Article, Book, Course, Video] {
        let mut all = 0.0;
        let mut done = 0.0;

        for item in items {
            if item.item_type == item_type {
                all += 1.0;
                total += 1.0;
                if item.done.unwrap_or(false) {
                    done += 1.0;
                    total_done += 1.0;
                }
            }
        }

        let progress = if done == 0.0 { 0.0 } else { done / all };

        stat_items.push(render_stats_item(item_type.to_plural(), progress));
    }

    let progress = if total_done == 0.0 { 0.0 } else { total_done / total };

//    stat_items.push("| | |".to_owned());
    stat_items.push(render_stats_item("__Total__".to_owned(), progress));

    stat_items.join("\n")
}

pub fn render_stats_item(title: String, progress: f32) -> String {
    format!(
        "| {0} | ![{1}](http://www.yarntomato.com/percentbarmaker/button.php?barPosition={1}&leftFill=%23{2}) |",
        title,
        (progress * 100.0) as i32,
        progress_bar_color(progress)
    )
}

pub fn render_toc(tags: &Vec<Tag>) -> String {
    render_toc_vec(tags).join("\n")
}

pub fn render_toc_vec(tags: &Vec<Tag>) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    for tag in tags {
        let title = tag.clone().title;
        let children = tag.clone().children;
        result.push(format!("- [{0}](#{1})", &title, tag_name_to_link(&title)));

        let children = render_toc_vec(&children);

        for child in children {
            result.push(format!("  {0}", child));
        }
    }

    result
}

pub fn tag_name_to_link(tag: &String) -> String {
    let splitted = tag.split_whitespace();

    let mut result = String::new();

    for (i, part) in splitted.enumerate() {
        let part = part.chars().filter(|c| { c.is_ascii_alphabetic() || *c == '-' }).collect::<String>().to_lowercase();
        result = if i == 0 { part } else { format!("{0}-{1}", result, part) }
    }

    result
}

pub fn render_list(tags: &Vec<Tag>, items: &Vec<ListItem>) -> String {
    render_list_with_level(tags, items, 2)
}

pub fn render_list_with_level(tags: &Vec<Tag>, items: &Vec<ListItem>, level: usize) -> String {
    use items::ListItemType::*;

    let mut lines: Vec<String> = vec![];

    for tag in tags {
        let tag_header = format!("{0} {1}", "#".repeat(level), tag.title);

        let tag_with_children = get_tags_with_children(&vec![tag.clone()]);
        let tag_items: Vec<_> = items.iter().filter(|i| {
            i.tags.iter().any(|t| {
                tag_with_children.iter().any(|tw| { *tw == *t })
            })
        }).map(|x| { x.clone() }).collect();

        if (tag_items.len() != 0) {
            lines.push("".to_owned());
            lines.push(tag_header);
            lines.push("".to_owned());

            lines.push(render_stats(&tag_items));
        }

        for item_type in vec![Cheatsheet, Article, Book, Course, Video] {
            let items = items.iter()
                .filter(|i| { i.item_type == item_type })
                .filter(|i| { i.tags.iter().any(|t| { *t == tag.title }) })
                .collect::<Vec<_>>();

            if items.len() != 0 {
                lines.push(format!("- {0} __{1}__", item_type.to_emoji(), item_type.to_plural()));
            }

            for item in items {
                lines.push(format!("  {0}", item.to_markdown()));
            }
        }

        lines.push(render_list_with_level(&tag.children, items, level + 1));
    }

    lines.join("\n")
}