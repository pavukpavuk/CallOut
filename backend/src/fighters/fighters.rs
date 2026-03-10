use axum::Json;

use serde::Serialize;
use time::OffsetDateTime;

use futures::StreamExt;

use crate::shb_error::BackendError;

use crate::AppState;
use axum::extract::State;
use std::sync::Arc;

#[derive(Serialize, Debug)]
pub struct FighterRow {
    user_id: i32,
    name: String,
    picture_url: Option<String>,
    description: Option<String>,
    rank: String,
    wins: i32,
    losses: i32,
    draws: i32,
    weight_kg: i32,
    created_at: OffsetDateTime,
    gym_id_fk: Option<i32>,
}

pub async fn get_fighters(
    app_state: State<Arc<AppState>>,
) -> Result<Json<Vec<FighterRow>>, BackendError> {
    let mut fighter_arr = Vec::<FighterRow>::new();
    let mut query = sqlx::query_file_as!(FighterRow, "src/fighters/sql/get_fighters.sql")
        .fetch(&app_state.db_pool);

    while let Some(res) = query.next().await {
        match res {
            Ok(val) => {
                fighter_arr.push(val);
            }
            Err(_) => {}
        }
    }
    Ok(Json(fighter_arr))
}

//mock data

//     let fighter_arr = vec![
//     FighterRow {
//     user_id: 1,
//     weight_kg: 1,
//     name: "Jimmy Johnson".to_string(),
//     picture_url: "jimmy_johnson_profile_img.jpg".to_string(),
//     description: "When I be losing I be gettin' my ass whupped. But when I be winnin' I be doing the ass whuppin!".to_string(),
//     rank: BeltRank::Black,
//     wins: 12,
//     losses: 96,
//     draws: 0,
//     created_at: Some(OffsetDateTime::parse("2026-02-25T14:03:12.345678+00:00", &time::format_description::well_known::Rfc3339).unwrap()),
//     gym_id_fk: 1,
// },
// FighterRow {
//     user_id: 2,
//     weight_kg: 1,
//     name: "Johnny Jimson".to_string(),
//     picture_url: "johnny_jimson_profile_img.jpg".to_string(),
//     description: "I am a shark and the ground is my ocean".to_string(),
//     rank: BeltRank::Blue,
//     wins: 1,
//     losses: 5,
//     draws: 20,
//     created_at: Some(OffsetDateTime::parse("2025-02-25T14:03:12.345678+00:00", &time::format_description::well_known::Rfc3339).unwrap()),
//     gym_id_fk: 2,
// },
// FighterRow {
//     user_id: 3,
//     weight_kg: 1,
//     name: "Samuel Gibblers".to_string(),
//     picture_url: "samuel_gibblers_profile_img.jpg".to_string(),
//     description: "My father was an extremely violent man".to_string(),
//     rank: BeltRank::Purple,
//     wins: 36,
//     losses: 0,
//     draws: 0,
//     created_at: Some(OffsetDateTime::parse("2026-01-25T14:03:12.345678+00:00", &time::format_description::well_known::Rfc3339).unwrap()),
//     gym_id_fk: 3,
// },
// FighterRow {
//     user_id: 4,
//     weight_kg: 1,
//     name: "Princess".to_string(),
//     picture_url: "princess_profile_img.jpg".to_string(),
//     description: "Call me Mr CTE because what?".to_string(),
//     rank: BeltRank::White,
//     wins: 1,
//     losses: 1,
//     draws: 1,
//     created_at: Some(OffsetDateTime::parse("2025-10-25T14:03:12.345678+00:00", &time::format_description::well_known::Rfc3339).unwrap()),
//     gym_id_fk: 1,
// },
//     ];
