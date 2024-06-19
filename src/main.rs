use iced::{
    alignment,
    widget::{button, column, container, row, scrollable, text, text_input, Column},
    Element, Length, Padding, Sandbox, Settings,
};
struct TodoList {
    todo_list_items: Vec<String>,
    input_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputValue(String),
    Submitted,
    DeleteItem(usize),
}

impl Sandbox for TodoList {
    type Message = Message;

    /* Initialize your app */
    fn new() -> TodoList {
        Self {
            todo_list_items: vec!["Eggs".to_owned(), "Milk".to_owned(), "Flour".to_owned()],
            input_value: String::default(),
        }
    }

    /**
    	* The title of the window. It will show up on the top of your application window.
    	*/
    fn title(&self) -> String {
        String::from("Todo List")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::InputValue(value) => self.input_value = value,
            Message::Submitted => {
                if !self.input_value.is_empty() {
                    self.todo_list_items.push(self.input_value.clone());
                    self.input_value = String::default(); // Clear the input value
                }
            }
            Message::DeleteItem(item) => {
                self.todo_list_items.remove(item);
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        container(
            column!(
                items_list_view(&self.todo_list_items),
                row!(
                    text_input("Input todo list item", &self.input_value)
                        .on_input(|value| Message::InputValue(value))
                        .on_submit(Message::Submitted),
                    button("Submit").on_press(Message::Submitted)
                )
                .spacing(30)
                .padding(Padding::from(30))
            )
            .align_items(iced::Alignment::Center),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Light
    }
}

fn items_list_view(items: &Vec<String>) -> Element<'static, Message> {
    let mut column = Column::new()
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .width(Length::Fill);

    for (index, value) in items.into_iter().enumerate() {
        column = column.push(todo_list_item(index, value));
    }

    scrollable(container(column))
        .height(250.0)
        .width(300)
        .into()
}

fn todo_list_item(index: usize, value: &str) -> Element<'static, Message> {
    row!(
        text(value),
        button("Delete").on_press(Message::DeleteItem(index))
    )
    .align_items(iced::Alignment::Center)
    .spacing(30)
    .into()
}

fn main() -> iced::Result {
    TodoList::run(Settings::default())
}
