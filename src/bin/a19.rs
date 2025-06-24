struct Adult {
    age: u8,
    name: String
}

impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str>{

        if age >= 21 {
            Ok(Self{
                age,
                name: name.to_string()
            })
        } else {
            Err("Age must be at least 21")
        }

    }
}

fn main(){
    let child = Adult::new(15, "Anita");
    let adult = Adult::new(25, "Kenney");

    match child {
        Ok(child) => println!("{} is {} years old.", child.name, child.age),
        Err(e) => println!("{e}"),
    }

    match adult {
        Ok(a) => println!("{} is {} years old.", a.name, a.age),
        Err(e) => println!("{e}"),
    }

}
