//======================================================================
// returning a function
//======================================================================
fn get_doubler() -> Box<Fn(i32) -> i32> {
    fn doubler(number: i32) -> i32 {
        number * 2
    }

    Box::new(doubler)
}
//----------------------------------------------------------------------

//======================================================================
// mutable pass-by-reference
//======================================================================
fn adds_5_to_input(num: &mut i32) {
    *num += 5;
}
//----------------------------------------------------------------------

/// Run below test with: `rustdoc hello.rs --test`
///
/// ```rust
/// assert_eq!('c', ascii_tolower(b'C') as char);
///
/// # fn ascii_tolower(c: u8) -> u8 {
/// #     match c {
/// #         b'A' ... b'Z' => c + b'a' - b'A',
/// #         _ => c
/// #     }
/// # }
/// ```
fn ascii_tolower(c: u8) -> u8 {
    match c {
        b'A' ... b'Z' => c + b'a' - b'A',
        _ => c
    }
}

// calculate factorial
fn factorial(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => n * factorial(n - 1),
    }
}

// find GCD of 2 integers
fn gcd(mut a: u64, mut b: u64) -> u64 {
    assert!(a != 0 && b != 0);
    while a != 0 {
        // if b > a then we swap them to make sure that a > b always
        if b > a {
            let temp = b; b = a; a = temp;
        }
        a = a % b;
    }
    b
}

fn extract_path<'a>(input: &'a str) -> Option<&'a str> {
    let res: &'a str;
    // res = &(String::from("testing"));
    // Some(String::from("testing").as_ref())
    Some("tung")
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(2 * 5 * 11 * 17,
                   3 * 7 * 13 * 19),
               1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
               3 * 11);
}

// ----------------------------
// trait is like interface in Java, but it can have default implementation
trait Talk {
    fn talk (&self) {
        println!("strange noises...");
    }
}

// struct is like lightweight class
struct Human;
struct Dog;
struct Tree;
impl Talk for Human {
    fn talk (&self) {
        println!("I'm a human blabla");
    }
}

impl Talk for Dog {
    fn talk (&self) {
        println!("I'm a dog, bark bark");
    }
}

impl Talk for Tree {}

fn talk(talkable: &Talk) {
    talkable.talk();
}

fn talk_improved<T: Talk>(talkable: &T) {
    talkable.talk();
}

fn main() {
    let doubler = get_doubler();
    println!("{:}", doubler(14));  // -> 28

    let mut num = 3;
    adds_5_to_input(&mut num);
    println!("New num: {}", num); // 8

    //======================================================================
    // closure
    //======================================================================
    let numbers = vec![20, 19, 7, 12];
    println!("Numbers vec original: {:?}", numbers);

    // fully annotate the types
    let numbers_mapped = numbers.iter().map(|number: &i32| -> i32 {number * 2}).collect::<Vec<_>>();
    println!("Numbers vec mapped: {:?}", numbers_mapped);

    // make use of type inference
    let numbers_mapped = numbers.iter().map(|number| number * 2).collect::<Vec<_>>();
    println!("Numbers vec mapped: {:?}", numbers_mapped);
    //----------------------------------------------------------------------

    let person = Human;
    let dog = Dog;
    let tree = Tree;

    talk_improved(&person);
    talk_improved(&dog);
    talk_improved(&tree);

    talk(&person);
    talk(&dog);
    talk(&tree);

    // println! format
    println!("Hello, world!");
    println!("Hello, {name:?}! The {job:?}", job="Software Engineer", name="Tung");

    // format printing decimal number
    println!("{:.2}", 1.234);

    // print number base
    println!("bin: {:b}, hex: {:x}, HEX: {:X}, oct: {:o}", 10, 10, 10, 10);

    // define multiple variables and print data multiple times
    let (f_name, l_name) = ("Tung", "Le");
    println!("My name is {0} {1} {0}", f_name, l_name);


    for num in 0..10 {
        println!("Hi {}", num);
    }
    println!("65u8 as char is {}", 65u8 as char);
    println!("Convert to lower case {}", ascii_tolower(b'C') as char);

    let tuple = (1, 2, 3);
    println!("First: {}, second: {}, third: {}", tuple.0, tuple.1, tuple.2);

    // function pointer
    let myFn: fn(u8) -> u8 = ascii_tolower;
    println!("Convert B to lower case: {}", myFn(b'B') as char);

    assert_eq!('c', ascii_tolower(b'C') as char);

    println!("GCD of 10 and 15 is: {}", gcd(10, 15));
    println!("GCD of 10 and 40 is: {}", gcd(40, 10));
}

