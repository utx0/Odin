# Node

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_address** | **String** |  | 
**status** | **String** |  | 
**pub_key_set** | [**crate::models::NodePubKeySet**](NodePubKeySet.md) |  | 
**validator_cons_pub_key** | **String** | the consensus pub key for the node | 
**peer_id** | **String** | the P2PID (:6040/p2pid endpoint) of the node | 
**active_block_height** | **i64** | the block height at which the node became active | 
**status_since** | **i64** | the block height of the current provided information for the node | 
**node_operator_address** | **String** |  | 
**total_bond** | **String** | current node bond | 
**bond_providers** | [**crate::models::NodeBondProviders**](NodeBondProviders.md) |  | 
**signer_membership** | **Vec<String>** | the set of vault public keys of which the node is a member | 
**requested_to_leave** | **bool** |  | 
**forced_to_leave** | **bool** | indicates whether the node has been forced to leave by the network, typically via ban | 
**leave_height** | **i64** |  | 
**ip_address** | **String** |  | 
**version** | **String** | the currently set version of the node | 
**slash_points** | **i64** | the accumlated slash points, reset at churn but excessive slash points may carry over | 
**jail** | [**crate::models::NodeJail**](NodeJail.md) |  | 
**current_award** | **String** |  | 
**observe_chains** | [**Vec<crate::models::ChainHeight>**](ChainHeight.md) | the last observed heights for all chain by the node | 
**preflight_status** | [**crate::models::NodePreflightStatus**](NodePreflightStatus.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


