mod account;
mod character;
mod models;

use crate::character::character::{Character, Skills};
use crate::models::response::Response;
use account::account::User;
use std::collections::HashMap;

fn create_account_and_login() -> Response {
    let account = User {
        username: String::from("opcode"),
        password: String::from("veryverystrongpassword"),
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
        rank: String::from("Iron"),
        health: None,
        skills: None,
    };

    character.register()
}

fn use_a_character(character: Character) -> Character {
    let mut skills: HashMap<String, Skills> = HashMap::new();

    skills.insert(
        String::from("Ballistic"),
        Skills {
            power: 40,
            required_exp: 1,
            current_exp: 0,
            name: String::from("Ballistic"),
        },
    );

    Character {
        skills: Option::from(skills),
        ..character
    }
}

fn main() {
    let response = create_account_and_login();

    if response.success {
        println!("{}", response.message);
        println!("A new character will be created...");

        let character_response = create_a_character(1);

        if character_response.success {
            println!("{}", character_response.message);

            let current_character = use_a_character(Character {
                account_id: 1,
                level: 46,
                name: String::from("SET"),
                tag: String::from("TAG"),
                rank: String::from("Iron"),
                hide_name: false,
                health: Option::from(100),
                skills: None,
            });

            println!("You're using character with this data: {:?}", current_character);

            let skill_will_use = current_character.get_skill(String::from("Ballistic")).unwrap();

            println!("Skill data: {:?}", skill_will_use.name);
        }
    } else {
        println!("{}", response.message);
    }
}
