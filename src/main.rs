use std::io;
use std::io::Write;
use std::num::ParseIntError;

use chrono::{NaiveDate, ParseResult};
use rand::prelude::IteratorRandom;

use crate::contributor::Contributor;
use crate::roadmap::RoadmapItem;

mod contributor;
mod roadmap;

static COMPLEXITY_FIRST_THRESHOLD: f64 = 0.3;
static COMPLEXITY_SECOND_THRESHOLD: f64 = 0.6;

fn main() {
    let mut contributors = create_contributors();
    let mut roadmap_items = create_roadmap_items();
    roadmap_items.sort();
    contributors.sort();

    let assigned_items = assign_contributors(roadmap_items, &mut contributors);

    println!(
        "{0},{1},{2},{3},{4}",
        "name", "start date", "target date", "urgency (0-1)", "contributors"
    );
    assigned_items
        .iter()
        .for_each(|roadmap_item| println!("{roadmap_item}"));
}

fn create_contributors() -> Vec<Contributor> {
    println!("Let's start by adding all contributors!");
    let mut contributors: Vec<Contributor> = Vec::new();
    contributors.push(create_contributor());
    loop {
        print!("Add another contributor? (y/n): ");
        let mut input = String::new();
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading from STDIN");

        match input.trim_end().to_lowercase().as_str() {
            "y" => {
                contributors.push(create_contributor());
            }
            "n" => {
                println!("All contributors added!");
                return contributors;
            }
            _ => {
                continue;
            }
        }
    }
}

fn create_contributor() -> Contributor {
    let name = parse_string("Contributor name");
    let seniority = parse_number("Contributor seniority (1-5)", 1, 5);

    return Contributor::new(name, seniority);
}

fn create_roadmap_items() -> Vec<RoadmapItem> {
    println!("Let's add all roadmap items!");
    let mut roadmap_items: Vec<RoadmapItem> = Vec::new();
    roadmap_items.push(create_roadmap_item());
    loop {
        print!("Add another roadmap item? (y/n): ");
        let mut input = String::new();
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading from STDIN");

        match input.trim_end().to_lowercase().as_str() {
            "y" => {
                roadmap_items.push(create_roadmap_item());
            }
            "n" => {
                println!("All roadmap items added!");
                return roadmap_items;
            }
            _ => {
                continue;
            }
        }
    }
}

fn create_roadmap_item() -> RoadmapItem {
    let name = parse_string("Roadmap item name");
    let estimated_complexity = parse_number("Estimated complexity (1-5)", 1, 5);
    let estimated_value = parse_number("Estimated value (1-5)", 1, 5);
    loop {
        let start_date = parse_date("Start date");
        let target_date = parse_date("Target date");
        let date_diff = target_date.signed_duration_since(start_date).num_days();
        if date_diff.is_negative() {
            println!("The target date cannot be before the start date.");
            continue;
        }

        let today = chrono::offset::Local::today().naive_utc();
        if target_date
            .signed_duration_since(today)
            .num_days()
            .is_negative()
        {
            println!("The target date cannot be before today.");
            continue;
        }

        return RoadmapItem::new(
            name,
            estimated_complexity,
            estimated_value,
            start_date,
            target_date,
            Vec::new(),
        );
    }
}

fn parse_string(text: &'static str) -> String {
    let mut input = String::new();
    print!("{text}: ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading from STDIN");
    return String::from(input.trim_end());
}

fn parse_number(text: &'static str, min: usize, max: usize) -> usize {
    loop {
        print!("{text}: ");
        match parse_usize() {
            Ok(complexity) => {
                if complexity < min || complexity > max {
                    println!("The value must be between {min} and {max}");
                    continue;
                }
                return complexity;
            }
            Err(err) => {
                println!("Could not parse number: {err}");
                continue;
            }
        };
    }
}

fn parse_date(input_text: &'static str) -> NaiveDate {
    loop {
        print!("{input_text} (YYYY-mm-dd): ");
        match read_date() {
            Ok(date) => {
                return date;
            }
            Err(err) => {
                println!("Could not parse date: {err}");
                continue;
            }
        };
    }
}

fn parse_usize() -> Result<usize, ParseIntError> {
    let mut input = String::new();
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading from STDIN");
    return input.trim_end().parse::<usize>();
}

fn read_date() -> ParseResult<NaiveDate> {
    let mut input = String::new();
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading from STDIN");
    NaiveDate::parse_from_str(input.trim_end(), "%Y-%m-%d")
}

fn assign_contributors(
    roadmap_items: Vec<RoadmapItem>,
    contributors: &mut Vec<Contributor>,
) -> Vec<RoadmapItem> {
    let mut new_items: Vec<RoadmapItem> = Vec::new();
    roadmap_items.iter().for_each(|item| {
        let mut item_contributors: Vec<Contributor> = Vec::new();
        if item.urgency >= COMPLEXITY_SECOND_THRESHOLD {
            match contributors.first() {
                None => {}
                Some(contributor) => {
                    item_contributors.push(Contributor::new(
                        String::from(contributor.name.as_str()),
                        contributor.seniority,
                    ));
                    contributors.remove(0);
                }
            }
            match contributors.last() {
                None => {}
                Some(contributor) => {
                    item_contributors.push(Contributor::new(
                        String::from(contributor.name.as_str()),
                        contributor.seniority,
                    ));
                    contributors.remove(contributors.len() - 1);
                }
            }
            match contributors
                .iter()
                .enumerate()
                .choose(&mut rand::thread_rng())
            {
                None => {}
                Some((i, _)) => {
                    let c = &contributors.remove(i);
                    item_contributors
                        .push(Contributor::new(String::from(c.name.as_str()), c.seniority));
                }
            }
        } else if item.urgency >= COMPLEXITY_FIRST_THRESHOLD {
            match contributors.first() {
                None => {}
                Some(contributor) => {
                    item_contributors.push(Contributor::new(
                        String::from(contributor.name.as_str()),
                        contributor.seniority,
                    ));
                    contributors.remove(0);
                }
            }
            match contributors.last() {
                None => {}
                Some(contributor) => {
                    item_contributors.push(Contributor::new(
                        String::from(contributor.name.as_str()),
                        contributor.seniority,
                    ));
                    contributors.remove(contributors.len() - 1);
                }
            }
        } else {
            match contributors
                .iter()
                .enumerate()
                .choose(&mut rand::thread_rng())
            {
                None => {}
                Some((i, _)) => {
                    let c = &contributors.remove(i);
                    item_contributors
                        .push(Contributor::new(String::from(c.name.as_str()), c.seniority));
                }
            }
        }
        let new_item = RoadmapItem::new(
            String::from(item.name.as_str()),
            item.estimated_complexity,
            item.estimated_value,
            item.start_date,
            item.target_date,
            item_contributors.clone(),
        );
        new_items.push(new_item);
    });
    return new_items;
}
