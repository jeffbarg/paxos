use super::items::Item;

/// This is a slightly modified binary search algorithm which goes through a 
/// slice of Items and returns the index and item with the price _as close to_ but not
/// greater than the input value.
/// 
/// # Examples
///
/// ```
///	let items = vec![
///		Item{ title: "".to_string(), price: 1},
///		Item{ title: "".to_string(), price: 3},
///		Item{ title: "".to_string(), price: 5},
/// ];
///
/// assert_eq!(binary_search_leq(items, 4), Some(&Item{ title: "".to_string(), price: 3));
/// ```
///
pub fn binary_search_leq(values: &[Item], value: u64) -> Option<&Item> {
	match binary_search_leq_helper(values, value, 0) {
		Some(index) => Some(&values[index]),
		None => None,
	}
}

/// This helper function finds the index of the item with the highest price less than or equal to value.
/// 
/// low_index is the index of values[0] from the perspective of original values array.  While values is a slice of the items currently
/// being searched against, low_index is always relative to the original values slice from the first recursive call.
///
fn binary_search_leq_helper(values: &[Item], value: u64, low_index: usize) -> Option<usize> {
	// Check degenerate base case
	if values.len() == 0 {
		return None
	}

	// Get the index of the item in `values` that we want to check
	let split_index = values.len() / 2;
	let target_item = &values[split_index];

	// Get the index of the target item from the perspective of the original `values` slice
	let target_index = low_index + split_index;

	// Base cases
	if target_item.price == value {
		return Some(target_index)
	}
	if target_item.price < value && values.len() == 1 {
		return Some(target_index)	
	}

	// Recurse
	if target_item.price < value {
		// Check if there is an item in the upper split of values that is closer to the desired value.
		// If so, return the index of that higher acceptable item.
		// If not, the current target item has the highest acceptable value, so return the current index.
		match binary_search_leq_helper(&values[split_index+1..], value, target_index+1) {
			Some(higher_index) => Some(higher_index),
			None => Some(target_index),
		}
	} else {
		// If the price of the current target item is too high, recurse on the lower split of values.
		binary_search_leq_helper(&values[..split_index], value, low_index)
	}
}

#[test]
fn test_binary_search_leq_simple() {
	let items = vec![
		Item{ title: "".to_string(), price: 1},
	];

	let item = binary_search_leq(&items, 1).expect("This should be found");
	assert_eq!(item.price, 1);

	let item = binary_search_leq(&items, 0);
	assert_eq!(item, None);

	let item = binary_search_leq(&items, 3).expect("This should be found");
	assert_eq!(item.price, 1);
}

#[test]
fn test_binary_search_leq_exact_values() {
	let items = vec![
		Item{ title: "".to_string(), price: 1},
		Item{ title: "".to_string(), price: 2},
		Item{ title: "".to_string(), price: 3},
		Item{ title: "".to_string(), price: 4},
	];

	let item = binary_search_leq(&items, 4).expect("This should be found");
	assert_eq!(item.price, 4);

	let item = binary_search_leq(&items, 3).expect("This should be found");
	assert_eq!(item.price, 3);

	let item = binary_search_leq(&items, 2).expect("This should be found");
	assert_eq!(item.price, 2);

	let item = binary_search_leq(&items, 1).expect("This should be found");
	assert_eq!(item.price, 1);
}

#[test]
fn test_binary_search_leq2_offset_values() {
	let items = vec![
		Item{ title: "".to_string(), price: 1},
		Item{ title: "".to_string(), price: 3},
		Item{ title: "".to_string(), price: 5},
		Item{ title: "".to_string(), price: 7},
	];

	let item = binary_search_leq(&items, 2).expect("This should be found");
	assert_eq!(item.price, 1);

	let item = binary_search_leq(&items, 9).expect("This should be found");
	assert_eq!(item.price, 7);
}

#[test]
fn test_binary_search_leq_odd_values() {
	let items = vec![
		Item{ title: "".to_string(), price: 1},
		Item{ title: "".to_string(), price: 3},
		Item{ title: "".to_string(), price: 5},
		Item{ title: "".to_string(), price: 7},
		Item{ title: "".to_string(), price: 10},
	];

	let item = binary_search_leq(&items, 2).expect("This should be found");
	assert_eq!(item.price, 1);

	let item = binary_search_leq(&items, 9).expect("This should be found");
	assert_eq!(item.price, 7);

	let item = binary_search_leq(&items, 0);
	assert_eq!(item, None);
}

#[test]
fn test_binary_search_leq_extensive() {
	let items = vec![
		Item{ title: "".to_string(), price: 1},
		Item{ title: "".to_string(), price: 3},
		Item{ title: "".to_string(), price: 5},
		Item{ title: "".to_string(), price: 7},
		Item{ title: "".to_string(), price: 9},
		Item{ title: "".to_string(), price: 11},
	];

	let item = binary_search_leq(&items, 6).expect("This should be found");
	assert_eq!(item.price, 5);

	let item = binary_search_leq(&items, 5).expect("This should be found");
	assert_eq!(item.price, 5);

	let item = binary_search_leq(&items, 8).expect("This should be found");
	assert_eq!(item.price, 7);

	let item = binary_search_leq(&items, 7).expect("This should be found");
	assert_eq!(item.price, 7);

	let item = binary_search_leq(&items, 10).expect("This should be found");
	assert_eq!(item.price, 9);

	let item = binary_search_leq(&items, 12).expect("This should be found");
	assert_eq!(item.price, 11);

	let item = binary_search_leq(&items, 11).expect("This should be found");
	assert_eq!(item.price, 11);

	let item = binary_search_leq(&items, 0);
	assert_eq!(item, None);
}
