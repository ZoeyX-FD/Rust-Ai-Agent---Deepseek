use tokio_rusqlite::Connection;
use std::path::Path;
use log::{info, error};
use thiserror::Error;
use std::sync::Arc;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] tokio_rusqlite::Error),
    #[error("Database connection error: {0}")]
    Connection(String),
}

#[derive(Clone)]
pub struct Database {
    conn: Arc<Connection>,
}

impl Database {
    pub async fn new<P: AsRef<Path>>(path: P) -> Result<Self, DatabaseError> {
        let conn = Connection::open(path)
            .await
            .map_err(|e| DatabaseError::Connection(e.to_string()))?;
        
        let db = Self { conn: Arc::new(conn) };
        db.initialize().await?;
        Ok(db)
    }

    async fn initialize(&self) -> Result<(), DatabaseError> {
        // Create tables if they don't exist
        self.conn.call(|conn| {
            conn.execute_batch(
                "CREATE TABLE IF NOT EXISTS conversations (
                    id INTEGER PRIMARY KEY,
                    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                    user_input TEXT NOT NULL,
                    ai_response TEXT NOT NULL,
                    personality TEXT NOT NULL
                );
                CREATE TABLE IF NOT EXISTS knowledge_base (
                    id INTEGER PRIMARY KEY,
                    key TEXT UNIQUE NOT NULL,
                    value TEXT NOT NULL,
                    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
                );"
            )
        })
        .await?;

        info!("Database initialized successfully");
        Ok(())
    }

    pub async fn save_conversation(
        &self,
        user_input: String,
        ai_response: String,
        personality: String,
    ) -> Result<(), DatabaseError> {
        self.conn
            .call(move |conn| {
                conn.execute(
                    "INSERT INTO conversations (user_input, ai_response, personality) VALUES (?1, ?2, ?3)",
                    [&user_input, &ai_response, &personality],
                )
            })
            .await?;
        
        Ok(())
    }

    pub async fn save_knowledge(
        &self,
        key: String,
        value: String,
    ) -> Result<(), DatabaseError> {
        self.conn
            .call(move |conn| {
                conn.execute(
                    "INSERT OR REPLACE INTO knowledge_base (key, value) VALUES (?1, ?2)",
                    [&key, &value],
                )
            })
            .await?;
        
        Ok(())
    }

    pub async fn get_recent_conversations(&self, limit: i64) -> Result<Vec<(String, String, String, String)>, DatabaseError> {
        let result = self.conn
            .call(move |conn| {
                let mut stmt = conn.prepare(
                    "SELECT timestamp, user_input, ai_response, personality 
                     FROM conversations 
                     ORDER BY timestamp DESC 
                     LIMIT ?"
                )?;
                
                let rows = stmt.query_map([limit], |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                    ))
                })?;

                let mut conversations = Vec::new();
                for row in rows {
                    conversations.push(row?);
                }
                
                Ok(conversations)
            })
            .await?;
            
        Ok(result)
    }

    pub async fn get_knowledge(&self, key: String) -> Result<Option<String>, DatabaseError> {
        let result = self.conn
            .call(move |conn| {
                let mut stmt = conn.prepare("SELECT value FROM knowledge_base WHERE key = ?")?;
                let mut rows = stmt.query([&key])?;
                
                if let Some(row) = rows.next()? {
                    Ok(Some(row.get::<_, String>(0)?))
                } else {
                    Ok(None)
                }
            })
            .await?;
            
        Ok(result)
    }
}
