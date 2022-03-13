use crate::models::response::Response;

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub password: String
}

impl User  {
    fn is_username_and_password_valid(&self) -> bool {
        self.username == String::from("opcode") && self.password == String::from("veryverystrongpassword")
    }

    pub fn register(&self) -> String {
        format!("You registered with username {}", self.username)
    }

    pub fn login(&self) -> Response {
        Response {
            status_code: if self.is_username_and_password_valid() { 200 } else { 404 },
            success: self.is_username_and_password_valid(),
            message: if self.is_username_and_password_valid() { String::from("Login successfully") } else { String::from("Wrong username or password") }
        }
    }
}

// TODO: Add tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_login_success() {
        let user = User {
            username: String::from("verystronguser"),
            password: String::from("thisisalsoverystrongpassword")
        };

        let login_response = user.login();

        assert_eq!(login_response.success, true, "User is not logged in");
    }

    #[test]
    fn test_for_login_fail() {
        let user = User {
            username: String::from("verystronguser"),
            password: String::from("thisisalsoverystrongpassword")
        };

        let login_response = user.login();

        assert_ne!(login_response.success, true, "User should not be logged in");
    }
}