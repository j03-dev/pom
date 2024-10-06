mod app;
mod model;

use app::App;

use iced::{application::Application, window, Font, Settings};
use model::Pom;
use rusql_alchemy::prelude::*;

#[tokio::main]
async fn main() -> Result<(), iced::Error> {
    let conn = Database::new().await.conn;

    let tasks = Pom::all(&conn).await;

    let settings = Settings {
        id: Some("Pom".to_string()),
        window: window::Settings {
            size: (200, 200),
            position: window::Position::Centered,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            level: window::Level::Normal,
            icon: None,
            platform_specific: window::PlatformSpecific {
                application_id: "com.pom.app".to_string(),
            },
            ..Default::default()
        },
        flags: tasks,
        default_font: Font::DEFAULT,
        default_text_size: 15.0,
        antialiasing: true,
        exit_on_close_request: true,
    };

    App::run(settings)
}
