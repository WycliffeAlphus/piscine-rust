#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub enum Link = {
    Cons(Worker, Box<Link>),
    None,
}

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment{
            grade:None
        }
    }
    pub fn add_worker(&mut self, role: String, name: String) {
        let wk = Worker{
            role:role,
            name: string,
        }
        let current = std::mem::replace(&mut self.grade, Link::None);
        self.grade = Link::Cons(wk, Box::new(current))
    }
    pub fn remove_worker(&mut self) -> Option<String> {
        match std::mem::replace(&mut self.grade, Link::None) {
            Link::Cons(worker, next_link) => {
                self.grade = *next_link; 
                Some(worker.name)
            }
            Link::None => None,
        }  

    }
    pub fn last_worker(&self) -> Option<(String, String)> {
        match &self.grade {
            Link::Cons(worker, _) => Some((worker.name.clone(), worker.role.clone())),
            Link::None => None,
        }
    }
}

