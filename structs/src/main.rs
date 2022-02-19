fn main() {
    let username = String::from("yazilimci_adam");
    let password = String::from("123456"); // don't use a password like that
    let name = String::from("Bug Skywalker");
    let mut social: Vec<SocialProfile> = Vec::new();

    let social_profile = SocialProfile {
        provider: String::from("github"),
        icon: String::from("github.svg"),
        social_url: String::from("https://github.com/aligoren"),
    };

    social.push(social_profile);

    let user = get_user(username, password, name, social);

    println!("{:?}", user);

    dbg!(user);
    /*
    [src\main.rs:19] user = User {
            username: "yazilimci_adam",
            password: "123456",
            name: "Bug Skywalker",
            social: [
                SocialProfile {
                    provider: "github",
                    icon: "github.svg",
                    social_url: "https://github.com/aligoren",
                },
            ],
        }
    */
}

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub name: String,
    pub social: Vec<SocialProfile>,
}
#[derive(Debug)]
pub struct SocialProfile {
    pub provider: String,
    pub icon: String,
    pub social_url: String,
}

fn get_user(username: String, password: String, name: String, social: Vec<SocialProfile>) -> User {
    User {
        username,
        password,
        name,
        social,
    }
}

fn set_user() -> User {
    let username = String::from("yazilimci_adam");
    let password = String::from("123456"); // don't use a password like that
    let name = String::from("Bug Skywalker");
    let mut social: Vec<SocialProfile> = Vec::new();

    let social_profile = SocialProfile {
        provider: String::from("github"),
        icon: String::from("github.svg"),
        social_url: String::from("https://github.com/aligoren"),
    };

    social.push(social_profile);

    return get_user(username, password, name, social);
}

fn define_another_user_from_existing_one() -> User {
    User {
        username: String::from("bug_skywalker"),
        ..set_user()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_account() {
        let user = set_user();
        let index: usize = 0;

        assert_eq!(
            user.social[index].social_url, "https://github.com/aligoren",
            "Social urls didn't match"
        );
    }

    #[test]
    fn test_username_changed() {
        let user = define_another_user_from_existing_one();

        assert_eq!(user.username, "bug_skywalker", "Usernames didn't match");
    }

    #[test]
    fn test_password() {
        let user = set_user();

        assert_eq!(user.password, "123456", "Passwords didn't match");
    }

    #[test]
    fn test_is_username_not_empty() {
        let user = set_user();

        assert!(!user.username.is_empty(), "Username is empty")
    }
}
