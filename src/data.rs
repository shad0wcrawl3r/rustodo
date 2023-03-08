use druid::{Data, Lens};
use im::Vector;
use serde::{Deserialize, Serialize};

#[derive(Clone, Data, Lens, Default)]
pub struct TodoState {
    pub todos: Vector<TodoItem>,
    pub new_text: String,
}

#[derive(Clone, Data, Lens, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoItem {
    pub checked: bool,
    pub text: String,
}
