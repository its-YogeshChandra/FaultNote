use dotenv::dotenv;
use reqwest::{
    Client,
    header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue},
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{env, time::Duration};

use crate::app::PageInfo;

/// Notion API client
pub struct NotionClient {
    pub base_url: String,
    pub http_client: Client,
}

/// Response from Notion search API
#[derive(Debug, Deserialize)]
struct SearchResponse {
    results: Vec<Value>,
    next_cursor: Option<String>,
    has_more: bool,
}

/// Data to be sent to Notion when submitting a fault log
#[derive(Debug, Clone)]
pub struct FaultLogEntry {
    pub error: String,
    pub problem: String,
    pub solution: String,
    pub code: Option<String>,
}

impl NotionClient {
    fn new(base_url: String, http_client: Client) -> Self {
        Self {
            base_url,
            http_client,
        }
    }
}

/// Create and configure a NotionClient from environment variables
pub fn create_notion_client() -> Result<NotionClient, String> {
    dotenv().ok();

    let api_key = env::var("API_KEY")
        .map_err(|_| "API_KEY not found in environment variables".to_string())?;

    let base_url = "https://api.notion.com".to_string();

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| format!("Invalid API key format: {}", e))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        "Notion-Version",
        HeaderValue::from_static("2022-06-28"),
    );

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .default_headers(headers)
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    Ok(NotionClient::new(base_url, client))
}

/// Create a toggleable error block for Notion with professional styling
pub fn create_error_block(
    error: &str,
    problem: &str,
    solution: &str,
    code: Option<&str>,
    language: Option<&str>,
) -> Value {
    let mut children: Vec<Value> = vec![
        // Divider for visual separation
        json!({
            "object": "block",
            "type": "divider",
            "divider": {}
        }),
        // ERROR Section - Red callout
        json!({
            "object": "block",
            "type": "callout",
            "callout": {
                "rich_text": [{
                    "type": "text",
                    "text": { "content": error }
                }],
                "icon": { "type": "emoji", "emoji": "🔴" },
                "color": "red_background"
            }
        }),
        // Heading for Error label
        json!({
            "object": "block",
            "type": "heading_3",
            "heading_3": {
                "rich_text": [{
                    "type": "text",
                    "text": { "content": "What was the problem?" },
                    "annotations": { "bold": true }
                }],
                "color": "orange"
            }
        }),
        // PROBLEM Section - Yellow/Orange callout
        json!({
            "object": "block",
            "type": "callout",
            "callout": {
                "rich_text": [{
                    "type": "text",
                    "text": { "content": problem }
                }],
                "icon": { "type": "emoji", "emoji": "🟡" },
                "color": "yellow_background"
            }
        }),
        // Heading for Solution label
        json!({
            "object": "block",
            "type": "heading_3",
            "heading_3": {
                "rich_text": [{
                    "type": "text",
                    "text": { "content": "How did you fix it?" },
                    "annotations": { "bold": true }
                }],
                "color": "green"
            }
        }),
        // SOLUTION Section - Green callout
        json!({
            "object": "block",
            "type": "callout",
            "callout": {
                "rich_text": [{
                    "type": "text",
                    "text": { "content": solution }
                }],
                "icon": { "type": "emoji", "emoji": "✅" },
                "color": "green_background"
            }
        }),
    ];

    // Add code block if provided
    if let Some(code_content) = code {
        if !code_content.trim().is_empty() {
            children.push(json!({
                "object": "block",
                "type": "heading_3",
                "heading_3": {
                    "rich_text": [{
                        "type": "text",
                        "text": { "content": "Code Reference" },
                        "annotations": { "bold": true }
                    }],
                    "color": "purple"
                }
            }));
            children.push(json!({
                "object": "block",
                "type": "code",
                "code": {
                    "caption": [],
                    "rich_text": [{
                        "type": "text",
                        "text": { "content": code_content }
                    }],
                    "language": language.unwrap_or("plain text")
                }
            }));
        }
    }

    // Add closing divider
    children.push(json!({
        "object": "block",
        "type": "divider",
        "divider": {}
    }));

    // The main toggleable heading with timestamp
    json!([{
        "object": "block",
        "type": "heading_2",
        "heading_2": {
            "rich_text": [
                {
                    "type": "text",
                    "text": { "content": "🐛 " }
                },
                {
                    "type": "text",
                    "text": { "content": &error[..std::cmp::min(error.len(), 50)] },
                    "annotations": { "bold": true }
                },
                {
                    "type": "text",
                    "text": { "content": if error.len() > 50 { "..." } else { "" } }
                }
            ],
            "color": "red",
            "is_toggleable": true,
            "children": children
        }
    }])
}

fn extract_page_info(result: &Value) -> Option<PageInfo> {
    let id = result.get("id")?.as_str()?.to_string();

    let title = result
        .get("properties")
        .and_then(|props| {
            props.get("title")
                .or_else(|| props.get("Name"))
                .or_else(|| props.get("Title"))
        })
        .and_then(|title_prop| title_prop.get("title"))
        .and_then(|title_array| title_array.as_array())
        .and_then(|arr| arr.first())
        .and_then(|first| first.get("plain_text"))
        .and_then(|text| text.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Untitled".to_string());

    Some(PageInfo { id, title })
}

/// Fetch all pages from Notion
pub async fn fetch_pages(client: &NotionClient) -> Result<Vec<PageInfo>, reqwest::Error> {
    let main_url = format!("{}/v1/search", client.base_url);

    let mut all_pages: Vec<PageInfo> = Vec::new();
    let mut start_cursor: Option<String> = None;

    loop {
        let mut body = json!({
            "filter": {
                "property": "object",
                "value": "page"
            },
            "page_size": 100
        });

        if let Some(cursor) = &start_cursor {
            body["start_cursor"] = json!(cursor);
        }

        let response: SearchResponse = client
            .http_client
            .post(&main_url)
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        for result in &response.results {
            if let Some(page_info) = extract_page_info(result) {
                all_pages.push(page_info);
            }
        }

        if response.has_more {
            start_cursor = response.next_cursor;
        } else {
            break;
        }
    }

    Ok(all_pages)
}

/// Create a fault log entry on a Notion page
pub async fn create_entry(
    client: &NotionClient,
    page_id: &str,
    entry: &FaultLogEntry,
) -> Result<(), reqwest::Error> {
    let main_url = format!("{}/v1/blocks/{}/children", client.base_url, page_id);

    let block = create_error_block(
        &entry.error,
        &entry.problem,
        &entry.solution,
        entry.code.as_deref(),
        Some("rust"),
    );

    let body = json!({ "children": block });

    client
        .http_client
        .patch(&main_url)
        .json(&body)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
