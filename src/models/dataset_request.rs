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
pub struct DatasetRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "schema")]
    pub schema: Vec<crate::models::Column>,
}

impl DatasetRequest {
    pub fn new(name: String, schema: Vec<crate::models::Column>) -> DatasetRequest {
        DatasetRequest {
            name,
            schema,
        }
    }
}


