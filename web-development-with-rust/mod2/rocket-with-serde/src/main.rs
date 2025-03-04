use std::sync::Mutex;
use rocket::serde::json::Json; // course issue: this is behind a feature flag

use models::Task;
use rocket::State;

#[macro_use] extern crate rocket;

mod models;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(TaskList { tasks: Mutex::new(Vec::new()) })
        .mount("/tasks", routes![tasks_get, tasks_get_single, task_create, task_update, task_delete])
}

#[get("/")]
fn tasks_get(state: &State<TaskList>) -> Json<Vec<Task>> {
    let tasks = state.tasks.lock().unwrap();
    
    // derive(Clone) is missing in code from course
    Json(tasks.clone())
}

#[get("/<id>")]
fn tasks_get_single(state: &State<TaskList>, id: u64) -> Option<Json<Task>> {
    let tasks = state.tasks.lock().unwrap();
    
    let requested_task = tasks.iter().find(|t| t.id == id).cloned();
    requested_task.map(Json)
}

#[post("/", format = "json", data = "<task>")]
fn task_create(state: &State<TaskList>, task: Json<Task>) -> Json<Task> {
    let mut tasks = state.tasks.lock().unwrap();
    tasks.push(task.0.clone());
    
    task
}

#[put("/<id>", format = "json", data = "<task>")]
fn task_update(state: &State<TaskList>, task: Json<Task>, id: u64) -> Option<Json<Task>> {
    if task.id != id {
        return None;
    }

    let mut tasks = state.tasks.lock().unwrap();
    
    let existing_task = tasks.iter_mut().find(|t| t.id == task.id)?;
    *existing_task = task.0.clone();

    Some(task)
}

#[delete("/<id>")]
fn task_delete(state: &State<TaskList>, id: u64) -> Option<Json<Task>> {
    let mut tasks = state.tasks.lock().unwrap();
    
    let index = tasks.iter().position(|t| t.id == id)?;
    
    let task = tasks.remove(index);
    Some(Json(task))
}

struct TaskList {
    tasks: Mutex<Vec<Task>>
}