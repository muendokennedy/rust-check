enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String)
}

fn main(){
 
   let tickets = vec![
      
      Ticket::Backstage(50.0, "Kennedy".to_owned()),
      Ticket::Standard(15.0),
      Ticket::Vip(30.0, "George".to_owned())
   ];

   for ticket in tickets {

       match ticket {

           Ticket::Backstage(price, holder) => println!("Backstage ticket Holder {:?}, price {:?}", holder, price),
           Ticket::Standard(price) => println!("Standard ticket price {:?}", price),
           Ticket::Vip(price, holder) => println!("Vip ticket Holder {:?}, price {:?}", holder, price),

       }
   }

}
