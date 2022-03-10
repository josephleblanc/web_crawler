// To Do:
// -Make it possible to update chapters instead of redownloading each time.
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

// Given seeds of the pattern <royal road><path to first chapter> from config
// file ../config/seeds.txt and page templates from ../config/page_templates.txt, 
// crawl and extract the page title and story text of each chapter as 
// formatted html to a file in ../webnovels/
fn main() -> Result<(), Box<dyn Error>> {
    let debug = true;

    let seed_file = fs::read_to_string("../config/seeds.txt").unwrap();
    let seed_list: Vec<Vec<&str>> = seed_file
        .split(",\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .collect();

    let config = fs::read_to_string("../config/page_templates.txt").unwrap();
    let template_list: Vec<Vec<&str>> = config
        .split("\n\n")
        .filter(|template| !template.is_empty())
        .map(|template| template
             .split('\n')
             .filter(|line| !line.is_empty())
             .collect())
        .collect();

    let mut full_profile: Vec<(&Vec<&str>, &Vec<&str>)> = vec![];
    for seed_profile in &seed_list {
        for template in &template_list {
            if template[0] == seed_profile[0] {
                full_profile.push((seed_profile, template));
            }
        }
    }
    
    match fs::create_dir("../web_novels") {
        Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => (),
        Err(e) => panic!("Can't create directory, error: {}", e),
        Ok(_) => (),
    };
        
    for (seed_profile, template) in &full_profile {
        if debug { println!("file_name and seed:{:?}", &seed_profile); }
        let web_novel = WebNovel::new_from_config(seed_profile, template).unwrap();

        let mut output_file = String::from(web_novel.output_folder);
        output_file.push_str(web_novel.file_name);
        output_file.push_str(web_novel.file_extension);
        if debug { println!("output_file:{}", output_file); }
    
        if debug { println!("crawling: {}", &seed_profile[1]); }
        crawl(web_novel, &output_file[..]).unwrap();
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

    // Scrape and save first chapter, and grab next link
    let mut html: Html = Html::parse_fragment(&reqwest::blocking::get(webnovel.seed)?.text()?);
    let mut body: String = extract_target(&html, &webnovel.body_extractor)
        .unwrap();
    file.write_all(body.as_bytes()).unwrap();
    let mut chapter_tail: Option<&str> = addr_next_chapter(&html, &webnovel.addr_next_chapter_btn);

    if debug { println!("chapter_tail:{:?}", chapter_tail); }

    // Main work of program
    // Loop through: format next link, reqwest next page, save, get next link
    while chapter_tail.is_some() {
        let addr_chapter;
        if chapter_tail.unwrap().starts_with('/') {
            addr_chapter = format!("{}{}", webnovel.base_page, chapter_tail.unwrap());
        } else {
            addr_chapter = String::from(chapter_tail.unwrap());
        }
        println!("Getting next page: {}", &addr_chapter);
        html = Html::parse_fragment(&reqwest::blocking::get(addr_chapter)?.text()?);
        body = extract_target(&html, &webnovel.body_extractor)
            .unwrap();
        file.write_all(body.as_bytes()).unwrap();

        chapter_tail = addr_next_chapter(&html, &webnovel.addr_next_chapter_btn);
        if debug { println!("chapter_tail:{:?}", chapter_tail); }
        thread::sleep(sleep_time);
    }
    
    Ok(())
}
