#[allow(unused_variables,dead_code)] // you'll learn what these are later

fn main () {

// Basic compilation
/*
	make sure the file has a main function

	to compile and run your code, do the following steps.
	$ rustc name_of_file.rs
	$ ./name_of_file

	for every edit you make, you must save and recompile to see the
	results in action
*/



// Types & Type Inference
/*

	Signed (i) and Unsigned (u) Integers
	 - i8 i16 i32 i64 i128 isize	[-2^(n-1), 2^(n-1)]
	 - u8 u16 u32 u64 u128 usize	[0, 2^(n)-1]
	 - isize and usize are both 64 bit on 64bit processors
	 - usize is used for indexing in arrays and other list types
	
	characters in rust = char
		- 4 bytes

	Typecasting exists with 'as' keyword

	string literals are called 'string slices'
		- later on, we'll discuss String objects

	.len() gives the size of strings in bites

	you can also add underscores for readability

	Floats
	 - If rust sees a dot after a number, compiler knows it's a float
	 - f32 f64 // rust will infer f64 if type is not inferred

*/
	let some_num: i8 = 100;
	let another_num = 100; // will infer the type on compile time (pretty sure i32)
	let yet_another_number = 100i64;
	let big_readable_number: i64 = 10_000_000_000; 
	// 10 billion, too big for compiler to infer as a i32, 
	// will throw compiler error
	// needs to be declared higher value

	let small_num: u8 = 0x5A; //only u8 can be cast as chars
	println!("hello, character {}", small_num as char); // hello, character Z
	println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes

	let string_slice = "This is a string slice";
    println!("Size of string containing '国': {}", "国".len()); // 3 bytes (unicode)
    println!("The length of stringSlice = {}", string_slice.len());

    let this_float = 5.; // valued at 5 but still a float 
    println!("what does a printed float look like? {}", this_float);
    let another_float = 5.982929292929;
   	println!("what does another float look like? {}", another_float);

   	// rust allows use before declare, because it's compiled, not interpreted
   	println!("function call - {}", fxn_with_params(9,10));
}

// Printing 'hello world!'
/*

	fn - declaring a function
	main - the entry-point to every rust project and file
	() - within these parenthesis, you define parameters used in the fxn
*/

fn some_function() -> i32 {
	69
}
/*
	This function returns a some kind of value of type i32
	 - on a standalone line, you do not need semicolons for return values
	 - return 69; // this is functionally equivalent

*/

fn fxn_with_params(some_number: i32, another_number: i32) -> i32 {
	let oogly_boogly = some_number + another_number;
	oogly_boogly
}
