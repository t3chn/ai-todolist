use reqwest::{multipart, Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct AiService {
    client: Client,
    api_key: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning: Option<ReasoningConfig>,
}

#[derive(Debug, Serialize)]
struct ReasoningConfig {
    effort: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChatMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedTask {
    pub title: String,
    pub due_at: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "intent")]
pub enum ParsedInput {
    #[serde(rename = "task")]
    Task(ParsedTask),
    #[serde(rename = "draft")]
    Draft { recipient: String, context: String, draft: String },
    #[serde(rename = "rejected")]
    Rejected { reason: String },
}

impl AiService {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn parse_task(&self, input: &str, current_date: &str) -> Result<ParsedTask, String> {
        let system_prompt = format!(
            r#"You are a task parser. Extract structured task data from natural language.

Current date: {current_date}

Return JSON only, no markdown:
{{"title": "task title", "due_at": "YYYY-MM-DD HH:MM" or null, "tags": ["tag1"]}}

Rules:
- title: Clean, actionable task title IN THE SAME LANGUAGE as input
- due_at: Parse relative dates (tomorrow, next monday, in 2 hours). Use 24h format. null if no time specified
- tags: Extract categories like "work", "personal", "shopping", "health"
- IMPORTANT: Keep the original language of the input. Do not translate.

Examples:
"call mom tomorrow at 5pm" -> {{"title": "Call mom", "due_at": "2024-01-02 17:00", "tags": ["personal"]}}
"buy milk" -> {{"title": "Buy milk", "due_at": null, "tags": ["shopping"]}}
"finish report by friday 3pm" -> {{"title": "Finish report", "due_at": "2024-01-05 15:00", "tags": ["work"]}}"#
        );

        let request = ChatRequest {
            model: "gpt-5-nano".to_string(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt,
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: input.to_string(),
                },
            ],
            reasoning: None,
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("API error {}: {}", status, text));
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| format!("Parse response failed: {}", e))?;

        let content = chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        // Clean up response (remove markdown if present)
        let json_str = content
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        serde_json::from_str(json_str)
            .map_err(|e| format!("Parse JSON failed: {} - content: {}", e, json_str))
    }

    /// Parse input with intent classification (task, draft, or rejected)
    pub async fn parse_input(&self, input: &str, current_date: &str, context: Option<&str>) -> Result<ParsedInput, String> {
        let context_section = context
            .map(|c| format!("\n\nRecent conversation:\n{}\n", c))
            .unwrap_or_default();

        let system_prompt = format!(
            r#"You are a TASK MANAGEMENT bot. You ONLY handle tasks, reminders, and message drafts.

Current date: {current_date}

Classify the input into one of these intents:
1. "task" - Creating a task/reminder/todo (actionable items the user needs to do)
2. "draft" - Drafting a message to someone
3. "rejected" - Everything else (questions, conversations, requests for information, etc.)

Return JSON only, no markdown.

For TASK intent (actionable items):
{{"intent": "task", "title": "task title", "due_at": "YYYY-MM-DD HH:MM" or null, "tags": ["tag1"]}}

For DRAFT intent (writing messages):
{{"intent": "draft", "recipient": "who", "context": "about what", "draft": "the message"}}

For REJECTED intent (anything that is NOT a task or draft):
{{"intent": "rejected", "reason": "brief explanation in user's language"}}

IMPORTANT - REJECT these types of input:
- Questions about anything ("what is...", "how to...", "why...", "когда...", "что такое...")
- Greetings ("hi", "hello", "привет")
- Conversations, chat, small talk
- Requests for information or explanations
- Asking the bot to do something other than create tasks
- General AI assistant requests

ACCEPT as tasks:
- Actions to do: "call mom", "buy groceries", "finish report"
- Reminders: "remind me to...", "напомни..."
- Todo items with or without deadlines

Examples:
"call mom tomorrow" -> {{"intent": "task", "title": "Call mom", "due_at": "2024-01-02", "tags": ["personal"]}}
"напиши сообщение боссу" -> {{"intent": "draft", "recipient": "boss", "context": "message", "draft": "..."}}
"what is the weather?" -> {{"intent": "rejected", "reason": "I only manage tasks. Try: 'Check weather tomorrow'"}}
"привет как дела" -> {{"intent": "rejected", "reason": "Привет! Я бот для задач. Напиши что нужно сделать."}}
"explain quantum physics" -> {{"intent": "rejected", "reason": "I'm a task bot. Try: 'Study quantum physics'"}}
"расскажи анекдот" -> {{"intent": "rejected", "reason": "Я только для задач. Попробуй: 'Найти хороший анекдот'"}}
{context_section}"#
        );

        let request = ChatRequest {
            model: "gpt-5-nano".to_string(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt,
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: input.to_string(),
                },
            ],
            reasoning: None,
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("API error {}: {}", status, text));
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| format!("Parse response failed: {}", e))?;

        let content = chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        let json_str = content
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        serde_json::from_str(json_str)
            .map_err(|e| format!("Parse JSON failed: {} - content: {}", e, json_str))
    }

    /// Parse reminder time from natural language
    pub async fn parse_reminder_time(&self, input: &str, current_date: &str) -> Result<String, String> {
        let system_prompt = format!(
            r#"You are a time parser. Extract a specific datetime from natural language input.

Current date and time: {current_date}

Return JSON only: {{"reminder_at": "YYYY-MM-DD HH:MM"}}

Rules:
- Parse relative times: "через 2 часа", "in 30 minutes", "завтра в 9", "tomorrow morning"
- "morning" = 09:00, "evening" = 18:00, "night" = 21:00
- Always return a specific datetime, never null
- Use 24-hour format

Examples:
"через 2 часа" -> {{"reminder_at": "2024-01-15 17:00"}}
"завтра в 9 утра" -> {{"reminder_at": "2024-01-16 09:00"}}
"in 30 minutes" -> {{"reminder_at": "2024-01-15 15:30"}}
"monday at 3pm" -> {{"reminder_at": "2024-01-20 15:00"}}"#
        );

        let request = ChatRequest {
            model: "gpt-5-nano".to_string(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt,
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: input.to_string(),
                },
            ],
            reasoning: None,
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("API error {}: {}", status, text));
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| format!("Parse response failed: {}", e))?;

        let content = chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        let json_str = content
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        #[derive(Deserialize)]
        struct ReminderResponse {
            reminder_at: String,
        }

        let parsed: ReminderResponse = serde_json::from_str(json_str)
            .map_err(|e| format!("Parse JSON failed: {} - content: {}", e, json_str))?;

        Ok(parsed.reminder_at)
    }

    /// Parse timezone from city name
    pub async fn parse_timezone(&self, input: &str) -> Result<String, String> {
        let system_prompt = r#"You are a timezone parser. Given a city name or location, return the IANA timezone.

Return JSON only: {"timezone": "America/New_York"}

Common mappings:
- Moscow, Москва -> Europe/Moscow
- New York, NYC -> America/New_York
- London -> Europe/London
- Paris -> Europe/Paris
- Berlin -> Europe/Berlin
- Tokyo -> Asia/Tokyo
- Dubai -> Asia/Dubai
- Singapore -> Asia/Singapore
- Sydney -> Australia/Sydney
- Los Angeles, LA -> America/Los_Angeles
- Chicago -> America/Chicago
- Киев, Kyiv -> Europe/Kyiv
- Минск, Minsk -> Europe/Minsk
- Алматы, Almaty -> Asia/Almaty

If unsure, make best guess based on the input."#;

        let request = ChatRequest {
            model: "gpt-5-nano".to_string(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: input.to_string(),
                },
            ],
            reasoning: None,
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("API error {}: {}", status, text));
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| format!("Parse response failed: {}", e))?;

        let content = chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        let json_str = content
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        #[derive(Deserialize)]
        struct TimezoneResponse {
            timezone: String,
        }

        let parsed: TimezoneResponse = serde_json::from_str(json_str)
            .map_err(|e| format!("Parse JSON failed: {} - content: {}", e, json_str))?;

        Ok(parsed.timezone)
    }

    /// Transcribe audio using Whisper API
    pub async fn transcribe_audio(&self, audio_data: Vec<u8>) -> Result<String, String> {
        let part = multipart::Part::bytes(audio_data)
            .file_name("audio.ogg")
            .mime_str("audio/ogg")
            .map_err(|e| format!("Failed to create multipart: {}", e))?;

        // Note: language hint helps Whisper with short audio clips
        // Using "ru" for Russian, but Whisper will still detect other languages
        let form = multipart::Form::new()
            .text("model", "whisper-1")
            .text("language", "ru")
            .part("file", part);

        let response = self
            .client
            .post("https://api.openai.com/v1/audio/transcriptions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await
            .map_err(|e| format!("Whisper request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("Whisper API error {}: {}", status, text));
        }

        #[derive(Deserialize)]
        struct WhisperResponse {
            text: String,
        }

        let whisper_response: WhisperResponse = response
            .json()
            .await
            .map_err(|e| format!("Parse Whisper response failed: {}", e))?;

        Ok(whisper_response.text)
    }

    /// Parse edit instruction and apply to existing task
    pub async fn parse_task_edit(&self, current_title: &str, edit_instruction: &str, current_date: &str) -> Result<ParsedTask, String> {
        let system_prompt = format!(
            r#"You are a task editor. Given a current task and an edit instruction, produce the updated task.

Current date: {current_date}

Current task: "{current_title}"
Edit instruction from user: "{edit_instruction}"

Return JSON only: {{"title": "updated title", "due_at": "YYYY-MM-DD HH:MM" or null, "tags": []}}

Rules:
- Apply ONLY the changes requested in the edit instruction
- Keep everything else from the original task unchanged
- If user asks to change wording/phrasing, update the title accordingly
- If user mentions a new time/date, update due_at
- Keep the same language as the original task
- Do NOT create a completely new task - modify the existing one

Examples:
Current: "Позвонить маме завтра"
Edit: "измени на послезавтра"
Result: {{"title": "Позвонить маме послезавтра", "due_at": "2024-01-17", "tags": []}}

Current: "Buy groceries tomorrow at 5pm"
Edit: "change to 3pm"
Result: {{"title": "Buy groceries tomorrow at 3pm", "due_at": "2024-01-16 15:00", "tags": []}}

Current: "Call John about project"
Edit: "change John to Mike"
Result: {{"title": "Call Mike about project", "due_at": null, "tags": []}}"#
        );

        let request = ChatRequest {
            model: "gpt-5-nano".to_string(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt,
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: format!("Current: \"{}\"\nEdit: \"{}\"", current_title, edit_instruction),
                },
            ],
            reasoning: None,
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("API error {}: {}", status, text));
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| format!("Parse response failed: {}", e))?;

        let content = chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        let json_str = content
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        serde_json::from_str(json_str)
            .map_err(|e| format!("Parse JSON failed: {} - content: {}", e, json_str))
    }
}
