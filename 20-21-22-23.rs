#[allow(unused_variables)] // you'll learn what these are later
#[allow(dead_code)] // you'll learn what these are later

fn main () {
	//Copy Types
	/*

		Some types in rust are very simple, called copy types

		On the stack, compiler knows their size

		typically don't have to worry about ownership for these types

		integers, floats, booleans, char

		Copy types have trait implementations
			- is copied when you send it to a function (Copy)
			- {} implements (Display)
			- {:?} implements (Debug)

		String is not a copy type, but among the list of Traits implemented for it, 
		there is Clone(String::new().clone())

		.clone() is expensive, faster to reference via &
	*/

	let a: i32 = 99;
	some_fxn(a); //since a is a copy type, some_fxn gets a copy of a
	some_fxn(a);

	let some_string = String::from("Sturinguh");
	another_fxn(some_string.clone());
	another_fxn(some_string.clone());

	// for _ in 0..500000000000 {
	// 	another_fxn(some_string.clone());
	// }										// do this speed test on your own
	// for _ in 0..500000000000 {
	// 	another_fxn_2(&some_string);
	// }

	// Variables outside values

	let my_variable;
	{
		let my_variable = { 57 }; // pretend the shadowed recent version of my_varable is needed
	}
	my_variable = 1;
	println!("declaring then defining - {}", my_variable);
	drop(my_variable); // makes variable stale manually

	// Collection Types
	/*
		Arrays - [...]
			- do not change their size
			- all elements must be same type
			- defined as [type;size]

	*/
	let arr1 = ["el","em"]; // type [&str, 2]
	println!("{:?}",arr1);
			//   0  1  2  3  4  5    6  7
	let arr2 = [55,22,63,69,11,9234,44,20];
	println!("idx3 to idx5 inclusive {:?}", &arr2[3..6]);
	println!("Everything before idx4 {:?}", &arr2[..4]);
	println!("Everything after idx3 {:?}", &arr2[4..]);
	// Vectors
	/*
	
		&str is to String as arrays are to vectors
		
		arrays are faster with less fxnality, vectors are slower with more fxnality

		Vec<(i32, i32)> this is a Vec where each item is a tuple: (i32, i32).
		Vec<Vec<String>> a Vec that has Vecs of Strings

		.capacity() helps look at how much space is left before reallocation needs to occur

	*/
	let mut my_vec: Vec<String> = Vec::new(); //type inference cannot be done here :/
	let mut another_vec = vec![8,1,1,99];
	let mut num_vec: Vec<char> = Vec::with_capacity(8);
    num_vec.push('a'); // add one character
    println!("{}", num_vec.capacity()); // prints 8
    num_vec.push('a'); // add one more
    println!("{}", num_vec.capacity()); // prints 8
    num_vec.push('a'); // add one more
    println!("{}", num_vec.capacity()); // prints 8.
    num_vec.push('a'); // add one more
    num_vec.push('a'); // add one more // Now we have 5 elements
    println!("{}", num_vec.capacity()); // Still 8

    // Tuples
    /*

	(), the empty tuple, fn fxn() {} actually short for fn fxn() -> () {}

	Tuples can hold many strings
    */

    let random_tuple = ("man", 1, "am", "t1r3d", true, 7.7, vec![777,77,7]);
    println!( "Inside the tuple is: \n\tFirst item: {:?} \n\tSecond item: {:?},\
    	\n\tThird item: {:?} \n\tFourth item: {:?} \n\tFifth item: {:?} \n\tSixth item: {:?}",
        random_tuple.0, random_tuple.1, random_tuple.2, random_tuple.3, 
        random_tuple.4, random_tuple.5);
    let str_vec = vec!["one","two","three"];
    let (a, b, c) = (str_vec[0], str_vec[1], str_vec[2]); // destructuring
    //to destructure without needing all variables...
    let (_, _, f) = (str_vec[0], str_vec[1], str_vec[2]);
}

fn some_fxn(a: i32) {
	println!("some_fxn - {} ", a);
}

fn another_fxn(a: String) {
	println!("another_fxn - {} ", a);
}

fn another_fxn_2(a: &String) {
	println!("another_fxn - {} ", a);
}