use super::profile::Profile;
use crate::Result;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub(crate) struct Message<T> {
    t: Type,
    e: Option<u32>,
    pub(crate) r: Vec<T>,
    b: Option<Vec<Value>>,
    pub(crate) p: Option<Vec<Profile>>,
    n: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Copy)]
enum Type {
    SuccessAtom,
    SuccessSequence,
    SuccessPartial,
    WaitComplete,
    ServerInfo,
    ClientError,
    CompileError,
    RuntimeError,
}

impl<T> Message<T> {
    pub(crate) fn is_valid(&self) -> Result<()> {
        use self::Type::*;
        match self.t {
            CompileError => unreachable!(),
            ClientError => unreachable!(),
            RuntimeError => unreachable!(),
            _ => {}
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use self::Type::*;

        match u8::deserialize(deserializer)? {
            1 => Ok(SuccessAtom),
            2 => Ok(SuccessSequence),
            3 => Ok(SuccessPartial),
            4 => Ok(WaitComplete),
            5 => Ok(ServerInfo),
            16 => Ok(ClientError),
            17 => Ok(CompileError),
            18 => Ok(RuntimeError),
            _ => unreachable!(),
        }
    }
}
