// Given the starting page, extract the link to the first chapter.
// Because the starting page is structured differently than the chapter pages,
// this needs to be different than the function extracting each following chapter.

use scraper::{Html, Selector};

#[derive(Debug)]
pub struct WebNovel <'a> {
    pub base_page: &'a str,
    pub seed: &'a str,
    pub addr_next_chapter_btn: Selector,
    pub body_extractor: Selector,
    pub output_folder: &'a str,
    pub file_name: &'a str,
    pub file_extension: &'a str,
}

impl WebNovel<'_> {
    pub fn new_from_config<'b>(seed_profile: &[&'b str], config_list: &[&'b str]) -> Option<WebNovel<'b>> {
        Some(WebNovel {
            seed: seed_profile[2],
            base_page: config_list[1],
            addr_next_chapter_btn: Selector::parse(config_list[2]).unwrap(),
            body_extractor: Selector::parse(config_list[3]).unwrap(),
            output_folder: config_list[4],
            file_name: seed_profile[1],
            file_extension: config_list[5],
        })
    }
}


// Given a chapter html page, extract the link to the following chapter.
// Will return None if there are not two of the chosen selector.
// Meant to be run on pages with links to both previous and next chapters.
pub fn addr_next_chapter<'a>(html: &'a Html, selector: &'a Selector) -> Option<&'a str> {
    html
        .select(selector)
        .nth(1)?
        .value()
        .attr("href")
}


// Given a chapter html page, extract the story text as formatted html.
pub fn extract_target(html: &Html, selector: &Selector) -> Option<String> {
//    let inner_chapter: Selector = Selector::parse(r#"div[class="chapter-inner chapter-content"]"#).unwrap();

    Some(html
        .select(selector)
        .next()?
        .html())
}
