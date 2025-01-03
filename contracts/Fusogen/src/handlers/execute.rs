use crate::{
    contract::{Fusogen, FusogenResult},
    msg::{FusogenExecuteMsg, FusogenIbcMsg, FusogenCallbackMsg},
    state::CONFIG,
};
use abstract_app::{
    sdk::{IbcClient, IbcInterface},
    std::ibc::Callback,
    traits::AbstractResponse,
};
use cosmwasm_std::{CosmosMsg, DepsMut, Env, MessageInfo};

pub fn execute_handler(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    module: Fusogen,
    msg: FusogenExecuteMsg,
) -> FusogenResult {
    match msg {
        FusogenExecuteMsg::ClaimLock {} => claim_lock(deps, env, info, module),
    }
}

pub(crate) fn claim_lock(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    module: Fusogen,
) -> FusogenResult {
    //TODO :: attach error handling here?
    let config = CONFIG.load(deps.storage)?;

    let fusogen_msg = FusogenIbcMsg {
        user_addr: info.sender.to_string(),
    };

    let callback = Callback::new(&FusogenCallbackMsg::AcknowledgedLock { 
        user_addr: info.sender.to_string(),
        success: true,
    })?;

    let self_module_info = module.module_info()?;
    //this call is different from ping-pong, taking an &env arg, see version diffs?
    let ibc_client: IbcClient<_> = module.ibc_client(deps.as_ref(), &env);
    
    let ibc_action: CosmosMsg = ibc_client.module_ibc_action(
        config.destination_chain,
        self_module_info, 
        &fusogen_msg,
        Some(callback)
    )?;

    Ok(module
        .response("claim_lock")
        .add_attribute("action", "claim_lock")
        .add_attribute("claimer", info.sender)
        .add_message(ibc_action))
}
