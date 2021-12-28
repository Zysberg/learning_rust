#![allow(unused_variables)]// you'll learn what these are later
#![allow(dead_code)] // you'll learn what these are later

fn main () {
	// The ? operator
	/*
		After a function returns a result, add a ? at the end of it.
		This will either unwrap if Result is Ok, or pass Err.
		It like does the work of handling results for you. pog.

		Best thinkable use case is file IO
		1. find the file - returns Result
		2a. read the file - return Result
		2b. write to file - returns Result
		the ? can chain these together in one line rather that using ifs and matches
	*/
	println!("safely_parsing 'seven' to i32|- {:?}", safely_parse_str("seven"));
	println!("safely_parsing '89' to i32|- {:?}", safely_parse_str("89"));
	println!("----");

	// panic! and unwrap
	/*
		panic! macro is a cheap way for error handling mid-development
		for unit testing:
			- assert!(): if the part inside is not true, program panics
			- assert_eq!(a,b); // equals
			- assert_ne!(a,b); // not equals

		unwrap(): when you want your program to crash when you want it to crash 
				when there's a problem
		expect(): same reason but you can pass an err msg if it fails
				so it's a we bit more verbose
		unwrap_or(): avoids panics, sends an alternative value instead
		
	*/
	// Traits
	/*
		Remember we needed to implement Debug, Copy, Clone to print and do stuff?
		as in adding this to print stuff?:
			#[derive(Debug)]
			struct thing { ... }

		Debug is called a trait, like bone-in methods, or spicy methods

		To implement traits, use keyword: impl as in implement
		To define traits, use the keyword: trait
	*/
	let joke_car = Car {
			make: String::from("Toyota"), 
			model: String::from("Camry")
		};
	println!("So we have a car: {}", joke_car);
	print!("Reving Engine - ");
	joke_car.rev_engine();
	print!("Ok, time to start driving... ");
	joke_car.accelerate();
	println!("Pull over, we need gas!");
	add_gas(&joke_car);
	println!("----");

	// The From trait
	/*
		A convenient way of making a type from another type
		Honestly, the example was very good, so it's copied from there
	*/
	let berlin = City::new("Berlin", 4_000_000);
	let cologne = City::new("Cologne", 900_000);
	let german_cities = vec![berlin, cologne];
	let germany = Country::from(german_cities);
	germany.print_cities();
}
//---- relevant functions for ?
use std::num::ParseIntError;
fn safely_parse_str(input: &str) -> Result<i32, ParseIntError> {
	Ok(input.parse::<i32>()?) 
	//Returning an Ok because that's what we need to return, proper value is already unwrapped
	//We aren't returning an i32
}
//---------------------------------
//-----relevant for Traits---------
#[derive(Debug)]
struct Car {
	make: String,
	model: String
}

trait V8Engine {
	fn rev_engine(&self) {
		println!("vroom vroom, that's $20");
	}
	fn accelerate(&self); 
	// protoype fxn, needs to be fleshed out for
	// each implementation of V8Engine trait
}

impl V8Engine for Car {
	fn accelerate(&self) {
		let car_type = &self.model;
		println!("LMAOO get gapped by a {}", car_type);
	}
}

use std::fmt;

impl fmt::Display for Car {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f,"{} {}", self.make, self.model)
	}
}

//The method below is called trait bounds, an alternative way of writing methods to traits
use std::fmt::Debug;
fn add_gas<T: V8Engine + Debug>(car: &T) {
	println!("Filling up gas tank for {:?}", car);
}
//-----------------------------------
//------relevant for The From trait--
#[derive(Debug)] // So we can print City
struct City {
    name: String,
    population: u32,
}

impl City {
    fn new(name: &str, population: u32) -> Self { // just a new function
        Self {
            name: name.to_string(),
            population,
        }
    }
}
#[derive(Debug)] // Country also needs to be printed
struct Country {
    cities: Vec<City>, // Our cities go in here
}

impl From<Vec<City>> for Country { // Note: we don't have to write From<City>, we can also do
                                   // From<Vec<City>>. So we can also implement on a type that
                                   // we didn't create
    fn from(cities: Vec<City>) -> Self {
        Self { cities }
    }
}

impl Country {
    fn print_cities(&self) { // function to print the cities in Country
        for city in &self.cities {
            // & because Vec<City> isn't Copy
            println!("{:?} has a population of {:?}.", city.name, city.population);
        }
    }
}
//----