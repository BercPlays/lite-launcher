pub mod types {

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct StandardRequestFormat {
        pub state: RequestState,
        pub message: String,
        pub content: String,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub enum RequestState {
        #[serde(rename = "successful")]
        Successful,
        #[serde(rename = "error")]
        Error,
    }
}
