pub struct Message {
    content: String,
    #[allow(dead_code)]
    user: String,
}

impl Message {
    pub fn new(content: &str, user: &str) -> Self {
        Self {
            content: content.to_string(),
            user: user.to_string(),
        }
    }

    pub fn send_ms(&self) -> Option<&str> {
        if self.content.is_empty() || self.content.contains("stupid") {
            None
        } else {
            Some(&self.content)
        }
    }
}

pub fn check_ms<'a>(message: &'a str) -> Result<&'a str, &'static str> {
    if message.is_empty() || message.contains("stupid") {
        Err("ERROR: illegal")
    } else {
        Ok(message)
    }
}
