#![feature(test)]

extern crate test;
extern crate rayon;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

use rayon::prelude::*;

#[cfg(test)]
mod tests;

mod items;
mod bin_search_utils;

use self::items::Item;
use self::bin_search_utils::binary_search_leq;

fn main() {
    // Note: could use a library here (such as Clap) to make argument parsing cleaner
	let args: Vec<String> = env::args().collect();
	if args.len() != 3 {
		panic!("Improper command line arguments.  Example: \"find-pair data/prices.txt 2500\"");
	}

    // Parse arguments
	let filename = &args[1];
	let desired_price: u64 = args[2].parse().expect("The second argument must be a positive integer");

    // Execute algorithm
    let items = get_items_from_file(filename);
    let closest_items = get_closest_items_parallel(&items, desired_price);

    // Print results
    match closest_items {
    	Some((item1, item2)) => println!("{}, {}", item1, item2),
    	None => println!("Not possible"),
    }
}

/// This function returns the items retrieved from a specific file.
fn get_items_from_file(filename: &str) -> Vec<Item> {
	let file = File::open(filename).expect("file not found");
	let reader = BufReader::new(&file);

	// Read in each Item, one per line
	reader.lines().map(|line| {
        let line = line.unwrap();

        // Skip empty lines
        if line.is_empty() {
            return None
        }

        Some(Item::from_input_line(&line))
    })
    .filter(Option::is_some)
    .map(Option::unwrap)
    .collect()
}

/// This function returns the closest pair of items that have combined price less than or equal to `desired_price`.
/// By using Rayon's multithreading, we can achieve a ~4x speedup for large datasets compared to the same algorithm 
/// in single-thread mode.
fn get_closest_items_parallel<'a>(items: &'a Vec<Item>, desired_price: u64) -> Option<(&'a Item, &'a Item)> {
    items.par_iter()
    	 .enumerate()
    	 .filter(|(_index, item)| item.price < desired_price)
    	 .map(|(index, possible_item1)| {	
	    	match binary_search_leq(&items[..index], desired_price - possible_item1.price) {
	    		Some(possible_item2) => Some((possible_item1, possible_item2)),
	    		None => None,
	    	}
    	 })
    	 .reduce(|| None, |x, y| {
    	 	// Get the pair with the maximum score (guarenteed by the map phase to be less than desired_price)
    	 	match (x, y) {
    	 		(None, None) => None,
    	 		(x, None) => x,
    	 		(None, y) => y,
    	 		(Some((i1, i2)), Some((i3, i4))) => {
    	 			let price1 = i1.price + i2.price;
    	 			let price2 = i3.price + i4.price;

    	 			if price1 > price2 { Some((i1, i2)) } else { Some((i3, i4)) }
    	 		}
    	 	}
    	 })
}