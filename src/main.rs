use iced::widget::{container, text};
use iced::{alignment, Element, Length, Sandbox, Settings};

fn main() -> iced::Result {
    SampleBox::run(Settings::default())
}

struct SampleBox {}

#[derive(Debug, Clone)]
enum Message {}

impl Sandbox for SampleBox {
    type Message = Message;

    fn new() -> SampleBox {
        Self {}
    }

    fn title(&self) -> String {
        String::from("SampleBox")
    }

    fn update(&mut self, _message: Self::Message) {
        // Update the state of your app
    }

    fn view(&self) -> Element<'_, Self::Message> {
        container(text("Hello World"))
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::CatppuccinMocha
    }
}
