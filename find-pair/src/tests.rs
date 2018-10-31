extern crate rand;
use rand::{Rng, SeedableRng, StdRng};

use super::*;
use super::test::Bencher;

const DATASET_SIZE: usize = 1000;
const GAP_SIZE: usize = 100;

// Tests
#[test]
fn test_closest_items_examples() {
	let items = get_items_from_file("data/prices.txt");
    
    let closest_items = get_closest_items_parallel(&items, 2500);
    assert_eq!(
    	closest_items,
    	Some((&Item {price: 2000, title: "Earmuffs".to_string()},
    		  &Item {price: 500, title: "Candy Bar".to_string()}
    	))
    );

    let closest_items = get_closest_items_parallel(&items, 2300);
    assert_eq!(
    	closest_items,
    	Some((&Item {price: 1400, title: "Headphones".to_string()},
    		  &Item {price: 700, title: "Paperback Book".to_string()}
    	))
    );

    let closest_items = get_closest_items_parallel(&items, 10000);
    assert_eq!(
    	closest_items,
    	Some((&Item {price: 6000, title: "Bluetooth Stereo".to_string()},
    		  &Item {price: 2000, title: "Earmuffs".to_string()}
    	))
    );

    let closest_items = get_closest_items_parallel(&items, 1100);
    assert_eq!(
    	closest_items,
    	None
    );
}

// Benchmarks
fn generate_test_dataset(size: usize) -> Vec<Item> {
	let mut rng: StdRng = SeedableRng::from_seed([0; 32]);

	let mut items = Vec::new();
	let mut current_price = 0;

	for index in 0..size {
		items.push(Item {
			price: current_price,
			title: format!("item {}", index)
		});

		current_price += rng.gen_range(1, GAP_SIZE) as u64;
	}

	items
}

#[bench]
fn bench_search_parallel(b: &mut Bencher) {
	let test_set = generate_test_dataset(DATASET_SIZE);
	let mut rng: StdRng = SeedableRng::from_seed([0; 32]);

    b.iter(|| {
	        let searched_price = rng.gen_range(0, GAP_SIZE * DATASET_SIZE) as u64;
	        let _ = get_closest_items_parallel(&test_set, searched_price);
        }
    );
}