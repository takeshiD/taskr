use crate::domain::{Category, DueDate, Priority, Task, TaskDomainService, TaskId, TaskRepository};
use anyhow::{Error, Result};
use chrono::{DateTime, DurationRound, Utc};

struct TaskDTO {}

pub struct CreateTaskCommand {
    title: String,
    description: String,
    priority: Priority,
    category: Category,
    due_date: Option<DateTime<Utc>>,
}
pub struct FindTaskCommand {}
pub struct UpdateTaskCommand {}
pub struct DeleteTaskCommand {}

pub struct CreateTaskApplicationService<R>
where
    R: TaskRepository,
{
    repository: R,
}

impl<R> CreateTaskApplicationService<R>
where
    R: TaskRepository,
{
    pub fn new(task_repository: R) -> Self {
        Self {
            repository: task_repository,
        }
    }
    pub fn execute(&mut self, command: CreateTaskCommand) -> Result<(), Error> {
        let task = Task::new(
            command.title,
            command.description,
            command.priority,
            command.category,
            command.due_date,
        )?;
        self.repository.save(&task)
    }
}

pub struct FindTaskApplicationService<R>
where
    R: TaskRepository,
{
    repository: R,
}

impl<R> FindTaskApplicationService<R>
where
    R: TaskRepository,
{
    pub fn new(task_repository: R) -> Self {
        Self {
            repository: task_repository,
        }
    }
    pub fn execute(&mut self, command: CreateTaskCommand) -> Result<(), Error> {
        let task = Task::new(
            command.title,
            command.description,
            command.priority,
            command.category,
            command.due_date,
        )?;
        self.repository.save(&task)
    }
}
