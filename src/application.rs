use crate::domain::{Category, DueDate, Priority, Task, TaskDomainService, TaskId, TaskRepository};
use anyhow::{Error, Result, anyhow};
use chrono::{DateTime, DurationRound, Utc};

pub struct TaskDTO {
    pub title: String,
    pub description: String,
    pub priority: String,
    pub category: String,
    pub due_date: String,
    pub status: String,
}
impl TaskDTO {
    fn new(
        title: String,
        description: String,
        priority: String,
        category: String,
        due_date: String,
        status: String,
    ) -> Self {
        Self {
            title,
            description,
            priority,
            category,
            due_date,
            status,
        }
    }
}

pub struct CreateTaskCommand {
    title: String,
    description: String,
    priority: Priority,
    category: Category,
    due_date: Option<DateTime<Utc>>,
}
pub struct FindTaskCommand {
    id: TaskId,
}
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
        if let Ok(t) = self.repository.find_by_id(&task.id) {
            Err(anyhow!("Already TaskId: {t:#?}"))
        } else {
            self.repository.save(&task)?;
            Ok(())
        }
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
    pub fn execute(&mut self, command: FindTaskCommand) -> Result<TaskDTO, Error> {
        if let Ok(task) = self.repository.find_by_id(&command.id) {
            Ok(TaskDTO::new(
                task.title.to_string(),
                task.description.to_string(),
                task.priority.to_string(),
                task.category.to_string(),
                task.due_date.expect("Failed DueDate").to_string(),
                task.status.to_string()
            ))
        } else {
            Err(anyhow!("Not found TaskId: {:#?}", command.id))
        }
    }
}
