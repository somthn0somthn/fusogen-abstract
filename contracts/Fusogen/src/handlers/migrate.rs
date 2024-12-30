use crate::{
    contract::{
        Fusogen, FusogenResult
    },
    msg::FusogenMigrateMsg,
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env};

/// Handle the app migrate msg
/// The top-level Abstract app does version checking and dispatches to this handler
pub fn migrate_handler(
    _deps: DepsMut,
    _env: Env,
    module: Fusogen,
    _msg: FusogenMigrateMsg,
) -> FusogenResult {
    Ok(module.response("migrate"))
}
