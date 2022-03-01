// Given the starting page, extract the link to the first chapter.
// Because the starting page is structured differently than the chapter pages,
// this needs to be different than the function extracting each following chapter.

use scraper::{Html, Selector};

pub fn html_extract_first_chapter(html: &Html) -> Option<&str> {
    let button: Selector = Selector::parse(r#"div[class="col-md-4 col-lg-3 fic-buttons text-center md-text-left"]"#).unwrap();
    let link_tail: Selector = Selector::parse(r#"a"#).unwrap();
    html
         .select(&button)
         .next()?
//         .unwrap()
         .select(&link_tail)
         .next()?
//         .unwrap()
         .value()
         .attr("href")
//         .unwrap()
}



// Given a chapter html page, extract the link to the following chapter.
// Will panic if there are not two of the chosen selector.
// Meant to be run on pages with links to both previous and next chapters.
pub fn addr_next_chapter(html: &Html) -> Option<&str> {
    let selector = Selector::parse(r#"a[class="btn btn-primary col-xs-12"]"#).unwrap();

    html
        .select(&selector)
        .nth(1)?
//        .unwrap()
        .value()
        .attr("href")
//        .unwrap()
}


// Given a chapter html page, extract the story text as formatted html.
pub fn extract_body(html: &Html) -> Option<String> {
    let inner_chapter: Selector = Selector::parse(r#"div[class="chapter-inner chapter-content"]"#).unwrap();

    Some(html
        .select(&inner_chapter)
        .next()?
//        .unwrap()
        .inner_html())
}


// Given a chapter html page, extract the chapter header as an html 
// formatted header.
pub fn extract_chapter_header(html: &Html) -> Option<String> {
    let selector = Selector::parse(r#"h1[style="margin-top: 10px"][class="font-white"]"#).unwrap();

    Some(html
        .select(&selector)
        .next()?
//        .unwrap()
        .inner_html())
}

pub fn final_button(html: &Html) -> Option<bool> {
    let selector = Selector::parse(r#"button[class="btn btn-primary col-xs-12"][disabled="disabled"]"#).unwrap();
//    let debug_print = html
//        .select(&selector)
//        .next()?
//        .inner_html();
//    println!("debug_print:{}", debug_print);
    Some(html
        .select(&selector)
        .next()?
        .inner_html()
        .as_str()
        .contains("Next"))
}
