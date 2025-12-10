use std::collections::HashMap;
use std::sync::Mutex;

const MAX_CONTEXT_MESSAGES: usize = 5;

#[derive(Debug, Clone)]
pub struct Message {
    pub role: String,  // "user" or "assistant"
    pub content: String,
}

/// Proposed edit for confirmation
#[derive(Debug, Clone)]
pub struct ProposedEdit {
    pub task_id: i64,
    pub old_title: String,
    pub new_title: String,
    pub new_due_at: Option<String>,
}

/// Pending edit state for a user
#[derive(Debug, Clone)]
pub enum PendingEdit {
    Title(i64),                    // task_id - waiting for edit instruction
    ConfirmEdit(ProposedEdit),     // waiting for user confirmation
    Reminder(i64),                 // task_id - waiting for custom reminder time
    Timezone,                      // waiting for city name for timezone
    Support,                       // waiting for support message
}

pub struct ConversationContext {
    contexts: Mutex<HashMap<i64, Vec<Message>>>,
    pending_edits: Mutex<HashMap<i64, PendingEdit>>,  // user_id -> pending edit
}

impl ConversationContext {
    pub fn new() -> Self {
        Self {
            contexts: Mutex::new(HashMap::new()),
            pending_edits: Mutex::new(HashMap::new()),
        }
    }

    /// Set pending edit for user
    pub fn set_pending_edit(&self, user_id: i64, edit: PendingEdit) {
        let mut edits = self.pending_edits.lock().unwrap();
        edits.insert(user_id, edit);
    }

    /// Get and clear pending edit for user
    pub fn take_pending_edit(&self, user_id: i64) -> Option<PendingEdit> {
        let mut edits = self.pending_edits.lock().unwrap();
        edits.remove(&user_id)
    }

    /// Check if user has pending edit
    pub fn has_pending_edit(&self, user_id: i64) -> bool {
        let edits = self.pending_edits.lock().unwrap();
        edits.contains_key(&user_id)
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
