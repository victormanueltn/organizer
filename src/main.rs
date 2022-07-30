use iced::Sandbox;

#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: iced::button::State,
    decrement_button: iced::button::State,
    task_completed: bool,
    //    task_state: Task,
}

struct Task {
    description: String,
    completed: bool,
    state: TaskState,
}

pub enum TaskState {}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    TaskCompleted(bool),
}

impl iced::Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Playing with ice")
    }

    fn view(&mut self) -> iced::Element<Message> {
        iced::Column::new()
            .padding(10)
            .align_items(iced::Alignment::Center)
            .push(
                iced::Button::new(&mut self.increment_button, iced::Text::new("+"))
                    .on_press(Message::IncrementPressed),
            )
            .push(iced::Text::new(self.value.to_string()).size(40))
            .push(
                iced::Button::new(&mut self.decrement_button, iced::Text::new("-"))
                    .on_press(Message::DecrementPressed),
            )
            .push(iced::Checkbox::new(
                self.task_completed,
                String::from(""),
                Message::TaskCompleted,
            ))
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => self.value += 1,
            Message::DecrementPressed => self.value -= 1,
            Message::TaskCompleted(completed) => self.task_completed = completed,
        }
    }
}

fn main() -> iced::Result {
    Counter::run(iced::Settings::default())
}
