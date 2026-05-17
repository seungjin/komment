use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DiscussionResponse {
    pub data: Option<DiscussionData>,
    pub errors: Option<Vec<GithubError>>,
}

#[derive(Serialize, Deserialize)]
pub struct GithubError {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct DiscussionData {
    pub repository: Option<RepositoryData>,
    pub search: Option<SearchData>,
    pub viewer: Option<ViewerData>,
}

#[derive(Serialize, Deserialize)]
pub struct ViewerData {
    pub login: String,
}

#[derive(Serialize, Deserialize)]
pub struct SearchData {
    pub edges: Vec<SearchEdge>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchEdge {
    pub node: Option<Discussion>,
}

#[derive(Serialize, Deserialize)]
pub struct RepositoryData {
    pub discussion: Option<Discussion>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Discussion {
    pub id: String,
    pub title: String,
    #[serde(rename = "bodyHTML")]
    pub body_html: String,
    pub comments: CommentsConnection,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommentsConnection {
    pub nodes: Vec<Comment>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: String,
    pub author: Author,
    #[serde(rename = "bodyHTML")]
    pub body_html: String,
    #[serde(rename = "body")]
    pub body: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Author {
    pub login: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: String,
}
