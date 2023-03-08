use std::{fs, path::Path};

use directories::BaseDirs;
use druid::Widget;
use serde::{Deserialize, Serialize};

use crate::data::{TodoItem, TodoState};

pub struct Saver;

impl Widget<TodoState> for Saver {
    fn event(
        &mut self,
        _ctx: &mut druid::EventCtx,
        _event: &druid::Event,
        _data: &mut TodoState,
        _env: &druid::Env,
    ) {
        // todo!()
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &TodoState,
        _env: &druid::Env,
    ) {
        // todo!()
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &TodoState,
        data: &TodoState,
        env: &druid::Env,
    ) {
        if data.todos != old_data.todos {
            if let Some(base_dirs) = BaseDirs::new() {
                let config = format!(
                    "{}/{}",
                    base_dirs.config_dir().to_str().unwrap(),
                    "MyTodo.json"
                );
                let config_path = Path::new(&config);
                let tasks = TaskData {
                    tasks: data.todos.clone().into_iter().collect(),
                };
                fs::write(config_path, serde_json::to_string(&tasks).unwrap())
                    .expect("Config path missing");
            }
        }
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        _bc: &druid::BoxConstraints,
        _data: &TodoState,
        _env: &druid::Env,
    ) -> druid::Size {
        druid::Size {
            width: 0.,
            height: 0.,
        }
    }

    fn paint(&mut self, _ctx: &mut druid::PaintCtx, _data: &TodoState, _env: &druid::Env) {
        // todo!()
    }
}

#[derive(Serialize, Deserialize)]
pub struct TaskData {
    pub tasks: Vec<TodoItem>,
}

// pub fn read_stored() ->  {
pub fn read_stored() -> TaskData {
    if let Some(base_dirs) = BaseDirs::new() {
        let config = format!(
            "{}/{}",
            base_dirs.config_dir().to_str().unwrap(),
            "MyTodo.json"
        );
        let config_path = Path::new(&config);
        let data = match fs::read_to_string(config_path) {
            Ok(a) => a,
            Err(_) => return TaskData { tasks: Vec::new() },
        };
        match serde_json::from_str(&data) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("The saved data is corrupted. Error: {e:?}");
                return TaskData { tasks: Vec::new() };
            }
        }
    } else {
        return TaskData { tasks: Vec::new() };
    }
}
