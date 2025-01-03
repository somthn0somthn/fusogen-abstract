use crate::{
    error::FusogenError,
    handlers,
    msg::{
        FusogenExecuteMsg, FusogenInstantiateMsg, FusogenMigrateMsg, FusogenQueryMsg
    },
//    replies::{self, INSTANTIATE_REPLY_ID},
    APP_VERSION, FUSOGEN_ID,
};
use abstract_app::{objects::dependency::StaticDependency, std::IBC_CLIENT, AppContract};
use cosmwasm_std::Response;

/// The type of the result returned by your app's entry points.
pub type FusogenResult<T = Response> = Result<T, FusogenError>;

/// The type of the app that is used to build your app and access the Abstract SDK features.
pub type Fusogen =
    AppContract<FusogenError, FusogenInstantiateMsg, FusogenExecuteMsg, FusogenQueryMsg, FusogenMigrateMsg>;

const APP: Fusogen = Fusogen::new(FUSOGEN_ID, APP_VERSION, None)
    .with_instantiate(handlers::instantiate_handler)
    .with_execute(handlers::execute_handler)
    .with_query(handlers::query_handler)
    .with_dependencies(&[StaticDependency::new(
        IBC_CLIENT,
        &[abstract_ibc_client::contract::CONTRACT_VERSION],
    )])
    //.with_migrate(handlers::migrate_handler)
    //.with_replies(&[(INSTANTIATE_REPLY_ID, replies::instantiate_reply)])
    .with_ibc_callback(crate::ibc::ibc_callback)
    .with_module_ibc(crate::ibc::receive_module_ibc);
    

// Export handlers
#[cfg(feature = "export")]
abstract_app::export_endpoints!(APP, Fusogen);

abstract_app::cw_orch_interface!(APP, Fusogen, FusogenInterface);


//TODO :: Is this section OK?? See docs?
#[cfg(not(target_arch = "wasm32"))]
use abstract_app::std::account::ModuleInstallConfig;
#[cfg(not(target_arch = "wasm32"))]
impl<Chain: cw_orch::environment::CwEnv> abstract_app::abstract_interface::DependencyCreation
    for crate::FusogenInterface<Chain>
{
    type DependenciesConfig = cosmwasm_std::Empty;

    fn dependency_install_configs(
        _configuration: Self::DependenciesConfig,
    ) -> Result<Vec<ModuleInstallConfig>, abstract_app::abstract_interface::AbstractInterfaceError>
    {
        // This ensures the IBC client is installed whenever Fusogen is installed
        Ok(vec![ModuleInstallConfig::new(
            abstract_app::objects::module::ModuleInfo::from_id(
                IBC_CLIENT,
                abstract_ibc_client::contract::CONTRACT_VERSION.into(),
            )?,
            None,
        )])
    }
}

//CONT :: update error.rs

//CONT :: deal with abstract-ib

//CONT :: pull over most of ping-pong functionality, but with minor changes
//     -- to incorporate placeholder values signifying token locking and minting
//     -- (e.g. just some static state updates mocking out these functions)
//CONT -- incorporate minting on XION side
//CONT -- incorporate payment & locking on Juno side

//CONT :: discuss the instantiation and execution handlers with chat gpt
//     -- basically, I think instantionation would take cw20 contract addrs
//     -- to define the DAO tokens being merged. You'll likely also have to handle
//     -- cw20 contract generation on the XION for the new token as well 
//     -- IS THERE CUSTOM LOGIC TO SPECIFY THE CHAIN HERE???
//     -- execution, on the other hand will take the XION addr as an argument

//CONT :: can you do the first round of testing with queries stubbed out? If so do so

//CONT :: add CW20 token reception last as there many some manaual refactorign required to msg types
//     -- see "Defining a Custom Module Endpoint" here :: https://docs.abstract.money/4_get_started/custom_endpoints.html?highlight=custom#defining-a-custom-module-endpoint

//TODO :: Add logic for removing from CLAIMED_LOCKS if destination should return a fai

//TODO :: possibly update design post initial steps:

/* pub const CONFIG: Item<Config> = Item::new("config");
pub const CLAIMED_LOCKS: Map<&Addr, ()> = Map::new("claimed_locks");
pub const MERGER_PAIRS: Map<(&TruncatedChainId, &TruncatedChainId), MergerState> = Map::new("merger_pairs");

#[cw_serde]
pub struct Config {
    pub dao_authority: Addr,  // DAO's control address
}

#[cw_serde]
pub enum MergerState {
    Pending,
    Active,
    Completed
} */


