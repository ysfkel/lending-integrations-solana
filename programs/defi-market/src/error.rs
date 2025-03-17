use anchor_lang::prelude::*;

#[error_code]
pub enum ProtocolErrorCode {  
    #[msg("ZeroAmount")]
    AmountZero, 
}
