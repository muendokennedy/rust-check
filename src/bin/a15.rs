struct Person {
    name: String,
    fav_color: String,
    age: i32
}

fn print(data: &str){
    println!("{:?}", data);
}

fn main(){

    let people = vec![
       
       Person {
           name: String::from("Kennedy"),
           fav_color: String::from("blue"),
           age: 12
       },
       Person {
           name: String::from("George"),
           fav_color: String::from("Purple"),
           age: 14
       },
       Person {
           name: String::from("Dorcas"),
           fav_color: String::from("Green"),
           age: 17
       }
    ];

    for person in people {

        if person.age <= 14 {
           print(&person.name);
           print(&person.fav_color);
        }
    }
}

