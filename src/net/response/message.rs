use {
    super::profile::Profile,
    crate::{err, Result},
    serde::{de, Deserialize, Deserializer},
    serde_json::Value,
};

#[derive(Deserialize, Debug)]
pub(crate) struct Success<T> {
    t: SuccessType,
    e: Option<u32>,
    pub(crate) r: Vec<T>,
    pub(crate) p: Option<Vec<Profile>>,
    n: Option<Vec<Value>>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Error {
    t: ErrorType,
    e: Option<u32>,
    pub(crate) r: Vec<String>,
    b: Vec<Value>,
    pub(crate) p: Option<Vec<Profile>>,
    n: Option<Vec<Value>>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Unexpected {
    t: SuccessType,
    e: Option<u32>,
    pub(crate) r: Vec<Value>,
    pub(crate) p: Option<Vec<Profile>>,
    n: Option<Vec<Value>>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum Message<T> {
    Ok(Success<T>),
    Err(Error),
    Unexpected(Unexpected),
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum SuccessType {
    SuccessAtom,
    SuccessSequence,
    SuccessPartial,
    WaitComplete,
    ServerInfo,
}

#[derive(Debug, Clone, Copy)]
enum ErrorType {
    Client,
    Compile,
    Runtime,
}

impl<T> Message<T> {
    pub(crate) fn extract(self) -> Result<(SuccessType, Vec<T>, Vec<Profile>)> {
        match self {
            Message::Ok(msg) => Ok((msg.t, msg.r, msg.p.unwrap_or_default())),
            Message::Err(mut msg) => {
                use ErrorType::*;
                let error = msg.r.pop().unwrap_or_default();
                let error = match msg.t {
                    Compile => err::Error::Compile(error),
                    Client => err::Driver::Other(error).into(),
                    Runtime => err::Runtime::QueryLogic(error).into(),
                };
                Err(error)
            }
            Message::Unexpected(mut msg) => {
                let resp = msg.r.pop().unwrap_or_default();
                Err(err::Driver::UnexpectedResponse(resp).into())
            }
        }
    }
}

impl<'de> Deserialize<'de> for SuccessType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use SuccessType::*;

        match u8::deserialize(deserializer)? {
            1 => Ok(SuccessAtom),
            2 => Ok(SuccessSequence),
            3 => Ok(SuccessPartial),
            4 => Ok(WaitComplete),
            5 => Ok(ServerInfo),
            typ => {
                let invalid = de::Unexpected::Unsigned(typ.into());
                Err(de::Error::invalid_value(invalid, &"a success type"))
            }
        }
    }
}

impl<'de> Deserialize<'de> for ErrorType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use ErrorType::*;

        match u8::deserialize(deserializer)? {
            16 => Ok(Client),
            17 => Ok(Compile),
            18 => Ok(Runtime),
            typ => {
                let invalid = de::Unexpected::Unsigned(typ.into());
                Err(de::Error::invalid_value(invalid, &"an error type"))
            }
        }
    }
}
