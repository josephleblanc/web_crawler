// To Do:
// -Significantly rework config files to reflect new logic after last refactor
// -Rework config files to be more intuitive by changing splitter from ',' to '\n'
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
    addr_next_chapter, 
    extract_target, 
    WebNovel};

// Given a seed of the pattern <royal road><path to first shcapter>, crawl and
// extract the story text of each chapter as formatted html to a file.
fn main() -> Result<(), Box<dyn Error>> {
    let debug = false;

    let seed_file = fs::read_to_string("../config/seeds.txt").unwrap();
    let seed_list: Vec<Vec<&str>> = seed_file
        .split(",\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .collect();

    let config = fs::read_to_string("../config/page_templates.txt").unwrap();
    let template: Vec<&str> = config
        .split("\n")
        .filter(|line| !line.is_empty())
        .collect();
    
    for seed in &seed_list {
        if debug { println!("file_name and seed:{:?}", &seed); }
        let web_novel = WebNovel::new_from_config(seed[1], &template, seed[0]).unwrap();

        let mut output_file = String::from(web_novel.output_folder);
        output_file.push_str(web_novel.file_name);
        output_file.push_str(web_novel.file_extension);
        if debug { println!("output_file:{}", output_file); }
    
        if debug { println!("crawling: {}", &seed[1]); }
        crawl(web_novel, &output_file[..])?;
        
    }

    Ok(())
}

fn crawl(webnovel: WebNovel, output_file: &str) -> Result<(), Box<dyn Error>> {
    let debug = false;

    // Create output file
    // fs::File::create() will truncate file if the file already exists
    fs::File::create(output_file)?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(output_file)
        .unwrap();

    // Rate limiting, chosen arbitrarily
    let sleep_time = time::Duration::from_millis(200);

    // Scrape and save first chapter
    let mut html: Html = Html::parse_fragment(&reqwest::blocking::get(webnovel.seed)?.text()?);
    let mut body: String = extract_target(&html, &webnovel.body_extractor)
        .unwrap();
    let mut page_head = extract_target(&html, &webnovel.page_title)
        .unwrap();
    file.write_all(page_head.as_bytes())?;
    file.write_all(body.as_bytes())?;
    let mut chapter_tail: Option<&str> = addr_next_chapter(&html, &webnovel.addr_next_chapter_btn);

    if debug { println!("chapter_tail:{:?}", chapter_tail); }

    while chapter_tail.is_some() {
        let addr_chapter = format!("{}{}", webnovel.base_page, chapter_tail.unwrap());
        println!("Getting next page: {}", &addr_chapter);
        html = Html::parse_fragment(&reqwest::blocking::get(addr_chapter)?.text()?);
        body = extract_target(&html, &webnovel.body_extractor)
            .unwrap();
        page_head = extract_target(&html, &webnovel.page_title)
            .unwrap();

        file.write_all(page_head.as_bytes())?;
        file.write_all(body.as_bytes())?;

        chapter_tail = addr_next_chapter(&html, &webnovel.addr_next_chapter_btn);

        thread::sleep(sleep_time);
    }
    
    Ok(())
}
