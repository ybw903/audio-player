use tauri_plugin_sql::{Migration, MigrationKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let migrations = vec![
      // Define your migrations here
      Migration {
          version: 1,
          description: "create_initial_tables",
          sql: "CREATE TABLE songs (id INTEGER PRIMARY KEY, path TEXT);",
          kind: MigrationKind::Up,
      }
  ];

  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_sql::Builder::default()
              .add_migrations("sqlite:musicuon.db", migrations)
              .build()
    )
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
