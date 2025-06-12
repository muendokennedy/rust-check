enum Color {
    Red,
    Yellow,
    Blue
}
fn print_color(my_color: Color){
    match my_color {
        Color::Red => println!("This is some red color"),
        Color::Yellow => println!("This is some yellow color"),
        Color::Blue => println!("This is some Blue color"),
    }
}
fn main(){
    print_color(Color::Blue);
}
