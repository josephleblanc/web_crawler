// To Do:
// -Actually do proper error handling (once I know better what that means).
// -Write some clever tests (once I know how to do that).
// -Prompt user to enter address of seed page

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
use scraper::Html;

use web_crawler::{
    html_extract_first_chapter, 
    addr_next_chapter, 
    extract_body, 
    extract_chapter_header,
    nav_buttons,
    WebNovel};

// Given a seed of the pattern <royal_road><path_to_coverpage>, crawl and
// extract the story text of each chapter as formatted html to a file.
fn main() -> Result<(), Box<dyn Error>> {
    let seed_file = fs::read_to_string("../config/seeds.txt").unwrap();
    let seed_list: Vec<Vec<&str>> = seed_file
        .split(",\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .collect();

    let config = fs::read_to_string("../config/page_templates.txt").unwrap();
    let template: Vec<&str> = config
        .split(',')
        .collect();
    
    for seed in &seed_list {
        println!("title and seed:{:?}", &seed);
        let web_novel = WebNovel::new_from_config(seed[1], &template, seed[0]).unwrap();
        let mut output_file = String::from(web_novel.output_folder);
        output_file.push_str(web_novel.title);
        output_file.push_str(web_novel.file_extension);
        println!("output_file:{}", output_file);
    
        println!("crawling: {}", &seed[1]);
        crawl(web_novel, &output_file[..])?;
        
    }

    Ok(())
}

fn crawl(webnovel: WebNovel, output_file: &str) -> Result<(), Box<dyn Error>> {
    let debug = true;

    // Reqwest first cover page and extract link to first chapter
    let first_chapter_html: Html = Html::parse_fragment(&reqwest::blocking::get(webnovel.seed)?.text()?);
    let mut chapter_tail: &str = html_extract_first_chapter(&first_chapter_html, &webnovel.first_chapter_btn)
        .unwrap();

    let mut addr_chapter = format!("{}{}", webnovel.base_page, chapter_tail);
    let mut html_chapter: Html = Html::parse_fragment(&reqwest::blocking::get(&addr_chapter)?.text()?);

    // Create output file
    // fs::File::create() will truncate file if the file already exists
    fs::File::create(output_file)?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(output_file)
        .unwrap();

    // Write Chapter 1 to output_file
    let mut current_body: String = extract_body(&html_chapter, &webnovel.body_extractor)
        .unwrap();
    file.write_all(extract_chapter_header(&html_chapter, &webnovel.chapter_title)
                   .unwrap()
                   .as_bytes())?;
    file.write_all(current_body.as_bytes())?;

    // Early exit if only one page to crawl with no next link
//    println!("nav_buttons:{:?}", nav_buttons(&html_chapter, &webnovel.nav_buttons, &webnovel.nav_validator));
    if nav_buttons(&html_chapter, &webnovel.nav_buttons, &webnovel.nav_validator, &webnovel.nav_name) == Some("disabled") { return Ok(()) }
    if debug == true { println!("nav_buttons returns:{:?}", nav_buttons(&html_chapter, &webnovel.nav_buttons, &webnovel.nav_validator, &webnovel.nav_name)); }
    if debug == true { println!("webnovel: {:?}", webnovel); }
    chapter_tail = html_chapter
        .select(&webnovel.addr_next_chapter_btn)
        .next()
        .unwrap()
        .value()
        .attr("href")
        .unwrap();
    addr_chapter = format!("{}{}", webnovel.base_page, chapter_tail);
    if debug == true { println!("addr_chapter:{}", addr_chapter); }

    // Rate limiting, chosen arbitrarily
    let sleep_time = time::Duration::from_millis(200);

    // Loop through chapters, extract next link, download contents, save
    println!("before");
    while nav_buttons(&html_chapter, &webnovel.nav_buttons, &webnovel.nav_validator, &webnovel.nav_name).is_none() {
        if debug == true { println!("nav_buttons returns:{:?}", nav_buttons(&html_chapter, &webnovel.nav_buttons, &webnovel.nav_validator, &webnovel.nav_name)); }

        println!("Getting next page: {}", &addr_chapter);
        html_chapter = Html::parse_fragment(&reqwest::blocking::get(addr_chapter)?.text()?);
        current_body = extract_body(&html_chapter, &webnovel.body_extractor)
            .unwrap();
        file.write_all(extract_chapter_header(&html_chapter, &webnovel.chapter_title)
                       .unwrap()
                       .as_bytes())?;
        file.write_all(current_body.as_bytes())?;

        if !nav_buttons(&html_chapter, &webnovel.nav_buttons, &webnovel.nav_validator, &webnovel.nav_name).is_none() {
            return Ok(());
        }
        chapter_tail = addr_next_chapter(&html_chapter, &webnovel.addr_next_chapter_btn)
            .unwrap();
        addr_chapter = format!("{}{}", webnovel.base_page, chapter_tail);

        thread::sleep(sleep_time);
    }
    
    Ok(())
}
