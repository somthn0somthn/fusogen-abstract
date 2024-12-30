use crate::contract::{
    Fusogen, FusogenResult
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, Reply};

pub fn instantiate_reply(_deps: DepsMut, _env: Env, module: Fusogen, _reply: Reply) -> FusogenResult {
    Ok(module.response("instantiate_reply"))
}
