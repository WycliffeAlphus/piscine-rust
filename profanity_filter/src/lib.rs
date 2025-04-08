struct Message {
    content: String,
    user: String,
}

impl Message {
    fn new(content: &str, user: &str) -> Self {
        Self {
            content: content.to_string(),
            user: user.to_string(),
        }
    }

    fn send_ms(&self) -> Option<&str> {
        if self.content.is_empty() || self.content.contains("stupid") {
            None
        } else {
            Some(&self.content)
        }
    }

    pub fn check_ms(message: &str) -> Result<&str, &str> {
        let msg = Messagge::new(message, "guest");
        match msg.send_ms() {
            Some(content) => Ok(content),
            None => Err("ERROR:illegal"),
        }
    }
}
