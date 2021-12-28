#![allow(unused_variables)]// you'll learn what these are later
#![allow(dead_code)] // you'll learn what these are later

use std::collections::HashMap; // useful later, scroll down

fn main() {
	// Chaining Methods
	/*
		put many methods together in a single statement, fxnal programming
	*/
	let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let _new_vec = my_vec
        .into_iter() // "iterate" over the items (iterate = work with each item inside it). into_iter() gives us owned values, not references
        .skip(3) // skip over three items: 0, 1, and 2
        .take(4) // take the next four: 3, 4, 5, and 6
        .collect::<Vec<i32>>(); // put them in a new Vec<i32>

    // Iterators
    /*
		Iterators are a construct that allows you to cycle thru a collection of elements

		- for keyword gives us an iterator that owns its values
			~ hence while its mutable and you can change the values when you use it
		- .iter() makes an iterator over references
			~ the v ref is still alive use to make v1 because we only used references, or borowed values
		- .iter_mut() makes iterator of mutable references
			~ you only need .collect::... when passed-in values are used
    		~ with keyword mut and iter_mut(), you're changing the values passed into it
		- .into_iter() makes an iterator of values
			~ this detroys v unlike .iter()
		
		- .map() lets you do things to every item then pass it on
		- .for_each() lets you manipulate each item, but keeps original reference
			~logically equivalent to a for loop

		Iterators work by using a method called .next(), returning Option<Some,None>
		Rust calls .next() repeatedly until it gets None (copied and pasted example)

		To implement the Iterator trait, you need to fles out next() function

		impl Iterator for NameOfStruct {
			fn next(&mut self) -> Option<ReturnedType> {
				match self.things.pop() {
					Some(thing) => Some(thing),
					None => None,
				}
			}
		}

		*/
    let v = vec![1,2,3];
    let v1 = v.iter().map(|x| x + 1).collect::<Vec<i32>>(); // [2,3,4]
    let v2 = v.into_iter().map(|x| x*10).collect::<Vec<i32>>(); // [10,20,30]
    let mut v = vec![1,2,3];
    v.iter_mut().for_each(|x| *x += 100); // [110,120,130]

    let my_vec = vec!["This", "That", "Also", "Finally"]; // Just a regular Vec

    let mut my_vec_iter = my_vec.iter(); // This is an Iterator type now, but we haven't called it yet

    assert_eq!(my_vec_iter.next(), Some(&"This"));  // Call the first item with .next()
    assert_eq!(my_vec_iter.next(), Some(&"That"));  // Call the next
    assert_eq!(my_vec_iter.next(), Some(&"Also")); // Again
    assert_eq!(my_vec_iter.next(), Some(&"Finally")); // Again
    assert_eq!(my_vec_iter.next(), None);        // Nothing is left: just None
    assert_eq!(my_vec_iter.next(), None);        // You can keep calling .next() but it will always be None

    // Closures
    /*
		Also called lambdas ( as in lambda calculus )
		anonymous fxns are a subset of closures

		Closures got it's name because they can take variables and enclose them inside
		- a || doesn't enclose a variable from outside is an anonymous fxn
		- a || that does enclose a variable from outside is a closure. 
			It encloses the variables around it to use them

		.map(|x| ... ) // take each element and do something with it 
			~ map is for doming something to each item and passing it on
		.enumerate() gives an iterator with an index paired with it (0,8),(1,9),(2,10)
		.for_each(|(idx,el)| ... )
			~ do something when you see each item

		iterators are lazy and do nothing unless/until consumed
		without using collect::... after map, it's just a Map object waiting to be collected
		use .zip() combines two iterators together ('zipping' them together)

		.filter(|x| condition involving x )
		.filter_map(), does filter() and map() together, it always needs a option, idrgi
		.ok() converts Result to Option, converts Err to None
		.ok_or() - gives an Ok or an Err
		.ok_or_else()
		.and_then() takes an Option, does something to its value and pass it on, it's output is an Option
		.and() is like a bool for Option, matches Option to each other, if all Some, it will give the last one
		.any(|x| condition) and .all(|x| condition) return bools instead of Option

		honestly it's long and covers a lot of minutiae, I think I got the gist of it

    */
    println!("-----");
    let basic_closure = ||  println!("This is from a closure!");
    basic_closure();

    let fancier_closure = |x: i32| { println!("Fancier closure #{}", x); };
    fancier_closure(8);
    fancier_closure(34+35);

    let variable_declared_outside_of_closure = "Yes!";

    let use_outside_var = || { println!("Can we use vars decl'd outside of lambdas, in lambdas? {}", variable_declared_outside_of_closure);};
    use_outside_var();

    let v = vec![8,9,10];

    let fourth_element = v.get(3).unwrap_or_else( || {		// if v does not have a 
    	if v.get(0).is_some() {
    		println!("Tried getting 4th element in {}-size vector: {:?}",v.len(), v);
    		&v[0]
    	} else { &0 }
    });

    let nums = vec![1,2,3];
    let words = vec!["val1","val2","val3"];
    let nums_to_words_hashmap: HashMap<_,_> = nums.into_iter().zip(words.into_iter()).collect();

    println!("key {} - value {}",2, nums_to_words_hashmap.get(&2).unwrap());

    // The dbg! macro and .inspect()
    /*
		dbg!(variable);

		- faster than print! and println!
		- you can wrap it around expressions

		.inspect() is similar to the dbg! macro but you use it like an iterator
    */
    let some_vec = dbg!(vec![8,9,10]);

    let double_vec = dbg!(some_vec.iter().map(|x| x*2).collect::<Vec<i32>>());

    let triple_vec = some_vec.iter().inspect(|x| print!("pre triple - {}|",x)).map(|x| x*3).inspect(|x| println!("post triple - {}", x)).collect::<Vec<i32>>();

    // Types of &str
    /*
    	String literals 
    	- made when doing let my_str = "This is a string literal"
    	- lifetime = the whole program, of type &'static str
    	- ' means its lifetime, and string literals have static lifetimes

    	Borrowed str
    	- regulate &strs without static lifetime
    	- if you create a String and get a reference to it, Rust will convert it to a &str
    */
}