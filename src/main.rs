use taskr::domain::TaskDomainService;
use taskr::infrastructure::TaskRepositoryInMemory;

fn main() {
    let repo = TaskRepositoryInMemory::new();
    let service = TaskDomainService::new(repo);
    println!("{service:#?}");
}
