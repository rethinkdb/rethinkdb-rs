//! ReQL command: changes
//!
//! ## Client syntax
//!
//! > stream.changes() → stream
//!
//! > objectSelection.changes() → stream
//!
//! ## Description
//!
//! Turn a query into a changefeed, an infinite stream of objects representing changes to the
//! query’s results as they occur. A changefeed may return changes to a table or an individual
//! document (a “point” changefeed). Clients such as `filter` or `map` may be used before the `changes`
//! command to transform or filter the output, and many commands that operate on sequences can be
//! chained after `changes`.
//!
//! You may specify one of six optional arguments:
//!
//! * `squash`: Controls how change notifications are batched. Acceptable values are `true`,
//! `false` and a numeric value:
//!     * `true`: When multiple changes to the same document occur before a batch of notifications
//!     is sent, the changes are “squashed” into one change. The client receives a notification
//!     that will bring it fully up to date with the server.
//!     * `false`: All changes will be sent to the client verbatim. This is the default.
//!     * `n`: A numeric value (floating point). Similar to `true`, but the server will wait `n`
//!     seconds to respond in order to squash as many changes together as possible, reducing
//!     network traffic. The first batch will always be returned immediately.
//! * `changefeed_queue_size`: the number of changes the server will buffer between client reads
//! before it starts dropping changes and generates an error (default: 100,000).
//! * `include_initial`: if `true`, the changefeed stream will begin with the current contents of the
//! table or selection being monitored. These initial results will have `new_val` fields, but no
//! `old_val` fields. The initial results may be intermixed with actual changes, as long as an
//! initial result for the changed document has already been given. If an initial result for a
//! document has been sent and a change is made to that document that would move it to the unsent
//! part of the result set (e.g., a changefeed monitors the top 100 posters, the first 50 have been
//! sent, and poster 48 has become poster 52), an “uninitial” notification will be sent, with an
//! `old_val` field but no `new_val` field.
//! * `include_states`: if `true`, the changefeed stream will include special status documents
//! consisting of the field `state` and a string indicating a change in the feed’s state. These
//! documents can occur at any point in the feed between the notification documents described
//! below. If `include_states` is `false` (the default), the status documents will not be sent.
//! * `include_offsets`:  if `true`, a changefeed stream on an `order_by.limit` changefeed will include
//! `old_offset` and `new_offset` fields in status documents that include `old_val` and `new_val`. This
//! allows applications to maintain ordered lists of the stream’s result set. If `old_offset` is set
//! and not `null`, the element at `old_offset` is being deleted; if `new_offset` is set and not `null`,
//! then `new_val` is being inserted at `new_offset`. Setting `include_offsets` to `true` on a changefeed
//! that does not support it will raise an error.
//! * `include_types`: if `true`, every result on a changefeed will include a `type` field with a
//! string that indicates the kind of change the result represents: `add`, `remove`, `change`, `initial`,
//! `uninitial`, `state`. Defaults to `false`.
//!
//! There are currently two states:
//!
//! * `{state: 'initializing'}` indicates the following documents represent initial values on the
//! feed rather than changes. This will be the first document of a feed that returns initial
//! values.
//! * `{state: 'ready'}` indicates the following documents represent changes. This will be the
//! first document of a feed that does `not` return initial values; otherwise, it will indicate the
//! initial values have all been sent.
//!
//! If the table becomes unavailable, the changefeed will be disconnected, and a runtime exception
//! will be thrown by the driver.
//!
//! Changefeed notifications take the form of a two-field object:
//!
//! ```json
//! {
//!     "old_val": <document before change>,
//!     "new_val": <document after change>
//! }
//! ```
//!
//! When `include_types` is `true`, there will be three fields:
//!
//! ```json
//! {
//!     "old_val": <document before change>,
//!     "new_val": <document after change>,
//!     "type": <result type>
//! }
//! ```
//!
//! When a document is deleted, `new_val` will be `None`; when a document is inserted, `old_val` will be
//! `None`.

#![allow(dead_code)]

use ql2::types;
use ql2::proto::Term_TermType as TermType;
use ::{Client, Command};
use super::{ChangesOpts, SquashArg};
use serde_json::value::ToJson;

macro_rules! define {
    ($typ:ty) => {
        impl<O> Client<$typ, O>
            where O: ToJson + Clone
            {
                /// Turn a query into a changefeed. [Read more](changes/index.html)
                pub fn changes(self) -> Client<types::Stream, ChangesOpts<bool>>
                    where ChangesOpts<bool>: Default + ToJson + Clone
                    {
                        let opts: ChangesOpts<bool> = Default::default();
                        super::make_cmd(TermType::CHANGES, NoArg!(), Some(opts), Some(self.cmd), self.errors)
                    }
            }
    }
}

define!{ types::Table }
define!{ types::Stream }
define!{ types::StreamSelection }
define!{ types::ObjectSelection }

impl<T, A> Client<T, ChangesOpts<A>>
    where A: SquashArg,
          ChangesOpts<A>: Default + ToJson + Clone
{
    pub fn squash<B>(self, arg: B) -> Client<T, ChangesOpts<B>>
        where B: SquashArg,
              ChangesOpts<B>: Default + ToJson + Clone
    {
        let o = self.cmd.opts();
        let opts = ChangesOpts {
            squash: arg,
            changefeed_queue_size: o.changefeed_queue_size,
            include_initial: o.include_initial,
            include_states: o.include_states,
            include_offsets: o.include_offsets,
            include_types: o.include_types,
        };
        Client {
            cmd: Command(self.cmd.0, Some(opts)),
            errors: self.errors,
        }
    }

    pub fn changefeed_queue_size(mut self, arg: u64) -> Self {
        let mut opts = self.cmd.opts();
        opts.changefeed_queue_size = arg;
        self.cmd.1 = Some(opts);
        self
    }

    pub fn include_initial(mut self, arg: bool) -> Self {
        let mut opts = self.cmd.opts();
        opts.include_initial = arg;
        self.cmd.1 = Some(opts);
        self
    }

    pub fn include_states(mut self, arg: bool) -> Self {
        let mut opts = self.cmd.opts();
        opts.include_states = arg;
        self.cmd.1 = Some(opts);
        self
    }

    pub fn include_offsets(mut self, arg: bool) -> Self {
        let mut opts = self.cmd.opts();
        opts.include_offsets = arg;
        self.cmd.1 = Some(opts);
        self
    }

    pub fn include_types(mut self, arg: bool) -> Self {
        let mut opts = self.cmd.opts();
        opts.include_types = arg;
        self.cmd.1 = Some(opts);
        self
    }
}
