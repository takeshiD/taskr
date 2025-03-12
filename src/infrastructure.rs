use crate::domain::{Task, TaskId, TaskRepository};
use std::collections::HashMap;
use anyhow::{anyhow, Result, Error};

/// repositry implementation
#[derive(Debug)]
pub struct TaskRepositoryInMemory {
    repo: HashMap<TaskId, Task>
}

impl TaskRepositoryInMemory {
    pub fn new() -> Self {
        TaskRepositoryInMemory {
            repo: HashMap::new()
        }
    }
}

impl TaskRepository for TaskRepositoryInMemory {
    fn save(&mut self, task: &Task) -> Result<(), Error> {
        if let Some(_) = self.repo.insert(task.id.clone(), task.clone()) {
            Ok(())
        } else {
            Err(anyhow!("Failed save task: '{task:#?}'"))
        }
    }
    fn find_by_id(&self, id: &TaskId) -> Result<Task, Error> {
        if let Some(task) = self.repo.get(id) {
            Ok(task.clone())
        } else {
            Err(anyhow!("Not found TaskId='{id:#?}'"))
        }
    }
    fn find_all(&self) -> Result<Vec<Task>, Error> {
        let tasks: Vec<Task> = self.repo.iter()
            .map(|val| val.1.clone())
            .collect();
        Ok(tasks)
    }
    fn delete(&mut self, id: &TaskId) -> Result<(), Error> {
        if let Some(_) = self.repo.remove(id) {
            Ok(())
        } else {
            Err(anyhow!("Not found TaskId='{id:#?}'"))
        }
    }
}
