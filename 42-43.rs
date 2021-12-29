#![allow(unused_variables)]// you'll learn what these are later
#![allow(dead_code)] // you'll learn what these are later

fn main () {
	// Cow
	/*
		cow means 'clone on write'
		returns a &str if you don't need a String, but a String if needed
		(same for arrays v. Vecs, etc.)
	*/
	//Type aliases
	/*
		give a new name to another type. shorten something that's long and often typed.
	*/
	println!("test returns on ['a','b','c','d','e','f','g']: {:?}", returns(&vec!['a','b','c','d','e','f','g']) );

	//todo! macro
	/*
		sort of like pass in python but not really
	*/
	println!("---");
	//Rc
	/*
		Rc means reference counter
		- allows for more than one owner
		- writes down who has ownership and how many
		- when the Rc goes to 0, the variable goes stale

		Use .clone() on an Rc-encapsulated varible to copy the pointer

		either Rc::clone(&item) or if item is already an Rc, then item.clone()

		can check the number of owners with Rc::strong_count(&item)

		What if two Rc's point to each other? It can't die.
		Strong v. Weak Pointers

		Weak
			- Rc will count the references, but if it only has weak references it can die
			- Rc::downgrade(&item) to make weak references
			- Rc::weak_count(&item)
	*/
	// Multiple Threads
	/*
		
	*/


}

//------Type aliases----
type SkipFourTakeFive<'a> = std::iter::Take<std::iter::Skip<std::slice::Iter<'a, char>>>;

fn returns<'a>(input: &'a Vec<char>) -> SkipFourTakeFive {
    input.iter().skip(4).take(5)
}
//----------------------
//---------todo!--------
fn fill_me_in_later() -> i32 {
	todo!()
}
//----------------------
//---------Rc-----------
use std::rc::Rc;

#[derive(Debug)]
struct BoxOfCandy {
	name: String,
	num_candies: Rc<i32>
}

#[derive(Debug)]
struct CandyShop {
	inventory: Vec<String>,
}

//----------------------