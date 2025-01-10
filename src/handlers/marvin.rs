use serde_json::{Value, from_value};
use crate::models::{tasks::Task, habits::Habit, timers::Timer};

/// This function is called from our main `marvin_webhook` route. We match the `webhook_type` 
/// that Marvin gave us.
pub async fn handle_marvin_webhook_type(webhook_type: &str, data: &Value) -> Result<(), anyhow::Error> {
    match webhook_type {
        // A new Task or Project
        "add" | "addTask" | "addProject" => {
            // Attempt to deserialize it as a Task or Project
            let task: Task = from_value(data.clone())?;
            println!("New Task/Project added: {:?}", task.title);
            // ... do something with your `task`
        }
        // A Habit
        "addHabit" => {
            let habit: Habit = from_value(data.clone())?;
            println!("New Habit: {:?}", habit.title);
            // ... do something ...
        }
        // Timers
        "addTimer" => {
            let timer: Timer = from_value(data.clone())?;
            println!("Timer started: {:?}", timer);
            // ...
        }
        "timerDone" => {
            let timer: Timer = from_value(data.clone())?;
            println!("Timer done: {:?}", timer);
            // ...
        }
        // Mark done
        "markDone" | "markDoneTask" | "markDoneProject" => {
            let task: Task = from_value(data.clone())?;
            println!("Task/Project marked done: {:?}", task.title);
        }
        // Edit a Habit: old doc plus "setter" field
        "editHabit" => {
            // For an edit, we might have a slightly different struct
            // e.g. `WebhookEditPayload<Habit>` if you want to see what changed
            println!("editHabit not yet implemented properly");
        }
        // etc. for all the other possible types...
        other => {
            println!("Unhandled Marvin webhook type: {}", other);
            // You could store the entire payload for debugging
            // or return an error if you want
        }
    }

    Ok(())
}

/// A second example for something else (the "other_webhook" route).
pub async fn handle_other_webhook(payload: &Value) -> Result<(), anyhow::Error> {
    println!("Received 'other' webhook: {}", payload);
    Ok(())
}
