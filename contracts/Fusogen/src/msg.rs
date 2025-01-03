use crate::contract::Fusogen;
use abstract_app::objects::TruncatedChainId;
use cosmwasm_schema::QueryResponses;

// This is used for type safety and re-exporting the contract endpoint structs.
abstract_app::app_msg_types!(Fusogen, FusogenExecuteMsg, FusogenQueryMsg);

/// App instantiate message
#[cosmwasm_schema::cw_serde]
pub struct FusogenInstantiateMsg {
    pub source_chain: TruncatedChainId,
    pub destination: TruncatedChainId,
}

/// App execute messages
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum FusogenExecuteMsg {
    ClaimLock {},
}

/* #[cosmwasm_schema::cw_serde]
pub struct FusogenMigrateMsg {} */

/// App query messages
#[cosmwasm_schema::cw_serde]
#[derive(QueryResponses, cw_orch::QueryFns)]
pub enum FusogenQueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(HasClaimedResponse)]
    //why string type and not Addr? because JSON
    HasClaimed { address: String},
}

#[cosmwasm_schema::cw_serde]
pub struct FusogenMigrateMsg {}

#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {
    pub source_chain: TruncatedChainId,
    pub destination_chain: TruncatedChainId,
}

#[cosmwasm_schema::cw_serde]
pub struct HasClaimedResponse {
    pub has_claimed: bool,
}

#[cosmwasm_schema::cw_serde]
pub struct FusogenIbcMsg {
    /// Simple example: say "Hey, I locked tokens for user X".
    pub user_addr: String,
}

#[cosmwasm_schema::cw_serde]
pub enum FusogenCallbackMsg {
    /// Example callback indicating success/failure of some action on the destination chain
    AcknowledgedLock { user_addr: String, success: bool },
}


//TODO :: Consider the following points
//     -- Do you want an updateconfig execute message? I think not right now
//     -- Is the IBC flow sufficient currently, will this simply query the CLAIMED_LOCKS Item 
//              and respond
//     -- Do you want to add a ratio merge ratio to the config?
//     -- Extend the FusogenIbcMsg to include amount_locked, lock_timestamp ?
//
// /* #[cosmwasm_schema::cw_serde]
// pub struct FusogenIbcMsg {
//    pub user_addr: String,
//    pub amount_locked: Uint128,
//    pub lock_timestamp: u64,
// } */
