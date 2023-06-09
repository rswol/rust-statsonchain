# Rust API client for rust-statsonchain

Stacksdata provides various information about Stacks blockchain through REST API. The data is as real-time as it appears on Stacks node. Finalized blocks, transactions and corresponding events are visible once the consensus is reached. This document describes what information is available and how to query it to produce various reports and dashboards.


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.0.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `rust-statsonchain` and add the following to `Cargo.toml` under `[dependencies]`:

```
rust-statsonchain = { path = "./rust-statsonchain" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.stacksdata.info*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*NftsApi* | [**address_history**](docs/NftsApi.md#address_history) | **GET** /nft/address/{address}/history | 
*NftsApi* | [**assets**](docs/NftsApi.md#assets) | **GET** /nft/assets/{recipient} | 
*NftsApi* | [**contracts**](docs/NftsApi.md#contracts) | **GET** /nft/contracts | 
*NftsApi* | [**floor**](docs/NftsApi.md#floor) | **GET** /nft/contracts/{contract}/floor | 
*NftsApi* | [**owners**](docs/NftsApi.md#owners) | **GET** /nft/contracts/{contract}/owners | 
*NftsApi* | [**prices**](docs/NftsApi.md#prices) | **GET** /nft/contracts/{contract}/price | 
*NftsApi* | [**token**](docs/NftsApi.md#token) | **GET** /nft/contracts/{contract}/tokens/{token} | 
*NftsApi* | [**tokens**](docs/NftsApi.md#tokens) | **GET** /nft/contracts/{contract}/tokens | 
*NftsApi* | [**total_volumes**](docs/NftsApi.md#total_volumes) | **GET** /nft/volume | 
*NftsApi* | [**transactions**](docs/NftsApi.md#transactions) | **GET** /nft/contracts/{contract}/transactions | 
*NftsApi* | [**volumes**](docs/NftsApi.md#volumes) | **GET** /nft/contracts/{contract}/volume | 
*SqlApi* | [**run_portfolio_tx_history**](docs/SqlApi.md#run_portfolio_tx_history) | **GET** /v1/owner/{address}/portfolio | 
*SqlApi* | [**run_sql**](docs/SqlApi.md#run_sql) | **POST** /v1/sql | 
*SqlApi* | [**run_sql_v2**](docs/SqlApi.md#run_sql_v2) | **POST** /v1/sql-v2 | 
*SqlApi* | [**run_tx_history**](docs/SqlApi.md#run_tx_history) | **GET** /v1/owner/{address}/history | 
*StacksApi* | [**arbitrage_mia**](docs/StacksApi.md#arbitrage_mia) | **GET** /v1/mia-arb | 
*StacksApi* | [**arbitrage_nyc**](docs/StacksApi.md#arbitrage_nyc) | **GET** /v1/nyc-arb | 
*StacksApi* | [**arkadiko_vaults**](docs/StacksApi.md#arkadiko_vaults) | **GET** /v1/owner/{address}/arkadiko-vaults | 
*StacksApi* | [**block**](docs/StacksApi.md#block) | **GET** /v1/block | 
*StacksApi* | [**coins**](docs/StacksApi.md#coins) | **GET** /v1/coins | 
*StacksApi* | [**deserialize**](docs/StacksApi.md#deserialize) | **GET** /v1/deserialize/{value} | 
*StacksApi* | [**price**](docs/StacksApi.md#price) | **GET** /v1/price | 
*StacksApi* | [**run**](docs/StacksApi.md#run) | **POST** /v1/run | 
*StacksApi* | [**ts**](docs/StacksApi.md#ts) | **POST** /v1/ts | 
*UserReportsApi* | [**create_dataset**](docs/UserReportsApi.md#create_dataset) | **POST** /cp-v1/dataset | 
*UserReportsApi* | [**create_user_report**](docs/UserReportsApi.md#create_user_report) | **POST** /cp-v1/user-report | 
*UserReportsApi* | [**delete_user_report**](docs/UserReportsApi.md#delete_user_report) | **DELETE** /cp-v1/user-report/{name} | 
*UserReportsApi* | [**drop_user**](docs/UserReportsApi.md#drop_user) | **DELETE** /cp-v1/user | 
*UserReportsApi* | [**get_user_report**](docs/UserReportsApi.md#get_user_report) | **GET** /cp-v1/user-report/{name} | 
*UserReportsApi* | [**list_datasets**](docs/UserReportsApi.md#list_datasets) | **GET** /cp-v1/dataset | 
*UserReportsApi* | [**list_user_reports**](docs/UserReportsApi.md#list_user_reports) | **GET** /cp-v1/user-report | 
*UserReportsApi* | [**list_users**](docs/UserReportsApi.md#list_users) | **GET** /cp-v1/user | 
*UserReportsApi* | [**register_user**](docs/UserReportsApi.md#register_user) | **POST** /cp-v1/user | 
*UserReportsApi* | [**run_user_report**](docs/UserReportsApi.md#run_user_report) | **POST** /cp-v1/run-user-report/{name} | 
*UserReportsApi* | [**upload**](docs/UserReportsApi.md#upload) | **POST** /cp-v1/dataset/{name} | 


## Documentation For Models

 - [ApiResponseUserReportInfo](docs/ApiResponseUserReportInfo.md)
 - [Arbitrage](docs/Arbitrage.md)
 - [BlockSpec](docs/BlockSpec.md)
 - [CoinInfo](docs/CoinInfo.md)
 - [Column](docs/Column.md)
 - [Dataset](docs/Dataset.md)
 - [DatasetInfo](docs/DatasetInfo.md)
 - [DatasetRequest](docs/DatasetRequest.md)
 - [DropUserRequest](docs/DropUserRequest.md)
 - [NftContract](docs/NftContract.md)
 - [NftFloor](docs/NftFloor.md)
 - [NftHistory](docs/NftHistory.md)
 - [NftHolding](docs/NftHolding.md)
 - [NftOwner](docs/NftOwner.md)
 - [NftPrice](docs/NftPrice.md)
 - [NftToken](docs/NftToken.md)
 - [NftTransactions](docs/NftTransactions.md)
 - [NftVolume](docs/NftVolume.md)
 - [Pagination](docs/Pagination.md)
 - [PaginationResponsePortfolioEntry](docs/PaginationResponsePortfolioEntry.md)
 - [PaginationResponseTaxmanEntry](docs/PaginationResponseTaxmanEntry.md)
 - [PriceSpec](docs/PriceSpec.md)
 - [RegisterUserRequest](docs/RegisterUserRequest.md)
 - [RunRequest](docs/RunRequest.md)
 - [RunSqlRequest](docs/RunSqlRequest.md)
 - [TableSpec](docs/TableSpec.md)
 - [User](docs/User.md)
 - [UserReport](docs/UserReport.md)
 - [UserReportInfo](docs/UserReportInfo.md)
 - [UserReportRequest](docs/UserReportRequest.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

info@stacksdata.info

