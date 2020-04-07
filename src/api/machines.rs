use super::util::{int_or_string, HtbParser, HtbResponder};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

#[derive(Debug, Deserialize)]
pub struct Machine {
    pub id: u32,
    pub retired: bool,
    pub name: String,
    pub ip: String,
    pub os: String,
    pub rating: String,
    pub release: Option<String>,
    pub points: u8,
    pub user_owns: u32,
    pub root_owns: u32,
}

#[derive(Debug, Deserialize)]
pub struct MachineDetails {
    pub avatar: String,
    pub avatar_thumb: String,
    pub maker: Maker,
    pub maker2: Option<Maker>,
    pub user_blood: Option<BloodUser>,
    pub root_blood: Option<BloodUser>,
}

#[derive(Debug, Deserialize)]
pub struct Maker {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct BloodUser {
    pub id: u64,
    pub name: String,
    pub time: String,
}

#[derive(Debug, Deserialize)]
pub struct Spawned {
    pub id: u32,
}

#[derive(Debug, Deserialize)]
pub struct Owned {
    pub id: u32,
    pub owned_user: bool,
    pub owned_root: bool,
}

#[derive(Debug, Deserialize)]
pub struct Difficulty {
    pub id: u32,
    pub difficulty_ratings: Vec<u64>,
}

#[derive(Debug, Deserialize)]
pub struct Todo {
    pub id: u32,
}

#[derive(Debug, Deserialize)]
pub struct Assigned {
    pub id: u32,
}

#[derive(Debug, Deserialize)]
pub struct StateResponse {
    #[serde(deserialize_with = "int_or_string")]
    pub success: u8,
    #[serde(alias = "output")]
    pub status: String,
}

pub async fn list() -> Result<Vec<Machine>, Box<dyn Error>> {
    let api = super::client()?;
    let response = api
        .get(&super::url("/machines/get/all"))
        .send()
        .await
        .check()?
        .from_json()
        .await?;

    Ok(response)
}

pub async fn get(id: u32) -> Result<MachineDetails, Box<dyn Error>> {
    let api = super::client()?;
    let response = api
        .get(&super::url(&format!("/machines/get/{}", id)))
        .send()
        .await
        .check()?
        .from_json()
        .await?;

    Ok(response)
}

pub async fn get_by_name(name: &str) -> Result<Option<Machine>, Box<dyn Error>> {
    let machines = list().await?;
    let machine = machines
        .into_iter()
        .find(|m| m.name.to_lowercase() == name.to_lowercase());

    Ok(machine)
}

pub async fn spawned() -> Result<Vec<u32>, Box<dyn Error>> {
    let api = super::client()?;
    let response: Vec<Spawned> = api
        .get(&super::url("/machines/spawned"))
        .send()
        .await
        .check()?
        .from_json()
        .await?;

    Ok(response.iter().map(|m| m.id).collect())
}

pub async fn owns() -> Result<HashMap<u32, (bool, bool)>, Box<dyn Error>> {
    let api = super::client()?;
    let response: Vec<Owned> = api
        .get(&super::url("/machines/owns"))
        .send()
        .await
        .check()?
        .from_json()
        .await?;

    Ok(response
        .iter()
        .map(|m| (m.id, (m.owned_user, m.owned_root)))
        .collect())
}

pub async fn difficulty(id: u32) -> Result<f64, Box<dyn Error>> {
    let api = super::client()?;
    let response: Vec<Difficulty> = api
        .get(&super::url("/machines/difficulty"))
        .send()
        .await
        .check()?
        .from_json()
        .await?;

    if let Some(machine) = response.iter().find(|d| d.id == id) {
        let votes: u64 = machine.difficulty_ratings.iter().sum();
        let weighted: u64 = machine
            .difficulty_ratings
            .iter()
            .enumerate()
            .map(|(score, votes)| (score + 1) as u64 * votes)
            .sum();

        return Ok(weighted as f64 / votes as f64);
    }

    Ok(0f64)
}

pub async fn todos() -> Result<Vec<u32>, Box<dyn Error>> {
    let api = super::client()?;
    let response: Vec<Todo> = api
        .get(&super::url("/machines/todo"))
        .send()
        .await
        .check()?
        .from_json()
        .await?;

    Ok(response.iter().map(|t| t.id).collect())
}

pub async fn assigned() -> Result<Vec<u32>, Box<dyn Error>> {
    let api = super::client()?;
    let response: Vec<Assigned> = api
        .get(&super::url("/machines/assigned"))
        .send()
        .await
        .check()?
        .from_json()
        .await?;

    Ok(response.iter().map(|t| t.id).collect())
}

#[derive(Debug, Serialize)]
pub struct OwnRequest {
    pub id: u32,
    pub flag: String,
    pub difficulty: u8,
}

pub async fn own(id: u32, flag: &str, difficulty: u8) -> Result<StateResponse, Box<dyn Error>> {
    let body = OwnRequest {
        id,
        flag: flag.to_string(),
        difficulty,
    };

    let api = super::client()?;
    let response = api
        .post(&super::url("/machines/own"))
        .json(&body)
        .send()
        .await
        .check()?
        .from_json()
        .await?;

    Ok(response)
}

pub async fn toggle_todo(id: u32) -> Result<Vec<Todo>, Box<dyn Error>> {
    let api = super::client()?;
    let response = api
        .post(&super::url(&format!("/machines/todo/update/{}", id)))
        .send()
        .await
        .check()?
        .from_json()
        .await?;

    Ok(response)
}

pub async fn reset(id: u32) -> Result<StateResponse, Box<dyn Error>> {
    let api = super::client()?;
    let response = api
        .post(&super::url(&format!("/vm/reset/{}", id)))
        .send()
        .await
        .check()?
        .from_json()
        .await?;

    Ok(response)
}

pub async fn start(id: u32) -> Result<StateResponse, Box<dyn Error>> {
    let api = super::client()?;
    let response = api
        .post(&super::url(&format!("/vm/vip/assign/{}", id)))
        .send()
        .await
        .check()?
        .from_json()
        .await?;

    Ok(response)
}

pub async fn stop(id: u32) -> Result<StateResponse, Box<dyn Error>> {
    let api = super::client()?;
    let response = api
        .post(&super::url(&format!("/vm/vip/remove/{}", id)))
        .send()
        .await
        .check()?
        .from_json()
        .await?;

    Ok(response)
}
