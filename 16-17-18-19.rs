#[allow(unused_variables,dead_code)] // you'll learn what these are later

fn main () {
	// Strings
	/*

	Rust has two main types of strings: String and &str
	
	&str
		- simple string, very fast
		- need the & because you need a reference to use a str
		- Because of &, you don't own it
		- str itself dynamically sized type
	
	String 
		- an object, slower, but more associated functions.
		- String is a pointer, and data is on the heap.
		- an owned type

	*/
	let a_string_slice = "ooglyboogly"; // defining &str
	let a_string_object = String::from("ooglyboogly"); // defining Strings
	let str_slice_to_string_object = "some string".to_string(); // &str -> String
	let format_string = format!("This is {} fun!", a_string_slice); // defining a String

	// cannot do this because compiler doesn't know what to infer
	//	let myString = "asdfasdfasf".into()
	// can do this:
	let my_string: String = "asdfasdfasdf".into();


	// const and static
	/*
		const and static are two other ways to declare vars

		Rust compiler cannot infer on vars declared with these words

		No shadowing allowed

		const - is for values that don't change; the name is replaced 
		with the value when its used
			- sort of like a C macro
			- usually uses this

		static - similar to const, but has a fixed memory location and can act as a global var
		across all files of a compiled project

		must be in all caps
	*/
	static SEASONS: [&str; 4] = ["Winter", "Spring", "Summer", "Fall"];

	// More on References
	let country = String::from("Austria");
	let ref_one = &country;
	let ref_two = &country; // both ref_one and ref_two are references to a String, of type &String

	let country_name = return_str();

	println!("Reference Demo: {}, {}, {}", country, ref_two, ref_one);

	// Mutable References
	/*

		If you want to use a reference to change data, you can use a mutable reference.
		'&mut' instead of just '&'

		Rule 1: If you have only immutable references, you can have as many as needed
		Rule 2: If you have a mutable reference, you can only have one
					- Also can't have immutable ref and mutable ref together
					- let mut a = 9; let compiler_err = &a // compiler error

	*/
	let mut my_number = 9;
	let num_ref =  &mut my_number; // num_ref is a mutable reference to an i32;
	*num_ref -= 3;
	println!("9 - 3 = {}", num_ref);	// value is borrowed to be used, checks out
	println!("9 - 3 = {}", num_ref);	// value is borrowed to be used, checks out
	println!("9 - 3 = {}", my_number);  // my_number is borrowed and then dies, 
										// therefore num_ref becomes stale / 'dangling pointer'
										// rust then considers num_ref dead too
	// println!("9 - 3 = {}", num_ref); // uncommenting this line will compiler error


	let country = String::from("Austria");
	let ctry_ref = &country;
	let country = 69;
	println!("Shadowing - county(older) {} | country(recent) {}", ctry_ref, country);

	//Giving References to Functions
	/*
		 Explanations of lines 124 - 126
		 - We create a String called qwery, qwery is the owner.
		 - give qwery to some_fxn, some_fxn now owns qwery.
		 - some_fxn doesnt return anything.
		 - when some_fxn is finished, qwery is destroyed/freed/gone stale/destroyed/dead.
		 - we try to give qwery to the fxn again, but qwery is stale/is 
		 		kind of a dangling pointer at this point.
		 - compiler error.

		Explanations of lines 128 - 137
		We can make the fxn get the string value back, but it's not technically the same.
		 - Create a String in which qpwo is the owner.
		 - Transfer ownership of said string to another_fxn.
		 - That String gets destroyed but another_fxn returns the value that was passed into it.
		 - this_thing now owns this returned value of another_fxn
		 - this_thing gets destroyed because of the printn statement

		Explanations of lines 133 - 138
		 - String created, owned by hua
		 - reference to string created
		 - reference is passed to yet_another_fxn (yet_another_fxn has borrowed the String)
		 - because original string wasn't destroyed, ref still points to fresh value
		 - value is still fresh so you can print that too

		Explanations of lines 140 - 147
		- create a String, ghg owns it
		- create a reference to said String
		- pass reference to modify_string, modify_string is borrowing the String
		- modify_string does something to the original string
		- pretty printing
		- make ghg stale

		Explanation of lines 154 - 157
		- same as lines 128-137 except it returns a modifed version of a new string
			but it contains the string passed as a parameter

	*/
	let qwery = String::from("the value");
	some_fxn(qwery);
	// some_fxn(qwery) // compiler error here if uncommented

	let qpwo = String::from("another value");
	let this_thing = another_fxn(qpwo);
	println!("this_thing - {}", this_thing);
	// println!("uncomment me I dare you - {}", qpwo);
	
	let hua = String::from("yet another value");
	let ref_hua = &hua;
	yet_another_fxn(ref_hua);
	println!("I'm a ref to hua{}", ref_hua); // as long as what's being ref'd isn't destroyed
											// you're good
	println!("print hua {}", hua); // hua has not been destroyed yet, still valid

	let mut ghg = String::from("this string");
	let ghg_ptr = &mut ghg;
	// println!("try uncommenting me too, {}", ghg);
	println!("before modify_String: {}", ghg_ptr);
	modify_string(ghg_ptr);
	println!("after modify_String: {}", ghg_ptr);
	println!("ghg before making it stale, {}", ghg);
	some_fxn(ghg);
	// println!("you already know what'll happen {}", ghg);

	let last_example = "one more string".to_string();
	println!("before another_modify_string_fxn, {}", last_example);
	let copied_and_modded = another_modify_string_fxn(last_example);
	println!("after another_modify_string_fxn, {}", copied_and_modded);
	
	
}

fn return_str() -> String {
	let ctry = String::from("some place");
	let _ctry_ref = &ctry; // for unused vars, put an underscore before the name 
						   // to supress warnings
	ctry
}

fn some_fxn (a: String) { // this fxn owns a
	println!("some_fxn called {}", a);
}

fn another_fxn(a: String) -> String { // this fxn owns a 
	a
}

fn yet_another_fxn(a: &String) { // this fxn borrows a
	println!("yet_another_fxn called - {}", a);
}

fn modify_string(a: &mut String) { // this fxn borrows a
	a.push_str(" has been modified");
}

fn another_modify_string_fxn(mut a: String) -> String { // this fxn owns a
	a.push_str(" has been modified too!");
	a
}