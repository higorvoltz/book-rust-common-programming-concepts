use std::io;

fn main() {
  let x = 5;
  println!("the value of x is: {}", x);

  // x = 6 error immutable
  // println!("the value of x is: {}", x);

  let mut y = 5; //mutable
  println!("the value of y is: {}", y);

  y = 6;
  println!("the value of y is: {}", y);

  const HOURS_PER_DAY: u8 = 24;
  println!("the value ofHOURS_PER_DAY is: {}", HOURS_PER_DAY);

  // HOURS_PER_DAY = 0; //cannot assign to this espression
  // println!("the value ofHOURS_PER_DAY is: {}", HOURS_PER_DAY);

  let spaces = "   ";
  println!("{}", spaces.len());

  //shadowing
  let alfa = 0;

  let alfa = alfa + 1;

  {
    let alfa = alfa * 3;
    println!("the value of alfa in inner scope is : {}", alfa);
  }
  println!("the value of alfa in outer scope is: {}", alfa);

  //Data Types

  // let guess = "42".parse().expect("not a number!");
  //  |       ^^^^^ consider giving `guess` a type
  let guess: u32 = "42".parse().expect("not a number!");
  println!("the value of guess is: {}", guess);

  //Scalar Types: types => integers, floating-point numbers, Booleans, and characters
  let _number_a = 2.0; //f64 float 64 bits more precision less memory
  let _number_b: f32 = 3.0; //f32 float 32 bits less precision more memory

// operators
let sum_a = 1 + 1;
println!("the value of sum_a is: {}", sum_a);

// let sum_b = 1.0 + 1;
// println!("the value of sum_b is: {}", sum_b);
// let sum_b = 1.0 + 1;
//    |                 ^ no implementation for `{float} + {integer}`

let my_bool = false;
println!("the inverted value of my_bool is: {}", !my_bool);

let _my_char = 'a';
let _other_char: char = 'b'; //explicit type annotation

//Tuples have a fixed length: once declared, they cannot grow or shrink in size
//permitted different type
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (first, second, thrid) = tup; //destructuring

println!("the value of first is: {}", first);
println!("the value of second is: {}", second);
println!("the value of thrid is: {}", thrid);

// we can access elements of tuples is with (.)
println!("{}", tup.0);
println!("{}", tup.1);
println!("{}", tup.2);

//array type: every element of an array must have the same type, fixed length
let arr = [1, 2, 3, 4, 5];

let first_arr = arr[0];
println!("{}", first_arr);
let second_arr = arr[1];
println!("{}", second_arr);

println!("please enter an array index between 0 and 4");
let mut index = String::new();

io::stdin()
  .read_line(&mut index)
  .expect("failed to read line");

let index: usize = index
  .trim()
  .parse()
  .expect("index entered was not a number");

let element = arr[index];
println!("the value of the element at index {index} is: {element}");

// if choice a number out of bounds...
// thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 8', src/main.rs:96:15
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

fn another_function(){
  println!("another function");
}

another_function();

fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {value}{unit_label}");
}

print_labeled_measurement(5, 'h');

fn plus_one(x: i32) -> i32 {
  x + 1
}

let new_value = plus_one(1);

println!("{}", new_value);

// if expressions
let sunday = true;

if sunday {
  println!("I going to the beach")
} else {
  println!("no way!")
}

// loop expessions
let mut counter = 0;
let result = loop {
  counter += 1;

  if counter == 10 {
    break counter * 2;
  }
};

println!("the result is: {}", result);

let mut count = 0;
  'counting_up: loop {
    println!("count = {count}");
      let mut remaining = 10;

      loop {
        println!("remaining = {remaining}");
          if remaining == 9 {
              break;
          }
          if count == 2 {
              break 'counting_up;
          }
        remaining -= 1;
      }
    count += 1;
  }
println!("End count = {count}");

let mut test = 3;
while test != 0 {
  println!("{}", test);
  test -= 1;
}

let collect = ["a", "b", "c", "d", "e", "f", "g"];
let mut i = 0;

while i < collect.len() {
  println!("{}", collect[i]);
  i += 1;
}

println!();

for num_collect in (1..4).rev(){
  println!("{}", num_collect);
}

println!();

for num_collect in 1..4{
  println!("{}", num_collect);
}




}