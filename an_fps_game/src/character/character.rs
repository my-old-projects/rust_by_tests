// TODO: Add last games,

use crate::models::response::Response;

#[derive(Debug)]
pub struct Character {
    pub account_id: i32,
    pub level: i32,
    pub name: String,
    pub tag: String,
    pub rank: String,
    pub hide_name: bool,
}

impl Character {
    pub fn register(&self) -> Response {
        if self.name.is_empty() {
            return Response {
                message: String::from("Character name must be defined"),
                success: false,
                status_code: 422
            };
        }

        Response {
            message: format!("Character created with name {}#{}", self.name, self.tag),
            success: true,
            status_code: 200
        }
    }
}

// TODO: Add tests