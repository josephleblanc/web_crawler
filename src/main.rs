// To Do:
// -Actually do proper error handling (once I know better what that means).
// -Write some clever tests (once I know how to do that).
// -Currently panics upon finishing successfully, design better
// -Prompt user to enter address of seed page
// -Refactor to allow multiple seed pages, possibly allowing user to queue up
// several seed pages or possibly read seed pages from file.

// Stretch Goals/Ideas:
// -Use concurrency
// -Can I set this up to run on a cloud server, checking for updates?
// -This would be nice to have on my phone. How difficult is that to set up?
// -The next step with this output is to convert to an .epub - how easy is
// it to do so? Would be great to have the tool spit out a nice .epub

use std::error::Error;
use std::fs;
use std::io::Write;
use std::{thread, time};
use scraper::{Html, Selector};

use web_crawler::{
    html_extract_first_chapter, 
    addr_next_chapter, 
    extract_body, 
    extract_chapter_header,
    final_button,
    WebNovel};

// Given a seed of the pattern <royal_road><path_to_coverpage>, crawl and
// extract the story text of each chapter as formatted html to a file named 
// 'body.html'.
fn main() -> Result<(), Box<dyn Error>> {
    let seed = "https://www.royalroad.com/fiction/21188/forge-of-destiny";
    if !seed.starts_with("https://www.royalroad.com/fiction/") {
        panic!("Did not enter a valid royalroad.com address. \
        Enter an address that starts with https://www.royalroad.com/fiction/");
    }

    let double_blind = WebNovel {
        base_page: "https://www.royalroad.com",
        seed: "https://www.royalroad.com/fiction/21188/forge-of-destiny",
        first_chapter_btn: Selector::parse(r#"div[class="col-md-4 col-lg-3 fic-buttons text-center md-text-left"]"#).unwrap(),
        addr_next_chapter_btn: Selector::parse(r#"a[class="btn btn-primary col-xs-12"]"#).unwrap(),
        body_extractor: Selector::parse(r#"div[class="chapter-inner chapter-content"]"#).unwrap(),
        chapter_title: Selector::parse(r#"h1[style="margin-top: 10px"][class="font-white"]"#).unwrap(),
        final_button: Selector::parse(r#"button[class="btn btn-primary col-xs-12"][disabled="disabled"]"#).unwrap(),
    };

    crawl(double_blind)?;

    Ok(())
}

fn crawl(webnovel: WebNovel) -> Result<(), Box<dyn Error>> {

    // Reqwest first cover page and extract link to first chapter
    let first_chapter_html: Html = Html::parse_fragment(&reqwest::blocking::get(webnovel.seed)?.text()?);
    let mut chapter_tail: &str = html_extract_first_chapter(&first_chapter_html, &webnovel.first_chapter_btn)
        .unwrap();

    let mut addr_chapter = format!("{}{}", webnovel.base_page, chapter_tail);
    let mut html_chapter: Html = Html::parse_fragment(&reqwest::blocking::get(&addr_chapter)?.text()?);

    // Create output file
    // fs::File::create() will truncate file if the file already exists
    fs::File::create("body.html")?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("body.html")
        .unwrap();

    // Write Chapter 1 to file body.html
    let mut current_body: String = extract_body(&html_chapter, &webnovel.body_extractor)
        .unwrap();
    file.write_all(extract_chapter_header(&html_chapter, &webnovel.chapter_title)
                   .unwrap()
                   .as_bytes())?;
    file.write_all(current_body.as_bytes())?;
    chapter_tail = html_chapter
        .select(&webnovel.addr_next_chapter_btn)
        .next()
        .unwrap()
        .value()
        .attr("href")
        .unwrap();
    addr_chapter = format!("{}{}", webnovel.base_page, chapter_tail);

    // Rate limiting, chosen arbitrarily
    let sleep_time = time::Duration::from_millis(200);

    // Loop through chapters, extract next link, download contents, save
    println!("before");
    while final_button(&html_chapter, &webnovel.final_button) != Some(true) {
        println!("Getting next page: {}", &addr_chapter);
        html_chapter = Html::parse_fragment(&reqwest::blocking::get(addr_chapter)?.text()?);
        current_body = extract_body(&html_chapter, &webnovel.body_extractor)
            .unwrap();
        file.write_all(extract_chapter_header(&html_chapter, &webnovel.chapter_title)
                       .unwrap()
                       .as_bytes())?;
        file.write_all(current_body.as_bytes())?;

        chapter_tail = addr_next_chapter(&html_chapter, &webnovel.addr_next_chapter_btn)
            .unwrap();
        addr_chapter = format!("{}{}", webnovel.base_page, chapter_tail);

        thread::sleep(sleep_time);
    }
    
    Ok(())
}
