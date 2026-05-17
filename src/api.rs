use wasm_bindgen::prelude::*;
use web_sys::{Request, RequestInit, RequestMode, Response, Headers};
use wasm_bindgen_futures::JsFuture;
use crate::config::KommentConfig;
use wasm_bindgen::JsCast;

fn escape_graphql_string(s: &str) -> String {
    serde_json::to_string(s).unwrap_or_else(|_| format!("\"{}\"", s.replace('"', "\\\"").replace('\n', "\\n")))
}

pub async fn execute_graphql(config: &KommentConfig, query: String) -> Result<JsValue, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);

    let headers = Headers::new()?;
    if let Some(token) = &config.token {
        headers.append("Authorization", &format!("Bearer {}", token))?;
    }
    opts.set_headers(&headers);

    let body = serde_json::json!({ "query": query });
    opts.set_body(&JsValue::from_str(&body.to_string()));

    let url = config.api_url.as_deref().unwrap_or("https://api.github.com/graphql");
    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window found"))?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    if !resp.ok() {
        let status = resp.status();
        let body_text = JsFuture::from(resp.text()?).await?;
        let body_str = body_text.as_string().unwrap_or_default();
        
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body_str) {
            if let Some(msg) = json["message"].as_str() {
                return Err(JsValue::from_str(&format!("GitHub API Error ({}): {}", status, msg)));
            }
        }
        
        return Err(JsValue::from_str(&format!("GitHub API Error: {} {}", status, body_str)));
    }

    let json_value = JsFuture::from(resp.json()?).await?;
    let json_serde: serde_json::Value = serde_wasm_bindgen::from_value(json_value.clone())?;

    if let Some(errors) = json_serde["errors"].as_array() {
        if !errors.is_empty() {
            let msg = errors[0]["message"].as_str().unwrap_or("Unknown GraphQL error");
            return Err(JsValue::from_str(&format!("GitHub API Error: {}", msg)));
        }
    }

    Ok(json_value)
}

pub async fn fetch_discussion(config: &KommentConfig) -> Result<JsValue, JsValue> {
    if config.token.is_none() {
        return Err(JsValue::from_str("401 Unauthorized: Please login to view comments"));
    }

    let (owner, name) = config.repo.split_once('/').ok_or("Invalid repo format")?;
    
    let query = match config.mapping.as_str() {
        "number" => format!(
            r#"query {{
                viewer {{ login }}
                repository(owner: "{owner}", name: "{name}") {{
                    discussion(number: {number}) {{
                        id
                        title
                        bodyHTML
                        comments(first: 100) {{
                            nodes {{
                                id
                                author {{
                                    login
                                    avatarUrl
                                }}
                                bodyHTML
                                body
                                createdAt
                            }}
                        }}
                    }}
                }}
            }}"#,
            owner = owner,
            name = name,
            number = config.term
        ),
        _ => format!(
            r#"query {{
                viewer {{ login }}
                search(query: "repo:{owner}/{name} is:discussion \"{term}\"", type: DISCUSSION, first: 1) {{
                    edges {{
                        node {{
                            ... on Discussion {{
                                id
                                title
                                bodyHTML
                                comments(first: 100) {{
                                    nodes {{
                                        id
                                        author {{
                                            login
                                            avatarUrl
                                        }}
                                        bodyHTML
                                        body
                                        createdAt
                                    }}
                                }}
                            }}
                        }}
                    }}
                }}
            }}"#,
            owner = owner,
            name = name,
            term = config.term
        ),
    };

    execute_graphql(config, query).await
}

pub async fn create_discussion(config: &KommentConfig, repo_owner: String, repo_name: String, category_name: String, title: String, body: String) -> Result<JsValue, JsValue> {
    let query_ids = format!(
        r#"query {{
            repository(owner: "{repo_owner}", name: "{repo_name}") {{
                id
                discussionCategories(first: 10) {{
                    nodes {{
                        id
                        name
                    }}
                }}
            }}
        }}"#,
        repo_owner = repo_owner,
        repo_name = repo_name
    );

    let ids_resp = execute_graphql(config, query_ids).await?;
    let ids_data: serde_json::Value = serde_wasm_bindgen::from_value(ids_resp)?;
    
    let repo_id = ids_data["data"]["repository"]["id"].as_str()
        .ok_or("Could not find repository ID")?;
        
    let category_id = ids_data["data"]["repository"]["discussionCategories"]["nodes"]
        .as_array()
        .ok_or("Could not find categories")?
        .iter()
        .find(|c| c["name"].as_str().map(|n| n.to_lowercase()) == Some(category_name.to_lowercase()))
        .and_then(|c| c["id"].as_str())
        .ok_or_else(|| format!("Category '{}' not found", category_name))?;

    let mutation = format!(
        r#"mutation {{
            createDiscussion(input: {{repositoryId: "{repo_id}", categoryId: "{category_id}", title: {title}, body: {body}}}) {{
                discussion {{
                    id
                }}
            }}
        }}"#,
        repo_id = repo_id,
        category_id = category_id,
        title = escape_graphql_string(&title),
        body = escape_graphql_string(&body)
    );

    execute_graphql(config, mutation).await
}

pub async fn post_comment(config: &KommentConfig, discussion_id: String, body: String) -> Result<JsValue, JsValue> {
    let query = format!(
        r#"mutation {{
            addDiscussionComment(input: {{discussionId: "{discussion_id}", body: {body}}}) {{
                comment {{
                    id
                }}
            }}
        }}"#,
        discussion_id = discussion_id,
        body = escape_graphql_string(&body)
    );

    execute_graphql(config, query).await
}

pub async fn delete_comment(config: &KommentConfig, comment_id: String) -> Result<JsValue, JsValue> {
    let query = format!(
        r#"mutation {{
            deleteDiscussionComment(input: {{id: "{comment_id}"}}) {{
                comment {{
                    id
                }}
            }}
        }}"#,
        comment_id = comment_id
    );

    execute_graphql(config, query).await
}

pub async fn update_comment(config: &KommentConfig, comment_id: String, body: String) -> Result<JsValue, JsValue> {
    let query = format!(
        r#"mutation {{
            updateDiscussionComment(input: {{commentId: "{comment_id}", body: {body}}}) {{
                comment {{
                    id
                }}
            }}
        }}"#,
        comment_id = comment_id,
        body = escape_graphql_string(&body)
    );

    execute_graphql(config, query).await
}
