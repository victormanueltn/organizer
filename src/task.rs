#[derive(Default)]
pub struct Task {
    task_completed: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    TaskCompleted(bool),
}

impl iced::Sandbox for Task {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Task")
    }

    fn view(&mut self) -> iced::Element<Message> {
        iced::Column::new()
            .padding(10)
            .align_items(iced::Alignment::Center)
            .push(iced::Checkbox::new(
                self.task_completed,
                String::from(""),
                Message::TaskCompleted,
            ))
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TaskCompleted(completed) => self.task_completed = completed,
        }
    }
}
