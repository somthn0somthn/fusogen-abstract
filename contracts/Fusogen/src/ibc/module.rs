use abstract_app::std::ibc::ModuleIbcInfo;
use cosmwasm_std::{Binary, DepsMut, Env, Response, from_json};

use crate::{
    contract::{
        Fusogen, FusogenResult
    },
    msg::IbcMsg,
};

pub fn receive_module_ibc(
    _deps: DepsMut,
    _env: Env,
    _module: Fusogen,
    _source_module: ModuleIbcInfo,
    msg: Binary,
) -> FusogenResult {
    let _msg: IbcMsg = from_json(&msg)?;
    // do something with received _msg
    Ok(Response::new())
}
