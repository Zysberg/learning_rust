#[allow(unused_variables)] // you'll learn what these are later
#[allow(dead_code)] // you'll learn what these are later

fn main () {
	// Control Flow
	/*
		true, false, == != <= >= < > != && ||
		If, else if , else, 

		match expression {...} is the switch case of rust

		its called pattern matching though
		
		does the expression match one of the patterns, otherwise make sure
		default case is accounted for

		match has to return the same type

	*/
	let my_number: u8 = 5;
	match my_number {
		0 => { /* do something */},
		1 => println!("one!"),
		_ => println!("default case = {}",my_number), // default case
	}

	let sky = "cloudy";
    let temperature = "warm";

    match (sky, temperature) {
        ("cloudy", "cold") => println!("It's dark and unpleasant today"),
        ("clear", "warm") => println!("It's a nice day"),
        ("cloudy", "warm") => println!("It's dark but not bad"),
        _ => println!("Not sure what the weather is."),
    }

    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);
    print!("First: ");
    match_colours(first);
    print!("Second: ");
    match_colours(second);
    print!("Third: ");
    match_colours(third);

    let example_variable = if 9+10==21 {"you rite"} else {"you stupid"};

    // Loops
    loop {
    	println!("inside the loop");
    	// do things
    	break; // exit loop
    }

    println!("Print all acronyms [A-N][A-Z]");
    let mut first_init = 65u8;
    'first_initial: loop {
    	let mut last_init = 65u8;
    	'second_initial: loop {
    		if first_init == 79 { break 'first_initial;}
    		print!("{}{} ", first_init as char, last_init as char);
    		if last_init == 90 {break; }
    		last_init += 1;
    	}
    	print!("\n");
    	first_init += 1;
    }

    while false {
    	// dead code 
    }

    print!("for n in 0..3 - ");
    for n in 0..3 { // iterates for when n = [0,1,2]
    	print!("{} ", n);
    }

    print!("\nfor n in 0..=3 - ");

    for n in 0..=3 { // iterates for n = [0,1,2,3]
    	print!("{} ",n);
    	// if you don't plan on using n in loops, add the underscore next 
    	// to it; suppressing unused variable warnings
    }
}

fn match_colours(rbg: (i32, i32, i32)) {
    match rbg {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, b, _) if b < 10 => println!("Not much blue"),
        (_, _, g) if g < 10 => println!("Not much green"),
        _ => println!("Each colour has at least 10"),
    }
}