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

    /// Parse input with intent classification (task or draft)
    pub async fn parse_input(&self, input: &str, current_date: &str, context: Option<&str>) -> Result<ParsedInput, String> {
        let context_section = context
            .map(|c| format!("\n\nRecent conversation:\n{}\n", c))
            .unwrap_or_default();

        let system_prompt = format!(
            r#"You are an assistant that classifies user input and takes appropriate action.

Current date: {current_date}

Classify the input into one of these intents:
1. "task" - Creating a task/reminder/todo
2. "draft" - Drafting a message to someone

Return JSON only, no markdown.

For TASK intent:
{{"intent": "task", "title": "task title", "due_at": "YYYY-MM-DD HH:MM" or null, "tags": ["tag1"]}}

For DRAFT intent (when user wants to write/draft/compose a message):
{{"intent": "draft", "recipient": "who the message is for", "context": "what the message is about", "draft": "the actual drafted message"}}

Rules:
- Keep the original language of the input
- For drafts: write a professional, friendly message based on context
- Look for keywords: "draft", "write", "compose", "message to", "напиши", "черновик"

Examples:
"call mom tomorrow" -> {{"intent": "task", "title": "Call mom", "due_at": "2024-01-02", "tags": ["personal"]}}
"draft a message to John about the meeting" -> {{"intent": "draft", "recipient": "John", "context": "meeting", "draft": "Hi John,\n\nI wanted to follow up about our meeting..."}}
"напиши сообщение боссу что опаздываю" -> {{"intent": "draft", "recipient": "boss", "context": "running late", "draft": "Добрый день,\n\nПрошу прощения, но я немного задержусь..."}}
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
}
