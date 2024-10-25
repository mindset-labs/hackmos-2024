use crate::ContractError;


pub enum ReplyMessageId {
    InstantiateDao = 1,
    InstantiateProperty = 2,
}

impl TryFrom<u64> for ReplyMessageId {
    type Error = ContractError;

    fn try_from(msg_id: u64) -> Result<Self, Self::Error> {
        match msg_id {
            1 => Ok(ReplyMessageId::InstantiateDao),
            2 => Ok(ReplyMessageId::InstantiateProperty),
            _ => Err(ContractError::InvalidReplyMessageId {
                msg_id,
            }),
        }
    }
}
