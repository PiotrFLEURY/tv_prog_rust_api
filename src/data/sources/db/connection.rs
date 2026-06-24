use dotenv::var;
use sea_orm::{Database, DatabaseConnection};
use tokio::sync::OnceCell;

static CONNECTION: OnceCell<DatabaseConnection> = OnceCell::const_new();

///
/// Get the shared database connection, creating it on first use.
///
pub async fn get() -> Result<&'static DatabaseConnection, String> {
    CONNECTION
        .get_or_try_init(|| async {
            let connection_string = var("CONNECTION_STRING").map_err(|_| {
                "CONNECTION_STRING must be set in the environment variables".to_string()
            })?;
            Database::connect(connection_string)
                .await
                .map_err(|e| e.to_string())
        })
        .await
}
