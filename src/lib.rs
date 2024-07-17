/// The core module containing the `commands` and `events` modules
pub mod core {
    /// Contains the functions used for commands
    pub mod commands {
        pub mod age;
        pub mod info;
        pub mod ping;
    }

    /// Contains the functions used for events
    pub mod events {
        pub mod interaction;
        pub mod ready;
    }
}

/// The utils module containing useful functions
pub mod utils {
    pub mod config;
    pub mod logger;
}

// Things used for the bot
// TODO Separate this into its own module
pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// Useful modules
/// ```rust
/// // Usually used like this:
/// use crate::prelude::*;
/// ```
pub mod prelude {
    pub use crate::utils::config::{Config, ToColor};
    pub use crate::utils::logger;
}
