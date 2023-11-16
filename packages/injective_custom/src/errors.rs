use thiserror::Error;



#[derive(Error, Debug)]
pub enum InjectiveCustomError {
    #[error("Generic Error: {msg}")]
    GenericError { msg: String },

    #[error("Never")]
    Never {},

    #[error("Not implemented")]
    NotImplemented {},

    #[error("Denom already exists")]
    DenomAlreadyExists { },

    #[error("Denom does not exists")]
    UnknownDenom { },

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Market does not exist")]
    UnknownMarket {}
}
