pub mod tasks;
pub mod labels;
pub mod timers;
pub mod habits;
pub mod tracking;
pub mod calendars;
pub mod rewards;
pub mod profiles;
pub mod reminders;
pub mod webhooks;

// Optional: Re-export common structs for easy access
pub use tasks::{Task, Subtask, ReminderInfo, ProjectOrCategory, RecurringTask, SavedItem, Goal};
pub use labels::{Label, LabelGroup};
pub use timers::{Timer, TomatoTimer};
pub use habits::{Habit, HabitRecord};
pub use tracking::Tracker;
pub use calendars::{Calendar, Event, EventException, TimeBlock, TimeBlockException};
pub use rewards::Reward;
pub use profiles::{Profile, ProfileItem};
pub use reminders::{Reminder, SetterInfo};
pub use webhooks::{WebhookEditPayload, WebhookRecordHabitPayload};
