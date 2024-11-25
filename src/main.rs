/*This is a program for a CLI address Book */

//Make nessesary imports here
use std::io::{self, Read};
use std::collections::VecDeque;

//Main Struct containing the datas to be inputted
struct Address {
    names: String,
    number: usize,
    address: String,
    country: String,
    state: String
}

//Implementation of the struct
impl Address{
    fn create(names: String, number: usize, address: String, country: String, state: String) -> Self {
        Self { names, number, address, country, state }
    }
}

//Struct to handle the info gathered
struct AddressBook {
    infos: VecDeque<Address>
}

//Implementation of struct
impl AddressBook {
    fn new() -> Self {
        Self { infos: VecDeque::new() }
    }

    fn add(&mut self, names: String, number: usize, address: String, country: String, state: String) {
        let info = Address::create(names, number, address, country, state);
        self.infos.push_back(info);
        println!("Info Added!!!\n");
    
    }
    
    fn view(&self) {
        if self.infos.is_empty(){
            println!("List is Empty");

        }else {
            for (index, info) in self.infos.iter().enumerate(){
                println!("No. {}\t Names: {}\t Number: {}\t Address: {}\t Country: {}\t State: {}", index + 1, info.names, info.number, info.address, info.country, info.state);
            }
        }
    }

    fn delete(&mut self, index: usize){
        if let Some(info) =  self.infos.get_mut(index){
            self.infos.remove(index);
            println!("Info Deleted!!!\n");
        }
    }

    fn update(&mut self, index: usize, names: String, number: usize, address: String, country: String, state: String){
        if let Some(info) = self.infos.get_mut(index){
            info.names = names;
            info.number = number;
            info.address = address;
            info.country = country;
            info.state = state;
        }
        println!("Info Updated!!!\n");
    }
}

fn main() {
    let mut address_book = AddressBook::new();

    loop {
        println!("\nADDRESS BOOK, SELECT AN OPTION: ");
        println!("1. Add: ");
        println!("2. View: ");
        println!("3. Delete: ");
        println!("4. Update: ");
        println!("5. Exit: ");
        println!();

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Couldn't read input");
        let option: i32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            1 => {
                println!("Enter the names: ");
                let mut names = String::new();
                io::stdin().read_line(&mut names).expect("Couldn't read input");
                let names = names.trim().to_string();
                println!("Enter the phone number: ");
                let mut number = String::new();
                io::stdin().read_line(&mut number).expect("Couldn't read input");
                let number: usize = number.trim().parse().expect("Enter your phone number: ");
                println!("Enter the address: ");
                let mut address = String::new();
                io::stdin().read_line(&mut address).expect("Couldn't read input");
                let address = address.trim().to_string();
                println!("Enter the country: ");
                let mut country = String::new();
                io::stdin().read_line(&mut country).expect("Couldn't read input");
                let country = country.trim().to_string();
                println!("Enter the state: ");
                let mut state = String::new();
                io::stdin().read_line(&mut state).expect("Couldn't read input");
                let state = state.trim().to_string();

                address_book.add(names, number, address, country, state);
            },

            2 => {
                address_book.view();
            },
            
            3 => {
                println!("Enter the index of the info to be deleted: ");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Couldn't read input");
                let index = match index.trim().parse::<usize>() {
                    Ok(num) => num - 1,
                    Err(_) => {
                        println!("Index not found in list");
                        continue;
                    },
                };
                address_book.delete(index);
            },

            4 => {
                
                println!("Enter the index of the info to be updated: ");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Couldn't read input");
                let index = match index.trim().parse::<usize>() {
                    Ok(num) => num - 1,
                    Err(_) => {
                        println!("Index not found in list");
                        continue;
                    },
                };

                println!("Enter the names: ");
                let mut names = String::new();
                io::stdin().read_line(&mut names).expect("Couldn't read input");
                let names = names.trim().to_string();
                println!("Enter the phone number: ");
                let mut number = String::new();
                io::stdin().read_line(&mut number).expect("Couldn't read input");
                let number: usize = number.trim().parse().expect("Enter your phone number: ");
                println!("Enter the address: ");
                let mut address = String::new();
                io::stdin().read_line(&mut address).expect("Couldn't read input");
                let address = address.trim().to_string();
                println!("Enter the country: ");
                let mut country = String::new();
                io::stdin().read_line(&mut country).expect("Couldn't read input");
                let country = country.trim().to_string();
                println!("Enter the state: ");
                let mut state = String::new();
                io::stdin().read_line(&mut state).expect("Couldn't read input");
                let state = state.trim().to_string();

                address_book.update(index, names, number, address, country, state);
            }

            5 => {
                println!("Have a nice day!!!!");
                break;
            },
            _ => {
                println!("Invalid Option");
            }
        }
    }

}
