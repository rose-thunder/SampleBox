use iced::widget::{column, container, text, Column};
use iced::{alignment, Element, Length, Sandbox, Settings};

fn main() -> iced::Result {
    SampleBox::run(Settings::default())
}

struct SampleBox {
    samples: Vec<String>,
}

#[derive(Debug, Clone)]
enum Message {}

impl Sandbox for SampleBox {
    type Message = Message;

    fn new() -> SampleBox {
        Self {
            samples: vec!["Kick".to_owned(), "Clap".to_owned(), "Snare".to_owned()],
        }
    }

    fn title(&self) -> String {
        String::from("SampleBox")
    }

    fn update(&mut self, _message: Self::Message) {
        // Update the state of your app
    }

    fn view(&self) -> Element<'_, Self::Message> {
        container(samples_list_view(&self.samples))
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

fn samples_list_view(samples: &Vec<String>) -> Element<'static, Message> {
    let mut column = Column::new()
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .width(Length::Fill);

    for value in samples {
        column = column.push(text(value))
    }

    container(column).height(250.0).width(300).into()
}
