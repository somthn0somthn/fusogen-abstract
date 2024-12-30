use crate::{
    contract::{
        Fusogen, FusogenResult
    },
    msg::FusogenInstantiateMsg,
    state::{Config, CONFIG, COUNT},
};

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn instantiate_handler(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _module: Fusogen,
    msg: FusogenInstantiateMsg,
) -> FusogenResult {
    let config: Config = Config {};

    CONFIG.save(deps.storage, &config)?;
    COUNT.save(deps.storage, &msg.count)?;
    Ok(Response::new())
}
