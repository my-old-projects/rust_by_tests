fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum UserType {
    Admin,
    Moderator,
    Editor,
    Standart,
}

#[derive(Debug, PartialEq)]
enum UserStatus {
    Registered,
    Confirmed,
    Banned,
    WaitingForValidate,
}

#[derive(Debug, PartialEq)]
enum User {
    Status(UserStatus),
    Type(UserType),
}

fn get_user_status(user: User) -> i32 {
    return match user {
        User::Status(status) => {
            println!("Status {:?}", status);
            return match status {
                UserStatus::Registered => 1,
                UserStatus::Banned => 0,
                UserStatus::WaitingForValidate => 2,
                UserStatus::Confirmed => 3,
            };
        }
        _ => 0,
    };
}

fn get_user_type_id(user_type: UserType) -> i32 {
    match user_type {
        UserType::Admin => 1,
        UserType::Moderator => 2,
        UserType::Editor => 3,
        UserType::Standart => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_status() {
        let status = User::Status(UserStatus::Confirmed);

        assert_eq!(get_user_status(status), 3, "Statuses didn't match");
    }

    #[test]
    fn test_get_user_type_id() {
        assert_eq!(
            get_user_type_id(UserType::Admin),
            1,
            "Type ids didn't match"
        );
    }
}
