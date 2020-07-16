use iced::{button, Align, Button, Element, Row, Sandbox, Settings, Text};

pub fn main() {
    App::run(Settings::default())
}

#[derive(Default)]
struct App {
    host_button: button::State,
    client_button: button::State,
    menu: Menu,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    HostPressed,
    ClientPressed,
}

enum Menu {
    Main,
    Host,
    Client,
}

impl Default for Menu {
    fn default() -> Self {
        Menu::Main
    }
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Test - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::HostPressed => {
                self.menu = Menu::Host;
            }
            Message::ClientPressed => {
                self.menu = Menu::Client;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        match self.menu {
            Menu::Client => Row::new()
                .padding(20)
                .align_items(Align::Center)
                .push(Text::new("client menu placeholder".to_string()).size(50))
                .into(),
            Menu::Host => Row::new()
                .padding(20)
                .align_items(Align::Center)
                .push(Text::new("host menu placeholder".to_string()).size(50))
                .into(),
            Menu::Main => Row::new()
                .padding(20)
                .align_items(Align::Center)
                .push(
                    Button::new(&mut self.host_button, Text::new("Host"))
                        .on_press(Message::HostPressed),
                )
                .push(
                    Button::new(&mut self.client_button, Text::new("Client"))
                        .on_press(Message::ClientPressed),
                )
                .into(),
        }
    }
}
