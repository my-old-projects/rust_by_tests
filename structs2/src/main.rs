fn main() {
    println!("Hello, world!");
}

struct User {
    username: String,
    password: String,
    email: String,
    birth_year: i32,
}

impl User {
    fn new(username: String, password: String, email: String, birth_year: i32) -> User {
        return User {
            username,
            password,
            email,
            birth_year,
        };
    }

    fn get_age(&self, current_year: i32) -> i32 {
        return current_year - self.birth_year;
    }

    fn is_registered(&self) -> bool {
        if self.email == "goren.ali@yandex.com" {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_registered() {
        let username = String::from("aligoren");
        let password = String::from("dummypassword");
        let email = String::from("goren.ali@yandex.com");
        let birth_year = 1993;

        let user = User::new(username, password, email, birth_year);

        let check_user_status = user.is_registered();

        assert_eq!(
            check_user_status, true,
            "You expected true. But result is different {}",
            check_user_status
        );
    }

    #[test]
    fn test_get_age() {
        let username = String::from("aligoren");
        let password = String::from("dummypassword");
        let email = String::from("imjohndoe@gmail.com");
        let birth_year = 1993;

        let user = User::new(username, password, email, birth_year);

        let age = user.get_age(2022);

        assert_eq!(age, 29, "You expected 29. But result is different {}", age);
    }

    #[test]
    fn test_user_birth_year() {
        let username = String::from("aligoren");
        let password = String::from("dummypassword");
        let email = String::from("imjohndoe@gmail.com");
        let birth_year = 1993;

        let user = User::new(username, password, email, birth_year);

        assert_eq!(user.birth_year, 1993, "Birth years didn't match");
    }
}
