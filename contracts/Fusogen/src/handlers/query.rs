use crate::{
    contract::{Fusogen, FusogenResult},
    msg::{ConfigResponse, FusogenQueryMsg, HasClaimedResponse},
    state::{CLAIMED_LOCKS, CONFIG},
};
use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};

pub fn query_handler(
    deps: Deps,
    _env: Env,
    _module: &Fusogen,
    msg: FusogenQueryMsg,
) -> FusogenResult<Binary> {
    match msg {
        FusogenQueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        FusogenQueryMsg::HasClaimed { address } => to_json_binary(&query_claim(deps, address)?),
    }
    .map_err(Into::into)
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        source_chain: config.source_chain,
        destination_chain: config.destination_chain,
    })
}

fn query_claim(deps: Deps, address: String) -> StdResult<HasClaimedResponse> {
    let validated = deps.api.addr_validate(&address)?;
    
    let has_claimed = CLAIMED_LOCKS.has(deps.storage, &validated);
    
    Ok(HasClaimedResponse { has_claimed })
}
