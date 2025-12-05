use std::collections::HashMap;
use std::sync::Mutex;

const MAX_CONTEXT_MESSAGES: usize = 5;

#[derive(Debug, Clone)]
pub struct Message {
    pub role: String,  // "user" or "assistant"
    pub content: String,
}

pub struct ConversationContext {
    contexts: Mutex<HashMap<i64, Vec<Message>>>,
}

impl ConversationContext {
    pub fn new() -> Self {
        Self {
            contexts: Mutex::new(HashMap::new()),
        }
    }

    /// Add a message to user's conversation context
    pub fn add_message(&self, user_id: i64, role: &str, content: &str) {
        let mut contexts = self.contexts.lock().unwrap();
        let messages = contexts.entry(user_id).or_insert_with(Vec::new);

        messages.push(Message {
            role: role.to_string(),
            content: content.to_string(),
        });

        // Keep only last N messages
        if messages.len() > MAX_CONTEXT_MESSAGES {
            messages.remove(0);
        }
    }

    /// Get user's conversation context as formatted string
    pub fn get_context(&self, user_id: i64) -> Option<String> {
        let contexts = self.contexts.lock().unwrap();
        contexts.get(&user_id).map(|messages| {
            messages
                .iter()
                .map(|m| format!("{}: {}", m.role, m.content))
                .collect::<Vec<_>>()
                .join("\n")
        })
    }

    /// Clear user's context
    pub fn clear(&self, user_id: i64) {
        let mut contexts = self.contexts.lock().unwrap();
        contexts.remove(&user_id);
    }
}

impl Default for ConversationContext {
    fn default() -> Self {
        Self::new()
    }
}
