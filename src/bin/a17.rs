struct Student {
    name: String,
    locker: Option<i32>
}

fn main(){

    let mary = Student {
        name: "Mary" . to_owned(),
        locker: None
    };

    println!("Student name: {:?}", mary.name);

    match mary.locker {

        Some(num) => println!("Locker number: {:?}", num),
        None => println!("There is no locker number")
    }

}
