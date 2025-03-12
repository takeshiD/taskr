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
impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)?;
        Ok(())
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
impl std::fmt::Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)?;
        Ok(())
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
impl std::fmt::Display for Description {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Priority {
    High,
    Medium,
    Low,
}
impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::High => write!(f, "High")?,
            Self::Medium => write!(f, "Medium")?,
            Self::Low => write!(f, "Low")?,
        }
        Ok(())
    }
}


#[derive(Debug, Clone)]
pub enum Category {
    Work,
    Personal,
    Study,
    Other,
}
impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Work => write!(f, "Work")?,
            Self::Personal => write!(f, "Personal")?,
            Self::Study => write!(f, "Study")?,
            Self::Other => write!(f, "Other")?,
        }
        Ok(())
    }
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
impl std::fmt::Display for DueDate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = self.value.to_string();
        write!(f, "{text}")?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Status {
    Todo,
    InProgress,
    Completed,
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Todo => write!(f, "Todo")?,
            Self::InProgress => write!(f, "InProgress")?,
            Self::Completed => write!(f, "Completed")?,
        }
        Ok(())
    }
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
