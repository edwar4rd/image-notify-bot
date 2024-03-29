pub mod prelude {
    pub use crate::commands;
    pub use crate::evaluater::{Evaluater, EvaluaterError, TagClass, TagConfidence, TagName};
    pub use crate::handler;
    pub use crate::image;
    pub use crate::MyEvaluater;
    pub use poise::serenity_prelude as serenity;

    #[derive(serde::Deserialize)]
    pub struct WrittenData {
        pub monitored_channels: std::collections::BTreeMap<
            Option<serenity::GuildId>,
            std::collections::BTreeSet<serenity::ChannelId>,
        >,
        pub evaluation_caches: dashmap::DashMap<String, std::collections::BTreeMap<u32, f32>>,
    }

    #[derive(serde::Serialize)]
    pub struct WritingData<'a> {
        pub monitored_channels: &'a std::collections::BTreeMap<
            Option<serenity::GuildId>,
            std::collections::BTreeSet<serenity::ChannelId>,
        >,
        pub evaluation_caches: &'a dashmap::DashMap<String, std::collections::BTreeMap<u32, f32>>,
    }

    pub struct Data {
        pub monitored_channels: std::sync::Arc<
            std::sync::Mutex<
                std::collections::BTreeMap<
                    Option<serenity::GuildId>,
                    std::collections::BTreeSet<serenity::ChannelId>,
                >,
            >,
        >,
        pub evaluater: std::sync::Arc<tokio::sync::Mutex<MyEvaluater>>,
    } // User data, which is stored and accessible in all command invocations
    pub type Error = Box<dyn std::error::Error + Send + Sync>;
    pub type Context<'a> = poise::Context<'a, Data, Error>;
}

pub mod commands;
pub mod evaluater;
pub mod handler;
pub mod image;
pub type MyEvaluater = evaluater::ChildEvaluater;
