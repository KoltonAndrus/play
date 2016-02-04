extern crate core;

use std::ops::Add;
use core::fmt;

struct MyInt {
    x: u32,
    y: u32
}

impl Add for MyInt {

    type Output = MyInt;
    fn add(self, other: MyInt) -> MyInt {
        MyInt{ x: self.x + other.x, y: self.y + other.y }
    }

}

impl fmt::Display for MyInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y) 
    }
}
fn plus_one(i: u32) -> u32 {
    i + 1
}

fn main() {
    // types

    let x1: u32 = 1_000;
    let y1: u32 = 2_000;
    let x2: u32 = 1_500;
    let y2: u32 = 2_500;

    let my_int = MyInt{ x: x1, y: y1};
    println!("my_int: {}",  my_int);
    let my_int2 = MyInt{ x: x2, y: y2};

    let my_int3 = my_int + my_int2;
    println!("my_int3: {}",  my_int3);
    
    let f = plus_one;
    let f_of_1 = f(x1);
    println!("f_of_1: {}", f_of_1);

    // arrays
    let arr = [x1, x2, y1, y2];
    println!("arr has {} elements, a[1] {}", arr.len(), arr[1]);
    println!("arr {:?}", arr);
    for a in &arr {
        println!("a: {}", a);
    }

    let slice = &arr[1..3];

    let pair = (1, "who");
    println!("slice {:?}, pair.0 is {}", slice, pair.0);

    for (i,j) in (5..10).enumerate() {
        println!("i,j: {},{}", i, j);
    }
}

