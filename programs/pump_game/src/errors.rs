use anchor_lang::error_code;

#[error_code]
pub enum GameErrorCode {
    #[msg("Not enough energy")]
    NotEnoughEnergy,
    #[msg("Wrong Authority")]
    WrongAuthority,
}

#[error_code]
pub enum ProgramErrorCode {
    #[msg("Invalid Mint account space")]
    InvalidMintAccountSpace,
    #[msg("Cant initialize metadata_pointer")]
    CantInitializeMetadataPointer,

    #[msg("Not Enough SOL sent")]
    NotEnoughSOLSent,

    #[msg("Not A CPI Call")]
    NotACPICall,

    #[msg("Not a pump program CPI caller")]
    NotPumpCPICall,
}
