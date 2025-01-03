use abstract_app::{
    sdk::AbstractResponse,
    std::ibc::ModuleIbcInfo,
};
use cosmwasm_std::{ensure_eq, from_json, Binary, DepsMut, Env, Response};

use crate::{
    contract::{
        Fusogen, FusogenResult
    },
    msg::FusogenIbcMsg,
    error::FusogenError,
    state::CLAIMED_LOCKS,
};

pub fn receive_module_ibc(
    deps: DepsMut,
    _env: Env,
    module: Fusogen,
    source_module: ModuleIbcInfo,
    msg: Binary,
) -> FusogenResult<Response> {
    let this_module_info = module.module_info()?;
    ensure_eq!(
        source_module.module,
        this_module_info,
        FusogenError::UnauthorizedModule {
            source_module: source_module.module.clone()
        }
    );

    //TODO :: connect error handling here?
    let ibc_msg: FusogenIbcMsg = from_json(msg)?;

    let user_addr = deps.api.addr_validate(&ibc_msg.user_addr)?;
    CLAIMED_LOCKS.save(deps.storage, &user_addr, &())?;

    let resp = module.response("receive_module_ibc")
        .add_attribute("action", "receive_module_ibc")
        .add_attribute("user_addr", user_addr.to_string());

    Ok(resp)
}
