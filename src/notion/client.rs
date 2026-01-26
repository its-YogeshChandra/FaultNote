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
pub struct SearchResponse {
    pub object: String,
    pub results: Vec<Value>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

/// Response from Notion append block API
#[derive(Debug, Deserialize)]
pub struct AppendBlockResponse {
    pub object: String,
    pub results: Vec<Value>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
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
    /// Create a new NotionClient with the given base URL and HTTP client
    pub fn new(base_url: String, http_client: Client) -> Self {
        Self {
            base_url,
            http_client,
        }
    }
}

/// Create and configure a NotionClient from environment variables
pub fn create_notion_client() -> Result<NotionClient, String> {
    // Load environment variables
    dotenv().ok();

    let api_key = env::var("API_KEY")
        .map_err(|_| "API_KEY not found in environment variables".to_string())?;

    let base_url = "https://api.notion.com".to_string();

    // Build headers for Notion API
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| format!("Invalid API key format: {}", e))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        "Notion-Version",
        HeaderValue::from_static("2022-06-28"), // Use stable API version
    );

    // Build HTTP client
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .default_headers(headers)
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    Ok(NotionClient::new(base_url, client))
}

/// Create a toggleable error block for Notion
pub fn create_error_block(
    error: &str,
    problem: &str,
    solution: &str,
    code: Option<&str>,
    language: Option<&str>,
) -> Value {
    // Build the children blocks inside the toggle
    let mut children: Vec<Value> = vec![
        // Error paragraph with bold label
        json!({
            "object": "block",
            "type": "paragraph",
            "paragraph": {
                "rich_text": [
                    {
                        "type": "text",
                        "text": { "content": "🔴 Error: " },
                        "annotations": { "bold": true, "color": "red" }
                    },
                    {
                        "type": "text",
                        "text": { "content": error }
                    }
                ],
                "color": "default"
            }
        }),
        // Problem paragraph
        json!({
            "object": "block",
            "type": "paragraph",
            "paragraph": {
                "rich_text": [
                    {
                        "type": "text",
                        "text": { "content": "🟡 Problem: " },
                        "annotations": { "bold": true, "color": "yellow" }
                    },
                    {
                        "type": "text",
                        "text": { "content": problem }
                    }
                ],
                "color": "default"
            }
        }),
        // Solution paragraph
        json!({
            "object": "block",
            "type": "paragraph",
            "paragraph": {
                "rich_text": [
                    {
                        "type": "text",
                        "text": { "content": "🟢 Solution: " },
                        "annotations": { "bold": true, "color": "green" }
                    },
                    {
                        "type": "text",
                        "text": { "content": solution }
                    }
                ],
                "color": "default"
            }
        }),
    ];

    // Add code block if provided
    if let Some(code_content) = code {
        if !code_content.trim().is_empty() {
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

    // The main toggleable heading 3
    json!([{
        "object": "block",
        "type": "heading_3",
        "heading_3": {
            "rich_text": [{
                "type": "text",
                "text": { "content": "📋 Error Log Entry" }
            }],
            "color": "red",
            "is_toggleable": true,
            "children": children
        }
    }])
}

/// Extract page info from Notion API response
fn extract_page_info(result: &Value) -> Option<PageInfo> {
    let id = result.get("id")?.as_str()?.to_string();

    // Try to get title from properties -> title -> title[0] -> plain_text
    let title = result
        .get("properties")
        .and_then(|props| {
            // Find the title property (it could be named "title", "Name", or "Title")
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

/// Fetch all pages from Notion (async function)
pub async fn fetch_pages(client: &NotionClient) -> Result<Vec<PageInfo>, reqwest::Error> {
    let main_url = format!("{}/v1/search", client.base_url);

    let mut all_pages: Vec<PageInfo> = Vec::new();
    let mut start_cursor: Option<String> = None;

    loop {
        // Build request body
        let mut body = json!({
            "filter": {
                "property": "object",
                "value": "page"
            },
            "page_size": 100
        });

        // Add cursor for pagination if we have one
        if let Some(cursor) = &start_cursor {
            body["start_cursor"] = json!(cursor);
        }

        // Make the API request
        let response: SearchResponse = client
            .http_client
            .post(&main_url)
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        // Extract page info from results
        for result in &response.results {
            if let Some(page_info) = extract_page_info(result) {
                all_pages.push(page_info);
            }
        }

        // Check if there are more pages
        if response.has_more {
            start_cursor = response.next_cursor;
        } else {
            break;
        }
    }

    Ok(all_pages)
}

/// Create a fault log entry on a Notion page (async function)
pub async fn create_entry(
    client: &NotionClient,
    page_id: &str,
    entry: &FaultLogEntry,
) -> Result<(), reqwest::Error> {
    let main_url = format!("{}/v1/blocks/{}/children", client.base_url, page_id);

    // Create the block content
    let block = create_error_block(
        &entry.error,
        &entry.problem,
        &entry.solution,
        entry.code.as_deref(),
        Some("rust"), // Default to Rust for code blocks
    );

    let body = json!({ "children": block });

    // Send the request
    let _response = client
        .http_client
        .patch(&main_url)
        .json(&body)
        .send()
        .await?
        .error_for_status()?; // This will return an error if status is not 2xx

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_error_block_without_code() {
        let block = create_error_block(
            "Test error",
            "Test problem",
            "Test solution",
            None,
            None,
        );

        assert!(block.is_array());
        let arr = block.as_array().unwrap();
        assert_eq!(arr.len(), 1);

        let heading = &arr[0];
        assert_eq!(heading["type"], "heading_3");
    }

    #[test]
    fn test_create_error_block_with_code() {
        let block = create_error_block(
            "Test error",
            "Test problem",
            "Test solution",
            Some("fn main() {}"),
            Some("rust"),
        );

        assert!(block.is_array());
        let arr = block.as_array().unwrap();
        assert_eq!(arr.len(), 1);

        let heading = &arr[0];
        let children = heading["heading_3"]["children"].as_array().unwrap();
        assert_eq!(children.len(), 4); // error, problem, solution, code
    }

    #[test]
    fn test_extract_page_info() {
        let page_json = json!({
            "id": "test-id-123",
            "properties": {
                "title": {
                    "title": [{
                        "plain_text": "My Test Page"
                    }]
                }
            }
        });

        let page_info = extract_page_info(&page_json).unwrap();
        assert_eq!(page_info.id, "test-id-123");
        assert_eq!(page_info.title, "My Test Page");
    }

    #[test]
    fn test_extract_page_info_with_name_property() {
        let page_json = json!({
            "id": "test-id-456",
            "properties": {
                "Name": {
                    "title": [{
                        "plain_text": "Named Page"
                    }]
                }
            }
        });

        let page_info = extract_page_info(&page_json).unwrap();
        assert_eq!(page_info.id, "test-id-456");
        assert_eq!(page_info.title, "Named Page");
    }
}
