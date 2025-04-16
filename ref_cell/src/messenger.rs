pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}
#[allow(dead_code)]
pub struct Tracker<'a> {
    logger: &'a dyn Logger,
    value: usize,
    max: usize,
}
#[allow(dead_code)]
impl<'a> Tracker<'a> {
    pub fn new(logger: &'a dyn Logger, max: usize) -> Self {
        Tracker {
            logger,
            value: 0,
            max,
        }
    }
    
    pub fn set_value<T>(&self, tracked: &std::rc::Rc<T>) {
        let count = std::rc::Rc::strong_count(tracked);
        let percent = ((count as f64 / self.max as f64) * 100.0).floor();
    
        if percent >= 100.0 {
            self.logger.error("you are over your quota!");
        } else if percent >= 70.0 {
            let msg = format!(
                "you have used up over {:.0}% of your quota! Proceeds with precaution",
                percent
            );
            self.logger.warning(&msg);
        }
    }
    
    pub fn peek<T>(&self, tracked: &std::rc::Rc<T>) {
        let count = std::rc::Rc::strong_count(tracked);
        let percent = ((count as f64 / self.max as f64) * 100.0).floor();
        let msg = format!("Info: you are using up to {:.0}% of your quota", percent);
        self.logger.info(&msg);
    }
    
}
