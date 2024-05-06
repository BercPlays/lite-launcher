pub mod rpc_handler {
    use discord_rich_presence::{
        activity::{Activity, Assets},
        DiscordIpc, DiscordIpcClient,
    };
    use std::error::Error;

    pub struct DefinedActivities;

    impl DefinedActivities {
        fn base_asset<'a>() -> Assets<'a> {
            Assets::new()
                .large_image("app_logo_1")
                .large_text("Lite Launcher")
        }
        pub fn in_launcher<'a>() -> Activity<'a> {
            Activity::new()
                .assets(DefinedActivities::base_asset())
                .details("In Launcher")
        }
        pub fn playing<'a>(image: &'a str, text: &'a str) -> Activity<'a> {
            Activity::new().details("Playing").assets(
                DefinedActivities::base_asset()
                    .small_image(image)
                    .small_text(text),
            )
        }
    }

    pub struct RPCHandler {
        client: DiscordIpcClient,
    }

    impl RPCHandler {
        pub fn new(id: &str) -> Self {
            Self {
                client: DiscordIpcClient::new(id).unwrap(),
            }
        }
        pub fn connect(&mut self) -> Result<(), Box<dyn Error>> {
            self.client.connect()
        }
        pub fn set_activity(&mut self, activity: Activity) -> Result<(), Box<dyn Error>> {
            self.client.set_activity(activity)
        }
        pub fn close(&mut self) {
            self.client.close();
        }
    }
}
