use abstract_app::{
    sdk::AbstractResponse,
    std::ibc::{Callback, IbcResult},
};
use cosmwasm_std::{from_json, DepsMut, Env, Response};

use crate::{
    contract::{Fusogen, FusogenResult}, 
    msg::FusogenCallbackMsg,
    error::FusogenError,
};

pub fn ibc_callback(
    _deps: DepsMut,
    _env: Env,
    module: Fusogen,
    callback: Callback,
    result: IbcResult,
) -> FusogenResult<Response> {
    match from_json(callback.msg)? {
        FusogenCallbackMsg::AcknowledgedLock { user_addr, success } => {
            let exec_events = result.get_execute_events()?;

            let received = exec_events.into_iter().find(|e| {
                e.ty == "wasm"
                    && e.attributes
                        .iter()
                        .any(|a| a.key == "action" && a.value == "receive_module_ibc")
            });

            if received.is_some() {
                Ok(module
                    .response("ibc_callback_ack")
                    .add_attribute("callback", "AcknowledgedLock")
                    .add_attribute("user", user_addr)
                    .add_attribute("final_result", success.to_string()))
            } else {
                
                Err(FusogenError::ClaimVerificationFailed { 
                    user: user_addr,
                    reason: "Claim verification not received on destination chain".to_string()
                })
            }
        }        
    }
}