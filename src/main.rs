use std::io;


struct Product {
    name: String,
    price: f32,
}


fn main() {
    // display title
    println!("\n*********************************************************************************");
    println!("Welcome to vending/change machine !!");
    println!("**********************************************************************************\n");
   
    
    'main: loop {

        let mut user_inputf: f32;

        let mut products: Vec<Product> = Vec::new();

        // create products individually
        products.push(Product{name: "Coke".to_owned(), price: 0.50f32});
        products.push(Product{name: "Pepsi".to_owned(), price: 0.55f32});
        products.push(Product{name: "Soda".to_owned(), price: 0.20f32});
        products.push(Product{name: "Fanta".to_owned(), price: 0.60f32});
        products.push(Product{name: "Oreo".to_owned(), price: 1.50f32});
        products.push(Product{name: "Princles".to_owned(), price: 4.50f32});
        products.push(Product{name: "Lays".to_owned(), price: 3.50f32});
        products.push(Product{name: "Cookies".to_owned(), price: 1.75f32});
        products.push(Product{name: "Sandwich".to_owned(), price: 6.80f32});
        products.push(Product{name: "Choclate".to_owned(), price: 7.10f32});


        let product = display_menu(&products);

        // display price & user guide
        println!("\n****************");
        println!("{} : {}", product.name, product.price);
        println!("****************\n");
        
        
        loop {
            println!("Please enter payment below in \"000.00\" format!");
            //----------------------------------------------------------------------
            // get user input in string format
            let mut user_input: String = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");
    
           
            // see if user wants to quit
            if user_input.trim().to_lowercase().contains("q"){

                println!("Are you sure do quit ?");
                println!("\n0: No");
                println!("1: Yes  \n");

                let mut user_input: String = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");
                if user_input.trim().to_lowercase().contains("1"){
                    println!("quitting...");
                    break 'main;
                }
                else if user_input.trim().to_lowercase().contains("0") {
                    user_input.clear();
                    continue;
                }
                else {
                    println!("This is not a Option ! try again Please..");
                    continue;

                }
            }

            // check user_input format 
            if user_input.trim().len() != 6 || user_input.chars().any(char::is_alphabetic) {
                println!("Wrong format!");
                user_input.clear();
                continue;
            }

            // try to parse user_input to f32 to be able to compare with price
            user_inputf = match user_input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            if user_inputf < product.price{
                println!("Not enough cash!");
                user_input.clear();
                continue;
            }

            let difference = ((user_inputf - product.price) * 100.00 ).round() / 100.00;
            if difference > 0.00f32 {
                println!("\nYour total change: {}", difference);
                println!("{}", print_coins(difference));
                break;
            } 
            if difference == 0.00f32 {
                println!("Thank you! Payment accepted!");
                break;
            }

        } // end of user_input check loop
    }// end of main loop
}
    


fn print_coins(change: f32)-> String{
    let coin_vals: [f32; 8] = [2.00, 1.00, 0.50, 0.20, 0.10, 0.05, 0.02, 0.01];
    let mut coins_to_display: String = String::new();
    let mut change_left = change;
    

    for c in coin_vals.iter(){
        let coin_count:f32 = (change_left - (change_left % c )) / c;

        if coin_count > 0.000f32 {
            let coins_formatted = format!("[{} x {} coins], \n", coin_count, c);
            coins_to_display.push_str(&coins_formatted);
        }
        change_left = change_left - coin_count * c; 
    }

    coins_to_display   
}

fn display_menu(products: &Vec<Product>) -> &Product{
    let mut menu_num = 1;
    for p in products.iter(){
        println!("{}. {}: {}", menu_num, p.name, p.price);
        menu_num += 1;
    }

    let item_index = get_user_menu_choice() - 1 as usize;
    &products[item_index]
}


fn get_user_menu_choice() -> usize{

    loop {
        let mut user_input: String = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input: i32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if user_input < 1 || user_input > 10{
            println!("Please enter number corresponding to menu items! (1-10)");
        }else {
            return user_input as usize;
        }
    }
}