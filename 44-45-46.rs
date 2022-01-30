#![allow(unused_variables)]// you'll learn what these are later
#![allow(dead_code)] // you'll learn what these are later

fn main () {
    // Type aliases
    /**
     * Good for when you want to shorthand a really long and complicated type
     * 
     */
    type CharVec = Vec<char>;
    type SkipFourTakeFive<'a> = std::iter::Take<std::iter::Skip<std::slice::Iter<'a, char>>>;
    fn returns <'a>(input: &'a Vec<char>) -> SkipFourTakeFive { input.iter().skip(4).take(5)}
    println!("-----end aliases-------");
    
    // todo! macro
    /**
     *   Useful in declaring functions without defining them
     * 
     */
    fn some_fxn() {
        todo!()
    }
    println!("-----end todo!-------");

    // Rc
    /**
     * Rc means reference counter; every variable needs 1 owner.
     * You could just do variable.clone() but that var could be part of a struct.
     * Rc allows for more than one owner; Rc records the owner, and how many.
     * Once Rc goes to 0, variable goes stale.
     */
    use std::rc::Rc;

    #[derive(Debug)]
    struct Thing {
        val: u32,
    }
    fn usse<T: std::fmt::Debug>(s: &str, t: T) {
        println!("{}, {:?}", s, t);
    }
    let some_var = Rc::new(Thing { val:44 });
    let str_rc = Rc::clone(&some_var); // essentially copies address
    usse("use once: ", some_var);
    // usse("some_var again: ", some_var); // compiler error here
    usse("use again: ", str_rc); // at this point, str_rc and some_var has gone stale
    /* // usse("some_var again: ", some_var); // compiler error here
    // usse("str_rc again: ", str_rc); // compiler error here */
    println!("----end Rc------");
    
    // Threading
    /**
     *  Things can be done at the same time wow
     * 
     *  3 types of closures; (callback exec fxns within the thread)
     *  FnOnce - takes whole value
     *  FnMut - takes  mutable reference
     *  Fn - takes a regular reference
     *  
     *  closures will try to use Fn, but if it needs to change
     *  the value it will use FnMut; if it needs the whole value
     *  it uses FnOnce; FnOnce is a good name bec it explains what
     *  it does: takes value once, then never again.
     * 
     * use the move keyword to reassign ownership of variables to
     * closures.
     */
    use::std::{thread,time::Duration};
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("CHILD THREAD - {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("PARENT THREAD - {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

    let mut my_str = String::from("k1ttypawss");
    let my_fn = || {
        println!("Fn {}", my_str);
    };
    my_fn();
    let mut my_fnmut = || {
        my_str.push_str(" is pog");
        println!("FnMut - {}", my_str);
    };
    my_fnmut();
    my_fnmut();
    let some_var: i32 = 69;
    let my_fnonce = || {
        println!("see code to learn about FnOnce");
        // use some_var for something, like a fn call or whatever
    };
    /*
        call this closure once
        compiler error is called when closure called again
    */
    println!("-----end Threading-----");
    
    // Closures in functions
    /**
     * When defining a param'd closure, you need to define the type
     * of closure it is (Fn FnOnce, FnMut)
     */
    fn some_fxn<F>(fxn: F)
    where F: FnOnce(), {
        fxn();
    }
    // -----end Closures in fxns -----
    //impl Trait
    /**
     * 
     * 
     * 
     */
}


