/**
* This file was automatically generated by @cosmwasm/ts-codegen@1.11.1.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
//@ts-nocheck

export type Uint128 = string;
export interface InstantiateMsg {
  context?: string | null;
  estimated_apy: number;
  estimated_monthly_income: Coin;
  image_uri: string;
  name: string;
  price_per_share: Coin;
  property_contract_address?: string | null;
  royalty_fee: number;
  status: string;
  subcategory: string;
  symbol: string;
  total_shares: Uint128;
}
export interface Coin {
  amount: Uint128;
  denom: string;
}
export type ExecuteMsg = {
  cw404_execute_msg: ExecuteMsg1;
} | {
  list_shares: {
    amount: Uint128;
  };
} | {
  buy_shares: {
    amount: Uint128;
    from?: Addr | null;
  };
} | {
  claim_payout: {};
} | {
  receive_payment: Cw20ReceiveMsg;
} | {
  receive_payment_native: {};
};
export type ExecuteMsg1 = {
  transfer_from: {
    amount: Uint128;
    owner: string;
    recipient: string;
  };
} | {
  transfer: {
    amount: Uint128;
    recipient: string;
  };
} | {
  transfer_nft: {
    recipient: string;
    token_id: Uint128;
  };
} | {
  send: {
    amount: Uint128;
    contract: string;
    msg: Binary;
  };
} | {
  send_nft: {
    contract: string;
    msg: Binary;
    token_id: Uint128;
  };
} | {
  increase_allowance: {
    amount: Uint128;
    expires?: Expiration | null;
    spender: string;
  };
} | {
  approve: {
    expires?: Expiration | null;
    spender: string;
    token_id: Uint128;
  };
} | {
  approve_all: {
    expires?: Expiration | null;
    operator: string;
  };
} | {
  revoke_all: {
    operator: string;
  };
} | {
  generate_nft_event: {
    recipient: string;
    sender: string;
    token_id: Uint128;
  };
} | {
  generate_nft_mint_event: {
    recipient: string;
    sender: string;
    token_id: Uint128;
  };
} | {
  generate_nft_burn_event: {
    sender: string;
    token_id: Uint128;
  };
} | {
  set_whitelist: {
    state: boolean;
    target: string;
  };
} | {
  set_lock: {
    state: boolean;
    token_id: Uint128;
  };
} | {
  set_base_token_uri: {
    uri: string;
  };
};
export type Binary = string;
export type Expiration = {
  at_height: number;
} | {
  at_time: Timestamp;
} | {
  never: {};
};
export type Timestamp = Uint64;
export type Uint64 = string;
export type Addr = string;
export interface Cw20ReceiveMsg {
  amount: Uint128;
  msg: Binary;
  sender: string;
}
export type QueryMsg = {
  cw404_query_msg: QueryMsg1;
} | {
  get_property_details: {};
} | {
  get_share_holders: {};
} | {
  get_share_balance: {
    address: Addr;
  };
} | {
  outstanding_shares: {};
};
export type QueryMsg1 = {
  owner_of: {
    include_expired?: boolean | null;
    token_id: string;
  };
} | {
  is_locked: {
    token_id: string;
  };
} | {
  user_info: {
    address: string;
  };
} | {
  allowance: {
    owner: string;
    spender: string;
  };
} | {
  extended_info: {
    token_id: string;
  };
} | {
  num_tokens: {};
} | {
  contract_info: {};
} | {
  nft_info: {
    token_id: string;
  };
} | {
  balance: {
    address: string;
  };
} | {
  token_info: {};
} | {
  all_nft_info: {
    include_expired?: boolean | null;
    token_id: string;
  };
} | {
  tokens: {
    limit?: number | null;
    owner: string;
    start_after?: string | null;
  };
} | {
  all_tokens: {
    limit?: number | null;
    start_after?: string | null;
  };
} | {
  minter: {};
};
export type NullableNull = null;
export interface OutstandingSharesResponse {
  outstanding_shares: Uint128;
  remaining_shares: Uint128;
  total_shares: Uint128;
}
