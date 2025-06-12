fn print_message(gt_100: bool){
    match gt_100 {
        true => println!("It is big"),
        false => println!("It is small"),
    }
} 

fn main(){

    let value = 5000;

    let is_greater_100 = value > 100;

    print_message(is_greater_100);

}
