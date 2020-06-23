use iced::{Application, Command, Element, TextInput, Text, text_input, button, Button, Settings, Length, HorizontalAlignment, Column, Scrollable, scrollable, Container, Row, Align, Checkbox, Space, Font};
use strum::IntoEnumIterator;
use crate::types::{Operations, u64x1};
use crate::calculations::*;
use crate::utils::*;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::fmt::Formatter;
use uuid::Uuid;

pub fn run() {
    App::run(Settings::default());
}

#[cfg_attr(feature = "debug", derive(Debug))]
struct App {
    mm1: String,
    mm1_state: text_input::State,
    mm2: String,
    mm2_state: text_input::State,
    operation: Operations,
    result: Result<Vec<CalculationResult>, Vec<String>>,
    calculate_button: button::State,
    scroll_state: scrollable::State,
}

impl Default for App {
    fn default() -> Self {
        Self {
            mm1: "".to_string(),
            mm1_state: Default::default(),
            mm2: "".to_string(),
            mm2_state: Default::default(),
            operation: Default::default(),
            result: Ok(Vec::new()),
            calculate_button: Default::default(),
            scroll_state: Default::default(),
        }
    }
}

#[cfg_attr(feature = "debug", derive(Debug))]
struct CalculationResult {
    identifier: Uuid,
    label: Option<String>,
    content: String,
    button_state: button::State,
    copied: bool,
}

impl Default for CalculationResult {
    fn default() -> Self {
        Self {
            identifier: Uuid::new_v4(),
            label: None,
            content: "".to_string(),
            button_state: Default::default(),
            copied: false,
        }
    }
}

impl CalculationResult {
    fn new(label: Option<String>, content: String) -> Self {
        Self { label, content, ..Default::default() }
    }
}

impl CalculationResult {
    fn view(&mut self) -> Element<Message> {
        Row::new()
            .push(
                Container::new(
                    Text::new(&self.to_string())
                        .size(25)
                )
                    .padding(10)
            )
            .push(
                Button::new(
                    &mut self.button_state,
                    Row::new()
                        .push(
                            Text::new(if self.copied { "\u{eed8} " } else { "" })
                                .font(ICONS)
                                .size(25)
                        )
                        .push(
                            Text::new(if self.copied { "Zkopírováno" } else { "Kopírovat" })
                                .size(25)
                        ),
                )
                    .style(style::Button::Copy(self.copied))
                    .padding(10)
                    .on_press(Message::CopyToClipBoard(self.identifier.clone()))
            )
            .spacing(20)
            .into()
    }
}

impl std::fmt::Display for CalculationResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.label {
            None => f.write_str(&self.content),
            Some(label) => f.write_fmt(format_args!("{}: {}", label, self.content)),
        }
    }
}

impl Default for Operations {
    fn default() -> Self {
        Operations::PADDB
    }
}

