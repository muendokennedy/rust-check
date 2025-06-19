fn main(){

    // let my_numbers = vec![1, 2, 3, 4];

    let mut my_numbers = Vec::new();

    my_numbers.push(1);
    my_numbers.push(2);
    my_numbers.push(3);
    my_numbers.push(4);
    my_numbers.push(5);
    my_numbers.push(6);
    my_numbers.push(7);
    my_numbers.push(8);
    my_numbers.pop();
    my_numbers.len();

    for num in my_numbers {

        println!("{:?}", num);

    }

}
