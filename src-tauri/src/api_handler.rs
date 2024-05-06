pub mod api_handlers {

    use std::fmt::format;

    use serde_json::to_string;

    use crate::backend_types::types::{RequestState, StandardRequestFormat};

    pub struct ApiHandler {
        pub client: reqwest::Client,
    }

    impl ApiHandler {
        pub async fn get(&self, url: &str) -> Result<String, String> {
            let response = match self.client.get(url).send().await {
                Ok(res) => res,
                Err(err) => return Err(err.to_string()),
            };

            match response.text().await {
                Ok(body) => Ok(body),

                Err(err) => Err(err.to_string()),
            }
        }
    }
}
