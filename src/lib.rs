// Given the starting page, extract the link to the first chapter.
// Because the starting page is structured differently than the chapter pages,
// this needs to be different than the function extracting each following chapter.

use scraper::{Html, Selector};
use std::fs;
use std::io::Write;

#[derive(Debug)]
pub struct WebNovel <'a> {
    pub website_name: &'a str,
    pub base_page: &'a str,
    pub seed: &'a str,
    pub addr_next_chapter_btn: Selector,
    pub body_extractor: Selector,
    pub output_folder: &'a str,
    pub file_name: &'a str,
    pub file_extension: &'a str,
    pub last_scraped: Option<String>,
    // indicator used to ensure next link is correct. inner_html of next link
    // will be searched for this pattern as confirmation that the link is
    // correct.
    pub indicator: &'a str,
}

impl WebNovel<'_> {
    pub fn new_from_config<'b>(seed_profile: &[&'b str], config_list: &[&'b str]) -> Option<WebNovel<'b>> {
        Some(WebNovel {
            website_name : seed_profile[0],
            seed: seed_profile[2],
            base_page: config_list[1],
            addr_next_chapter_btn: Selector::parse(config_list[2]).unwrap(),
            body_extractor: Selector::parse(config_list[3]).unwrap(),
            output_folder: config_list[4],
            file_name: seed_profile[1],
            file_extension: config_list[5],
            last_scraped: match seed_profile[3].is_empty() {
                true => None,
                false => Some(String::from(seed_profile[3])),
            },
            indicator: config_list[6],
        })
    }
}


// Given a chapter html page, extract the link to the following chapter.
// Will return None if there are not two of the chosen selector.
// Meant to be run on pages with links to both previous and next chapters.
pub fn addr_next_chapter<'a>(html: &'a Html, selector: &'a Selector, indicator: &'a str) 
-> Option<&'a str> {
    for addr in html.select(selector) {
        if addr.value().attr("href").is_some() {
            if addr.html().as_str().contains(indicator) {
                return addr.value().attr("href");
            }
        }
    }
    return None;
}


// Given a chapter html page, extract the story text as formatted html.
pub fn extract_target(html: &Html, selector: &Selector) -> Option<String> {
//    let inner_chapter: Selector = Selector::parse(r#"div[class="chapter-inner chapter-content"]"#).unwrap();

    Some(html
        .select(selector)
        .next()?
        .html())
}

// Updates ../config/seeds.txt with the address of the last scraped page.
// Every time a new page is scraped, seeds.txt is updated, so an early
// interrupt will at worst result in one chapter duplicated the next time
// the program is run..
pub fn update_last_scraped<'a>(webnovel: &'a WebNovel) -> () {
    let last_scraped = match &webnovel.last_scraped {
        None => "",
        Some(addr) => addr,
    };
    let seed_file = fs::read_to_string("../config/seeds.txt");
    let seed_list: String = seed_file.unwrap()
        .trim()
        .trim_end_matches(',')
        .split(",\n")
        .filter(|seed_profile| !seed_profile.is_empty())
        .map(|seed_profile| seed_profile.split(',').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
        .iter()
        .map(|seed_profile| seed_profile.iter()
             .enumerate()
             .flat_map(|(i, elem)|
                       match (i, elem) {
                        (3, _) => [last_scraped, ","],
                        (_, _) => [*elem, ","]
                       })
             .collect::<String>())
        .flat_map(|seed_profile| [seed_profile, "\n".to_string()])
        .collect();

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("../config/seeds.txt")
        .unwrap();
    file.write_all(seed_list.as_bytes()).unwrap();
}
