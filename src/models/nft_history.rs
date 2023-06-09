/*
 * StacksData API
 *
 * Stacksdata provides various information about Stacks blockchain through REST API. The data is as real-time as it appears on Stacks node. Finalized blocks, transactions and corresponding events are visible once the consensus is reached. This document describes what information is available and how to query it to produce various reports and dashboards.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: info@stacksdata.info
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NftHistory {
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    #[serde(rename = "recipient", skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(rename = "contract", skip_serializing_if = "Option::is_none")]
    pub contract: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "block_height", skip_serializing_if = "Option::is_none")]
    pub block_height: Option<i32>,
    #[serde(rename = "block_time", skip_serializing_if = "Option::is_none")]
    pub block_time: Option<String>,
    #[serde(rename = "tx_hash", skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    #[serde(rename = "asset_event_type", skip_serializing_if = "Option::is_none")]
    pub asset_event_type: Option<String>,
}

impl NftHistory {
    pub fn new() -> NftHistory {
        NftHistory {
            sender: None,
            recipient: None,
            contract: None,
            value: None,
            block_height: None,
            block_time: None,
            tx_hash: None,
            asset_event_type: None,
        }
    }
}


