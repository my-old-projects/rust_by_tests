mod account;
mod models;
mod character;

use account::account::User;
use crate::character::character::Character;
use crate::models::response::Response;

fn create_account_and_login() -> Response {
    let account = User {
        username: String::from("opcode"),
        password: String::from("veryverystrongpassword")
    };

    let register_result = account.register();

    println!("{}", register_result);

    account.login()
}

fn create_a_character(account_id: i32) -> Response {
    let character = Character {
        account_id,
        name: String::from("SET"),
        tag: String::from("TRUE"),
        hide_name: false,
        level: 1,
        rank: String::from("Iron")
    };

    character.register()
}

fn main() {

    let response = create_account_and_login();

    if response.success {
        println!("{}", response.message);
        println!("A new character will be created...");

        let character_response = create_a_character(1);

        if character_response.success {
            println!("{}", character_response.message);
        }
    } else {
        println!("{}", response.message);
    }
}
