use crate::configuration_repository::ConfigurationRepository;
use crate::time_rs::TimeRs;
use iced::{
    alignment, button, executor, text_input, time, Alignment, Button, Column,
    Command, Container, Element, Length, Row, Subscription, Text, TextInput,
};
use ini::Ini;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Seek};
use std::process::exit;
use std::time::{Duration, Instant};

const CONFIG_PATH: &str = "time-rs.conf";
const TIME_RESOLUTION: u64 = 1; // in seconds

pub struct Application {
    configuration: ConfigurationRepository,
    time_rs: TimeRs,
    state: State,
    ui: Ui,
}

pub struct Ui {
    start_stop: button::State,
    reset: button::State,
    record: button::State,
    save: button::State,
    cancel: button::State,
    settings: button::State,
    quit: button::State,
    task_input_state: text_input::State,
    task_input_value: String,
    records_file_path_input_state: text_input::State,
    records_file_path_input_value: String,
}

enum State {
    Idle,
    Running,
    Finished,
    Settings,
}

#[derive(Debug, Clone)]
pub enum Message {
    StartStop,
    Reset,
    Record,
    Save,
    Cancel,
    Settings,
    ChangeSettings,
    Quit,
    Tick(Instant),
    TaskInputChanged(String),
    RecordsFilePathInputChanged(String),
}

impl iced::Application for Application {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Application, Command<Message>) {
        let mut config_repo = create_config_repository(CONFIG_PATH);
        let output_file =
            open_output_file(&config_repo.get("records_file_path"));
        let time_rs = TimeRs::new(output_file);
        let time_rs = Application {
            configuration: config_repo,
            time_rs,
            state: State::Idle,
            ui: Ui::new(),
        };

        let command = Command::none();

