use cosmwasm_schema::write_api;

use tokifykyc::msg::{ExecuteMsg, QueryMsg};

fn main() {
    write_api! {
        execute: ExecuteMsg,
        query: QueryMsg
    }
}
