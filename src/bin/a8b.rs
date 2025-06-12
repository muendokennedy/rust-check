enum Flavor {
    Sparkling,
    Sweet,
    Fruity
}
struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink: Drink){
    match drink.flavor {
        Flavor::Sparkling => println!("This drink has a Sparkling flavor"),
        Flavor::Sweet => println!("This drink has a sweet flavor"),
        Flavor::Fruity => println!("This drink has a fruity flavor"),
    }
    println!("oz: {:?}", drink.fluid_oz);
}
fn main()
{
    let sweet = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 6.0
    };

    print_drink(sweet);

    let fruity = Drink {
        flavor: Flavor::Fruity,
        fluid_oz: 10.0
    };

    print_drink(fruity);

    let sparkling: Drink = Drink {
        flavor: Flavor::Sparkling,
        fluid_oz: 7.8
    };

    print_drink(sparkling);
}
