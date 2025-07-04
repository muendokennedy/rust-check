struct Temperature {
    degrees_f: f64,
}



impl Temperature {

  fn freezing() -> Self {

      Self { degrees_f: 18.0 }
  }

  fn boiling() -> Self {

      Self { degrees_f: 212.46 }

  }


  fn show_temp(&self){

    println!("{:?} degrees F", self.degrees_f);

  }
}
fn main(){

    let hot = Temperature { degrees_f: 99.4 };

    hot.show_temp();

    let cold = Temperature::freezing();

    cold.show_temp();

    let boiling = Temperature::boiling();

    boiling.show_temp();
    boiling.show_temp();
    boiling.show_temp();
    boiling.show_temp();




}
