# Vault

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block_height** | Option<**i64**> |  | [optional]
**pub_key** | Option<**String**> |  | [optional]
**coins** | [**Vec<crate::models::Coin>**](Coin.md) |  | 
**r#type** | Option<**String**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**status_since** | Option<**i64**> |  | [optional]
**membership** | Option<**Vec<String>**> | the list of node public keys which are members of the vault | [optional]
**chains** | Option<**Vec<String>**> |  | [optional]
**inbound_tx_count** | Option<**i64**> |  | [optional]
**outbound_tx_count** | Option<**i64**> |  | [optional]
**pending_tx_block_heights** | Option<**Vec<i64>**> |  | [optional]
**routers** | [**Vec<crate::models::VaultRouter>**](VaultRouter.md) |  | 
**addresses** | [**Vec<crate::models::VaultAddress>**](VaultAddress.md) |  | 
**frozen** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


