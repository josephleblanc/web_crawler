use std::error::Error;
use std::fs;

pub fn download_seed(seed: &str) -> Result<(), Box<dyn Error>> {
    if !seed.starts_with("https://www.royalroad.com/fiction/") {
        panic!("Did not enter a valid royalroad.com address. \
        Enter an address that starts with https://www.royalroad.com/fiction/");
    }

    let resp = reqwest::blocking::get(seed)?.text()?;
    println!("get html from seed: success");

    fs::write("output.html", resp)?;
    println!("write html to output.html: success");

    Ok(())
}

// Given the starting page, extract the link to the first chapter.
// Because the starting page is structured differently than the chapter pages,
// this needs to be different than the function extracting each following chapter.
pub fn html_extract_first_chapter(html_text: &str) -> &str {
    let next_start_location = html_text
        .find("<div class=\"col-md-4 col-lg-3 fic-buttons text-center md-text-left\">")
        .expect("next_start_location is wrong");
    let first_chapter_link_index = html_text[next_start_location..]
        .find("<a href=\"")
        .expect("first_chapter_link_index is wrong")
        + "<a href=\"".len();

    let next_html_start = next_start_location + first_chapter_link_index;
    let next_html_end = html_text[next_html_start..]
        .find("\"")
        .expect("No closing \" found for next_html_end")
        + next_html_start;

    let next_tail = &html_text[next_html_start..next_html_end];
    println!("next chapter link: {}", next_tail);

    next_tail
}

// Given a chapter html page, extract the link to the following chapter.
// Because the identifying tag for the 'next chapter' link is very similar to the
// 'previous chapter' link, this function calls itself recursively.
// This recursion will not happen infinitely, even if there is no next link, because
// there are only 2 cases where the identifying tag is used.
pub fn addr_next_chapter(chapter_html: &str) -> Option<&str> {
    let next_chapter_button_id = "<a class=\"btn btn-primary col-xs-12\" href=\"";
    let button_id_close = "\">";

    let next_start_i = chapter_html
        .find(next_chapter_button_id)?
        + next_chapter_button_id.len();
    let next_end_i = chapter_html[next_start_i..]
        .find(button_id_close)?
        + next_start_i;

    if chapter_html[next_end_i+button_id_close.len()..
        next_end_i+button_id_close.len()+100].find("Next") == None {
            return addr_next_chapter(&chapter_html[next_end_i+button_id_close.len()..]);
    }

    Some(&chapter_html[next_start_i..next_end_i])
}

// Given a chapter html page, extract the story text as formatted html.
pub fn extract_body(html_text: &str) -> Option<&str> {
    let body_id_start = "<div class=\"chapter-inner chapter-content\">";
    let body_id_end = "</div>";

    let start_body = html_text.find(body_id_start)? + body_id_start.len();
    let end_body = html_text[start_body..].find(body_id_end)? + start_body;
    
    Some(&html_text[start_body..end_body])
}
