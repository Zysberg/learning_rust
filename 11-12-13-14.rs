#[allow(unused_variables,dead_code)] // you'll learn what these are later

fn main () {

// Display and Debug Printing
/*
	Some variables can't be printed within {} in the println! macro
	(macros explained later), therefore you need to use the debug print

	To print the largest and smallest possible numbers for each type print...
	std::nM::MIN , std::nM :MAX, for n = {i,u} M = {2^n: 2 < n < 8} 
*/
	let will_not_print_with_curlies_alone = ();
	println!("Try removing the ':?' - {:?}", will_not_print_with_curlies_alone);
	let some_str = "blahblahblah";
	println!("What does :#? do? - {:#?}", some_str);
	print!("This line is equivalent to println!(...)\n"); // note the '\n'


// Mutability
/*
	Variables cannot be change in value once defined

	If you'd like to make a variable manipulatable, use the 'mut' keyword
		- this will change the value, but you cannot change the type of the variable

	Shadowing
		- using 'let' to declare a new variable with the same name as other variables.
		- useful when you need to change a variable a lot, doing constant let statements
			that shadow bears the same name is faster than making the var mutable
		- It happens where you want to quickly take variable, do something to it, 
			and do something else again. 
		- You usually use it for quick variables that you don't care too much about.
		- Space complex but time simple
*/
	let mut a_number = 9;
	print!("Before add : {} |", a_number);
	a_number += 1;
	println!(" After: {}", a_number);

	let something = 99;
	println!("something: {}", something);

	let something = 12.3; 
	// this something is stored at a different place in memory
	println!("something (same scope): {}", something);
	// this print statement will reference the latest definition 'something'

	{
		let something = "woah";
		println!("something (in block): {}", something);
	}

	println!("something (after block): {}",something);

// Stack v. Heap places in memory
/*
	stack
		- faster than heap
		- stores variables
		- cannot use all the time because:
			- Rust needs to know size of variables at compile time (i32's u8's etc.)
			- some types you do not know at compile time
	
	heap 
		- slower, but can use as much space as you need for non-standard types
		- literally allocated chunks of memory

	pointers (in rust, they are called references).
		- reference points to the memory address of another value
		- reference means you borrow the value, but you don't own it
			(ie, ToC doesn't own the info, the chapters of the book own the info)
*/
	let some_variable = 69; // some_variable = 69;
	let ref_to_var = &some_variable; // ref_to_var = 69;
	// ref_to_var is just looking at some_variable's data but doesn't own it.
	// It's like opening a book and reading a page rather than ripping than
	// 		page out for yourself to read

	let mut mutable_var = 69;
	let mut ref_to_mutable_var = &mut mutable_var;
	*ref_to_mutable_var += 1;
	println!("69 + 1 = {}", mutable_var);
	// //* indicates de-referencing the variable. 
	// //*ref_to_var is the same as some_variable

// More about Printing

/*

	\n - newline
	\t - tab
	\\ - '\'
	r#"....."# - print everything within the quotes as-is, \\ will print as \\
	b"..." print the individual bytes of a &str or char (works for ASCII)
	"{:x}" input value will print as hexidecimal bytes, swap for o(ctal) b(inary)
	"\u{hexVals}" prints unicode chars
fat

*/

	let r#let = 6; // The variable's name is let
    let mut r#mut = 10; // This variable's name is mut

    let my_number = r#return(); // This is stupid, please don't do this
    println!("{}", my_number);

    // println!("This is {1} {2}, son of {0} {2}.", father_name, son_name, family_name);
    let letter = "a";
    println!("{:ㅎ^11}", letter);
    // padding characters < pad those  11 chars to the left,^ middle, > right

}

fn r#return() -> u8 {
    println!("Here is your number.");
    8
}