        (time_rs, command)
    }

    fn title(&self) -> String {
        String::from("Time-rs")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::StartStop => match self.state {
                State::Idle => {
                    self.time_rs.start();
                    self.state = State::Running;
                }
                State::Running => {
                    self.time_rs.stop();
                    self.state = State::Idle;
                }
                State::Finished => {}
                State::Settings => {}
            },
            Message::Reset => {
                self.time_rs.reset();
                self.state = State::Idle;
            }
            Message::Record => {
                self.time_rs.stop();
                self.state = State::Finished;
            }
            Message::Tick(_now) => {
                if let State::Running = self.state {
                    self.time_rs.advance(TIME_RESOLUTION)
                }
            }
            Message::TaskInputChanged(value) => {
                self.ui.task_input_value = value
            }
            Message::RecordsFilePathInputChanged(value) => {
                self.ui.records_file_path_input_value = value
            }
            Message::Save => {
                self.time_rs.write(self.ui.task_input_value.clone());
                self.time_rs.reset();
                self.state = State::Idle;
            }
            Message::Cancel => self.state = State::Idle,
            Message::Settings => {
                self.ui.records_file_path_input_value =
                    self.configuration.get("records_file_path");
                self.state = State::Settings;
            }
            Message::ChangeSettings => {
                self.configuration.set(
                    "records_file_path",
                    self.ui.records_file_path_input_value.to_owned(),
                );
                self.configuration.write();

                self.state = State::Idle;
            }
            Message::Quit => exit(0),
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.state {
            State::Idle => Subscription::none(),
            State::Running => {
                time::every(Duration::from_secs(TIME_RESOLUTION))
                    .map(Message::Tick)
            }
            State::Finished => Subscription::none(),
            State::Settings => Subscription::none(),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = match self.state {
            State::Idle | State::Running => {
                let total_seconds = self.time_rs.get_length_for_human();
                self.ui.build_stopwatch_view(&self.state, total_seconds)
            }
            State::Finished => self.ui.build_record_view(),
            State::Settings => self.ui.build_settings_view(),
        };
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

impl Ui {
    fn new() -> Ui {
        Ui {
            start_stop: button::State::new(),
            reset: button::State::new(),
            record: button::State::new(),
            save: button::State::new(),
            cancel: button::State::new(),
            settings: button::State::new(),
            quit: button::State::new(),
            task_input_state: text_input::State::new(),
            task_input_value: String::new(),
            records_file_path_input_state: text_input::State::new(),
            records_file_path_input_value: String::new(),
        }
    }

    fn build_stopwatch_view(
        &mut self,
        state: &State,
        length: String,
    ) -> Column<Message> {
        let length = Text::new(length).size(40);

        let start_stop_button = {
            let (label, color) = match state {
                State::Idle => ("Start", style::Button::Primary),
                State::Running => ("Stop", style::Button::Destructive),
                _ => ("", style::Button::Primary),
            };

            Button::new(
                &mut self.start_stop,
                Text::new(label)
                    .horizontal_alignment(alignment::Horizontal::Center),
            )
            .min_width(80)
            .padding(10)
            .style(color)
            .on_press(Message::StartStop)
        };

        let reset_button = Button::new(
            &mut self.reset,
            Text::new("Reset")
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .min_width(80)
        .padding(10)
        .style(style::Button::Secondary)
        .on_press(Message::Reset);

        let record_button = Button::new(
            &mut self.settings,
            Text::new("Record")
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .min_width(80)
        .padding(10)
        .style(style::Button::Secondary)
        .on_press(Message::Record);

        let settings_button = Button::new(
            &mut self.record,
            Text::new("Settings")
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .min_width(80)
        .padding(10)
        .style(style::Button::Secondary)
        .on_press(Message::Settings);

        let quit_button = Button::new(
            &mut self.quit,
            Text::new("Quit")
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .min_width(80)
        .padding(10)
        .style(style::Button::Destructive)
        .on_press(Message::Quit);

        let controls = Row::new()
            .spacing(20)
            .push(start_stop_button)
            .push(reset_button);

        Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(length)
            .push(controls)
            .push(record_button)
            .push(settings_button)
            .push(quit_button)
    }

    fn build_record_view(&mut self) -> Column<Message> {
        let question = Text::new("What did you do?").size(40);
        let answer = TextInput::new(
            &mut self.task_input_state,
            "",
            &self.task_input_value,
            Message::TaskInputChanged,
        )
        .width(Length::from(500))
        .padding(10)
        .size(18)
        .on_submit(Message::Save);

        let save_button = Button::new(
            &mut self.save,
            Text::new("Save")
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .min_width(80)
        .padding(10)
        .style(style::Button::Primary)
        .on_press(Message::Save);

        let cancel_button = Button::new(
            &mut self.cancel,
            Text::new("Cancel")
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .min_width(80)
        .padding(10)
        .style(style::Button::Secondary)
        .on_press(Message::Cancel);

        let controls =
            Row::new().spacing(20).push(save_button).push(cancel_button);

        Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(question)
            .push(answer)
            .push(controls)
    }

    fn build_settings_view(&mut self) -> Column<Message> {
        let title = Text::new("Settings").size(40);
        let records_file_path_label = Text::new("Records file path:");
        let records_file_path_input = TextInput::new(
            &mut self.records_file_path_input_state,
            "",
            &self.records_file_path_input_value,
            Message::RecordsFilePathInputChanged,
        )
        .width(Length::from(200))
        .padding(10)
        .size(18)
        .on_submit(Message::ChangeSettings);

        let save_button = Button::new(
            &mut self.save,
            Text::new("Save")
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .min_width(80)
        .padding(10)
        .style(style::Button::Primary)
        .on_press(Message::ChangeSettings);

        let cancel_button = Button::new(
            &mut self.cancel,
            Text::new("Cancel")
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .min_width(80)
        .padding(10)
        .style(style::Button::Secondary)
        .on_press(Message::Cancel);

        let records_file_path_row = Row::new()
            .spacing(20)
            .push(records_file_path_label)
            .push(records_file_path_input);
        let controls =
            Row::new().spacing(20).push(save_button).push(cancel_button);

        Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(title)
            .push(records_file_path_row)
            .push(controls)
    }
}

fn create_config_repository(file_path: &str) -> ConfigurationRepository {
    let config_file = open_config_file(file_path);
    ConfigurationRepository::new(config_file)
}

fn create_config_file(file_path: &str) -> File {
    let mut config_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();
    let mut config = Ini::new();
    config
        .with_section(Some("default"))
        .set("records_file_path", "work_entries.csv");
    config.write_to(&mut config_file).unwrap();
    config_file.rewind().unwrap();

    config_file
}

fn open_config_file(file_path: &str) -> File {
    let config_file =
        OpenOptions::new().read(true).write(true).open(CONFIG_PATH);
    match config_file {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                create_config_file(file_path)
            } else {
                panic!("{}", e)
            }
        }
    }
}

fn open_output_file(file_path: &str) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap()
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
        Secondary,
        Destructive,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                    Button::Destructive => Color::from_rgb(0.8, 0.2, 0.2),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }
}
