#![allow(unused_variables)]// you'll learn what these are later
#![allow(dead_code)] // you'll learn what these are later


/*-----------------------------------------------------------------------
|Everything between these boxes are explained after 'car talk' example |
-----------------------------------------------------------------------*/
#[derive(Debug)]
enum VehicleType {
	Sedan,
	Pickup,
	SUV,
	Coupe
}

#[derive(Debug)]
struct Vehicle {
	num_wheels: u8,
	name: String,
	color:String,
	v_type:VehicleType,
}

impl Vehicle {
	fn new() -> Self {
		Self { // Vehicle::new() gives us this struct, like a filled in constructor
			num_wheels: 4,
			name: String::from("Cadillac Seville"),
			color:String::from("White"),
			v_type: VehicleType::Sedan
		}
	}

	fn check_tires(&self) {
		println!("Number of Wheels - {}", self.num_wheels);
	}
	fn morph_into_pickup(&mut self) { // borrow a mutuable Vehicle to change its properties
		self.v_type = VehicleType::Pickup;
		self.name = String::from("Ford F-150");
	}
}

impl VehicleType {
	fn car_sound(&self) {
		match self {
			VehicleType::Pickup => println!("HOOOONNNK!"),
			VehicleType::Coupe => println!("vroom, vroom"),
			_ => println!("beep beep!"),
		}
	}
}

/*-----------------------------------------------------------------------
|Everything between these boxes are explained after 'car talk' example |
-----------------------------------------------------------------------*/

struct AUnitStruct;

struct AnotherTupleStruct(bool,i32,bool,bool);

struct ACLikeStruct {
	x_coord: u32,
	y_coord: u32,
	some_details: AnotherTupleStruct,
} // <-- No semicolons after C-struct declarations 

#[derive(Debug)] // this allows you to use debug print for this class
struct Human {
	country: String,
	age: i8,
	eye_color:String,
}

enum Military {
	Army,
	Navy,
	AirForce
}

fn main () {

	// Structs
	/*
		'flexible' type creation via structs
		
		Each struct kind can be declared outside of main()

		Three different kinds of structs
			- unit structs ~ no properties, just StructName;
			- tuple structs
			- C-like 
		
		Yes you can put structs inside structs

		if you pass variables of the same name as struct properties,
		then you can type less
	*/ 

	struct ATupleStruct<'a>(u8,i32,char, &'a str);
	// to add &strs to tuple structs, need that &'a idk why yet - 8Dec21

	struct AnotherCLikeStruct {param1:String, param2:String} // no ;!

	let tuple_instance = ATupleStruct(99,180,'q',"apple");

	println!("You access elements like this: {}", tuple_instance.3);
	println!("Accessing another element: {}", tuple_instance.0);

	let country = String::from("USA");
	let age: i8 = 32;
	let eye_color = String::from("green");

	let some_guy = Human {
		country,
		age,
		eye_color
	};

	println!("Some Guy - {:?}", some_guy);
	println!("--------");
	// Enums
	/*
		Structs - when you want to have many attributes at once
		Enums - when you only need one of something, but have the choice of many

		you can also pass some kind of data to enums too
	*/
	// use Military::*;

	fn send_in_the(environment: String) -> Military {
		if environment == "Land" {
			Military::Army
		} else if environment == "Sea" {
			Military::Navy
		} else { Military::AirForce }
	}

	fn attack_from(mil: Military) {
		match mil {
			Military::Army => println!("Rolling Thunder"),
			Military::Navy => println!("Can't pass blockade"),
			_ => println!("Air Strike Inbound")
		}
	}

	enum CarPart {
		TireCnt(u8),
		PaintColor(String)
	}

	fn car_talk(car: CarPart, service: Military) {
		use Military::*;
		match service {
			Army => println!("Beat Navy!"),
			Navy => println!("Beat Army!"),
			_ => println!("meh")
		}
		match car {
			CarPart::TireCnt(4) => println!("You have all 4 tires"),
			CarPart::TireCnt(3) => println!("Where's your last tire?"),
			CarPart::TireCnt(5) => println!("Give your extra to that other guy"),
			CarPart::PaintColor(desc) => println!("{} is nice, but flames make it go faster",desc),
			_ => println!("are you sure that's a car?")
		}
	}

	car_talk(CarPart::TireCnt(3), Military::Army);
	car_talk(CarPart::PaintColor(String::from("green")), Military::Navy);
	println!("--------");

	// Implementing structs and enums
	/*
		To create methods for structs, wrap fxns in an impl block
		impl(ementation) of traits for types or for other traits
		(more on traits later)
		
		methods take self/&self/&mut self
			- objInst.method();
		static methods (aka associated fxns) rely on struct's """class"""
			- SomeClassOrStruct::static_method()
		
		You see #[derive(debug)]?
		#[...] <- these are called attributes
		They're like macro/metadata to your code (more no them later)
	*/
	let mut my_car = Vehicle::new(); // using static method
	println!("my car - {:?}",my_car);
	println!("now watch this... *poof*");
	my_car.morph_into_pickup();
	println!("now my car is - {:?}", my_car);
	println!("{:?}", my_car.v_type.car_sound()); 
	/* The extra parenthesis are printed as thats what's in the enum
	  See https://doc.rust-lang.org/rust-by-example/custom_types/enum.html for more */
	println!("--------");

	// Destructuring
	/*
		Basically like the spread operator for rust
	*/
}