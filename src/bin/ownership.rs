struct GroceryItem {
    quantity: i32,
    id: i32
}

fn display_quantity(item: &GroceryItem){
    println!("The quantity of this item is: {:?}", item.quantity);
}

fn display_id(item: &GroceryItem){
    
    println!("The id of this item is: {:?}", item.id);
}

fn main(){
   let my_item = GroceryItem {
       quantity: 3,
       id: 95
   };

   display_quantity(&my_item);
   display_id(&my_item);
}
