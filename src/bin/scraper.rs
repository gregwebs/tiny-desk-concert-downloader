use anyhow::{Context, Result};
use reqwest::blocking::Client;
use scraper::{Html, Selector, ElementRef, node::Element};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Musician {
    name: String,
    instruments: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Song {
    songNumber: usize,
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConcertInfo {
    artist: String,
    source: String,
    show: String,
    date: Option<String>,
    album: Option<String>,
    description: Option<String>,
    setList: Vec<Song>,
    musicians: Vec<Musician>,
}

fn scrape_data(url: &str) -> Result<()> {
    println!("Navigating to {}...", url);
    
    // Create HTTP client
    let client = Client::new();
    
    // Fetch the page
    let response = client.get(url)
        .send()
        .context("Failed to send request")?;
    
    let html = response.text().context("Failed to get response text")?;
    let document = Html::parse_document(&html);
    
    // Extract the artist name from the title
    let title_selector = Selector::parse("title").unwrap();
    let title = document.select(&title_selector)
        .next()
        .map(|element| element.inner_html())
        .unwrap_or_default();
    
    let artist_name = title.split(':')
        .next()
        .unwrap_or("")
        .trim()
        .to_string();
    
    println!("Artist: {}", artist_name);
    
    // Extract story title
    let story_title_selector = Selector::parse(".storytitle h1").unwrap();
    let story_title = document.select(&story_title_selector)
        .next()
        .map(|element| element.inner_html().trim().to_string());
    
    if let Some(title) = &story_title {
        println!("Story Title: {}", title);
    } else {
        println!("No story title found");
    }
    
    // Extract date
    let date_selector = Selector::parse(".dateblock time").unwrap();
    let date = document.select(&date_selector)
        .next()
        .and_then(|element| element.value().attr("datetime"))
        .map(|date_str| date_str.to_string());
    
    if let Some(date_str) = &date {
        println!("Date: {}", date_str);
    } else {
        println!("No date found");
    }
    
    // Extract description from paragraphs
    let storytext_selector = Selector::parse("#storytext").unwrap();
    let p_selector = Selector::parse("p").unwrap();
    
    let mut description = None;
    let mut set_list = Vec::new();
    let mut musicians = Vec::new();
    
    if let Some(storytext) = document.select(&storytext_selector).next() {
        let paragraphs: Vec<_> = storytext.select(&p_selector).collect();
        
        // Get description from first paragraphs until SET LIST or MUSICIANS
        let mut desc_text = String::new();
        let mut description_done = false;
        
        for p in &paragraphs {
            let text = p.inner_html();
            
            if text.contains("SET LIST") || text.contains("MUSICIANS") {
                description_done = true;
                continue;
            }
            
            if !description_done {
                if !desc_text.is_empty() {
                    desc_text.push_str("\n\n");
                }
                desc_text.push_str(&text);
            }
        }
        
        if !desc_text.is_empty() {
            description = Some(desc_text);
        }
        
        // Find SET LIST
        let ul_selector = Selector::parse("ul").unwrap();
        let li_selector = Selector::parse("li").unwrap();
        
        for (i, p) in paragraphs.iter().enumerate() {
            let text = p.inner_html();
            
            if text.contains("SET LIST") {
                // Look for the next UL element
                if i + 1 < paragraphs.len() {
                    // Find the next sibling that is a UL element
                    let mut next_element = p.next_sibling();
                    while let Some(element) = next_element {
                        if let Some(el) = element.value().as_element() {
                            if el.name() == "ul" {
                                let ul = element;
                                for (idx, li) in ul.select(&li_selector).enumerate() {
                                    let song_text = li.inner_html().trim()
                                        .trim_start_matches(|c| c == '"' || c == '\'')
                                        .trim_end_matches(|c| c == '"' || c == '\'')
                                        .to_string();
                                    
                                    set_list.push(Song {
                                        songNumber: idx + 1,
                                        title: song_text,
                                    });
                                }
                                break;
                            }
                        }
                        next_element = element.next_sibling();
                    }
                    }
                }
            }
            
            if text.contains("MUSICIANS") {
                // Look for the next UL element
                if i + 1 < paragraphs.len() {
                    // Find the next sibling that is a UL element
                    let mut next_element = p.next_sibling();
                    while let Some(element) = next_element {
                        if let Some(el) = element.value().as_element() {
                            if el.name() == "ul" {
                                let ul = element;
                                for li in ul.select(&li_selector) {
                                    let musician_text = li.inner_html().trim()
                                        .trim_start_matches(|c| c == '"' || c == '\'')
                                        .trim_end_matches(|c| c == '"' || c == '\'')
                                        .to_string();
                                    
                                    // Parse musician name and instruments
                                    let parts: Vec<&str> = musician_text.split(':').collect();
                                    if parts.len() == 2 {
                                        let name = parts[0].trim().to_string();
                                        let instruments = parts[1]
                                            .split(',')
                                            .map(|s| s.trim().to_string())
                                            .collect();
                                        
                                        musicians.push(Musician { name, instruments });
                                    } else {
                                        musicians.push(Musician {
                                            name: musician_text,
                                            instruments: Vec::new(),
                                        });
                                    }
                                }
                                break;
                            }
                        }
                        next_element = element.next_sibling();
                    }
                    }
                }
            }
        }
    
    if !set_list.is_empty() {
        println!("\nSet list:");
        for song in &set_list {
            println!("{}. {}", song.songNumber, song.title);
        }
    } else {
        println!("No set list found");
    }
    
    if !musicians.is_empty() {
        println!("\nMusicians:");
        for (idx, musician) in musicians.iter().enumerate() {
            println!("{}. {}", idx + 1, musician.name);
            if !musician.instruments.is_empty() {
                println!("   Instruments: {}", musician.instruments.join(", "));
            }
        }
    } else {
        println!("No musicians list found");
    }
    
    // Create JSON structure
    let concert_info = ConcertInfo {
        artist: artist_name.clone(),
        source: url.to_string(),
        show: "Tiny Desk Concerts".to_string(),
        date,
        album: story_title,
        description,
        setList: set_list,
        musicians,
    };
    
    // Create output filename based on artist name
    let sanitized_artist_name = artist_name
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .replace(" ", "_")
        .to_lowercase();
    
    let output_file_name = format!("{}_info.json", sanitized_artist_name);
    
    // Write to file as JSON
    let json = serde_json::to_string_pretty(&concert_info)
        .context("Failed to serialize concert info")?;
    
    fs::write(&output_file_name, json)
        .context("Failed to write JSON file")?;
    
    println!("\nInformation saved to {}", output_file_name);
    
    Ok(())
}

fn main() -> Result<()> {
    // Get URL from command line arguments
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Please provide a URL as an argument");
        eprintln!("Usage: cargo run --bin scraper <URL>");
        std::process::exit(1);
    }
    
    let url = &args[1];
    scrape_data(url)?;
    
    Ok(())
}
