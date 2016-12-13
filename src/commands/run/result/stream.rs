use futures::sync::mpsc::Receiver;

use ::Result;
use conn::ResponseValue;
//use serde::Deserialize;

/// ReQL Response
///
/// Response returned by `run()`
pub type Response<T> = Receiver<Result<ResponseValue<T>>>;
