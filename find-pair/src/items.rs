use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Item {
	/// The price of the item
	pub price: u64,
	/// The associated string title of the item
	pub title: String
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.title, self.price)
    }
}

impl Item {
	/// Constructs an Item from an input line
	pub fn from_input_line(line: &str) -> Self {
		let partitions: Vec<&str> = line.split(", ").collect();
		if partitions.len() != 2 {
			panic!("There is an improperly formatted line!");
		}
		let title: String = partitions[0].to_string();
		let price: u64 = partitions[1].parse().expect("Not a positive number!");
		Item {
			price: price,
			title: title
		}
	}
}

#[test]
fn test_item_creation() {
	let item = Item::from_input_line("Crayons, 300");

	assert_eq!(item.title, "Crayons");
	assert_eq!(item.price, 300);
}