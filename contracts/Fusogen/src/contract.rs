use crate::{
    error::FusogenError,
    handlers,
    msg::{
        FusogenExecuteMsg, FusogenInstantiateMsg, FusogenMigrateMsg, FusogenQueryMsg
    },
    replies::{self, INSTANTIATE_REPLY_ID},
    APP_VERSION, FUSOGEN_ID,
};

use abstract_app::AppContract;
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
    .with_migrate(handlers::migrate_handler)
    .with_replies(&[(INSTANTIATE_REPLY_ID, replies::instantiate_reply)])
    .with_ibc_callback(crate::ibc::ibc_callback)
    .with_module_ibc(crate::ibc::receive_module_ibc)
    .with_dependencies(&[]);

// Export handlers
#[cfg(feature = "export")]
abstract_app::export_endpoints!(APP, Fusogen);

abstract_app::cw_orch_interface!(APP, Fusogen, FusogenInterface);

// TODO: add to docmuentation
// https://linear.app/abstract-sdk/issue/ABS-414/add-documentation-on-dependencycreation-trait
#[cfg(not(target_arch = "wasm32"))]
impl<Chain: cw_orch::environment::CwEnv> abstract_interface::DependencyCreation
    for crate::FusogenInterface<Chain>
{
    type DependenciesConfig = cosmwasm_std::Empty;
}
