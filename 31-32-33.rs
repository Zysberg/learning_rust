#![allow(unused_variables)]// you'll learn what these are later
#![allow(dead_code)] // you'll learn what these are later

fn main () {
	// Generics
	/*
		Allow for generalization for different data types for processing 
		different kinds of inputs.

		Generics denoted with angle brackets with the type inside (<T>).
		Rust notation denotes generics with a capital letter.
	*/

	println!("Test generic_response() on a String");
	print!("{}",generic_response(String::from("~|~|~|~")));
	println!("\nTest generic_response() on a i128, probably");
	print!("{}\n",generic_response(8_000_000_000_000i128)); 
	// 8 quadrillion and ~|~|~|~ print fine because we know what types the are at compile time
	// both String and i128 implement the print trait, see fxn definition for more info


	#[derive(Debug)]
	struct DebuggableStruct (u8, String);

	println!("Trying Debuggable Generics");
	another_gen_resp(DebuggableStruct(9,String::from("hi :)")));
	println!("------------");

	// Option and Result
	/*
		Option and Result build on Generics.
		In a way, they deal with handling generalizing inputs and outputs
		
		Option<defined_type>
			- use when a value that might or might not exist
			- when a value exists, it is Some(value) amd when it doesn't it's just None
			- defined as enum Option<T> {Some(T), None}
			- to get the value within the Some object, do SomeObj.unwrap()

		Result<some_type,another_type>
			- enum Result<T,E> { Ok(T), Err(E) }
			- same thing as Option but with a bit more detail
			- Option basically means "there's at least something"
				Result will provide more info it's something bad or good
			- Every function that returns Result<T,E>, must return Err(E) or Ok(T)
			- some_result.is_ok() true or false if some_result is Ok(T) or Err(E)

		if let ("do something if condition is true")
		if let Some(number) = my_vec.get(idx) { ... }
		It uses declaration, '=' not equals '=='
		while let ... same idea

		while let Some(information) = city.pop() {
		    // This means: keep going until you can't pop anymore
		    // When the vector reaches 0 items, it will return None
		    // and it will stop.
		    if let Ok(number) = information.parse::<i32>() {
		        // Try to parse the variable we called information
		        // This returns a result. If it's Ok(number), it will print it
		        println!("The number is: {}", number);
		    }  // We don't write anything here because we do nothing if we get an error. Throw them all away
		}
	*/
	println!("Testing get_fifth_element([88,5,22,3,62]): {}", get_fifth_element(vec![88,5,22,3,62]).unwrap());
	// If the 62 was not in the vector, then the thread would panic, basically a runtime error
	// A more runtime-safe approach would be something like this
	let none_returned = get_fifth_element(vec![88,5,22,3]);
	match none_returned {
		Some(number) => println!("The fifth element of vector is {}", number),
		None => println!("vec![88,5,22,3] (aka none_returned) too small to print fifth element"),
	}
	println!("none_returned is None. therefore none_returned.is_none() returns: {}", none_returned.is_none());
	println!("Therefore none_returned.is_some() returns: {}", none_returned.is_some());
	
	println!("-----------");
	// Other Collections
	/*
		Many types of collections:
		- Sequences: Vec, VecDeque, LinkedList
		- Maps: HashMap, BTreeMap
		- Sets: HashSet, BTreeSet
		- Misc: BinaryHeap

		https://doc.rust-lang.org/beta/std/collections/
		lists out when to use which data structure

		HashMap & BTreeMap
			- HashMap: 
				~ Collection of keys and values, the key is
					the used to look up the value that matches key
				~ .get(key) returns Option -> Some(value), None
					+ safer than hash_map[key] // will crash if doesn't exist
				~ let mut some_value = hash_map.entry(key).or_insert(val)
					+ if key exists in hash map, otherwise add key-val pair
					+ can now directly manipulate value within the hashmap
			- BTreeMap
				~ a sortable HashMap
		HashSet & BTreeSet
			- HashSet:
				~ A HashMap with only keys, each element is unique
			- BTreeSet
				~ Sorted HashSet
		BinaryHeap
			- Larger the element, smaller it's index
			- good for managing collection of things to do
		VecDeque
			- Similar to Vec except you can pop items off front and back
				~ my_vec.remove(0) is slower than my_vdQ.pop_front();
	*/	
	use std::collections::HashMap;
	use std::collections::BinaryHeap;

	// let mut key_chain: HashMap<&str,u32> = HashMap::new();
	let mut key_chain = HashMap::new();
	key_chain.insert("front door",0xbadbeef);
	key_chain.insert("garage",0x0ddba11);
	key_chain.insert("money safe",0x515ca11);

	if key_chain.contains_key("front door") {
		println!("Front Door Password: {:#x}", key_chain.get("front door").unwrap());
	}
	println!("-");
	let mut to_do_list = BinaryHeap::new();
	to_do_list.push((100, "Finish homework"));
	to_do_list.push((75, "Learn Rust"));
	to_do_list.push((5, "Annoy friends"));
	to_do_list.push((50, "Quit job"));
	to_do_list.push((0, "Fight God or become Him"));
	to_do_list.push((-1, "Bring back the McRib"));
	to_do_list.push((-2, "Die peacefully"));
	println!("Sample to do list:");
	while let Some(thing_to_do) = to_do_list.pop() {
		println!("Do this: {}", thing_to_do.1);
	}
}


//------Additional Info on Generics-------------
fn generic_response<T>(thing: T) -> T {
	print!("\tYou gave this thing, sending it back... - ");
	// print!("{}, T"); // <- This will give compiler error
	/* rust doesn't know if type ikmplements print trait at runtime, 
	   not every thing we pass here might implement the print or debug traits */
	thing
}

use std::fmt::Debug;
// This allows any type as long as it has the debug trait to print
fn another_gen_resp<Anything: Debug>(thing: Anything) -> Anything {
	print!("\tA debuggable thing! - {:?}\n", thing);
	thing
}

use std::cmp::PartialOrd;
fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
    println!("{}! Is {} greater than {}? {}", statement, num_1, num_2, num_1 > num_2);
}

use std::fmt::Display;
fn easier_to_read_comparator<T,U>(statement:T, thing_a: U, thing_b: U)
where
	T: Display,
	U: Display + PartialOrd,
{
	println!("{}! Is {} greater than {}? {}", statement, thing_a, thing_b, thing_a > thing_b);
}
//------------End Generics-----------------------
//-------Addtional info on Option & Result ------

fn get_fifth_element(value: Vec<i32>) -> Option<i32> {
	if value.len() < 5 { None }
	else { Some(value[4]) }
}

fn detailed_get_fifth_element(value: Vec<i32>) -> Result<i32,()> {
		if value.len() < 5 { Err(()) }
		else { Ok(value[4]) }
}

fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok(number),
        _ => Err("Sorry, the number wasn't five.".to_string()), // This is our error message
    }
}
//--------------End Option & Result--------------
