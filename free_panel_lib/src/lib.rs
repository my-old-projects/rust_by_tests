// source: /.rustup/toolchains/stable-x86_64-pc-windows-msvc/share/doc/rust/html/book/ch06-02-match.html

mod network_configs {
    pub mod card {

        #[derive(Debug)]
        pub enum IpAddr {
            V4(String),
            V6(String),
        }

        #[derive(Debug)]
        pub struct NetworkCard {
            pub name: String,
            pub ip: IpAddr,
        }

        fn create_config_file(card: &NetworkCard) -> String {
            format!("card {} initialized with ip: {:?}", card.name, card.ip)
        }

        pub fn set_new_card_config(card: &NetworkCard) -> String {
            create_config_file(card)
        }
    }

    pub mod service_settings {

        use super::card::{IpAddr, NetworkCard};

        #[derive(Debug)]
        pub enum ServiceCommand {
            Start,
            Info,
            Restart,
            Stop,
        }

        pub fn run_service_command(card: &NetworkCard, command: ServiceCommand) -> String {
            match command {
                ServiceCommand::Start => {
                    format!("Card {} started with ip: {:?}", card.name, card.ip)
                }
                ServiceCommand::Info => {
                    format!("Card {} is running with ip: {:?}", card.name, card.ip)
                }
                ServiceCommand::Restart => {
                    format!("Card {} restarted with ip: {:?}", card.name, card.ip)
                }
                ServiceCommand::Stop => format!(
                    "Card {} stopped with ip: {:?}. Network operations will not be used",
                    card.name, card.ip
                ),
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::network_configs::card::*;
    use super::network_configs::service_settings::*;

    #[test]
    fn test_run_service_command_stop() {
        let card = NetworkCard {
            name: String::from("PCI 3COM 3C905B 10/100 Ethernet"),
            ip: IpAddr::V4(String::from("192.168.1.1")),
        };

        let response_message = run_service_command(&card, ServiceCommand::Stop);

        assert_eq!(
            response_message,
            format!(
                "Card {} stopped with ip: {:?}. Network operations will not be used",
                card.name, card.ip
            ),
            "command 'stop' couldn't run"
        );
    }

    #[test]
    fn test_run_service_command_restart() {
        let card = NetworkCard {
            name: String::from("PCI 3COM 3C905B 10/100 Ethernet"),
            ip: IpAddr::V4(String::from("192.168.1.1")),
        };

        let response_message = run_service_command(&card, ServiceCommand::Restart);

        assert_eq!(
            response_message,
            format!("Card {} restarted with ip: {:?}", card.name, card.ip),
            "command 'restart' couldn't run"
        );
    }

    #[test]
    fn test_run_service_command_info() {
        let card = NetworkCard {
            name: String::from("PCI 3COM 3C905B 10/100 Ethernet"),
            ip: IpAddr::V4(String::from("192.168.1.1")),
        };

        let response_message = run_service_command(&card, ServiceCommand::Info);

        assert_eq!(
            response_message,
            format!("Card {} is running with ip: {:?}", card.name, card.ip),
            "command 'info' couldn't run"
        );
    }

    #[test]
    fn test_run_service_command_start() {
        let card = NetworkCard {
            name: String::from("PCI 3COM 3C905B 10/100 Ethernet"),
            ip: IpAddr::V4(String::from("192.168.1.1")),
        };

        let response_message = run_service_command(&card, ServiceCommand::Start);

        assert_eq!(
            response_message,
            format!("Card {} started with ip: {:?}", card.name, card.ip),
            "command 'start' couldn't run"
        );
    }

    #[test]
    fn test_set_new_card_config() {
        let card = NetworkCard {
            name: String::from("PCI 3COM 3C905B 10/100 Ethernet"),
            ip: IpAddr::V4(String::from("192.168.1.1")),
        };

        let response_message = set_new_card_config(&card);

        assert_eq!(
            response_message,
            format!("card {} initialized with ip: {:?}", card.name, card.ip),
            "network card couldn't create"
        );
    }
}
