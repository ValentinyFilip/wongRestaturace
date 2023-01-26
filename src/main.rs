use std::io;
use mysql;
use mysql::prelude::Queryable;
use mysql::params;

struct Food {
    id: u32,
    name: String,
    price: u32,
}

struct Drinks {
    id: u32,
    name: String,
    price: u32,
}

struct Tables {
    id: u32,
    food: String,
    drinks: String,
    price: u32,
}

fn main() {
    println!("Welcome to the restaurant!");
    println!("Please select a command:");
    println!("1. Add Food to menu");
    println!("2. Add Drinks to menu");
    println!("3. Add table");
    println!("4. Add order to table");

    println!("> ");
    let mut command_in = String::new();
    io::stdin()
        .read_line(&mut command_in)
        .expect("error when reading line");

    let command_parsed: u32 = match command_in.trim().parse() {
        Ok(cmd) => cmd,
        Err(error) => panic!("Error when parsing to Command Struct - {}", error),
    };

    match command_parsed {
        1 => command_1(),
        2 => command_2(),
        3 => command_3(),
        4 => command_4(),
        _ => println!("Invalid command"),
    }
}

// add Food to menu
fn command_1() {
    let url = "mysql://root:R0otP1s3wo5d@localhost:3306/project1";
    let pool = mysql::Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    println!("Command 1");

    println!("> Enter Food name: ");
    let mut food_name = String::new();
    io::stdin()
        .read_line(&mut food_name)
        .expect("error when reading line");

    let food_name_parsed: String = match food_name.trim().parse() {
        Ok(cmd) => cmd,
        Err(error) => panic!("Error when parsing to Command Struct - {}", error),
    };


    println!("> Enter Food price: ");
    let mut food_price = String::new();
    io::stdin()
        .read_line(&mut food_price)
        .expect("error when reading line");

    let food_price_parsed: u32 = match food_price.trim().parse() {
        Ok(cmd) => cmd,
        Err(error) => panic!("Error when parsing to Command Struct - {}", error),
    };

    let food = Food {
        id: 0,
        name: food_name_parsed,
        price: food_price_parsed,
    };

    conn.exec_drop(
        "INSERT INTO Food (name, price) VALUES (:name, :price)",
        params! {
            "name" => food.name,
            "price" => food.price,
        },
    ).unwrap();
}

fn command_2() {
    let url = "mysql://root:R0otP1s3wo5d@localhost:3306/project1";
    let pool = mysql::Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    println!("Command 2");

    println!("> Enter drink name: ");
    let mut drink_name = String::new();
    io::stdin()
        .read_line(&mut drink_name)
        .expect("error when reading line");
    let drink_name_parsed: String = match drink_name.trim().parse() {
        Ok(cmd) => cmd,
        Err(error) => panic!("Error when parsing to Command Struct - {}", error),
    };

    println!("> Enter drink price: ");
    let mut drink_price = String::new();
    io::stdin()
        .read_line(&mut drink_price)
        .expect("error when reading line");
    let drink_price_parsed: u32 = match drink_price.trim().parse() {
        Ok(cmd) => cmd,
        Err(error) => panic!("Error when parsing to Command Struct - {}", error),
    };

    let drink = Drinks {
        id: 0,
        name: drink_name_parsed,
        price: drink_price_parsed,
    };

    conn.exec_drop(
        "INSERT INTO Drinks (name, price) VALUES (:name, :price)",
        params! {
            "name" => drink.name,
            "price" => drink.price,
        },
    ).unwrap();
}

fn command_3() {
    let url = "mysql://root:R0otP1s3wo5d@localhost:3306/project1";
    let pool = mysql::Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    println!("Command 3");
    println!("> Enter table number: ");
    let mut table_number = String::new();
    io::stdin()
        .read_line(&mut table_number)
        .expect("error when reading line");
    let table_number_parsed: u32 = match table_number.trim().parse() {
        Ok(cmd) => cmd,
        Err(error) => panic!("Error when parsing to Command Struct - {}", error),
    };

    let table = Tables {
        id: table_number_parsed,
        food: "".to_string(),
        drinks: "".to_string(),
        price: 0,
    };

    conn.exec_drop(
        "INSERT INTO Tables (id) VALUES (:id)",
        params! {
            "id" => table.id,
        },
    ).unwrap();
}

fn command_4() {
    let url = "mysql://root:R0otP1s3wo5d@localhost:3306/project1";
    let pool = mysql::Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    println!("Command 4");
    println!("> Enter table number: ");
    let mut table_number = String::new();
    io::stdin()
        .read_line(&mut table_number)
        .expect("error when reading line");
    let table_number_parsed: u32 = match table_number.trim().parse() {
        Ok(cmd) => cmd,
        Err(error) => panic!("Error when parsing to Command Struct - {}", error),
    };

    println!("> Enter food name: ");
    let mut food_name = String::new();
    io::stdin()
        .read_line(&mut food_name)
        .expect("error when reading line");
    let food_name_parsed: String = match food_name.trim().parse() {
        Ok(cmd) => cmd,
        Err(error) => panic!("Error when parsing to Command Struct - {}", error),
    };

    println!("> Enter drink name: ");
    let mut drink_name = String::new();
    io::stdin()
        .read_line(&mut drink_name)
        .expect("error when reading line");
    let drink_name_parsed: String = match drink_name.trim().parse() {
        Ok(cmd) => cmd,
        Err(error) => panic!("Error when parsing to Command Struct - {}", error),
    };

    let mut total_price: u32 = 0;
    
}