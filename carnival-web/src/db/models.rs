use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[allow(dead_code)]
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct SessionToken {
    pub for_user: i32,
    pub remote_addr: String,
    pub unique_hmac_key: String,
    pub token: String,
    pub is_valid: bool,
    pub invalidation_source: String,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug, Default)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub role: String,
    pub battletag: String,
    pub rating: i32,
    pub wins: i32,
    pub losses: i32,
    pub password: String,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug)]
pub struct OverwatchMap {
    pub id: i32,
    pub name: String,
    pub mode: String,
}

#[allow(dead_code, Serialize, Default, Debug)]
#[derive(sqlx::FromRow, Clone, Serialize, Debug, Default)]
pub struct OverwatchMatch {
    pub id: i32,
    pub map_id: i32,
    pub winner: u8,
    pub completed: bool,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug)]
pub struct OverwatchMatchPlayer {
    pub id: i32,
    pub user_id: i32,
    pub match_id: i32,
    // Blue 1
    // Red  2
    pub team_id: u8,
}

#[allow(dead_code)]
pub struct Queue {
    pub id: i32,
    pub title: String,
    pub demographic: String,
}

#[derive(FromRow)]
pub struct QueuedPlayer {
    pub id: i32,
    pub queue_id: i32,
    pub user_id: i32,
    pub role: String,
}
