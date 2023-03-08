use druid::{Data, Lens};
use im::Vector;

#[derive(Clone, Data, Lens, Default, Debug)]
pub struct TodoState {
    pub todos: Vector<TodoItem>,
    pub new_text: String,
}

#[derive(Clone, Data, Lens, Default, Debug, PartialEq, Eq)]
pub struct TodoItem {
    pub checked: bool,
    pub text: String,
}
