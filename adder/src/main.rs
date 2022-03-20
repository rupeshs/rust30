mod addition;
mod submodules;
fn main() {
    let result: i32 = addition::add_num(10, 5);
    println!("Sum = {}", result);
    println!("Difference = {}", submodules::substraction::sub_num(50, 8));
}
