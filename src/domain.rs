use anyhow::{Error, Result, anyhow};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Entity
#[derive(Debug, Clone)]
pub struct Task {
    pub id: TaskId,
    pub title: Title,
    pub description: Description,
    pub priority: Priority,
    pub category: Category,
    pub due_date: Option<DueDate>,
    pub status: Status,
}

impl Task {
    pub fn new(
        title: String,
        description: String,
        priority: Priority,
        category: Category,
        due_date: Option<DateTime<Utc>>,
    ) -> Result<Self, Error> {
        Ok(Task {
            id: TaskId::new(),
            title: Title::new(title)?,
            description: Description::new(description)?,
            priority,
            category,
            due_date: due_date.map(DueDate::new),
            status: Status::Todo,
        })
    }
    pub fn mark_completed(self) -> Self {
        Self {
            status: Status::Completed,
            ..self
        }
    }
    pub fn mark_high_priority(self) -> Self {
        Self {
            priority: Priority::High,
            ..self
        }
    }
    pub fn mark_medium_priority(self) -> Self {
        Self {
            priority: Priority::Medium,
            ..self
        }
    }
    pub fn mark_low_priority(self) -> Self {
        Self {
            priority: Priority::Low,
            ..self
        }
    }
}

/// Value Object
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TaskId {
    value: String,
}
impl TaskId {
    fn new() -> Self {
        let id = Uuid::new_v4();
        Self {
            value: id.to_string(),
        }
    }
}
impl From<String> for TaskId {
    fn from(item: String) -> Self {
        TaskId {
            value: item
        }
    }
}

#[derive(Debug, Clone)]
pub struct Title {
    value: String,
}
impl Title {
    fn new(title: String) -> Result<Self, Error> {
        if title.is_empty() {
            return Err(anyhow!("Title cannot be empty."));
        }
        if title.len() > 100 {
            return Err(anyhow!("Title is too long (max 100 characters)"));
        }
        Ok(Self { value: title })
    }
}

#[derive(Debug, Clone)]
pub struct Description {
    value: String,
}
impl Description {
    fn new(description: String) -> Result<Self, Error> {
        if description.len() > 500 {
            return Err(anyhow!("Description is too long (max 500 characters)"));
        }
        Ok(Self { value: description })
    }
}

#[derive(Debug, Clone)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
pub enum Category {
    Work,
    Personal,
    Study,
    Other,
}

#[derive(Debug, Clone)]
pub struct DueDate {
    value: DateTime<Utc>,
}

impl DueDate {
    fn new(due_date: DateTime<Utc>) -> Self {
        Self { value: due_date }
    }
}

#[derive(Debug, Clone)]
pub enum Status {
    Todo,
    InProgress,
    Completed,
}

/// Interface
pub trait TaskRepository {
    fn save(&mut self, task: &Task) -> Result<(), Error>;
    fn find_by_id(&self, id: &TaskId) -> Result<Task, Error>;
    fn find_all(&self) -> Result<Vec<Task>, Error>;
    fn delete(&mut self, id: &TaskId) -> Result<(), Error>;
}

/// Service
#[derive(Debug)]
pub struct TaskDomainService<R>
where
    R: TaskRepository,
{
    repository: R,
}

impl<R> TaskDomainService<R>
where
    R: TaskRepository,
{
    pub fn new(task_repositry: R) -> Self {
        TaskDomainService {
            repository: task_repositry,
        }
    }
    pub fn existed(&self, id: String) -> bool {
        let task_id = TaskId::from(id);
        if let Ok(_) = self.repository.find_by_id(&task_id) {
            true
        } else {
            false
        }
    }
}
