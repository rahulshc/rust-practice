use std::io;

fn main() {

    loop {
        let mut option = String::new();
        let mut input_temp = String::new();

        println!("Select an option.\n1. Convert Farenheit to celsius.\n2. Convert celsius to farenheit.");

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
            
            let option: u32 = match option.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };

            if !(option == 1 || option == 2) {
                continue;
            }

            loop {
            
                println!("Input the temperature ");
                
                io::stdin()
                    .read_line(&mut input_temp)
                    .expect("Failed to read line");
                    
                    let input_temp: i32 = match input_temp.trim().parse(){
                        Ok(num) => num,
                        Err(_) => continue,
                    };

                    if option == 1 {
                        println!("Temperature in celsius is {}", (input_temp-32)*5/9);
                        break;
                    }
                    else {
                        println!("Temperature in farenheit is {}", (input_temp*9/5)+32);
                        break;
                    }

                }
        }
    

}
 