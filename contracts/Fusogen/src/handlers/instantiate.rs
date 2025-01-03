use crate::{
    contract::{
        Fusogen, FusogenResult
    },
    msg::FusogenInstantiateMsg,
    state::{Config, CONFIG},
};
use abstract_app::objects::TruncatedChainId;
use abstract_app::sdk::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub const XION_CHAIN: &str = "xion-1";

pub fn instantiate_handler(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    module: Fusogen,
    msg: FusogenInstantiateMsg,
) -> FusogenResult<Response> {
    let destination_chain = TruncatedChainId::from_chain_id(XION_CHAIN);

    let config: Config = Config {
        source_chain: msg.source_chain,
        destination_chain: destination_chain, 
    };

    CONFIG.save(deps.storage, &config)?;
    
    Ok(module
        .response("instantiate")
        .add_attribute("source_chain", config.source_chain.to_string())
        .add_attribute("destination_chain", config.destination_chain.to_string()))
}
