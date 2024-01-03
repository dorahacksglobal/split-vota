//! Split Contract
//! Author: noodles@dorahacks.com
//! Version: 0.1.0
//! License: Apache-2.0

use cosmwasm_std::{
    Addr, BankMsg, Coin, Deps, DepsMut, Empty, Env, MessageInfo, Order, Response, StdResult,
    Uint128,
};
use cw_storage_plus::Map;
use sylvia::contract;

#[cfg(not(feature = "library"))]
use sylvia::entry_points;

use crate::{error::ContractError, responses::AdminListResp};

pub struct SplitContract<'a> {
    pub(crate) admins: Map<'a, &'a Addr, Empty>,
}

#[cfg_attr(not(feature = "library"), entry_points)]
#[contract]
#[error(ContractError)]
impl SplitContract<'_> {
    pub const fn new() -> Self {
        Self {
            admins: Map::new("admins"),
        }
    }

    #[msg(instantiate)]
    pub fn instantiate(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        admins: Vec<String>,
    ) -> Result<Response, ContractError> {
        let (deps, _, _info) = ctx;

        for admin in admins {
            let admin = deps.api.addr_validate(&admin)?;
            self.admins.save(deps.storage, &admin, &Empty {})?;
        }
        Ok(Response::new())
    }

    // ============= Query ============= //
    #[msg(query)]
    pub fn admin_list(&self, ctx: (Deps, Env)) -> StdResult<AdminListResp> {
        let (deps, _) = ctx;

        let admins: Result<_, _> = self
            .admins
            .keys(deps.storage, None, None, Order::Ascending)
            .map(|addr| addr.map(String::from))
            .collect();

        Ok(AdminListResp { admins: admins? })
    }

    // ============= Execute ============= //
    #[msg(exec)]
    pub fn add_member(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        admin: String,
    ) -> Result<Response, ContractError> {
        let (deps, _, info) = ctx;
        if !self.admins.has(deps.storage, &info.sender) {
            return Err(ContractError::Unauthorized {
                sender: info.sender,
            });
        }

        let admin = deps.api.addr_validate(&admin)?;
        if self.admins.has(deps.storage, &admin) {
            return Err(ContractError::NoDupAddress { address: admin });
        }

        self.admins.save(deps.storage, &admin, &Empty {})?;

        let resp = Response::new()
            .add_attribute("action", "add_member")
            .add_event(Event::new("admin_added").add_attribute("addr", admin));
        Ok(resp)
    }

    #[msg(exec)]
    pub fn split(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        amounts: Vec<(Addr, Uint128, String)>,
    ) -> Result<Response, ContractError> {
        let (deps, _env, info) = ctx;
        if !self.admins.has(deps.storage, &info.sender) {
            return Err(ContractError::Unauthorized {
                sender: info.sender,
            });
        }

        let mut messages: Vec<BankMsg> = vec![];
        for (receiver, amount, denom) in amounts {
            let transfer_message = BankMsg::Send {
                to_address: receiver.into_string(),
                amount: vec![Coin::new(amount.u128(), denom.clone())],
            };
            messages.push(transfer_message);
        }

        let resp = Response::new().add_messages(messages);
        Ok(resp)
    }

    #[msg(exec)]
    pub fn withdraw_remains(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
    ) -> Result<Response, ContractError> {
        let (deps, env, info) = ctx;
        if !self.admins.has(deps.storage, &info.sender) {
            return Err(ContractError::Unauthorized {
                sender: info.sender,
            });
        }

        let contract_address = env.contract.address.clone();
        let amount = deps.querier.query_all_balances(contract_address)?;

        let message = BankMsg::Send {
            to_address: info.sender.to_string(),
            amount,
        };

        let resp = Response::new().add_message(message);
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use crate::contract::entry_points::{execute, instantiate, query};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_json, Coin, SubMsg, Uint128};

    use super::*;

    #[test]
    fn admin_list_query() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        instantiate(
            deps.as_mut(),
            env.clone(),
            mock_info("sender", &[]),
            InstantiateMsg {
                admins: vec!["admin1".to_owned(), "admin2".to_owned()],
            },
        )
        .unwrap();

        let msg = QueryMsg::AdminList {};
        let resp = query(deps.as_ref(), env, ContractQueryMsg::SplitContract(msg)).unwrap();
        let resp: AdminListResp = from_json(&resp).unwrap();
        assert_eq!(
            resp,
            AdminListResp {
                admins: vec!["admin1".to_owned(), "admin2".to_owned()],
            }
        );
    }

    #[test]
    fn test_split() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        // Instantiate
        instantiate(
            deps.as_mut(),
            env.clone(),
            mock_info("sender", &[]),
            InstantiateMsg {
                admins: vec!["admin1".to_owned(), "admin2".to_owned()],
            },
        )
        .unwrap();

        let msg = ExecMsg::Split {
            amounts: vec![
                (
                    Addr::unchecked("user2"),
                    Uint128::from(160000u128),
                    "inj".to_string(),
                ),
                (
                    Addr::unchecked("user2"),
                    Uint128::from(10000u128),
                    "ibc/1234".to_string(),
                ),
                (
                    Addr::unchecked("user3"),
                    Uint128::from(260000u128),
                    "inj".to_string(),
                ),
                (
                    Addr::unchecked("user3"),
                    Uint128::from(60000u128),
                    "ibc/1234".to_string(),
                ),
            ],
        };
        let resp = execute(
            deps.as_mut(),
            env.clone(),
            mock_info("sender", &[]),
            ContractExecMsg::SplitContract(msg),
        )
        .unwrap();
        assert_eq!(resp.messages.len(), 4);
        assert_eq!(
            resp.messages[0],
            SubMsg::new(BankMsg::Send {
                to_address: "user2".to_string(),
                amount: vec![Coin::new(160000u128, "inj")],
            }),
        );
    }
}
