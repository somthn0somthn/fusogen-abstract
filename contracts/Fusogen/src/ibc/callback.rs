use abstract_app::{
    sdk::AbstractResponse,
    std::ibc::{Callback, IbcResult},
};
use cosmwasm_std::{DepsMut, Env};

use crate::contract::{
    Fusogen, FusogenResult
};

pub fn ibc_callback(
    _deps: DepsMut,
    _env: Env,
    module: Fusogen,
    _callback: Callback,
    _result: IbcResult,
) -> FusogenResult {
    Ok(module.response("callback"))
}