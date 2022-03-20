fn main() {
    let x: i32; // declare x, cannot modify this value
    x = 42; //assign 42 to x
    println!("Value of x -> {}", x);

    let mut y: i32 = 42; // declare mutable variable y (modification possible)
    println!("Value of y -> {}", y);
    y = 50;
    println!("Value of y after modification -> {}", y);

    // Uninitialized variables throw error
    // let num;
    // foobar(num); //error

    /*
     The underscore _
     Compiler error uninitialized can be removed for work in progress.
     _variable_name

     _ = get_thing();

    */
    let _num: i8 = 124; // 0 - 127
    let _pixel_value: u8 = 212; //0 - 255

    // Variable shadowing is allowed
    let z: i32 = 13;
    let z: i32 = z + 3;
    println!("z -> {}", z);

    //tuples
    let _pair: (char, i32) = ('a', 17);
    let (_filter_type, _count) = ('t', 100);

    let is_visible: bool = true;

    println!("Visibility -> {}", is_visible);

    // Inference
    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;
    println!("Inference -> {}", elem);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);

    //
    let a = {
        let b: i32 = 20;
        let c: i32 = 30;
        b + c
    };
    println!("a {}", a);

    let minimum = std::cmp::min(10, 100);
    println!("Minimum {}", minimum);

    let name: String = String::from("Rupesh");
    println!("Minimum {}", name);
}
