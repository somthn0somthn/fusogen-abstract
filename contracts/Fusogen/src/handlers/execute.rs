use crate::{
    contract::{
        Fusogen, FusogenResult
    },
    msg::FusogenExecuteMsg,
    state::{CONFIG, COUNT},
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, MessageInfo};

pub fn execute_handler(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    module: Fusogen,
    msg: FusogenExecuteMsg,
) -> FusogenResult {
    match msg {
        FusogenExecuteMsg::UpdateConfig {} => update_config(deps, env, info, module),
        FusogenExecuteMsg::Increment {} => increment(deps, module),
        FusogenExecuteMsg::Reset { count } => reset(deps, env, info, count, module),
    }
}

/// Update the configuration of the app
fn update_config(deps: DepsMut, env: Env, msg_info: MessageInfo, module: Fusogen) -> FusogenResult {
    // Only the admin should be able to call this
    module.admin.assert_admin(deps.as_ref(), &env, &msg_info.sender)?;
    let mut _config = CONFIG.load(deps.storage)?;

    Ok(module.response("update_config"))
}

fn increment(deps: DepsMut, module: Fusogen) -> FusogenResult {
    COUNT.update(deps.storage, |count| FusogenResult::Ok(count + 1))?;

    Ok(module.response("increment"))
}

fn reset(deps: DepsMut, env: Env, info: MessageInfo, count: i32, module: Fusogen) -> FusogenResult {
    module.admin.assert_admin(deps.as_ref(), &env, &info.sender)?;
    COUNT.save(deps.storage, &count)?;

    Ok(module.response("reset"))
}
