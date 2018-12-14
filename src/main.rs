extern crate cacher;
use cacher::Cacher;

fn main() {
    let mut cacher = Cacher::new(|a| a * a);
    let val = cacher.value(42);
    println!("The value is : {}", val);
    let val2 = cacher.value(2);
    println!("The value is : {}", val2);
    let val3 = cacher.value(5);
    println!("The value is : {}", val3);
    let hv = cacher.values_ref();
    println!("{:?}", hv);
}
