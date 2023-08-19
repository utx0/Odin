# NetworkResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bond_reward_rune** | **String** | total amount of RUNE awarded to node operators | 
**burned_bep_2_rune** | **String** | total of burned BEP2 RUNE | 
**burned_erc_20_rune** | **String** | total of burned ERC20 RUNE | 
**total_bond_units** | **String** | total bonded RUNE | 
**effective_security_bond** | **String** | effective security bond used to determine maximum pooled RUNE | 
**total_reserve** | **String** | total reserve RUNE | 
**vaults_migrating** | **bool** | Returns true if there exist RetiringVaults which have not finished migrating funds to new ActiveVaults | 
**gas_spent_rune** | **String** | Sum of the gas the network has spent to send outbounds | 
**gas_withheld_rune** | **String** | Sum of the gas withheld from users to cover outbound gas | 
**outbound_fee_multiplier** | Option<**String**> | Current outbound fee multiplier, in basis points | [optional]
**native_outbound_fee_rune** | **String** | the outbound transaction fee in rune, converted from the NativeOutboundFeeUSD mimir | 
**native_tx_fee_rune** | **String** | the native transaction fee in rune, converted from the NativeTransactionFeeUSD mimir | 
**tns_register_fee_rune** | **String** | the thorname register fee in rune, converted from the TNSRegisterFeeUSD mimir | 
**tns_fee_per_block_rune** | **String** | the thorname fee per block in rune, converted from the TNSFeePerBlockUSD mimir | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


