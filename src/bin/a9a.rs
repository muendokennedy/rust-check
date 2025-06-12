fn coordinate() -> (i32, i32) {
    (1, 7)
}

fn main(){
    let (x, y) = coordinate();

    if y > 5 {
        println!("This is greate than 5");
    } else if y < 5 {
        println!("This is less than 5");
    } else {
        println!("This is equal to 5");
    }
}
