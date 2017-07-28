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
}

