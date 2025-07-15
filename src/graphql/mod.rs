//! GraphQL API Integration
//!
//! This module provides utilities for interacting with GraphQL APIs,
//! including a client for sending queries and mutations.

use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphQLError {
    #[error("Network error: {0}")]
    Network(reqwest::Error),
    #[error("GraphQL error: {0}")]
    GraphQL(String),
}

#[derive(Serialize)]
struct GraphQLRequest<T: Serialize> {
    query: String,
    variables: T,
}

#[derive(Deserialize)]
struct GraphQLResponse<T> {
    data: Option<T>,
    errors: Option<Vec<GraphQLErrorDetail>>,
}

#[derive(Deserialize, Debug)]
struct GraphQLErrorDetail {
    message: String,
}

pub struct GraphQLClient {
    client: Client,
    endpoint: String,
}

impl GraphQLClient {
    pub fn new(endpoint: &str) -> Self {
        Self {
            client: Client::new(),
            endpoint: endpoint.to_string(),
        }
    }

    pub async fn query<V: Serialize, T: for<'de> Deserialize<'de>>(
        &self,
        query: &str,
        variables: V,
    ) -> Result<T, GraphQLError> {
        let request_body = GraphQLRequest {
            query: query.to_string(),
            variables,
        };

        let response = self
            .client
            .post(&self.endpoint)
            .json(&request_body)
            .send()
            .await
            .map_err(GraphQLError::Network)?;

        let response_body: GraphQLResponse<T> = response
            .json()
            .await
            .map_err(GraphQLError::Network)?;

        if let Some(errors) = response_body.errors {
            let error_messages = errors
                .into_iter()
                .map(|e| e.message)
                .collect::<Vec<_>>()
                .join(", ");
            return Err(GraphQLError::GraphQL(error_messages));
        }

        if let Some(data) = response_body.data {
            Ok(data)
        } else {
            Err(GraphQLError::GraphQL("No data in response".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[tokio::test]
    async fn test_graphql_query() {
        // This test requires a running GraphQL endpoint.
        // For now, we'll just test the client construction.
        let client = GraphQLClient::new("https://api.github.com/graphql");
        assert_eq!(client.endpoint, "https://api.github.com/graphql");
    }
}
