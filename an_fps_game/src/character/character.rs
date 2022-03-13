// TODO: Add last games,

use crate::models::response::Response;
use std::collections::HashMap;

/// By default, skills can be None. Because Character also has dynamic field such as skills
///
/// A character can have a health value of 150 as maximum.
#[derive(Debug)]
pub struct Character {
    pub account_id: i32,
    pub level: i32,
    pub name: String,
    pub tag: String,
    pub rank: String,
    pub hide_name: bool,
    pub health: Option<u8>,
    pub skills: Option<HashMap<String, Skills>>,
}

// TODO: These skills are simple for now. For example, if I want to give health to others, what should I do?

/// current_exp and required_exp fields are can be the same.
///
/// for example, **current_exp: 1** and **required_exp: 1**
///
/// In this case, player can use the first skill.
///
/// For example, player want to use ultimate but player don't have enough exp, skill should not be used.
///
/// and skill should be implemented by a function
#[derive(Debug)]
pub struct Skills {
    pub name: String,
    pub current_exp: u8,
    pub required_exp: u8,
    pub power: u8,
}

impl Skills {
    pub fn use_skill(&self, name: String) -> (bool, String) {
        if self.current_exp < self.required_exp {
            return (
                false,
                String::from("Your experience is not enough for this operation"),
            );
        }

        return (true, format!("dec:{}", self.power.to_string()));
    }
}

impl Character {
    pub fn register(&self) -> Response {
        if self.name.is_empty() {
            return Response {
                message: String::from("Character name must be defined"),
                success: false,
                status_code: 422,
            };
        }

        Response {
            message: format!("Character created with name {}#{}", self.name, self.tag),
            success: true,
            status_code: 200,
        }
    }

    pub fn get_skill(&self, skill_name: String) -> Option<&Skills> {
        if let Some(val) = &self.skills {
            return match val.get(&skill_name) {
                Some(data) => Option::from(data),
                None => None
            };
        }

        return None;
    }
}

// TODO: Add tests
