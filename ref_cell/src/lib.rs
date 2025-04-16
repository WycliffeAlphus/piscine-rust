
use std::collections::HashMap;
pub use std::cell::RefCell;
pub use std::rc::Rc;

mod messenger;
pub use messenger::{Logger, Tracker};

pub struct Worker {
    pub track_value: Rc<u64>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(value: u64) -> Self {
        Self {
            track_value: Rc::new(value),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
}

impl Logger for Worker {
    fn warning(&self, msg: &str) {
        let formatted = format!("Warning: {}", msg);
        self.mapped_messages
            .borrow_mut()
            .insert("Warning".to_string(), formatted.clone());
        self.all_messages.borrow_mut().push(formatted);
    }

    fn info(&self, msg: &str) {
        let formatted = format!("Info: {}", msg);
        self.mapped_messages
            .borrow_mut()
            .insert("Info".to_string(), formatted.clone());
        self.all_messages.borrow_mut().push(formatted);
    }

    fn error(&self, msg: &str) {
        let formatted = format!("Error: {}", msg);
        self.mapped_messages
            .borrow_mut()
            .insert("Error".to_string(), formatted.clone());
        self.all_messages.borrow_mut().push(formatted);
    }
}
