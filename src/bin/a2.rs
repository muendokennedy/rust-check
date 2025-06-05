fn add(num1: i32, num2: i32) -> i32{
  num1 + num2
}

fn display_result(result: i32){
  println!("{:?}", result);
}

fn main(){

  let sum = add(100, 300);

  display_result(sum);
}