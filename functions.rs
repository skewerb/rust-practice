fn add_two(x : int) -> int {

  x + 2 //no ; for return types

}

fn sub_three(x: int) -> int {

  x - 3

}

fn print_num(x : int){

  println!("The output of the number is: {}", x);

}

fn main(){

  //The i represents 'integer'
  let mut num = add_two(5i); //all variables immutable by default

  num = sub_three(num);

  print_num(num);

  println!("This works!");

}
