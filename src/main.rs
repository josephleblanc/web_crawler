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
    final_button};

// Given a seed of the pattern <royal_road><path_to_coverpage>, crawl and
// extract the story text of each chapter as formatted html to a file named 
// 'body.html'.
fn main() -> Result<(), Box<dyn Error>> {
    let royal_road = "https://www.royalroad.com";
    let seed = "https://www.royalroad.com/fiction/50553/double-blind-a-modern-litrpg";
    if !seed.starts_with("https://www.royalroad.com/fiction/") {
        panic!("Did not enter a valid royalroad.com address. \
        Enter an address that starts with https://www.royalroad.com/fiction/");
    }

    // Reqwest first cover page and extract link to first chapter
    let first_chapter_html: Html = Html::parse_fragment(&reqwest::blocking::get(seed)?.text()?);
    let mut chapter_tail: &str = html_extract_first_chapter(&first_chapter_html)
        .unwrap();

    let mut addr_chapter = format!("{}{}", royal_road, chapter_tail);
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
    let mut current_body: String = extract_body(&html_chapter)
        .unwrap();
    file.write_all(extract_chapter_header(&html_chapter)
                   .unwrap()
                   .as_bytes())?;
    file.write_all(current_body.as_bytes())?;
    chapter_tail = html_chapter
        .select(&Selector::parse(r#"a[class="btn btn-primary col-xs-12"]"#).unwrap())
        .next()
        .unwrap()
        .value()
        .attr("href")
        .unwrap();
    addr_chapter = format!("{}{}", royal_road, chapter_tail);

    // Rate limiting, chosen arbitrarily
    let sleep_time = time::Duration::from_millis(200);

    // Loop through chapters, extract next link, download contents, save
    println!("before");
    while final_button(&html_chapter) != Some(true) {
        println!("Getting next page: {}", &addr_chapter);
        html_chapter = Html::parse_fragment(&reqwest::blocking::get(addr_chapter)?.text()?);
        current_body = extract_body(&html_chapter)
            .unwrap();
        file.write_all(extract_chapter_header(&html_chapter)
                       .unwrap()
                       .as_bytes())?;
        file.write_all(current_body.as_bytes())?;

        chapter_tail = addr_next_chapter(&html_chapter)
            .unwrap();
        addr_chapter = format!("{}{}", royal_road, chapter_tail);

        thread::sleep(sleep_time);
    }
    
    Ok(())
}