#[derive(Clone, Debug)]
enum Message {
    InputMM1Changed(String),
    InputMM2Changed(String),
    OperationChanged(Operations),
    Calculate,
    CopyToClipBoard(Uuid),
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            App::default(),
            Command::perform(async {}, |_| { Message::Calculate })
        )
    }

    fn title(&self) -> String {
        "Kalkulačka!".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputMM1Changed(value) => {
                self.mm1 = cleanup_hex_string(value);
                self.mm1.truncate(16);
            }
            Message::InputMM2Changed(value) => {
                self.mm2 = cleanup_hex_string(value);
                self.mm2.truncate(16);
            }
            Message::Calculate => {
                let mut errors = Vec::new();

                if self.mm1.is_empty() {
                    errors.push("Register MM1 nesmí být prázdný.".to_string());
                }
                if self.operation.requires_mm2() && self.mm2.is_empty() {
                    errors.push(format!("Register MM2 nesmí být prázdný při operaci \"{}.\"", self.operation));
                }
                if errors.is_empty() {
                    self.result = Ok(
                        match calculate(
                            u64x1::new(parse_hex(&self.mm1).unwrap()),
                            if self.mm2.is_empty() {
                                u64x1::new(0)
                            } else {
                                u64x1::new(parse_hex(&self.mm2).unwrap())
                            },
                            &self.operation,
                        ) {
                            EitherRegisters::OneRegister(x) => {
                                vec![CalculationResult::new(None, x.to_string())]
                            }
                            EitherRegisters::TwoRegisters(x, y) => {
                                let mut result = Vec::new();
                                result.push(CalculationResult::new(Some("MM1".to_string()), x.to_string()));
                                if !self.mm2.is_empty() {
                                    result.push(CalculationResult::new(Some("MM2".to_string()), y.to_string()));
                                }
                                result
                            }
                        }
                    );
                } else {
                    self.result = Err(errors);
                }
            }
            Message::OperationChanged(operation) => {
                self.operation = operation;
            }
            Message::CopyToClipBoard(value) => {
                if let Ok(results) = &mut self.result {
                    let res = results.iter_mut().filter_map(|result| {
                        let eq = result.identifier == value;
                        result.copied = eq;
                        if eq {
                            Some(result.content.clone())
                        } else {
                            None
                        }
                    }).collect::<String>();

                    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
                    clipboard.set_contents(res);
                }
            }
        };

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let title = Text::new("Kalkulačka!")
            .width(Length::Fill)
            .size(100)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(HorizontalAlignment::Center);

        let operations: Column<Message> = Operations::iter().fold(
            Column::new()
                .push(
                    Text::new("Operace")
                        .size(35)
                ),
            |operations, operation| {
                operations.push(
                    Checkbox::new(
                        self.operation == operation,
                        operation.to_string(),
                        move |_checked| {
                            Message::OperationChanged(operation.clone())
                        },
                    ).text_size(25)
                )
            },
        );

        let mm1 = Row::new()
            .spacing(20)
            .align_items(Align::Center)
            .push(
                Text::new("MM1")
                    .size(30)
            )
            .push(
                TextInput::new(
                    &mut self.mm1_state,
                    "Chtělo by to zadat hexadecimální číslo.",
                    &self.mm1,
                    Message::InputMM1Changed,
                )
                    .size(30)
                    .padding(15)
            );

        let mm2 = Row::new()
            .spacing(20)
            .align_items(Align::Center)
            .push(
                Text::new("MM2")
                    .size(30)
            )
            .push(
                TextInput::new(
                    &mut self.mm2_state,
                    "Chtělo by to zadat hexadecimální číslo.",
                    &self.mm2,
                    Message::InputMM2Changed,
                )
                    .size(30)
                    .padding(15)
            );

        let calculate = Button::new(
            &mut self.calculate_button,
            Text::new("Vypočítat")
                .size(25),
        )
            .style(style::Button::Default)
            .padding(10)
            .on_press(Message::Calculate);

        let result = Column::new()
            .push(
                Text::new("Výsledek")
                    .size(40)
            )
            .push::<Element<Message>>(
                match &mut self.result {
                    Ok(results) =>
                        results.iter_mut()
                            .fold(
                                Column::new()
                                    .spacing(10),
                                |column, result|
                                    column.push(result.view()
                                    ),
                            ).into(),
                    Err(errors) => Text::new(errors.join("\n")).size(25).into(),
                }
            )
            .spacing(10)
            .width(Length::Fill)
            .align_items(Align::Center);

        let content = Column::new()
            .max_width(800)
            .spacing(20)
            .push(title)
            .push(
                Row::new()
                    .push(
                        Column::new()
                            .width(Length::FillPortion(3))
                            .spacing(20)
                            .push(mm1)
                            .push(mm2)
                            .push(Container::new(calculate).width(Length::Fill).center_x())
                            .push(Space::new(Length::Fill, Length::Units(30)))
                            .push(result)
                    )
                    .push(
                        operations
                            .width(Length::FillPortion(1))
                            .spacing(8)
                    )
                    .width(Length::Fill)
                    .spacing(20)
            );

        Scrollable::new(&mut self.scroll_state)
            .padding(40)
            .push(
                Container::new(content).width(Length::Fill).center_x(),
            )
            .into()
    }
}

const ICONS: Font = Font::External {
    name: "IcoFont",
    bytes: include_bytes!("../../fonts/icofont.ttf"),
};

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Default,
        Copy(bool),
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            match self {
                Button::Default => {
                    button::Style {
                        background: Some(Background::Color(
                            Color::from_rgb(0.2, 0.2, 0.7)
                        )),
                        border_radius: 10,
                        text_color: Color::WHITE,
                        ..button::Style::default()
                    }
                }
                Button::Copy(copied) => {
                    button::Style {
                        background: Some(Background::Color(
                            if *copied {
                                Color::from_rgb8(0, 200, 20)
                            } else {
                                Color::from_rgb8(106, 4, 29)
                            }
                        )),
                        border_radius: 10,
                        text_color: Color::WHITE,
                        ..button::Style::default()
                    }
                }
            }
        }

        fn hovered(&self) -> button::Style {
            let active = self.active();

            button::Style {
                shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
                ..active
            }
        }
    }
}