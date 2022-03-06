// Given the starting page, extract the link to the first chapter.
// Because the starting page is structured differently than the chapter pages,
// this needs to be different than the function extracting each following chapter.

use scraper::{Html, Selector};

pub struct WebNovel <'a> {
    pub base_page: &'a str,
    pub seed: &'a str,
    pub first_chapter_btn: Selector,
    pub addr_next_chapter_btn: Selector,
    pub body_extractor: Selector,
    pub chapter_title: Selector,
    pub final_button: Selector
}

impl WebNovel<'_> {
    pub fn new_from_config<'b>(seed: &'b str, config_list: Vec<&'b str>) -> Option<WebNovel<'b>> {
        Some(WebNovel {
            seed,
            base_page: config_list[1],
            first_chapter_btn: Selector::parse(config_list[2]).unwrap(),
            addr_next_chapter_btn: Selector::parse(config_list[3]).unwrap(),
            body_extractor: Selector::parse(config_list[4]).unwrap(),
            chapter_title: Selector::parse(config_list[5]).unwrap(),
            final_button: Selector::parse(config_list[6]).unwrap()
        })
    }
}


pub fn html_extract_first_chapter<'a>(html: &'a Html, button: &'a Selector) -> Option<&'a str> {
//    let button: Selector = Selector::parse(r#"div[class="col-md-4 col-lg-3 fic-buttons text-center md-text-left"]"#).unwrap();
    let link_tail: Selector = Selector::parse(r#"a"#).unwrap();
    html
         .select(button)
         .next()?
         .select(&link_tail)
         .next()?
         .value()
         .attr("href")
}



// Given a chapter html page, extract the link to the following chapter.
// Will panic if there are not two of the chosen selector.
// Meant to be run on pages with links to both previous and next chapters.
pub fn addr_next_chapter<'a>(html: &'a Html, selector: &'a Selector) -> Option<&'a str> {
//    let selector = Selector::parse(r#"a[class="btn btn-primary col-xs-12"]"#).unwrap();
    html
        .select(selector)
        .nth(1)?
        .value()
        .attr("href")
}


// Given a chapter html page, extract the story text as formatted html.
pub fn extract_body(html: &Html, selector: &Selector) -> Option<String> {
//    let inner_chapter: Selector = Selector::parse(r#"div[class="chapter-inner chapter-content"]"#).unwrap();

    Some(html
        .select(selector)
        .next()?
        .html())
}


// Given a chapter html page, extract the chapter header as an html 
// formatted header.
pub fn extract_chapter_header(html: &Html, selector: &Selector) -> Option<String> {
//    let selector = Selector::parse(r#"h1[style="margin-top: 10px"][class="font-white"]"#).unwrap();

    Some(html
        .select(selector)
        .next()?
        .html())
}

pub fn final_button(html: &Html, selector: &Selector) -> Option<bool> {
//    let selector = Selector::parse(r#"button[class="btn btn-primary col-xs-12"][disabled="disabled"]"#).unwrap();
    Some(html
        .select(selector)
        .next()?
        .inner_html()
        .as_str()
        .contains("Next"))
}
