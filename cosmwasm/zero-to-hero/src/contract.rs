#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, Poll, CONFIG, POLLS};


const CONTRACT_NAME: &str = "crates.io:zero-to-hero";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    //dependencies of the contract. DepsMut means it is mutable
    _deps: DepsMut,
    // store environmental variables around the contract
    _env: Env,
    //contains metadata of the message
    _info: MessageInfo,
    // actual instantiate message
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(_deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // This will error, if the user gives an invalid address, "foo"
    let validated_admin_address = _deps.api.addr_validate(&_msg.admin_address)?;

    let config = Config {
        admin_address: validated_admin_address, // Set to the validated address
    };

    // ? -> assert it because we want it to sucess. you can use it when function returns std result
    CONFIG.save(_deps.storage, &config)?;

    // Result<Response>
    // use add_attribute to send text metadata
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

fn execute_create_poll(deps: DepsMut, _env: Env, _info: MessageInfo, question: String) -> Result<Response, ContractError>{

    // Does the map have a key of this value
    if POLLS.has(deps.storage, question.clone()){
        // If it does, we want to error!
        return Err(ContractError::CustomError {
            val: "Key already taken".to_string(),
        });
    }

    let poll = Poll { question: question.clone(), yes_votes:0, no_votes:0};

    POLLS.save(deps.storage, question, &poll)?;

    Ok(Response::new().add_attribute("action", "create_poll"))

}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}


#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::msg::{ExecuteMsg, GetPollResponse, InstantiateMsg, QueryMsg};
    use crate::state::{Config, Poll};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, from_binary, Addr};

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("addr1", &[]);
        let msg = InstantiateMsg {
            admin_address: "addr1".to_string(), // String, String::from("addr1")
        };

        let resp = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
        assert_eq!(resp.attributes, vec![attr("action", "instantiate")]);

        let msg = QueryMsg::GetConfig {};
        let resp = query(deps.as_ref(), env, msg).unwrap();
        let config: Config = from_binary(&resp).unwrap();
        assert_eq!(
            config,
            Config {
                admin_address: Addr::unchecked("addr1")
            }
        );
    }


}
