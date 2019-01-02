#[macro_use] extern crate clap;
extern crate mustache;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

use clap::{App, Arg};
use mustache::MapBuilder;

use items::ItemsList;
use tags::TagsList;
use render::markdown::render_stats;
use render::markdown::render_toc;
use render::markdown::render_list;

mod items;
mod tags;
mod render;

fn main() {
    let matches = App::new("from-junior-to-senior")
        .version(crate_version!())
        .about("Render a TODO list to markdown")
        .arg(Arg::with_name("INPUT_FORMAT")
            .short("f")
            .long("input-format")
            .value_name("INPUT_FORMAT")
            .takes_value(true)
            .default_value("json")
            .possible_value("json")
            .help("Sets input format")
        )
        .arg(Arg::with_name("OUTPUT_FORMAT")
            .short("F")
            .long("output-format")
            .value_name("OUTPUT_FORMAT")
            .takes_value(true)
            .default_value("markdown")
            .possible_value("markdown")
            .help("Sets output format")
        )
        .arg(Arg::with_name("OUTPUT_FILE")
            .short("o")
            .long("output-file")
            .value_name("OUTPUT_FILE")
            .takes_value(true)
            .required(true)
            .help("Sets output file path")
        )
        .arg(Arg::with_name("RESOURCES_DIR")
            .required(true)
            .help("Sets resources directory")
        )
        .get_matches();

    let resources_dir = matches.value_of("RESOURCES_DIR").unwrap();
    let output_file = matches.value_of("OUTPUT_FILE").unwrap();

    let items_file_path = Path::new(resources_dir).join("items.json");
    let tags_file_path = Path::new(resources_dir).join("tags.json");
    let template_file_path = Path::new(resources_dir).join("template.md");
    let output_file_path = Path::new("./").join(output_file);

    let items_json = read_file(items_file_path);
    let tags_json = read_file(tags_file_path);
    let template = read_file(template_file_path);

    let items: Result<ItemsList, _> = serde_json::from_str(items_json.as_ref());
    let tags: Result<TagsList, _> = serde_json::from_str(tags_json.as_ref());

    let items = items.expect("failed to parse items.json");
    let tags = tags.expect("failed to parse tags.json");

    let stats = render_stats(&items.0, false);
    let toc = render_toc(&tags.0);
    let list = render_list(&tags.0, &items.0);

    let data = MapBuilder::new()
        .insert_str("stats", stats)
        .insert_str("table_of_contents", toc)
        .insert_str("content", list)
        .build();

    let template = mustache::compile_str(template.as_ref()).unwrap();

    let mut output_file = File::create(output_file_path).expect("failed to open output file");

    template.render_data(&mut output_file, &data).unwrap();
}

fn read_file(path: PathBuf) -> String {
    let mut buf = String::new();
    let mut file = File::open(path).expect("file not found");

    file.read_to_string(&mut buf)
        .expect("something went wrong reading the file");

    buf
}