use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub gov_contract: String,   // pylon gov contract
    pub pylon_token: String,    // pylon token address
    pub whitelist: Vec<String>, // whitelisted contract addresses to spend distributor
    pub spend_limit: Uint128,   // spend limit per each `spend` request
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig { spend_limit: Option<Uint128> },
    Spend { recipient: String, amount: Uint128 },
    AddDistributor { distributor: String },
    RemoveDistributor { distributor: String },
}

/// We currently take no arguments for migrations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub gov_contract: String,
    pub pylon_token: String,
    pub whitelist: Vec<String>,
    pub spend_limit: Uint128,
}
