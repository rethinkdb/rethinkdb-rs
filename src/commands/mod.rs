
// AUTO GENERATED
// Edit in `build/commands.rs` instead

/*
mod args;
#[cfg(feature = "with_io")]
mod io;
#[cfg(feature = "with_io")]
pub use self::io::*;
*/

use {Client, ToArg};
use ql2::proto::Term;
use protobuf::repeated::RepeatedField;
use ql2::proto::Term_TermType;


fn cmd(name: &str) -> Client {
    unimplemented!();
}



fn cmd_with_args<T: ToArg>(name: &str, args: T) -> Client {
    unimplemented!();
}



impl Client {
    
    /// Create a new connection to the database server
///
/// <img src="https://rethinkdb.com/assets/images/docs/api_illustrations/connect_javascript.png" class="api_command_illustration" />
///
/// `connection` returns a builder object with the following methods:
                /// 
    /// - `hostname()`: the host to connect to (default `localhost`).
    /// - `port()`: the port to connect on (default `28015`).
    /// - `dbname()`: the default database (default `test`).
    /// - `user()`: the user account and password to connect as (default `"admin", ""`).
    /// - `timeout()`: timeout period in seconds for the connection to be opened (default `20`).
    /// - `connect()`: instantiate a connection object with the parameters previously passed to the builder.
    /// - `certFile()`: a path to an SSL CA certificate.
    /// - `sslContext()`: an instance of an [SSLContext](https://docs.oracle.com/javase/8/docs/api/javax/net/ssl/SSLContext.html) class to use for SSL connections.
    /// 
    /// Either `certFile` or `sslContext` must be supplied to make an SSL connection to the RethinkDB server. Only one should be used.
    /// 
    /// If the connection cannot be established, a `ReqlDriverError` will be thrown.
    /// 
    /// <!-- break -->
    /// 
    /// The returned connection object will have two methods on it returning the connection's port and address:
    /// 
    /// ```java
    /// conn.clientPort();      // returns Optional<Integer>
    /// conn.clientAddress();   // returns Optional<SocketAddress>
    /// ```
    /// 
    /// {% infobox %}
    /// Using SSL with RethinkDB requires proxy software on the server, such as [Nginx][], [HAProxy][] or an SSL tunnel. RethinkDB will encrypt traffic and verify the CA certification to prevent [man-in-the-middle][mitm] attacks. Consult your proxy's documentation for more details.
    /// 
    /// [Nginx]: http://nginx.org/
    /// [HAProxy]: http://www.haproxy.org/
    /// [mitm]: http://en.wikipedia.org/wiki/Man-in-the-middle_attack
    /// 
    /// Alternatively, you may use RethinkDB's built-in [TLS support][tls].
    /// 
    /// [tls]: /docs/security/
    /// {% endinfobox %}
    /// 
    /// __Example:__ Open a connection using the default host and port, specifying the default database.
    /// 
    /// ```java
    /// Connection conn = r.connection().connect();
    /// ```
    /// 
    /// __Example:__ Open a new connection, specifying parameters.
    /// 
    /// ```java
    /// Connection conn = r.connection()
    ///     .hostname("localhost")
    ///     .port(28015)
    ///     .dbname("marvel")
    ///     .connect();
    /// ```
    /// 
    /// __Example:__ Open a new connection, specifying a user/password combination for authentication.
    /// 
    /// ```java
    /// Connection conn = r.connection()
    ///     .hostname("localhost")
    ///     .port(28015)
    ///     .dbname("marvel")
    ///     .user("herofinder", "metropolis")
    ///     .connect();
    /// ```
    /// 
    /// __Example:__ Open a new connection to the database using an SSL proxy.
    /// 
    /// ```java
    /// Connection conn = r.connection()
    ///     .hostname("localhost")
    ///     .port(28015)
    ///     .dbname("marvel")
    ///     .authKey("hunter2")
    ///     .certFile("/path/to/ca.crt")
    ///     .connect();
    /// ```

    pub fn connect(&self) -> Client {
        cmd("connect")
    }


    /// Close an open connection
///
/// 
///
/// 
                /// 
    /// Closing a connection normally waits until all outstanding requests have finished and then frees any open resources associated with the connection. By passing `false` as the boolean argument to `close`, the connection will be closed immediately, possibly aborting any outstanding noreply writes.
    /// 
    /// A noreply query is executed by using the [runNoReply](/api/java/run_noreply/) command, indicating that the command should not wait for the query to complete before returning. You may also explicitly wait for a noreply query to complete by using the [noreplyWait](/api/java/noreply_wait) command.
    /// 
    /// __Example:__ Close an open connection, waiting for noreply writes to finish.
    /// 
    /// ```java
    /// conn.close();
    /// ```
    /// 
    /// __Example:__ Close an open connection immediately.
    /// 
    /// ```java
    /// conn.close(false);
    /// ```

    pub fn close(&self) -> Client {
        cmd("close")
    }


    /// Close and reopen a connection
///
/// 
///
/// 
                /// 
    /// Closing a connection normally waits until all outstanding requests have finished and then frees any open resources associated with the connection. By passing `false` as an optional boolean argument to `reconnect`, the connection will be closed immediately, possibly aborting any outstanding noreply writes. An optional second argument is a (long integer) timeout indicating how long you would like `reconnect` to wait before closing the existing connection.
    /// 
    /// A noreply query is executed by using the [runNoReply](/api/java/run_noreply/) command, indicating that the command should not wait for the query to complete before returning. You may also explicitly wait for a noreply query to complete by using the [noreplyWait](/api/java/noreply_wait) command.
    /// 
    /// __Example:__ Cancel outstanding requests/queries that are no longer needed.
    /// 
    /// ```java
    /// conn.reconnect(false);
    /// ```
    /// 
    /// __Example:__ Wait up to 5 seconds for outstanding requests to finish before reconnecting.
    /// 
    /// ```java
    /// conn.reconnect(true, 5);
    /// ```

    pub fn reconnect<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("reconnect", args)
    }


    /// 
///
/// 
///
/// 
                /// Run a query on a connection, returning either a single JSON result or
    /// a cursor, depending on the query.
    /// 
    /// You can pass the following options using [optArg](/api/java/optarg/). Note that unlike other Java ReQL commands, you must create an OptArg object and pass it as an optional second argument to `run`:
    /// 
    /// ```java
    /// import com.rethinkdb.model.OptArgs;
    /// 
    /// r.table("table").run(conn, OptArgs.of("read_mode", "outdated"));
    /// 
    /// // for two or more optArgs, use "with"
    /// r.table("table").run(conn,
    ///     OptArgs.of("read_mode", "outdated").with("db", "database"));
    /// ```
    /// 
    /// - `read_mode`: One of three possible values affecting the consistency guarantee for the query (default: `'single'`).
    ///     - `'single'` (the default) returns values that are in memory (but not necessarily written to disk) on the primary replica.
    ///     - `'majority'` will only return values that are safely committed on disk on a majority of replicas. This requires sending a message to every replica on each read, so it is the slowest but most consistent.
    ///     - `'outdated'` will return values that are in memory on an arbitrarily-selected replica. This is the fastest but least consistent.
    /// - `time_format`: what format to return times in (default: `native`).
    ///   Set this to `raw` if you want times returned as JSON objects for exporting.
    /// - `profile`: whether or not to return a profile of the query's
    ///   execution (default: `false`).
    /// - `durability`: possible values are `hard` and `soft`. In soft durability mode RethinkDB
    /// will acknowledge the write immediately after receiving it, but before the write has
    /// been committed to disk.
    /// - `group_format`: what format to return `grouped_data` and `grouped_streams` in (default: `native`).
    ///   Set this to `raw` if you want the raw pseudotype.
    /// - `db`: the database to run this query against as a string. The default is the database specified in the `db` [connection](/api/java/connect/) method (which defaults to `test`). The database may also be specified with the [db](/api/java/db/) command.
    /// - `array_limit`: the maximum numbers of array elements that can be returned by a query (default: 100,000). This affects all ReQL commands that return arrays. Note that it has no effect on the size of arrays being _written_ to the database; those always have an upper limit of 100,000 elements.
    /// - `binary_format`: what format to return binary data in (default: `native`). Set this to `raw` if you want the raw pseudotype.
    /// - `min_batch_rows`: minimum number of rows to wait for before batching a result set (default: 8). This is an integer.
    /// - `max_batch_rows`: maximum number of rows to wait for before batching a result set (default: unlimited). This is an integer.
    /// - `max_batch_bytes`: maximum number of bytes to wait for before batching a result set (default: 1MB). This is an integer.
    /// - `max_batch_seconds`: maximum number of seconds to wait before batching a result set (default: 0.5). This is a float (not an integer) and may be specified to the microsecond.
    /// - `first_batch_scaledown_factor`: factor to scale the other parameters down by on the first batch (default: 4). For example, with this set to 8 and `max_batch_rows` set to 80, on the first batch `max_batch_rows` will be adjusted to 10 (80 / 8). This allows the first batch to return faster.
    /// 
    /// __Example:__ If you are OK with potentially out of date data from all
    /// the tables involved in this query and want potentially faster reads,
    /// pass a flag allowing out of date data in an options object. Settings
    /// for individual tables will supercede this global setting for all
    /// tables in the query.
    /// 
    /// ```java
    /// import com.rethinkdb.model.OptArgs;
    /// 
    /// r.table("marvel").run(conn, OptArgs.of("read_mode", "outdated"));
    /// ```
    /// 
    /// __Example:__ If you want to specify whether to wait for a write to be
    /// written to disk (overriding the table's default settings), you can set
    /// `durability` to `hard` or `soft` in the options.
    /// 
    /// ```java
    /// r.table("marvel").insert(r.hashMap("superhero", "Iron Man")
    ///     .with("superpower", "Arc Reactor"))
    ///     .run(conn, OptArgs.of("durability", "soft"));
    /// ```
    /// 
    /// __Example:__ If you do not want a time object to be converted to a
    /// native date object, you can pass a `time_format` flag to prevent it
    /// (valid flags are "raw" and "native"). This query returns an object
    /// with two fields (`epoch_time` and `$reql_type$`) instead of a [Java 8 ZonedDateTime][dt] object.
    /// 
    /// [dt]: https://docs.oracle.com/javase/8/docs/api/java/time/ZonedDateTime.html
    /// 
    /// ```java
    /// r.now().run(conn, OptArgs.of("time_format", "raw"));
    /// ```
    /// 
    /// __Example:__ Specify the database to use for the query.
    /// 
    /// ```java
    /// for (Object doc : r.table("marvel").run(conn, OptArgs.of("db", "heroes")) {
    ///     System.out.println(doc);
    /// }
    /// ```
    /// 
    /// This is equivalent to using the `db` command to specify the database:
    /// 
    /// ```java
    /// r.db("heroes").table("marvel").run(conn);
    /// ```
    /// 
    /// __Example:__ Change the batching parameters for this query.
    /// 
    /// ```java
    /// r.table("marvel").run(conn, OptArgs.of("max_batch_rows", 16).with("max_batch_bytes", 2048));
    /// ```
    /// 
    /// {% infobox %}
    /// If you want to send a write and forget about it&mdash;that is, not wait for a reply&mdash;use the [runNoReply](/api/java/run_noreply) command. (Unlike other official RethinkDB drivers, Java does not support a `noreply` option to `run` but instead implements a separate term.)
    /// {% endinfobox %}

    pub fn run<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("run", args)
    }


    /// Run a query on a connection and immediately return, without waiting for any result data to be returned by the server
///
/// 
///
/// 
                /// 
    /// You can pass the following options using [optArg](/api/java/optarg/). Note that unlike other Java ReQL commands, you must create an OptArg object and pass it as an optional second argument to `run`:
    /// 
    /// ```java
    /// import com.rethinkdb.model.OptArgs;
    /// 
    /// r.table("table").runNoReply(conn, OptArgs.of("read_mode", "outdated"));
    /// 
    /// // for two or more optArgs, use "with"
    /// r.table("table").runNoReply(conn,
    ///     OptArgs.of("read_mode", "outdated").with("db", "database"));
    /// ```
    /// 
    /// - `read_mode`: One of three possible values affecting the consistency guarantee for the query (default: `'single'`).
    ///     - `'single'` (the default) returns values that are in memory (but not necessarily written to disk) on the primary replica.
    ///     - `'majority'` will only return values that are safely committed on disk on a majority of replicas. This requires sending a message to every replica on each read, so it is the slowest but most consistent.
    ///     - `'outdated'` will return values that are in memory on an arbitrarily-selected replica. This is the fastest but least consistent.
    /// - `time_format`: what format to return times in (default: `native`).
    ///   Set this to `raw` if you want times returned as JSON objects for exporting.
    /// - `profile`: whether or not to return a profile of the query's
    ///   execution (default: `false`).
    /// - `durability`: possible values are `hard` and `soft`. In soft durability mode RethinkDB
    /// will acknowledge the write immediately after receiving it, but before the write has
    /// been committed to disk.
    /// - `group_format`: what format to return `grouped_data` and `grouped_streams` in (default: `native`).
    ///   Set this to `raw` if you want the raw pseudotype.
    /// - `db`: the database to run this query against as a string. The default is the database specified in the `db` [connection](/api/java/connect/) method (which defaults to `test`). The database may also be specified with the [db](/api/java/db/) command.
    /// - `array_limit`: the maximum numbers of array elements that can be returned by a query (default: 100,000). This affects all ReQL commands that return arrays. Note that it has no effect on the size of arrays being _written_ to the database; those always have an upper limit of 100,000 elements.
    /// - `binary_format`: what format to return binary data in (default: `native`). Set this to `raw` if you want the raw pseudotype.
    /// - `min_batch_rows`: minimum number of rows to wait for before batching a result set (default: 8). This is an integer.
    /// - `max_batch_rows`: maximum number of rows to wait for before batching a result set (default: unlimited). This is an integer.
    /// - `max_batch_bytes`: maximum number of bytes to wait for before batching a result set (default: 1MB). This is an integer.
    /// - `max_batch_seconds`: maximum number of seconds to wait before batching a result set (default: 0.5). This is a float (not an integer) and may be specified to the microsecond.
    /// - `first_batch_scaledown_factor`: factor to scale the other parameters down by on the first batch (default: 4). For example, with this set to 8 and `max_batch_rows` set to 80, on the first batch `max_batch_rows` will be adjusted to 10 (80 / 8). This allows the first batch to return faster.
    /// 
    /// __Example:__ Send a write and return immediately.
    /// 
    /// ```java
    /// r.table("marvel").insert(document).runNoReply(conn);
    /// ```
    /// 
    /// __Example:__ If you want to specify whether to wait for a write to be
    /// written to disk (overriding the table's default settings), you can set
    /// `durability` to `hard` or `soft` in the options.
    /// 
    /// ```java
    /// r.table("marvel").insert(r.hashMap("superhero", "Iron Man")
    ///     .with("superpower", "Arc Reactor"))
    ///     .runNoReply(conn, OptArgs.of("durability", "soft"));
    /// ```
    /// 
    /// For more examples, read the API documentation for [run](/api/java/run); the available optArgs are the same, and any query can be executed with `runNoReply` rather than `run` (although `runNoReply` is usually not appropriate for read queries).
    /// 

    pub fn run_noreply<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("run_noreply", args)
    }


    /// Turn a query into a changefeed, an infinite stream of objects representing changes to the query's results as they occur
///
/// 
///
/// A changefeed may return changes to a table or an individual document (a "point" changefeed). Commands such as `filter` or `map` may be used before the `changes` command to transform or filter the output, and many commands that operate on sequences can be chained after `changes`.
                /// 
    /// You may specify one of six optional arguments via [optArg](/api/java/optarg).
    /// 
    /// * `squash`: Controls how change notifications are batched. Acceptable values are `true`, `false` and a numeric value:
    ///     * `true`: When multiple changes to the same document occur before a batch of notifications is sent, the changes are "squashed" into one change. The client receives a notification that will bring it fully up to date with the server.
    ///     * `false`: All changes will be sent to the client verbatim. This is the default.
    ///     * `n`: A numeric value (floating point). Similar to `true`, but the server will wait `n` seconds to respond in order to squash as many changes together as possible, reducing network traffic. The first batch will always be returned immediately.
    /// * `changefeed_queue_size`: the number of changes the server will buffer between client reads before it starts dropping changes and generates an error (default: 100,000).
    /// * `include_initial`: if `true`, the changefeed stream will begin with the current contents of the table or selection being monitored. These initial results will have `new_val` fields, but no `old_val` fields. The initial results may be intermixed with actual changes, as long as an initial result for the changed document has already been given. If an initial result for a document has been sent and a change is made to that document that would move it to the unsent part of the result set (e.g., a changefeed monitors the top 100 posters, the first 50 have been sent, and poster 48 has become poster 52), an "uninitial" notification will be sent, with an `old_val` field but no `new_val` field.
    /// * `include_states`: if `true`, the changefeed stream will include special status documents consisting of the field `state` and a string indicating a change in the feed's state. These documents can occur at any point in the feed between the notification documents described below. If `include_states` is `false` (the default), the status documents will not be sent.
    /// * `include_offsets`: if `true`, a changefeed stream on an `orderBy.limit` changefeed will include `old_offset` and `new_offset` fields in status documents that include `old_val` and `new_val`. This allows applications to maintain ordered lists of the stream's result set. If `old_offset` is set and not `null`, the element at `old_offset` is being deleted; if `new_offset` is set and not `null`, then `new_val` is being inserted at `new_offset`. Setting `include_offsets` to `true` on a changefeed that does not support it will raise an error.
    /// * `include_types`: if `true`, every result on a changefeed will include a `type` field with a string that indicates the kind of change the result represents: `add`, `remove`, `change`, `initial`, `uninitial`, `state`. Defaults to `false`.
    /// 
    /// There are currently two states:
    /// 
    /// * `{state: 'initializing'}` indicates the following documents represent initial values on the feed rather than changes. This will be the first document of a feed that returns initial values.
    /// * `{state: 'ready'}` indicates the following documents represent changes. This will be the first document of a feed that does *not* return initial values; otherwise, it will indicate the initial values have all been sent.
    /// 
    /// If the table becomes unavailable, the changefeed will be disconnected, and a runtime exception will be thrown by the driver.
    /// 
    /// Changefeed notifications take the form of a two-field object:
    /// 
    /// ```json
    /// {
    ///     "old_val": <document before change>,
    ///     "new_val": <document after change>
    /// }
    /// ```
    /// 
    /// When `include_types` is `true`, there will be three fields:
    /// 
    /// ```js
    /// {
    ///     "old_val": <document before change>,
    ///     "new_val": <document after change>,
    ///     "type": <result type>
    /// }
    /// ```
    /// 
    /// When a document is deleted, `new_val` will be `null`; when a document is inserted, `old_val` will be `null`.
    /// 
    /// {% infobox %}
    /// Certain document transformation commands can be chained before changefeeds. For more information, read the [discussion of changefeeds](/docs/changefeeds/) in the "Query language" documentation.
    /// 
    /// __Note:__ Changefeeds ignore the `read_mode` flag to `run`, and always behave as if it is set to `single` (i.e., the values they return are in memory on the primary replica, but have not necessarily been written to disk yet). For more details read [Consistency guarantees](/docs/consistency).
    /// {% endinfobox %}
    /// 
    /// The server will buffer up to `changefeed_queue_size` elements (default 100,000). If the buffer limit is hit, early changes will be discarded, and the client will receive an object of the form `{error: "Changefeed cache over array size limit, skipped X elements."}` where `X` is the number of elements skipped.
    /// 
    /// Commands that operate on streams (such as [filter](/api/java/filter/) or [map](/api/java/map/)) can usually be chained after `changes`.  However, since the stream produced by `changes` has no ending, commands that need to consume the entire stream before returning (such as [reduce](/api/java/reduce/) or [count](/api/java/count/)) cannot.
    /// 
    /// __Example:__ Subscribe to the changes on a table.
    /// 
    /// Start monitoring the changefeed in one client:
    /// 
    /// ```java
    /// Cursor changeCursor = r.table("games").changes().run(conn);
    /// for (Object change : changeCursor) {
    ///     System.out.println(change);
    /// }
    /// ```
    /// 
    /// As these queries are performed in a second client, the first
    /// client would receive and print the following objects:
    /// 
    /// ```java
    /// r.table("games").insert(r.hashMap("id", 1)).run(conn);
    /// ```
    /// 
    /// ```json
    /// {"old_val": null, "new_val": {"id": 1}}
    /// ```
    /// 
    /// ```java
    /// r.table("games").get(1).update(r.hashMap("player1", "Bob")).run(conn);
    /// ```
    /// 
    /// ```json
    /// {"old_val": {"id": 1}, "new_val": {"id": 1, "player1": "Bob"}}
    /// ```
    /// 
    /// ```java
    /// r.table("games").get(1).replace(
    ///     r.hashMap("id", 1).with("player1", "Bob").with("player2", "Alice")
    /// ).run(conn);
    /// ```
    /// 
    /// ```json
    /// {"old_val": {"id": 1, "player1": "Bob"},
    ///  "new_val": {"id": 1, "player1": "Bob", "player2": "Alice"}}
    /// ```
    /// 
    /// ```java
    /// r.table("games").get(1).delete().run(conn);
    /// ```
    /// 
    /// ```json
    /// {"old_val": {"id": 1, "player1": "Bob", "player2": "Alice"}, "new_val": null}
    /// ```
    /// 
    /// ```java
    /// r.tableDrop("games").run(conn);
    /// ```
    /// 
    /// ```
    /// ReqlRuntimeError: Changefeed aborted (table unavailable)
    /// ```
    /// 
    /// __Example:__ Return all the changes that increase a player's score.
    /// 
    /// ```java
    /// r.table("test").changes().filter(
    ///     row -> row.g("new_val").g("score").gt(row.g("old_val").g("score"))
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Return all the changes to a specific player's score that increase it past 10.
    /// 
    /// ```java
    /// r.table("test").get(1).filter(row -> row.g("score").gt(10)).changes().run(conn);
    /// ```
    /// 
    /// __Example:__ Return all the inserts on a table.
    /// 
    /// ```java
    /// r.table("test").changes().filter(
    ///     row -> row.g("old_val").eq(null)
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Return all the changes to game 1, with state notifications and initial values.
    /// 
    /// ```java
    /// r.table("games").get(1).changes()
    ///  .optArg("include_initial", true).optArg("include_states", true).run(conn);
    /// ```
    /// 
    /// Result returned on changefeed:
    /// 
    /// ```json
    /// {"state": "initializing"}
    /// {"new_val": {"id": 1, "score": 12, "arena": "Hobbiton Field"}}
    /// {"state": "ready"}
    /// {
    /// 	"old_val": {"id": 1, "score": 12, "arena": "Hobbiton Field"},
    /// 	"new_val": {"id": 1, "score": 14, "arena": "Hobbiton Field"}
    /// }
    /// {
    /// 	"old_val": {"id": 1, "score": 14, "arena": "Hobbiton Field"},
    /// 	"new_val": {"id": 1, "score": 17, "arena": "Hobbiton Field", "winner": "Frodo"}
    /// }
    /// ```
    /// 
    /// __Example:__ Return all the changes to the top 10 games. This assumes the presence of a `score` secondary index on the `games` table.
    /// 
    /// ```java
    /// r.table("games").orderBy().optArg("index", r.desc("score"))
    ///  .limit(10).changes().run(conn);
    /// ```
    /// 
    /// __Example:__ Maintain the state of a list based on a changefeed.
    /// 
    /// ```java
    /// Cursor changeCursor = r.table("data").changes()
    ///     .optArg("include_initial", true)
    ///     .optArg("include_offsets", true)
    ///     .run((conn);
    /// for (Object change : changeCursor) {
    ///     // Delete item at old_offset before inserting at new_offset
    ///     if (change.old_offset != null) {
    ///         myList.remove(change.old_offset);
    ///     }
    ///     if (change.new_offset != null) {
    ///         myList.add(change.new_offset, change_new.val);
    ///     }
    /// };
    /// ```
    /// 
    /// (This is a simplistic implementation. For a more sophisticated example, see the `applyChange` function in Horizon's [client/src/ast.js][ast] source; it's written in JavaScript, but the principles apply to all languages.)
    /// 
    /// [ast]: https://github.com/rethinkdb/horizon/blob/next/client/src/ast.js

    pub fn changes(&self) -> Client {
        cmd("changes")
    }


    /// Ensure that previous queries executed with [runNoReply](/api/java/run_noreply) have been processed by the server
///
/// 
///
/// Note that this guarantee only apples to queries run on the same connection.
                /// 
    /// __Example:__ We have previously executed queries with `runNoReply`. Now wait until the server has processed them.
    /// 
    /// ```java
    /// conn.noreplyWait();
    /// ```

    pub fn noreply_wait(&self) -> Client {
        cmd("noreply_wait")
    }


    /// Return information about the server being used by a connection
///
/// 
///
/// 
                /// 
    /// The `server` command returns either two or three fields:
    /// 
    /// * `id`: the UUID of the server the client is connected to.
    /// * `proxy`: a boolean indicating whether the server is a [RethinkDB proxy node][rp].
    /// * `name`: the server name. If `proxy` is `true`, this field will not be returned.
    /// 
    /// [rp]: /docs/sharding-and-replication/#running-a-proxy-node
    /// 
    /// __Example:__ Return server information.
    /// 
    /// ```java
    /// conn.server();
    /// ```
    /// 
    /// ```json
    /// {
    ///     "id": "404bef53-4b2c-433f-9184-bc3f7bda4a15",
    ///     "name": "amadeus",
    ///     "proxy": false
    /// }
    /// ```

    pub fn server(&self) -> Client {
        cmd("server")
    }


    /// Specify an optional argument to a Java ReQL term
///
/// 
///
/// 
                /// 
    /// Some terms in ReQL accept optional arguments. Since Java doesn't support named arguments, the RethinkDB Java driver allows you to pass them by chaining the `optArg` command after them.
    /// 
    /// __Example:__ Pass the `right_bound` optional argument to [between](/api/java/between/).
    /// 
    /// ```java
    /// r.table("marvel").between(10, 20).optArg("right_bound", "closed").run(conn);
    /// ```
    /// 
    /// To pass more than one optional argument, chain `optArg` once for each argument.
    /// 
    /// 
    /// __Example:__ Pass the `right_bound` and `index` optional arguments to [between](/api/java/between/).
    /// 
    /// ```java
    /// r.table("marvel").between(10, 20).optArg("right_bound", "closed")
    ///  .optArg("index", "power").run(conn);
    /// ```

    pub fn optarg<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("optarg", args)
    }


    /// Create a database
///
/// 
///
/// A RethinkDB database is a collection of tables, similar to
                /// relational databases.
    /// 
    /// If successful, the command returns an object with two fields:
    /// 
    /// * `dbs_created`: always `1`.
    /// * `config_changes`: a list containing one object with two fields, `old_val` and `new_val`:
    ///     * `old_val`: always `null`.
    ///     * `new_val`: the database's new [config](/api/java/config) value.
    /// 
    /// If a database with the same name already exists, the command throws `ReqlRuntimeError`.
    /// 
    /// Note: Only alphanumeric characters and underscores are valid for the database name.
    /// 
    /// __Example:__ Create a database named 'superheroes'.
    /// 
    /// ```java
    /// r.dbCreate("superheroes").run(conn);
    /// ```
    /// 
    /// Return:
    /// 
    /// ```json
    /// {
    ///     "config_changes": [
    ///         {
    ///             "new_val": {
    ///                 "id": "e4689cfc-e903-4532-a0e6-2d6797a43f07",
    ///                 "name": "superheroes"
    ///             },
    ///             "old_val": null
    ///         }
    ///     ],
    ///     "dbs_created": 1
    /// }
    /// ```
    /// 
    /// 

    pub fn db_create<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("db_create", args)
    }


    /// Drop a database
///
/// 
///
/// The database, all its tables, and corresponding data will be deleted.
                /// 
    /// If successful, the command returns an object with two fields:
    /// 
    /// * `dbs_dropped`: always `1`.
    /// * `tables_dropped`: the number of tables in the dropped database.
    /// * `config_changes`: a list containing one two-field object, `old_val` and `new_val`:
    ///     * `old_val`: the database's original [config](/api/java/config) value.
    ///     * `new_val`: always `null`.
    /// 
    /// If the given database does not exist, the command throws `ReqlRuntimeError`.
    /// 
    /// __Example:__ Drop a database named 'superheroes'.
    /// 
    /// ```java
    /// r.dbDrop("superheroes").run(conn);
    /// ```
    /// 
    /// Return:
    /// 
    /// ```json
    /// {
    ///     "config_changes": [
    ///         {
    ///             "old_val": {
    ///                 "id": "e4689cfc-e903-4532-a0e6-2d6797a43f07",
    ///                 "name": "superheroes"
    ///             },
    ///             "new_val": null
    ///         }
    ///     ],
    ///     "tables_dropped": 3,
    ///     "dbs_dropped": 1
    /// }
    /// ```
    /// 

    pub fn db_drop<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("db_drop", args)
    }


    /// List all database names in the cluster
///
/// 
///
/// The result is a list of strings.
                /// 
    /// __Example:__ List all databases.
    /// 
    /// ```java
    /// r.dbList().run(conn);
    /// ```

    pub fn db_list(&self) -> Client {
        cmd("db_list")
    }


    /// Create a table
///
/// <img src="https://rethinkdb.com/assets/images/docs/api_illustrations/table_create_javascript.png" class="api_command_illustration" />
///
/// A RethinkDB table is a collection of JSON documents.
                /// 
    /// If successful, the command returns an object with two fields:
    /// 
    /// * `tables_created`: always `1`.
    /// * `config_changes`: a list containing one two-field object, `old_val` and `new_val`:
    ///     * `old_val`: always `null`.
    ///     * `new_val`: the table's new [config](/api/java/config) value.
    /// 
    /// If a table with the same name already exists, the command throws `ReqlOpFailedError`.
    /// 
    /// {% infobox %}
    /// __Note:__ Only alphanumeric characters and underscores are valid for the table name.
    /// 
    /// Invoking `tableCreate` without specifying a database using [db](/api/java/db/) creates a table in the database specified in [connect](/api/java/connect/), or `test` if no database was specified.
    /// {% endinfobox %}
    /// 
    /// When creating a table you can specify the following options using [optArg](/api/java/optarg):
    /// 
    /// * `primaryKey`: the name of the primary key. The default primary key is `id`.
    /// * `durability`: if set to `soft`, writes will be acknowledged by the server immediately and flushed to disk in the background. The default is `hard`: acknowledgment of writes happens after data has been written to disk.
    /// * `shards`: the number of shards, an integer from 1-64. Defaults to `1`.
    /// * `replicas`: either an integer or a mapping object. Defaults to `1`.
    ///     * If `replicas` is an integer, it specifies the number of replicas per shard. Specifying more replicas than there are servers will return an error.
    ///     * If `replicas` is an object, it specifies key-value pairs of server tags and the number of replicas to assign to those servers: `{tag1: 2, tag2: 4, tag3: 2, ...}`.
    /// * `primaryReplicaTag`: the primary server specified by its server tag. Required if `replicas` is an object; the tag must be in the object. This must *not* be specified if `replicas` is an integer.
    /// 
    /// The [data type](/docs/data-types/) of a primary key is usually a string (like a UUID) or a number, but it can also be a time, binary object, boolean or an array. Data types can be mixed in the primary key field, but all values must be unique. Using an array as a primary key causes the primary key to behave like a compound index; read the documentation on [compound secondary indexes][ci] for more information, as it applies to primary keys as well. (Note that the primary index still only covers a single field, while compound secondary indexes can cover multiple fields in a single index.) Primary keys cannot be objects.
    /// 
    /// [ci]: /docs/secondary-indexes/javascript/#compound-indexes
    /// 
    /// Tables will be available for writing when the command returns.
    /// 
    /// __Example:__ Create a table named 'dc_universe' with the default settings.
    /// 
    /// ```java
    /// r.db("heroes").tableCreate("dc_universe").run(conn);
    /// ```
    /// 
    /// Result:
    /// 
    /// ```json
    /// {
    ///     "config_changes": [
    ///         {
    ///             "new_val": {
    ///                 "db": "test",
    ///                 "durability":  "hard",
    ///                 "id": "20ea60d4-3b76-4817-8828-98a236df0297",
    ///                 "name": "dc_universe",
    ///                 "primary_key": "id",
    ///                 "shards": [
    ///                     {
    ///                         "primary_replica": "rethinkdb_srv1",
    ///                         "replicas": [
    ///                             "rethinkdb_srv1",
    ///                             "rethinkdb_srv2"
    ///                         ]
    ///                     }
    ///                 ],
    ///                 "write_acks": "majority"
    ///             },
    ///             "old_val": null
    ///         }
    ///     ],
    ///     "tables_created": 1
    /// }
    /// ```
    /// 
    /// __Example:__ Create a table named 'dc_universe' using the field 'name' as primary key.
    /// 
    /// ```java
    /// r.db("test").tableCreate("dc_universe").optArg("primary_key", "name").run(conn);
    /// ```
    /// 
    /// __Example:__ Create a table set up for two shards and three replicas per shard. This requires three available servers.
    /// 
    /// ```java
    /// r.db("test").tableCreate("dc_universe").optArg("shards", 2).optArg("replicas", 3).run(conn);
    /// ```
    /// 
    /// Read [Sharding and replication](/docs/sharding-and-replication/) for a complete discussion of the subject, including advanced topics.

    pub fn table_create<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("table_create", args)
    }


    /// Drop a table from a database
///
/// 
///
/// The table and all its data will be deleted.
                /// 
    /// If successful, the command returns an object with two fields:
    /// 
    /// * `tables_dropped`: always `1`.
    /// * `config_changes`: a list containing one two-field object, `old_val` and `new_val`:
    ///     * `old_val`: the dropped table"s [config](/api/java/config) value.
    ///     * `new_val`: always `null`.
    /// 
    /// If the given table does not exist in the database, the command throws `ReqlRuntimeError`.
    /// 
    /// __Example:__ Drop a table named "dc_universe".
    /// 
    /// ```java
    /// r.db("test").tableDrop("dc_universe").run(conn);
    /// ```
    /// 
    /// Result:
    /// 
    /// ```json
    /// {
    ///     "config_changes": [
    ///         {
    ///             "old_val": {
    ///                 "db": "test",
    ///                 "durability":  "hard",
    ///                 "id": "20ea60d4-3b76-4817-8828-98a236df0297",
    ///                 "name": "dc_universe",
    ///                 "primary_key": "id",
    ///                 "shards": [
    ///                     {
    ///                         "primary_replica": "rethinkdb_srv1",
    ///                         "replicas": [
    ///                             "rethinkdb_srv1",
    ///                             "rethinkdb_srv2"
    ///                         ]
    ///                     }
    ///                 ],
    ///                 "write_acks": "majority"
    ///             },
    ///             "new_val": null
    ///         }
    ///     ],
    ///     "tables_dropped": 1
    /// }
    /// ```
    /// 
    /// 

    pub fn table_drop<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("table_drop", args)
    }


    /// List all table names in a database
///
/// 
///
/// The result is a list of strings.
                /// 
    /// __Example:__ List all tables of the 'test' database.
    /// 
    /// ```java
    /// r.db("test").tableList().run(conn);
    /// ```
    /// 

    pub fn table_list(&self) -> Client {
        cmd("table_list")
    }


    /// Create a new secondary index on a table
///
/// 
///
/// Secondary indexes improve the speed of many read queries at the slight cost of increased storage space and decreased write performance. For more information about secondary indexes, read the article "[Using secondary indexes in RethinkDB](/docs/secondary-indexes/)."
                /// 
    /// RethinkDB supports different types of secondary indexes:
    /// 
    /// - *Simple indexes* based on the value of a single field.
    /// - *Compound indexes* based on multiple fields.
    /// - *Multi indexes* based on arrays of values, created when the `multi` [optArg](/api/java/optarg) argument is `true`.
    /// - *Geospatial indexes* based on indexes of geometry objects, created when the `geo` optArg is `true`.
    /// - Indexes based on *arbitrary expressions*.
    /// 
    /// The `indexFunction` can be an anonymous function or a binary representation obtained from the `function` field of [indexStatus](/api/java/index_status). The function must be deterministic, and so cannot use a subquery or the `r.js` command.
    /// 
    /// If successful, `createIndex` will return an object of the form `{"created": 1}`. If an index by that name already exists on the table, a `ReqlRuntimeError` will be thrown.
    /// 
    /// {% infobox %}
    /// Note that an index may not be immediately available after creation. If your application needs to use indexes immediately after creation, use the [indexWait](/api/java/index_wait) command to ensure the indexes are ready before use.
    /// {% endinfobox %}
    /// 
    /// __Example:__ Create a simple index based on the field `postId`.
    /// 
    /// ```java
    /// r.table("comments").indexCreate("postId").run(conn);
    /// ```
    /// 
    /// __Example:__ Create a simple index based on the nested field `author > name`.
    /// 
    /// 
    /// ```java
    /// r.table("comments").indexCreate("author_name", row -> row.g("author").g("name"))
    ///  .run(conn);
    /// ```
    /// 
    /// __Example:__ Create a geospatial index based on the field `location`.
    /// 
    /// ```java
    /// r.table("places").indexCreate("location").optArg("geo", true).run(conn);
    /// ```
    /// 
    /// A geospatial index field should contain only geometry objects. It will work with geometry ReQL terms ([getIntersecting](/api/java/get_intersecting/) and [getNearest](/api/java/get_nearest/)) as well as index-specific terms ([indexStatus](/api/java/index_status), [indexWait](/api/java/index_wait), [indexDrop](/api/java/index_drop) and [indexList](/api/java/index_list)). Using terms that rely on non-geometric ordering such as [getAll](/api/java/get_all/), [orderBy](/api/java/order_by/) and [between](/api/java/between/) will result in an error.
    /// 
    /// __Example:__ Create a compound index based on the fields `postId` and `date`.
    /// 
    /// ```java
    /// r.table("comments").indexCreate("postAndDate",
    ///     row -> r.array(row.g("postId"), row.g("date"))
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Create a multi index based on the field `authors`.
    /// 
    /// ```java
    /// r.table("posts").indexCreate("authors").optArg("multi", true).run(conn);
    /// ```
    /// 
    /// __Example:__ Create a geospatial multi index based on the field `towers`.
    /// 
    /// ```java
    /// r.table("networks").indexCreate("towers")
    ///  .optArg("geo", true).optArg("multi", true).run(conn);
    /// ```
    /// 
    /// __Example:__ Create an index based on an arbitrary expression.
    /// 
    /// ```java
    /// r.table("posts").indexCreate("authors", doc -> r.branch(
    ///     doc.hasFields("updatedAt"),
    ///     doc.g("updatedAt"),
    ///     doc.g("createdAt")
    /// )).run(conn);
    /// ```
    /// 
    /// __Example:__ Create a new secondary index based on an existing one.
    /// 
    /// ```java
    /// byte[] index = r.table("posts").indexStatus("authors").nth(0).g("function")
    ///     .run(conn);
    /// r.table("newPosts").indexCreate("authors", index).run(conn);
    /// ```
    /// 
    /// __Example:__ Rebuild an outdated secondary index on a table.
    /// 
    /// ```java
    /// byte[] oldIndex = r.table("posts")
    ///     .indexStatus("oldIndex").nth(0).g("function").run(conn);
    /// 
    /// r.table("posts").indexCreate("newIndex", oldIndex).run(conn);
    /// r.table("posts").indexWait("newIndex").run(conn);
    /// r.table("posts").indexRename("newIndex", "oldIndex")
    ///  .optArg("overwrite", true).run(conn);
    /// ```

    pub fn index_create<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("index_create", args)
    }


    /// Delete a previously created secondary index of this table
///
/// 
///
/// 
                /// 
    /// __Example:__ Drop a secondary index named 'code_name'.
    /// 
    /// ```java
    /// r.table("dc").indexDrop("code_name").run(conn);
    /// ```
    /// 
    /// 

    pub fn index_drop<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("index_drop", args)
    }


    /// List all the secondary indexes of this table
///
/// 
///
/// 
                /// 
    /// __Example:__ List the available secondary indexes for this table.
    /// 
    /// ```java
    /// r.table('marvel').indexList().run(conn);
    /// ```
    /// 

    pub fn index_list(&self) -> Client {
        cmd("index_list")
    }


    /// Rename an existing secondary index on a table
///
/// 
///
/// If the [optArg](/api/java/optarg) `overwrite` is specified as `true`, a previously existing index with the new name will be deleted and the index will be renamed. If `overwrite` is `false` (the default) an error will be raised if the new index name already exists.
                /// 
    /// The return value on success will be an object of the format `{"renamed": 1}`, or `{"renamed": 0}` if the old and new names are the same.
    /// 
    /// An error will be raised if the old index name does not exist, if the new index name is already in use and `overwrite` is `false`, or if either the old or new index name are the same as the primary key field name.
    /// 
    /// __Example:__ Rename an index on the comments table.
    /// 
    /// ```java
    /// r.table("comments").indexRename("postId", "messageId").run(conn);
    /// ```
    /// 
    /// __Example:__ Rename an index on the users table, overwriting any existing index with the new name.
    /// 
    /// ```java
    /// r.table("users").indexRename("mail", "email").optArg("overwrite", true)
    ///  .run(conn);
    /// ```

    pub fn index_rename<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("index_rename", args)
    }


    /// 
///
/// 
///
/// 
                /// Get the status of the specified indexes on this table, or the status
    /// of all indexes on this table if no indexes are specified.
    /// 
    /// The result is an array where for each index, there will be an object like this one (shown as JSON):
    /// 
    /// ```json
    /// {
    ///     "index": <indexName>,
    ///     "ready": true,
    ///     "function": <binary>,
    ///     "multi": <bool>,
    ///     "geo": <bool>,
    ///     "outdated": <bool>
    /// }
    /// ```
    /// 
    /// or this one:
    /// 
    /// ```json
    /// {
    ///     "index": <indexName>,
    ///     "ready": false,
    ///     "progress": <float>,
    ///     "function": <binary>,
    ///     "multi": <bool>,
    ///     "geo": <bool>,
    ///     "outdated": <bool>
    /// }
    /// ```
    /// 
    /// The `multi` field will be `true` or `false` depending on whether this index was created as a multi index; the `geo` field will be `true` or `false` depending on whether this index was created as a geospatial index. See [indexCreate](/api/java/index_create/) for details. The `outdated` field will be true if the index is outdated in the current version of RethinkDB and needs to be rebuilt. The `progress` field is a float between `0` and `1`, indicating how far along the server is in constructing indexes after the most recent change to the table that would affect them. (`0` indicates no such indexes have been constructed; `1` indicates all of them have.)
    /// 
    /// The `function` field is a binary object containing an opaque representation of the secondary index (including the `multi` argument if specified). It can be passed as the second argument to [indexCreate](/api/java/index_create/) to create a new index with the same function; see `indexCreate` for more information.
    /// 
    /// __Example:__ Get the status of all the indexes on `test`:
    /// 
    /// ```java
    /// r.table("test").indexStatus().run(conn);
    /// ```
    /// 
    /// __Example:__ Get the status of the `timestamp` index:
    /// 
    /// ```java
    /// r.table("test").indexStatus("timestamp").run(conn);
    /// ```
    /// 
    /// __Example:__ Save the binary representation of the index:
    /// 
    /// ```java
    /// byte[] func = r.table("test").indexStatus("timestamp").nth(0).g("function")
    ///     .run(conn);
    /// 
    /// ```

    pub fn index_status(&self) -> Client {
        cmd("index_status")
    }


    /// 
///
/// 
///
/// 
                /// Wait for the specified indexes on this table to be ready, or for all
    /// indexes on this table to be ready if no indexes are specified.
    /// 
    /// The result is an array containing one object for each table index:
    /// 
    /// ```json
    /// {
    ///     "index": <indexName>,
    ///     "ready": true,
    ///     "function": <binary>,
    ///     "multi": <bool>,
    ///     "geo": <bool>,
    ///     "outdated": <bool>
    /// }
    /// ```
    /// 
    /// See the [indexStatus](/api/java/index_status) documentation for a description of the field values.
    /// 
    /// __Example:__ Wait for all indexes on the table `test` to be ready:
    /// 
    /// ```java
    /// r.table("test").indexWait().run(conn);
    /// ```
    /// 
    /// __Example:__ Wait for the index `timestamp` to be ready:
    /// 
    /// ```java
    /// r.table("test").indexWait("timestamp").run(conn);
    /// ```

    pub fn index_wait(&self) -> Client {
        cmd("index_wait")
    }


    /// 
///
/// <img src="https://rethinkdb.com/assets/images/docs/api_illustrations/insert_javascript.png" class="api_command_illustration" />
///
/// 
                /// You can pass the following options using [optArg](/api/java/optarg/):
    /// 
    /// - `durability`: possible values are `hard` and `soft`. This option will override the table or query's durability setting (set in [run](/api/java/run/)). In soft durability mode RethinkDB will acknowledge the write immediately after receiving and caching it, but before the write has been committed to disk.
    /// - `return_changes`:
    ///     - `true`: return a `changes` array consisting of `old_val`/`new_val` objects describing the changes made, only including the documents actually updated.
    ///     - `false`: do not return a `changes` array (the default).
    ///     - `"always"`: behave as `true`, but include all documents the command tried to update whether or not the update was successful. (This was the behavior of `true` pre-2.0.)
    /// - `conflict`: Determine handling of inserting documents with the same primary key as existing entries. There are three built-in methods: `"error"`, `"replace"` or `"update"`; alternatively, you may provide a conflict resolution function.
    ///     - `"error"`: Do not insert the new document and record the conflict as an error. This is the default.
    ///     - `"replace"`: [Replace](/api/java/replace/) the old document in its entirety with the new one.
    ///     - `"update"`: [Update](/api/java/update/) fields of the old document with fields from the new one.
    ///     - `(id, oldDoc, newDoc) -> resolvedDoc`: a function that receives the id, old and new documents as arguments and returns a document which will be inserted in place of the conflicted one.
    /// 
    /// If `return_changes` is set to `true` or `"always"`, the `changes` array will follow the same order as the inserted documents. Documents in `changes` for which an error occurs (such as a key conflict) will have a third field, `error`, with an explanation of the error.
    /// 
    /// Insert returns an object that contains the following attributes:
    /// 
    /// - `inserted`: the number of documents successfully inserted.
    /// - `replaced`: the number of documents updated when `conflict` is set to `"replace"` or `"update"`.
    /// - `unchanged`: the number of documents whose fields are identical to existing documents with the same primary key when `conflict` is set to `"replace"` or `"update"`.
    /// - `errors`: the number of errors encountered while performing the insert.
    /// - `first_error`: If errors were encountered, contains the text of the first error.
    /// - `deleted` and `skipped`: 0 for an insert operation.
    /// - `generated_keys`: a list of generated primary keys for inserted documents whose primary keys were not specified (capped to 100,000).
    /// - `warnings`: if the field `generated_keys` is truncated, you will get the warning _"Too many generated keys (&lt;X&gt;), array truncated to 100000."_.
    /// - `changes`: if `returnChanges` is set to `true`, this will be an array of objects, one for each objected affected by the `insert` operation. Each object will have two keys: `{new_val: <new value>, old_val: null}`.
    /// 
    /// {% infobox alert %}
    /// RethinkDB write operations will only throw exceptions if errors occur before any writes. Other errors will be listed in `first_error`, and `errors` will be set to a non-zero count. To properly handle errors with this term, code must both handle exceptions and check the `errors` return value!
    /// {% endinfobox %}
    /// 
    /// __Example:__ Insert a document into the table `posts`.
    /// 
    /// ```java
    /// r.table("posts").insert(
    ///     r.hashMap("id", 1)
    ///      .with("title", "Lorem ipsum")
    ///      .with("content", "Dolor sit amet")
    /// ).run(conn);
    /// ```
    /// 
    /// <!-- stop -->
    /// 
    /// The result will be:
    /// 
    /// ```json
    /// {
    ///     "deleted": 0,
    ///     "errors": 0,
    ///     "inserted": 1,
    ///     "replaced": 0,
    ///     "skipped": 0,
    ///     "unchanged": 0
    /// }
    /// ```
    /// 
    /// 
    /// __Example:__ Insert a document without a defined primary key into the table `posts` where the
    /// primary key is `id`.
    /// 
    /// ```java
    /// r.table("posts").insert(
    ///     r.hashMap("title", "Lorem ipsum")
    ///      .with("content", "Dolor sit amet")
    /// ).run(conn);
    /// ```
    /// 
    /// RethinkDB will generate a primary key and return it in `generated_keys`.
    /// 
    /// ```json
    /// {
    ///     "deleted": 0,
    ///     "errors": 0,
    ///     "generated_keys": [
    ///         "dd782b64-70a7-43e4-b65e-dd14ae61d947"
    ///     ],
    ///     "inserted": 1,
    ///     "replaced": 0,
    ///     "skipped": 0,
    ///     "unchanged": 0
    /// }
    /// ```
    /// 
    /// Retrieve the document you just inserted with:
    /// 
    /// ```java
    /// r.table("posts").get("dd782b64-70a7-43e4-b65e-dd14ae61d947").run(conn);
    /// ```
    /// 
    /// And you will get back:
    /// 
    /// ```json
    /// {
    ///     "id": "dd782b64-70a7-43e4-b65e-dd14ae61d947",
    ///     "title": "Lorem ipsum",
    ///     "content": "Dolor sit amet",
    /// }
    /// ```
    /// 
    /// 
    /// __Example:__ Insert multiple documents into the table `users`.
    /// 
    /// ```java
    /// r.table("users").insert(r.array(
    ///     r.hashMap("id", "william").with("email", "william@rethinkdb.com"),
    ///     r.hashMap("id", "lara").with("email", "lara@rethinkdb.com")
    /// )).run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ Insert a document into the table `users`, replacing the document if it already exists.  
    /// 
    /// ```java
    /// r.table("users").insert(
    ///     r.hashMap("id", "william").with("email", "william@rethinkdb.com")
    /// ).optArg("conflict", "replace").run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ Copy the documents from `posts` to `postsBackup`.
    /// 
    /// ```java
    /// r.table("postsBackup").insert(r.table("posts")).run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ Get back a copy of the inserted document (with its generated primary key).
    /// 
    /// ```java
    /// r.table("posts").insert(
    ///     r.hashMap("title", "Lorem ipsum")
    ///      .with("content", "Dolor sit amet")
    /// ).optArg("return_changes", true).run(conn);
    /// ```
    /// 
    /// The result will be
    /// 
    /// ```json
    /// {
    ///     "deleted": 0,
    ///     "errors": 0,
    ///     "generated_keys": [
    ///         "dd782b64-70a7-43e4-b65e-dd14ae61d947"
    ///     ],
    ///     "inserted": 1,
    ///     "replaced": 0,
    ///     "skipped": 0,
    ///     "unchanged": 0,
    ///     "changes": [
    ///         {
    ///             "old_val": null,
    ///             "new_val": {
    ///                 "id": "dd782b64-70a7-43e4-b65e-dd14ae61d947",
    ///                 "title": "Lorem ipsum",
    ///                 "content": "Dolor sit amet"
    ///             }
    ///         }
    ///     ]
    /// }
    /// ```
    /// 
    /// __Example:__ Provide a resolution function that concatenates memo content in case of conflict.
    /// 
    /// ```java
    /// // assume newMemos is a list of memo documents to insert
    /// r.table("memos").insert(new_memos).optArg("conflict",
    ///     (id, old_doc, new_doc) -> new_doc.merge(
    ///         r.hashMap(content, old_doc.g("content").add("\n")
    ///                   .add(new_doc.g("content")))
    ///     )
    /// ).run(conn);
    /// ```

    pub fn insert<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("insert", args)
    }


    /// Update JSON documents in a table
///
/// 
///
/// Accepts a JSON document, a ReQL expression, or a combination of the two.
                /// 
    /// You can pass the following options using [optArg](/api/java/optarg/):
    /// 
    /// - `durability`: possible values are `hard` and `soft`. This option will override the table or query's durability setting (set in [run](/api/java/run/)). In soft durability mode RethinkDB will acknowledge the write immediately after receiving it, but before the write has been committed to disk.
    /// - `return_changes`:
    ///     - `true`: return a `changes` array consisting of `old_val`/`new_val` objects describing the changes made, only including the documents actually updated.
    ///     - `false`: do not return a `changes` array (the default).
    ///     - `"always"`: behave as `true`, but include all documents the command tried to update whether or not the update was successful. (This was the behavior of `true` pre-2.0.)
    /// - `non_atomic`: if set to `true`, executes the update and distributes the result to replicas in a non-atomic fashion. This flag is required to perform non-deterministic updates, such as those that require reading data from another table.
    /// 
    /// Update returns an object that contains the following attributes:
    /// 
    /// - `replaced`: the number of documents that were updated.
    /// - `unchanged`: the number of documents that would have been modified except the new value was the same as the old value.
    /// - `skipped`: the number of documents that were skipped because the document didn't exist.
    /// - `errors`: the number of errors encountered while performing the update.
    /// - `first_error`: If errors were encountered, contains the text of the first error.
    /// - `deleted` and `inserted`: 0 for an update operation.
    /// - `changes`: if `return_changes` is set to `true`, this will be an array of objects, one for each objected affected by the `update` operation. Each object will have two keys: `{new_val: <new value>, old_val: <old value>}`.
    /// 
    /// {% infobox alert %}
    /// RethinkDB write operations will only throw exceptions if errors occur before any writes. Other errors will be listed in `first_error`, and `errors` will be set to a non-zero count. To properly handle errors with this term, code must both handle exceptions and check the `errors` return value!
    /// {% endinfobox %}
    /// 
    /// __Example:__ Update the status of the post with `id` of `1` to `published`.
    /// 
    /// ```java
    /// r.table("posts").get(1).update(r.hashMap("status", "published")).run(conn);
    /// ```
    /// 
    /// __Example:__ Update the status of all posts to `published`.
    /// 
    /// ```java
    /// r.table("posts").update(r.hashMap("status", "published")).run(conn);
    /// ```
    /// 
    /// __Example:__ Update the status of all the posts written by William.
    /// 
    /// ```java
    /// r.table("posts").filter(
    ///     r.hashMap("author", "William")).update(r.hashMap("status", "published")
    /// ).run(conn);
    /// ```
    /// 
    /// {% infobox alert %}
    /// Note that `filter`, `getAll` and similar operations do _not_ execute in an atomic fashion with `update`. Read [Consistency guarantees](/docs/consistency) for more details. Also, see the example for conditional updates below for a solution using `branch` in an `update` clause.
    /// {% endinfobox %}
    /// 
    /// __Example:__ Increment the field `view` of the post with `id` of `1`.
    /// This query will throw an error if the field `views` doesn't exist.
    /// 
    /// ```java
    /// r.table("posts").get(1).update(
    ///     post -> r.hashMap("views", post.g("views").add(1))
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Increment the field `view` of the post with `id` of `1`.
    /// If the field `views` does not exist, it will be set to `0`.
    /// 
    /// ```java
    /// r.table("posts").get(1).update(
    ///     post -> r.hashMap("views", post.g("views").add(1).default_(0))
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Perform a conditional update.  
    /// If the post has more than 100 views, set the `type` of a post to `hot`, else set it to `normal`.
    /// 
    /// ```java
    /// r.table("posts").get(1).update(
    ///     post -> r.branch(post.g("views").gt(100),
    ///                      r.hashMap("type", "hot"),
    ///                      r.hashMap("type", "normal")
    ///     )
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Update the field `numComments` with the result of a sub-query. Because this update is not atomic, you must pass the `non_atomic` flag.
    /// 
    /// ```java
    /// r.table("posts").get(1).update(
    ///     r.hashMap("numComments", r.table("comments")
    ///      .filter(r.hashMap("id_post", 1)).count())
    /// ).optArg("non_atomic", true).run(conn);
    /// ```
    /// 
    /// If you forget to specify the `non_atomic` flag, you will get a `ReqlRuntimeError`:
    /// 
    /// ```
    /// ReqlRuntimeError: Could not prove function deterministic.  Maybe you want to use the non_atomic flag? 
    /// ```
    /// 
    /// __Example:__ Update the field `numComments` with a random value between 0 and 100. This update cannot be proven deterministic because of `r.js` (and in fact is not), so you must pass the `nonAtomic` flag.
    /// 
    /// ```java
    /// r.table("posts").get(1).update(
    ///     r.hashMap("numComments", r.js("Math.floor(Math.random()*100)"))
    /// ).optArg("non_atomic", true).run(conn);
    /// ```
    /// 
    /// __Example:__ Update the status of the post with `id` of `1` using soft durability.
    /// 
    /// ```java
    /// r.table("posts").get(1).update(
    ///     r.hashMap(status, "published")
    /// ).optArg("durability", "soft").run(conn);
    /// ```
    /// 
    /// __Example:__ Increment the field `views` and return the values of the document before and after the update operation.
    /// 
    /// ```java
    /// r.table("posts").get(1).update(
    ///     post -> r.hashMap("views", post.g("views").add(1))
    /// ).optArg("return_changes", true).run(conn);
    /// ```
    /// 
    /// The result will now include a `changes` field:
    /// 
    /// ```java
    /// {
    ///     "deleted": 1,
    ///     "errors": 0,
    ///     "inserted": 0,
    ///     "changes": [
    ///         {
    ///             "new_val": {
    ///                 "id": 1,
    ///                 "author": "Julius_Caesar",
    ///                 "title": "Commentarii de Bello Gallico",
    ///                 "content": "Aleas jacta est",
    ///                 "views": 207
    ///             },
    ///             "old_val": {
    ///                 "id": 1,
    ///                 "author": "Julius_Caesar",
    ///                 "title": "Commentarii de Bello Gallico",
    ///                 "content": "Aleas jacta est",
    ///                 "views": 206
    ///             }
    ///         }
    ///     ],
    ///     "replaced": 0,
    ///     "skipped": 0,
    ///     "unchanged": 0
    /// }
    /// ```
    /// 
    /// 
    /// ## Updating nested fields ##
    /// 
    /// The `update` command supports RethinkDB's [nested field][nf] syntax to update subdocuments. Consider a user table with contact information in this format:
    /// 
    /// [nf]: /docs/nested-fields/java
    /// 
    /// ```json
    /// {
    /// 	"id": 10001,
    /// 	"name": "Bob Smith",
    /// 	"contact": {
    /// 		"phone": {
    /// 			"work": "408-555-1212",
    /// 			"home": "408-555-1213",
    /// 			"cell": "408-555-1214"
    /// 		},
    /// 		"email": {
    /// 			"work": "bob@smith.com",
    /// 			"home": "bobsmith@example.com",
    /// 			"other": "bobbys@moosecall.net"
    /// 		},
    /// 		"im": {
    /// 			"skype": "Bob Smith",
    /// 			"aim": "bobmoose",
    /// 			"icq": "nobodyremembersicqnumbers"
    /// 		}
    /// 	},
    /// 	"notes": [
    /// 		{
    /// 			"date": r.time(2014,1,1,'Z'),
    /// 			"from": "John Doe",
    /// 			"subject": "My name is even more boring than Bob's"
    /// 		},
    /// 		{
    /// 			"date": r.time(2014,2,2,'Z'),
    /// 			"from": "Bob Smith Sr",
    /// 			"subject": "Happy Second of February"
    /// 		}
    /// 	]
    /// }
    /// ```
    /// 
    /// __Example:__ Update Bob Smith's cell phone number.
    /// 
    /// ```java
    /// r.table("users").get(10001).update(
    ///     r.hashMap("contact",
    ///         r.hashMap("phone",
    ///             r.hashMap("cell", "408-555-4242")))
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Add another note to Bob Smith's record.
    /// 
    /// ```java
    /// import com.rethinkdb.model.MapObject;
    /// 
    /// MapObject newNote = r.hashMap("date", r.now())
    ///                      .with("from", "Admin")
    ///                      .with("subject", "You killed my father");
    /// 
    /// r.table("users").get(10001).update(
    ///     row -> r.hashMap("notes", row.g("notes").append(newNote))
    /// ).run(conn);
    /// ```
    /// 
    /// This will fail if the `notes` field does not exist in the document. To perform this as an "upsert" (update or insert), use the [default_][def] command to ensure the field is initialized as an empty list.
    /// 
    /// [def]: /api/java/default/
    /// 
    /// ```java
    /// r.table("users").get(10001).update(
    ///     row -> r.hashMap(notes, row.g("notes").default_(r.array()).append(newNote))
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Send a note to every user with an ICQ number.
    /// 
    /// ```java
    /// import com.rethinkdb.model.MapObject;
    /// 
    /// MapObject icqNote = r.hashMap("date", r.now())
    ///                      .with("from", "Admin")
    ///                      .with("subject", "Welcome to the future");
    /// 
    /// r.table("users").filter(
    ///     row -> row.hasFields(r.hashMap("contact", r.hashMap("im", "icq")))
    /// ).update(r.hashMap("notes", row.g("notes").append(icqNote))).run(conn);
    /// ```
    /// 
    /// __Example:__ Replace all of Bob's IM records. Normally, `update` will merge nested documents together; to replace the entire `"im"` document, use the [literal][] command.
    /// 
    /// [literal]: /api/java/literal/
    /// 
    /// ```java
    /// r.table("users").get(10001).update(
    ///     r.hashMap("contact",
    ///         r.hashMap("im",
    ///             r.literal(r.hashMap("aim", "themoosemeister"))))
    /// ).run(conn);
    /// ```

    pub fn update<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("update", args)
    }


    /// Replace documents in a table
///
/// 
///
/// Accepts a JSON document or a ReQL expression,
                /// and replaces the original document with the new one. The new document must
    /// have the same primary key as the original document.
    /// 
    /// The `replace` command can be used to both insert and delete documents. If
    /// the "replaced" document has a primary key that doesn't exist in the table,
    /// the document will be inserted; if an existing document is replaced with
    /// `null`, the document will be deleted. Since `update` and `replace` operations
    /// are performed atomically, this allows atomic inserts and deletes as well.
    /// 
    /// You can pass the following options using [optArg](/api/java/optarg/):
    /// 
    /// - `durability`: possible values are `hard` and `soft`. This option will override
    ///   the table or query's durability setting (set in [run](/api/java/run/)).
    ///   In soft durability mode RethinkDB will acknowledge the write immediately after
    ///   receiving it, but before the write has been committed to disk.
    /// - `return_changes`:
    ///     - `true`: return a `changes` array consisting of `old_val`/`new_val` objects
    ///       describing the changes made, only including the documents actually
    ///       updated.
    ///     - `false`: do not return a `changes` array (the default).
    ///     - `"always"`: behave as `true`, but include all documents the command tried
    ///       to update whether or not the update was successful. (This was the behavior
    ///       of `true` pre-2.0.)
    /// - `non_atomic`: if set to `true`, executes the replacement and distributes the
    ///   result to replicas in a non-atomic fashion. This flag is required to perform
    ///   non-deterministic updates, such as those that require reading data from
    ///   another table.
    /// 
    /// Replace returns an object that contains the following attributes:
    /// 
    /// - `replaced`: the number of documents that were replaced.
    /// - `unchanged`: the number of documents that would have been modified, except
    ///   that the new value was the same as the old value.
    /// - `inserted`: the number of new documents added. A document is considered inserted if its primary key did not exist in the table at the time of the `replace` operation.
    /// - `deleted`: the number of deleted documents when doing a replace with `null`.
    /// - `errors`: the number of errors encountered while performing the replace.
    /// - `first_error`: If errors were encountered, contains the text of the first
    ///   error.
    /// - `skipped`: 0 for a replace operation.
    /// - `changes`: if `returnChanges` is set to `true`, this will be an array of
    ///   objects, one for each objected affected by the `replace` operation. Each
    ///   object will have two keys: `{"new_val": <new value>, "old_val": <old value>}`.
    /// 
    /// {% infobox alert %}
    /// RethinkDB write operations will only throw exceptions if errors occur before any writes. Other errors will be listed in `first_error`, and `errors` will be set to a non-zero count. To properly handle errors with this term, code must both handle exceptions and check the `errors` return value!
    /// {% endinfobox %}
    /// 
    /// __Example:__ Replace the document with the primary key `1`.
    /// 
    /// ```java
    /// r.table("posts").get(1).replace(
    ///     r.hashMap("id", 1).with("title", "Lorem ipsum")
    ///      .with("content", "Aleas jacta est")
    ///      .with("status", "draft")
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Remove the field `status` from all posts.
    /// 
    /// ```java
    /// r.table("posts").replace(post -> post.without("status")).run(conn);
    /// ```
    /// 
    /// __Example:__ Remove all the fields that are not `id`, `title` or `content`.
    /// 
    /// ```java
    /// r.table("posts").replace(
    ///     post -> post.pluck("id", "title", "content")
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Replace the document with the primary key `1` using soft durability.
    /// 
    /// ```java
    /// r.table("posts").get(1).replace(
    ///     r.hashMap("id", 1)
    ///      .with("title", "Lorem ipsum")
    ///      .with("content", "Aleas jacta est")
    ///      .with("status", "draft")
    /// ).optArg("durability", "soft").run(conn);
    /// ```
    /// 
    /// __Example:__ Replace the document with the primary key `1` and return the values of the document before
    /// and after the replace operation.
    /// 
    /// ```java
    /// r.table("posts").get(1).replace(
    ///     r.hashMap("id", 1)
    ///      .with("title", "Lorem ipsum")
    ///      .with("content", "Aleas jacta est")
    ///      .with("status", "published")
    /// ).optArg("return_changes", true).run(conn);
    /// ```
    /// 
    /// The result will have a `changes` field:
    /// 
    /// ```json
    /// {
    ///     "deleted": 0,
    ///     "errors":  0,
    ///     "inserted": 0,
    ///     "changes": [
    ///         {
    ///             "new_val": {
    ///                 "id":1,
    ///                 "title": "Lorem ipsum"
    ///                 "content": "Aleas jacta est",
    ///                 "status": "published",
    ///             },
    ///             "old_val": {
    ///                 "id":1,
    ///                 "title": "Lorem ipsum"
    ///                 "content": "TODO",
    ///                 "status": "draft",
    ///                 "author": "William",
    ///             }
    ///         }
    ///     ],   
    ///     "replaced": 1,
    ///     "skipped": 0,
    ///     "unchanged": 0
    /// }
    /// ```

    pub fn replace<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("replace", args)
    }


    /// Delete one or more documents from a table
///
/// 
///
/// 
                /// 
    /// You can pass the following options using [optArg](/api/java/optarg/):
    /// 
    /// - `durability`: possible values are `hard` and `soft`. This option will override the
    /// table or query's durability setting (set in [run](/api/java/run/)).  
    /// In soft durability mode RethinkDB will acknowledge the write immediately after
    /// receiving it, but before the write has been committed to disk.
    /// - `return_changes`:
    ///     - `true`: return a `changes` array consisting of `old_val`/`new_val` objects describing the changes made, only including the documents actually updated.
    ///     - `false`: do not return a `changes` array (the default).
    ///     - `"always"`: behave as `true`, but include all documents the command tried to update whether or not the update was successful. (This was the behavior of `true` pre-2.0.)
    /// 
    /// 
    /// `delete` returns an object that contains the following attributes:
    /// 
    /// - `deleted`: the number of documents that were deleted.
    /// - `skipped`: the number of documents that were skipped.  
    /// For example, if you attempt to delete a batch of documents, and another concurrent query
    /// deletes some of those documents first, they will be counted as skipped.
    /// - `errors`: the number of errors encountered while performing the delete.
    /// - `first_error`: If errors were encountered, contains the text of the first error.
    /// - `inserted`, `replaced`, and `unchanged`: all 0 for a delete operation..
    /// - `changes`: if `returnChanges` is set to `true`, this will be an array of objects, one for each objected affected by the `delete` operation. Each object will have two keys: `{new_val: null, old_val: <old value>}`.
    /// 
    /// {% infobox alert %}
    /// RethinkDB write operations will only throw exceptions if errors occur before any writes. Other errors will be listed in `first_error`, and `errors` will be set to a non-zero count. To properly handle errors with this term, code must both handle exceptions and check the `errors` return value!
    /// {% endinfobox %}
    /// 
    /// __Example:__ Delete a single document from the table `comments`.
    /// 
    /// ```java
    /// r.table("comments").get("7eab9e63-73f1-4f33-8ce4-95cbea626f59").delete().run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ Delete all documents from the table `comments`.
    /// 
    /// ```java
    /// r.table("comments").delete().run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ Delete all comments where the field `idPost` is `3`.
    /// 
    /// ```java
    /// r.table("comments").filter(r.hashMap("idPost", 3)).delete().run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ Delete a single document from the table `comments` and return its value.
    /// 
    /// ```java
    /// r.table("comments").get("7eab9e63-73f1-4f33-8ce4-95cbea626f59")
    ///  .delete().optArg("return_changes", true).run(conn);
    /// ```
    /// 
    /// The result looks like:
    /// 
    /// ```json
    /// {
    ///     "deleted": 1,
    ///     "errors": 0,
    ///     "inserted": 0,
    ///     "changes": [
    ///         {
    ///             "new_val": null,
    ///             "old_val": {
    ///                 "id": "7eab9e63-73f1-4f33-8ce4-95cbea626f59",
    ///                 "author": "William",
    ///                 "comment": "Great post",
    ///                 "idPost": 3
    ///             }
    ///         }
    ///     ],
    ///     "replaced": 0,
    ///     "skipped": 0,
    ///     "unchanged": 0
    /// }
    /// ```
    /// 
    /// 
    /// __Example:__ Delete all documents from the table `comments` without waiting for the
    /// operation to be flushed to disk.
    /// 
    /// ```java
    /// r.table("comments").delete().optArg("durability", "soft").run(conn);
    /// ```

    pub fn delete(&self) -> Client {
        cmd("delete")
    }


    /// Ensure that writes on a given table are written to permanent storage
///
/// 
///
/// Queries that specify soft durability do not wait for writes to be committed to disk; a call to `sync` on a table will not return until all previous writes to the table are completed, guaranteeing the data's persistence.
                /// 
    /// If successful, the operation returns an object: `{"synced": 1}`.
    /// 
    /// __Example:__ After having updated multiple heroes with soft durability, we now want to wait
    /// until these changes are persisted.
    /// 
    /// ```java
    /// r.table("marvel").sync().run(conn);
    /// ```
    /// 
    /// 

    pub fn sync(&self) -> Client {
        cmd("sync")
    }


    /// Reference a database
///
/// 
///
/// 
                /// 
    /// The `db` command is optional. If it is not present in a query, the query will run against the default database for the connection, specified in the `db` argument to [connect](/api/java/connect).
    /// 
    /// __Example:__ Explicitly specify a database for a query.
    /// 
    /// ```java
    /// r.db("heroes").table("marvel").run(conn);
    /// ```
    /// 

    pub fn db<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("db", args)
    }


    /// Return all documents in a table
///
/// 
///
/// Other commands may be chained after `table` to return a subset of documents (such as [get](/api/java/get/) and [filter](/api/java/filter/)) or perform further processing.
                /// 
    /// __Example:__ Return all documents in the table 'marvel' of the default database.
    /// 
    /// ```java
    /// r.table("marvel").run(conn);
    /// ```
    /// 
    /// __Example:__ Return all documents in the table 'marvel' of the database 'heroes'.
    /// 
    /// ```java
    /// r.db("heroes").table("marvel").run(conn);
    /// ```
    /// 
    /// There are two [optArgs](/api/java/optarg) that may be specified.
    /// 
    /// * `read_mode`: One of three possible values affecting the consistency guarantee for the table read:
    ///     * `single` returns values that are in memory (but not necessarily written to disk) on the primary replica. This is the default.
    ///     * `majority` will only return values that are safely committed on disk on a majority of replicas. This requires sending a message to every replica on each read, so it is the slowest but most consistent.
    ///     * `outdated` will return values that are in memory on an arbitrarily-selected replica. This is the fastest but least consistent.
    /// * `identifier_format`: possible values are `name` and `uuid`, with a default of `name`. If set to `uuid`, then [system tables](/docs/system-tables/) will refer to servers, databases and tables by UUID rather than name. (This only has an effect when used with system tables.)
    /// 
    /// __Example:__ Allow potentially out-of-date data in exchange for faster reads.
    /// 
    /// ```java
    /// r.db("heroes").table("marvel").optArg("read_mode", "outdated").run(conn);
    /// ```

    pub fn table<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("table", args)
    }


    /// Get a document by primary key
///
/// 
///
/// 
                /// 
    /// If no document exists with that primary key, `get` will return `null`.
    /// 
    /// __Example:__ Find a document by UUID.
    /// 
    /// ```java
    /// r.table("posts").get("a9849eef-7176-4411-935b-79a6e3c56a74").run(conn);
    /// ```
    /// 
    /// __Example:__ Find a document and merge another document with it.
    /// 
    /// ```java
    /// r.table("heroes").get(3).merge(
    ///     r.hashMap("powers", r.array("invisibility", "speed"))
    /// ).run(conn);
    /// ```
    /// 
    /// ___Example:__ Subscribe to a document's [changefeed](/docs/changefeeds/).
    /// 
    /// ```java
    /// r.table("heroes").get(3).changes().run(conn);
    /// ```

    pub fn get<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("get", args)
    }


    /// Get all documents where the given value matches the value of the requested index
///
/// 
///
/// 
                /// 
    /// __Example:__ Secondary index keys are not guaranteed to be unique so we cannot query via [get](/api/java/get/) when using a secondary index.
    /// 
    /// ```java
    /// r.table("marvel").getAll("man_of_steel").optArg("index", "code_name").run(conn);
    /// ```
    /// 
    /// __Example:__ Without an index argument, we default to the primary index. While `get` will either return the document or `null` when no document with such a primary key value exists, this will return either a one or zero length stream.
    /// 
    /// ```java
    /// r.table("dc").getAll("superman").run(conn);
    /// ```
    /// 
    /// __Example:__ You can get multiple documents in a single call to `get_all`.
    /// 
    /// ```java
    /// r.table("dc").getAll("superman", "ant man").run(conn);
    /// ```
    /// 
    /// {% infobox %}
    /// __Note:__ `getAll` does not perform any de-duplication. If you pass the same key more than once, the same document will be returned multiple times.
    /// {% endinfobox %}
    /// 
    /// __Example:__ You can use [args](/api/java/args/) with `getAll` to retrieve multiple documents whose keys are in a list. This uses `getAll` to get a list of female superheroes, coerces that to an array, and then gets a list of villains who have those superheroes as enemies.
    /// 
    /// ```java
    /// r.do(
    ///     r.table("heroes").getAll("f").optArg("index", "gender")
    ///         .g("id").coerceTo("array"),
    ///     heroines -> r.table("villains").getAll(r.args(heroines))
    /// ).run(conn);
    /// ```
    /// 
    /// Calling `getAll` with zero arguments&mdash;which could happen in this example if the `heroines` list had no elements&mdash;will return nothing, i.e., a zero length stream.
    /// 
    /// Secondary indexes can be used in extremely powerful ways with `getAll` and other commands; read the full article on [secondary indexes](/docs/secondary-indexes) for examples using boolean operations, `contains` and more.

    pub fn get_all<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("get_all", args)
    }


    /// Get all documents between two keys
///
/// 
///
/// Accepts three [optArgs](/api/java/optarg): `index`, `left_bound`, and `right_bound`. If `index` is set to the name of a secondary index, `between` will return all documents where that index's value is in the specified range (it uses the primary key by default). `left_bound` or `right_bound` may be set to `open` or `closed` to indicate whether or not to include that endpoint of the range (by default, `left_bound` is closed and `right_bound` is open).
                /// 
    /// You may also use the special constants `r.minval` and `r.maxval` for boundaries, which represent "less than any index key" and "more than any index key" respectively. For instance, if you use `r.minval` as the lower key, then `between` will return all documents whose primary keys (or indexes) are less than the specified upper key.
    /// 
    /// If you use arrays as indexes (compound indexes), they will be sorted using [lexicographical order][lo]. Take the following range as an example:
    /// 
    /// 	[[1, "c"] ... [5, "e"]]
    /// 
    /// This range includes all compound keys:
    /// 
    /// * whose first item is 1 and second item is equal or greater than "c";
    /// * whose first item is between 1 and 5, *regardless of the value of the second item*;
    /// * whose first item is 5 and second item is less than or equal to "e".
    /// 
    /// [lo]: https://en.wikipedia.org/wiki/Lexicographical_order
    /// 
    /// __Example:__ Find all users with primary key >= 10 and < 20 (a normal half-open interval).
    /// 
    /// ```java
    /// r.table("marvel").between(10, 20).run(conn);
    /// ```
    /// 
    /// __Example:__ Find all users with primary key >= 10 and <= 20 (an interval closed on both sides).
    /// 
    /// ```py
    /// r.table("marvel").between(10, 20).optArg("right_bound", "closed").run(conn);
    /// ```
    /// 
    /// __Example:__ Find all users with primary key < 20.
    /// 
    /// ```py
    /// r.table("marvel").between(r.minval(), 20).run(conn);
    /// ```
    /// 
    /// __Example:__ Find all users with primary key > 10.
    /// 
    /// ```py
    /// r.table("marvel").between(10, r.maxval()).optArg("left_bound", "open").run(conn);
    /// ```
    /// 
    /// __Example:__ Between can be used on secondary indexes too. Just pass an optional index argument giving the secondary index to query.
    /// 
    /// ```py
    /// r.table("dc").between("dark_knight", "man_of_steel").optArg("index", "code_name").run(conn);
    /// ```
    /// 
    /// __Example:__ Get all users whose full name is between "John Smith" and "Wade Welles."
    /// 
    /// ```py
    /// r.table("users").between(r.array("Smith", "John"), r.array("Welles", "Wade")).optArg("index", "full_name").run(conn);
    /// ```
    /// 
    /// __Note:__ Between works with secondary indexes on date fields, but will not work with unindexed date fields. To test whether a date value is between two other dates, use the [during](/api/java/during) command, not `between`.
    /// 
    /// Secondary indexes can be used in extremely powerful ways with `between` and other commands; read the full article on [secondary indexes](/docs/secondary-indexes) for examples using boolean operations, `contains` and more.
    /// 
    /// __Note:__ RethinkDB uses byte-wise ordering for `between` and does not support Unicode collations; non-ASCII characters will be sorted by UTF-8 codepoint.
    /// 
    /// __Note:__ If you chain `between` after [orderBy](/api/java/order_by), the `between` command must use the index specified in `orderBy`, and will default to that index. Trying to specify another index will result in a `ReqlRuntimeError`.

    pub fn between<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("between", args)
    }


    /// Return all the elements in a sequence for which the given predicate is true
///
/// 
///
/// The return value of `filter` will be the same as the input (sequence, stream, or array). Documents can be filtered in a variety of ways&mdash;ranges, nested values, boolean conditions, and the results of anonymous functions.
                /// 
    /// By default, `filter` will silently skip documents with missing fields: if the predicate tries to access a field that doesn't exist (for instance, the predicate `{age: 30}` applied to a document with no `age` field), that document will not be returned in the result set, and no error will be generated. This behavior can be changed with the `default` [optArg](/api/java/optarg).
    /// 
    /// * If `default` is set to `true`, documents with missing fields will be returned rather than skipped.
    /// * If `default` is set to `r.error()`, a `ReqlRuntimeError` will be thrown when a document with a missing field is tested.
    /// * If `default` is set to `false` (the default), documents with missing fields will be skipped.
    /// 
    /// {% infobox %}
    /// __Note:__ `filter` does not use [secondary indexes](/docs/secondary-indexes/). For retrieving documents via secondary indexes, consider [getAll](/api/java/get_all/), [between](/api/java/between/) and [eqJoin](/api/java/eq_join/).
    /// {% endinfobox %}
    /// 
    /// ## Basic predicates ##
    /// 
    /// __Example:__ Get all users who are 30 years old.
    /// 
    /// 
    /// ```java
    /// r.table("users").filter(r.hashMap(age, 30)).run(conn);
    /// ```
    /// 
    /// The predicate `r.hashMap(age, 30)` selects documents in the `users` table with an `age` field whose value is `30`. Documents with an `age` field set to any other value *or* with no `age` field present are skipped.
    /// 
    /// <!-- stop -->
    /// 
    /// While the `r.hashMap(field, value)` style of predicate is useful for exact matches, a more general way to write a predicate is to use an anonymous function that returns `true` or `false`.
    /// 
    /// ```java
    /// r.table("users").filter(row -> row.g("age").eq(30)).run(conn);
    /// ```
    /// 
    /// In this case, the function returns `true` if the field `age` is equal to 30.
    /// 
    /// Predicates to `filter` are evaluated on the server, and must use ReQL expressions. You cannot use standard Java comparison operators such as `==`, `<`/`>` and `||`/`&&`.
    /// 
    /// __Example:__ Get all users who are more than 18 years old.
    /// 
    /// ```java
    /// r.table("users").filter(row -> row.g("age").gt(18)).run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ Get all users who are less than 18 years old and more than 13 years old.
    /// 
    /// ```java
    /// r.table("users").filter(
    ///     row -> row.g("age").lt(18).and(row.g("age").gt(13))
    /// ).run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ Get all users who are more than 18 years old or have their parental consent.
    /// 
    /// ```java
    /// r.table("users").filter(
    ///     row -> row.g("age").ge(18).or(row.g("hasParentalConsent"))
    /// ).run(conn);
    /// ```
    /// 
    /// ## More complex predicates ##
    /// 
    /// __Example:__ Retrieve all users who subscribed between January 1st, 2012
    /// (included) and January 1st, 2013 (excluded).
    /// 
    /// ```java
    /// r.table("users").filter(
    ///     user -> user.g("subscription_date").during(
    ///         r.time(2012, 1, 1, "Z"), r.time(2013, 1, 1, "Z"))
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Retrieve all users who have a gmail account (whose field `email` ends with `@gmail.com`).
    /// 
    /// ```java
    /// r.table("users").filter(
    ///     user -> user.g("email").match("@gmail.com$")
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Filter based on the presence of a value in an array.
    /// 
    /// Given this schema for the `users` table:
    /// 
    /// ```json
    /// {
    ///     "name": String
    ///     "placesVisited": [String]
    /// }
    /// ```
    /// 
    /// Retrieve all users whose field `placesVisited` contains `France`.
    /// 
    /// ```java
    /// r.table("users").filter(
    ///     user -> user.g("placesVisited").contains("France")
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Filter based on nested fields.
    /// 
    /// Given this schema for the `users` table:
    /// 
    /// ```json
    /// {
    ///     "id": String
    ///     "name": {
    ///         "first": String,
    ///         "middle": String,
    ///         "last": String
    ///     }
    /// }
    /// ```
    /// 
    /// Retrieve all users named "William Adama" (first name "William", last name
    /// "Adama"), with any middle name.
    /// 
    /// 
    /// ```json
    /// r.table("users").filter(
    ///     r.hashMap("name", r.hashMap("first", "William").with("last", "Adama"))
    /// ).run(conn);
    /// ```
    /// 
    /// If you want an exact match for a field that is an object, you will have to use `r.literal`.
    /// 
    /// Retrieve all users named "William Adama" (first name "William", last name
    /// "Adama"), and who do not have a middle name.
    /// 
    /// ```java
    /// r.table("users").filter(r.literal(
    ///     r.hashMap("name", r.hashMap("first", "William").with("last", "Adama"))
    /// )).run(conn);
    /// ```
    /// 
    /// You may rewrite these with anonymous functions.
    /// 
    /// ```java
    /// r.table("users").filter(
    ///     user -> user.g("name").g("first").eq("William")
    ///         .and(user.g("name").g("last").eq("Adama"))
    /// ).run(conn);
    /// 
    /// r.table("users").filter(
    ///     user -> user.g("name")
    ///         .eq(r.hashMap("first", "William").with("last", "Adama"))
    /// ).run(conn);
    /// ```
    /// 
    /// ## Handling missing fields ##
    /// 
    /// By default, documents missing fields tested by the `filter` predicate are skipped. In the previous examples, users without an `age` field are not returned. By passing the optional `default` argument to `filter`, you can change this behavior.
    /// 
    /// __Example:__ Get all users less than 18 years old or whose `age` field is missing.
    /// 
    /// ```java
    /// r.table("users").filter(row -> row.g("age").lt(18)).optArg("default", true).run(conn);
    /// ```
    /// 
    /// __Example:__ Get all users more than 18 years old. Throw an error if a
    /// document is missing the field `age`.
    /// 
    /// ```java
    /// r.table("users").filter(row -> row.g("age").gt(18))
    ///  .optArg("default", r.error()).run(conn);
    /// ```
    /// 
    /// __Example:__ Get all users who have given their phone number (all the documents whose field `phoneNumber` exists and is not `null`).
    /// 
    /// ```java
    /// r.table("users").filter(user -> user.hasFields("phone_number")).run(conn);
    /// ```
    /// 
    /// __Example:__ Get all users with an "editor" role or an "admin" privilege.
    /// 
    /// ```java
    /// r.table("users").filter(
    ///     user -> user.g("role").eq("editor").default_(false)
    ///         .or(user.g("privilege").eq("admin").default_(false))
    /// ).run(conn);
    /// ```
    /// 
    /// Instead of using a `default` optArg with `filter`, we have to use default values on the fields within the `or` clause. Why? If the field on the left side of the `or` clause is missing from a document&mdash;in this case, if the user doesn't have a `role` field&mdash;the predicate will generate an error, and will return `false` (or the value the `default` argument is set to) without evaluating the right side of the `or`. By using `.default_(false)` on the fields, each side of the `or` will evaluate to either the field's value or `false` if the field doesn't exist.

    pub fn filter<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("filter", args)
    }


    /// Returns an inner join of two sequences
///
/// 
///
/// 
                /// 
    /// The returned sequence represents an intersection of the left-hand sequence and the right-hand sequence: each row of the left-hand sequence will be compared with each row of the right-hand sequence to find all pairs of rows which satisfy the predicate. Each matched pair of rows of both sequences are combined into a result row. In most cases, you will want to follow the join with [zip](/api/java/zip) to combine the left and right results.
    /// 
    /// {% infobox %}
    /// Note that `innerJoin` is slower and much less efficient than using [eqJoin](/api/java/eq_join/) or [concatMap](/api/java/concat_map/) with [getAll](/api/java/get_all/). You should avoid using `innerJoin` in commands when possible.
    /// {% endinfobox %}
    /// 
    /// __Example:__ Return a list of all matchups between Marvel and DC heroes in which the DC hero could beat the Marvel hero in a fight.
    /// 
    /// ```java
    /// r.table("marvel").innerJoin(r.table("dc"),
    ///     (marvel_row, dc_row) -> marvel_row.g("strength").lt(dc_row.g("strength"))
    /// ).zip().run(conn);
    /// ```
    /// 
    /// <!-- stop -->
    /// 
    /// (Compare this to an [outerJoin](/api/java/outer_join) with the same inputs and predicate, which would return a list of *all* Marvel heroes along with any DC heroes with a higher strength.)

    pub fn inner_join<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("inner_join", args)
    }


    /// Returns a left outer join of two sequences
///
/// 
///
/// The returned sequence represents a union of the left-hand sequence and the right-hand sequence: all documents in the left-hand sequence will be returned, each matched with a document in the right-hand sequence if one satisfies the predicate condition. In most cases, you will want to follow the join with [zip](/api/java/zip) to combine the left and right results.
                /// 
    /// 
    /// {% infobox %}
    /// Note that `outerJoin` is slower and much less efficient than using [concatMap](/api/java/concat_map/) with [getAll](/api/java/get_all). You should avoid using `outerJoin` in commands when possible.
    /// {% endinfobox %}
    /// 
    /// __Example:__ Return a list of all Marvel heroes, paired with any DC heroes who could beat them in a fight.
    /// 
    /// ```java
    /// r.table("marvel").outerJoin(r.table("dc"),
    ///     (marvel_row, dc_row) -> marvel_row.g("strength").lt(dc_row.g("strength"))
    /// ).zip().run(conn);
    /// ```
    /// 
    /// (Compare this to an [innerJoin](/api/java/inner_join) with the same inputs and predicate, which would return a list only of the matchups in which the DC hero has the higher strength.)

    pub fn outer_join<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("outer_join", args)
    }


    /// <img alt="Data Modeling Illustration" class="api_command_illustration" src="/assets/images/docs/api_illustrations/table-joins
///
/// 
///
/// png" />
                /// 
    /// Join tables using a field or function on the left-hand sequence matching primary keys or secondary indexes on the right-hand table. `eqJoin` is more efficient than other ReQL join types, and operates much faster. Documents in the result set consist of pairs of left-hand and right-hand documents, matched when the field on the left-hand side exists and is non-null and an entry with that field's value exists in the specified index on the right-hand side.
    /// 
    /// The result set of `eqJoin` is a stream or array of objects. Each object in the returned set will be an object of the form `{ "left": <left-document>, "right": <right-document> }`, where the values of `left` and `right` will be the joined documents. Use the [zip](/api/java/zip/) command to merge the `left` and `right` fields together.
    /// 
    /// The results from `eqJoin` are, by default, not ordered. Providing <code><a href="/api/java/optarg">optArg</a>('ordered', 'true')</code>
    ///  will cause `eqJoin` to order the output based on the left side input stream. (If there are multiple matches on the right side for a document on the left side, their order is not guaranteed even if `ordered` is `true`.) Requiring ordered results can significantly slow down `eqJoin`, and in many circumstances this ordering will not be required. (See the first example, in which ordered results are obtained by using `orderBy` after `eqJoin`.)
    /// 
    /// Suppose the players table contains these documents (shown in JSON form):
    /// 
    /// ```json
    /// [
    ///     { "id": 1, "player": "George", "gameId": 1 },
    ///     { "id": 2, "player": "Agatha", "gameId": 3 },
    ///     { "id": 3, "player": "Fred", "gameId": 2 },
    ///     { "id": 4, "player": "Marie", "gameId": 2 },
    ///     { "id": 5, "player": "Earnest", "gameId": 1 },
    ///     { "id": 6, "player": "Beth", "gameId": 3 }
    /// ]
    /// ```
    /// 
    /// The games table contains these documents:
    /// 
    /// ```json
    /// [
    ///     { "id": 1, "field": "Little Delving" },
    ///     { "id": 2, "field": "Rushock Bog" },
    ///     { "id": 3, "field": "Bucklebury" }
    /// ]
    /// ```
    /// 
    /// __Example:__ Match players with the games they've played against one another.
    /// 
    /// Join these tables using `gameId` on the player table and `id` on the games table:
    /// 
    /// ```java
    /// r.table("players").eqJoin("gameId", r.table("games")).run(conn);
    /// ```
    /// 
    /// This will return a result set such as the following:
    /// 
    /// ```json
    /// [
    ///     {
    ///         "left" : { "gameId" : 3, "id" : 2, "player" : "Agatha" },
    ///         "right" : { "id" : 3, "field" : "Bucklebury" }
    ///     },
    ///     {
    ///         "left" : { "gameId" : 2, "id" : 3, "player" : "Fred" },
    ///         "right" : { "id" : 2, "field" : "Rushock Bog" }
    ///     },
    ///     ...
    /// ]
    /// ```
    /// 
    /// <!-- stop -->
    /// 
    /// What you likely want is the result of using `zip` with that. For clarity, we'll use `without` to drop the `id` field from the games table (it conflicts with the `id` field for the players and it's redundant anyway), and we'll order it by the games.
    /// 
    /// ```java
    /// r.table("players")
    ///  .eqJoin("game_id", r.table("games"))
    ///  .without(r.hashMap("right", "id"))
    ///  .zip()
    ///  .orderBy("game_id")
    ///  .run(conn);
    /// ```
    /// 
    /// ```json
    /// [
    ///     { "field": "Little Delving", "gameId": 1, "id": 5, "player": "Earnest" },
    ///     { "field": "Little Delving", "gameId": 1, "id": 1, "player": "George" },
    ///     { "field": "Rushock Bog", "gameId": 2, "id": 3, "player": "Fred" },
    ///     { "field": "Rushock Bog", "gameId": 2, "id": 4, "player": "Marie" },
    ///     { "field": "Bucklebury", "gameId": 3, "id": 6, "player": "Beth" },
    ///     { "field": "Bucklebury", "gameId": 3, "id": 2, "player": "Agatha" }
    /// ]
    /// ```
    /// 
    /// For more information, see [Table joins in RethinkDB](/docs/table-joins/).
    /// 
    /// __Example:__ Use a secondary index on the right table rather than the primary key. If players have a secondary index on their cities, we can get a list of arenas with players in the same area.
    /// 
    /// ```java
    /// r.table("players").eqJoin("city_id", r.table("arenas"))
    ///  .optArg("index", "city_id").run(conn);
    /// ```
    /// 
    /// __Example:__ Use a nested key as the join field. Suppose the documents in the players table were structured like this:
    /// 
    /// ```json
    /// { "id": 1, "player": "George", "game": {"id": 1} },
    /// { "id": 2, "player": "Agatha", "game": {"id": 3} },
    /// ...
    /// ```
    /// 
    /// Simply specify the field using a lambda instead of a string.
    /// 
    /// ```java
    /// r.table("players").eqJoin(
    ///     row -> row.g("game").g("id"),
    ///     r.table("games")
    /// ).without(r.hashMap("right", "id")).zip().run(conn);
    /// ```
    /// 
    /// ```json
    /// [
    ///     { "field": "Little Delving", "game": { "id": 1 }, "id": 5, "player": "Earnest" },
    ///     { "field": "Little Delving", "game": { "id": 1 }, "id": 1, "player": "George" },
    ///     ...
    /// ]
    /// ```
    /// 
    /// __Example:__ Use a function instead of a field to join on a more complicated expression. Suppose the players have lists of favorite games ranked in order in a field such as `favorites: [3, 2, 1]`. Get a list of players and their top favorite:
    /// 
    /// ```java
    /// r.table("players").eqJoin(
    ///     player -> player.g("favorites").nth(0),
    ///     r.table("games")
    /// ).without(
    ///     r.array(r.hashMap("left", r.array("favorites", "game_id", "id")),
    ///     r.hashMap("right", "id"))
    /// ).zip().run(conn);
    /// ```
    /// 
    /// ```json
    /// [
    /// 	{ "field": "Rushock Bog", "name": "Fred" },
    /// 	{ "field": "Little Delving", "name": "George" },
    /// 	...
    /// ]
    /// ```

    pub fn eq_join<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("eq_join", args)
    }


    /// Used to 'zip' up the result of a join by merging the 'right' fields into 'left' fields of each member of the sequence
///
/// 
///
/// 
                /// 
    /// __Example:__ 'zips up' the sequence by merging the left and right fields produced by a join.
    /// 
    /// ```java
    /// r.table("marvel").eqJoin("main_dc_collaborator", r.table("dc")).zip().run(conn);
    /// ```
    /// 
    /// 

    pub fn zip(&self) -> Client {
        cmd("zip")
    }


    /// Transform each element of one or more sequences by applying a mapping function to them
///
/// 
///
/// If `map` is run with two or more sequences, it will iterate for as many items as there are in the shortest sequence.
                /// 
    /// Note that `map` can only be applied to sequences, not single values. If you wish to apply a function to a single value/selection (including an array), use the [do](/api/java/do) command.
    /// 
    /// __Example:__ Return the first five squares.
    /// 
    /// ```java
    /// r.expr(r.array(1, 2, 3, 4, 5)).map(val -> r.mul(val, val)).run(conn);
    /// 
    /// // Result:
    /// [1, 4, 9, 16, 25]
    /// ```
    /// 
    /// __Example:__ Sum the elements of three sequences.
    /// 
    /// ```java
    /// int[] sequence1 = { 100, 200, 300, 400 };
    /// int[] sequence2 = { 10, 20, 30, 40 };
    /// int[] sequence3 = { 1, 2, 3, 4 };
    /// r.map(sequence1, sequence2, sequence3,
    ///     (val1, val2, val3) -> r.add(val1, val2).add(val3)
    /// ).run(conn);
    /// 
    /// // Result:
    /// [111, 222, 333, 444]
    /// ```
    /// 
    /// __Example:__ Rename a field when retrieving documents using `map` and [merge](/api/java/merge/).
    /// 
    /// This example renames the field `id` to `userId` when retrieving documents from the table `users`.
    /// 
    /// ```java
    /// r.table("users").map(
    ///     doc -> doc.merge(r.hashMap("user_id", doc.g("id"))).without("id")
    /// ).run(conn);
    /// ``` 
    /// 
    /// __Example:__ Assign every superhero an archenemy.
    /// 
    /// ```java
    /// r.table("heroes").map(r.table("villains"),
    ///     (hero, villain) -> hero.merge(r.hashMap("villain", villain))
    /// ).run(conn);
    /// ```

    pub fn map<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("map", args)
    }


    /// Plucks one or more attributes from a sequence of objects, filtering out any objects in the sequence that do not have the specified fields
///
/// 
///
/// Functionally, this is identical to [hasFields](/api/java/has_fields/) followed by [pluck](/api/java/pluck/) on a sequence.
                /// 
    /// __Example:__ Get a list of users and their posts, excluding any users who have not made any posts.
    /// 
    /// Existing table structure:
    /// 
    /// ```json
    /// [
    ///     { "id": 1, "user": "bob", "email": "bob@foo.com", "posts": [ 1, 4, 5 ] },
    ///     { "id": 2, "user": "george", "email": "george@foo.com" },
    ///     { "id": 3, "user": "jane", "email": "jane@foo.com", "posts": [ 2, 3, 6 ] }
    /// ]
    /// ```
    /// 
    /// Command and output:
    /// 
    /// ```java
    /// r.table("users").withFields("id", "user", "posts").run(conn);
    /// 
    /// // Result passed to callback
    /// [
    ///     { "id": 1, "user": "bob", "posts": [ 1, 4, 5 ] },
    ///     { "id": 3, "user": "jane", "posts": [ 2, 3, 6 ] }
    /// ]
    /// ```
    /// 
    /// __Example:__ Use the [nested field syntax](/docs/nested-fields/) to get a list of users with cell phone numbers in their contacts.
    /// 
    /// ```java
    /// r.table("users").withFields("id", "user",
    ///     r.hashMap("contact", r.hashMap("phone", "work"))
    /// ).run(conn);
    /// ```

    pub fn with_fields<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("with_fields", args)
    }


    /// Concatenate one or more elements into a single sequence using a mapping function
///
/// 
///
/// 
                /// 
    /// `concatMap` works in a similar fashion to [map](/api/java/map/), applying the given function to each element in a sequence, but it will always return a single sequence. If the mapping function returns a sequence, `map` would produce a sequence of sequences:
    /// 
    /// ```java
    /// r.expr(r.array(1, 2, 3)).map(x -> r.array(x, x.mul(2))).run(conn);
    /// ```
    /// 
    /// Result:
    /// 
    /// ```json
    /// [[1, 2], [2, 4], [3, 6]]
    /// ```
    /// 
    /// Whereas `concatMap` with the same mapping function would merge those sequences into one:
    /// 
    /// ```java
    /// r.expr(r.array(1, 2, 3)).concatMap(x -> r.array(x, x.mul(2))).run(conn);
    /// ```
    /// 
    /// Result:
    /// 
    /// ```json
    /// [1, 2, 2, 4, 3, 6]
    /// ```
    /// 
    /// The return value, array or stream, will be the same type as the input.
    /// 
    /// __Example:__ Construct a sequence of all monsters defeated by Marvel heroes. The field "defeatedMonsters" is an array of one or more monster names.
    /// 
    /// ```java
    /// r.table("marvel").concatMap(hero -> hero.g("defeatedMonsters")).run(conn);
    /// ```
    /// 
    /// __Example:__ Simulate an [eqJoin](/api/java/eq_join/) using `concatMap`. (This is how ReQL joins are implemented internally.)
    /// 
    /// ```java
    /// r.table("posts").concatMap(
    ///     post -> r.table("comments").getAll(post.g("id")).optArg("index", "post_id")
    ///              .map(comment -> r.hashMap("left", post).with("right", comment))
    /// ).run(conn);
    /// ```

    pub fn concat_map<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("concat_map", args)
    }


    /// Sort the sequence by document values of the given key(s)
///
/// 
///
/// To specify
                /// the ordering, wrap the attribute with either `r.asc` or `r.desc`
    /// (defaults to ascending).
    /// 
    /// __Note:__ RethinkDB uses byte-wise ordering for `orderBy` and does not support Unicode collations; non-ASCII characters will be sorted by UTF-8 codepoint. For more information on RethinkDB's sorting order, read the section in [ReQL data types](/docs/data-types/#sorting-order).
    /// 
    /// Sorting without an index requires the server to hold the sequence in
    /// memory, and is limited to 100,000 documents (or the setting of the `arrayLimit` option for [run](/api/java/run)). Sorting with an index can
    /// be done on arbitrarily large tables, or after a [between](/api/java/between/) command
    /// using the same index. This applies to both secondary indexes and the primary key (e.g., `{"index": "id"}`).
    /// 
    /// Sorting functions passed to `orderBy` must be deterministic. You cannot, for instance, order rows using the [random](/api/java/random/) command. Using a non-deterministic function with `orderBy` will raise a `ReqlQueryLogicError`.
    /// 
    /// __Example:__ Order all the posts using the index `date`.   
    /// 
    /// ```java
    /// r.table("posts").orderBy().optArg("index", "date").run(conn);
    /// ```
    /// 
    /// <!-- stop -->
    /// 
    /// The index must either be the primary key or have been previously created with [indexCreate](/api/java/index_create/).
    /// 
    /// ```java
    /// r.table("posts").indexCreate("date").run(conn);
    /// ```
    /// 
    /// You can also select a descending ordering:
    /// 
    /// ```java
    /// r.table("posts").orderBy().optArg("index", r.desc("date")).run(conn);
    /// ```
    /// 
    /// __Example:__ Order a sequence without an index.
    /// 
    /// ```java
    /// r.table("posts").get(1).g("comments").orderBy("date").run(conn);
    /// ```
    /// 
    /// You can also select a descending ordering:
    /// 
    /// ```java
    /// r.table("posts").get(1).g("comments").orderBy(r.desc("date")).run(conn);
    /// ```
    /// 
    /// If you're doing ad-hoc analysis and know your table won't have more then 100,000
    /// elements (or you've changed the setting of the `array_limit` option for [run](/api/java/run)) you can run `orderBy` without an index:
    /// 
    /// ```java
    /// r.table("small_table").orderBy("date").run(conn);
    /// ```
    /// 
    /// __Example:__ You can efficiently order using multiple fields by using a
    /// [compound index](http://www.rethinkdb.com/docs/secondary-indexes/java/).
    /// 
    /// Order by date and title.
    /// 
    /// ```java
    /// r.table("posts").orderBy().optArg("index", "date_and_title").run(conn);
    /// ```
    /// 
    /// The index must either be the primary key or have been previously created with [indexCreate](/api/java/index_create/).
    /// 
    /// ```java
    /// r.table("posts").indexCreate("date_and_title",
    ///     post -> r.array(post.g("date"), post.g("title"))
    /// ).run(conn);
    /// ```
    /// 
    /// _Note_: You cannot specify multiple orders in a compound index. See [issue #2306](https://github.com/rethinkdb/rethinkdb/issues/2306)
    /// to track progress.
    /// 
    /// __Example:__ If you have a sequence with fewer documents than the `arrayLimit`, you can order it
    /// by multiple fields without an index.
    /// 
    /// ```java
    /// r.table("small_table").orderBy("date", r.desc("title")).run(conn);
    /// ```
    /// 
    /// __Example:__ Notice that an index ordering always has highest
    /// precedence. The following query orders posts by date, and if multiple
    /// posts were published on the same date, they will be ordered by title.
    /// 
    /// ```java
    /// r.table("post").orderBy("title").optArg("index", "date").run(conn);
    /// ```
    /// 
    /// __Example:__ Use [nested field](/docs/cookbook/javascript/#filtering-based-on-nested-fields) syntax to sort on fields from subdocuments. (You can also create indexes on nested fields using this syntax with `indexCreate`.)
    /// 
    /// ```java
    /// r.table("user").orderBy(user -> user.g("group").g("id")).run(conn);
    /// ```
    /// 
    /// __Example:__ You can efficiently order data on arbitrary expressions using indexes.
    /// 
    /// ```java
    /// r.table("posts").orderBy().optArg("index", "votes").run(conn);
    /// ```
    /// 
    /// The index must have been previously created with [indexCreate](/api/java/index_create/).
    /// 
    /// ```java
    /// r.table("posts").indexCreate("votes",
    ///     post -> post.g("upvotes").sub(post.g("downvotes"))
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ If you have a sequence with fewer documents than the `arrayLimit`, you can order it with an arbitrary function directly.
    /// 
    /// ```java
    /// r.table("small_table").orderBy(
    ///     doc -> doc.g("upvotes").sub(doc.g("downvotes"))
    /// ).run(conn);
    /// ```
    /// 
    /// You can also select a descending ordering:
    /// 
    /// ```java
    /// r.table("small_table").orderBy(
    ///     r.desc(doc -> doc.g("upvotes").sub(doc.g("downvotes")))
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Ordering after a `between` command can be done as long as the same index is being used.
    /// 
    /// ```java
    /// r.table("posts")
    ///  .between(r.time(2013, 1, 1, "+00:00"), r.time(2013, 1, 1, "+00:00"))
    ///  .optArg("index", "date")
    ///  .orderBy().optArg("index", "date")
    ///  .run(conn);
    /// ```

    pub fn order_by(&self) -> Client {
        cmd("order_by")
    }


    /// Skip a number of elements from the head of the sequence
///
/// 
///
/// 
                /// 
    /// __Example:__ Here in conjunction with [orderBy](/api/java/order_by/) we choose to ignore the most successful heroes.
    /// 
    /// ```java
    /// r.table("marvel").orderBy("successMetric").skip(10).run(conn);
    /// ```

    pub fn skip<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("skip", args)
    }


    /// End the sequence after the given number of elements
///
/// 
///
/// 
                /// 
    /// __Example:__ Only so many can fit in our Pantheon of heroes.
    /// 
    /// ```java
    /// r.table("marvel").orderBy("belovedness").limit(10).run(conn);
    /// ```
    /// 
    /// 

    pub fn limit<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("limit", args)
    }


    /// Return the elements of a sequence within the specified range
///
/// 
///
/// 
                /// 
    /// `slice` returns the range between `startOffset` and `endOffset`. If only `startOffset` is specified, `slice` returns the range from that index to the end of the sequence. Use the [optArgs](/api/java/optarg) `left_bound` or `right_bound` as `open` or `closed` to indicate whether to include that endpoint of the range by default: `closed` returns that endpoint, while `open` does not. By default, `left_bound` is closed and `right_bound` is open, so the range `(10,13)` will return the tenth, eleventh and twelfth elements in the sequence.
    /// 
    /// If `endOffset` is past the end of the sequence, all elements from `startOffset` to the end of the sequence will be returned. If `startOffset` is past the end of the sequence or `endOffset` is less than `startOffset`, a zero-element sequence will be returned.
    /// 
    /// Negative `startOffset` and `endOffset` values are allowed with arrays; in that case, the returned range counts back from the array's end. That is, the range `(-2)` returns the last two elements, and the range of `(2,-1)` returns the second element through the next-to-last element of the range. An error will be raised on a negative `startOffset` or `endOffset` with non-arrays. (An `endOffset` of &minus;1 *is* allowed with a stream if `rightBound` is closed; this behaves as if no `endOffset` was specified.)
    /// 
    /// If `slice` is used with a [binary](/api/java/binary) object, the indexes refer to byte positions within the object. That is, the range `(10,20)` will refer to the 10th byte through the 19th byte.
    /// 
    /// With a string, `slice` behaves similarly, with the indexes referring to Unicode codepoints. String indexes start at `0`. (Note that [combining codepoints][cc] are counted separately.)
    /// 
    /// [cc]: https://en.wikipedia.org/wiki/Combining_character
    /// 
    /// __Example:__ Return the fourth, fifth and sixth youngest players. (The youngest player is at index 0, so those are elements 3&ndash;5.)
    /// 
    /// ```java
    /// r.table("players").orderBy().optArg("index", "age").slice(3, 6).run(conn);
    /// ```
    /// 
    /// __Example:__ Return all but the top three players who have a red flag.
    /// 
    /// ```java
    /// r.table("players").filter(r.hashMap("flag", "red")).orderBy()
    ///  .optArg("index", r.desc("score")).slice(3).run(conn);
    /// ```
    /// 
    /// __Example:__ Return holders of tickets `X` through `Y`, assuming tickets are numbered sequentially. We want to include ticket `Y`.
    /// 
    /// ```java
    /// r.table("users").orderBy().optArg("index", "ticket")
    ///  .slice(x, y).optArg("right_bound", "closed").run(conn);
    /// ```
    /// 
    /// __Example:__ Return the elements of an array from the second through two from the end (that is, not including the last two).
    /// 
    /// ```java
    /// r.expr(r.array(0, 1, 2, 3, 4, 5)).slice(2, -2).run(conn);
    /// ```
    /// 
    /// Result:
    /// 
    /// ```json
    /// [2,3]
    /// ```
    /// 
    /// __Example:__ Return the third through fifth characters of a string.
    /// 
    /// ```java
    /// r.expr("rutabaga").slice(2,5).run(conn);
    /// ```
    /// 
    /// Result:
    /// 
    /// ```json
    /// "tab"
    /// ```

    pub fn slice<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("slice", args)
    }


    /// Get the *nth* element of a sequence, counting from zero
///
/// 
///
/// If the argument is negative, count from the last element.
                /// 
    /// __Example:__ Select the second element in the array.
    /// 
    /// ```java
    /// r.expr(r.array(1,2,3)).nth(1).run(conn);
    /// ```
    /// 
    /// __Example:__ Select the bronze medalist from the competitors.
    /// 
    /// ```java
    /// r.table("players").orderBy().optArg("index", r.desc("score")).nth(3).run(conn);
    /// ```
    /// 
    /// __Example:__ Select the last place competitor.
    /// 
    /// ```java
    /// r.table("players").orderBy().optArg("index", r.desc("score")).nth(-1).run(conn);
    /// ```

    pub fn nth<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("nth", args)
    }


    /// Get the indexes of an element in a sequence
///
/// 
///
/// If the argument is a predicate, get the indexes of all elements matching it.
                /// 
    /// __Example:__ Find the position of the letter 'c'.
    /// 
    /// ```java
    /// r.expr(r.array("a", "b", "c")).offsetsOf("c").run(conn);
    /// ```
    /// 
    /// __Example:__ Find the popularity ranking of invisible heroes.
    /// 
    /// ```java
    /// r.table("marvel").union(r.table("dc")).orderBy("popularity").offsetsOf(
    ///     row -> row.g("superpowers").contains("invisibility")
    /// ).run(conn);
    /// ```

    pub fn offsets_of<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("offsets_of", args)
    }


    /// Test if a sequence is empty
///
/// 
///
/// 
                /// 
    /// __Example:__ Are there any documents in the marvel table?
    /// 
    /// ```java
    /// r.table("marvel").isEmpty().run(conn);
    /// ```

    pub fn is_empty(&self) -> Client {
        cmd("is_empty")
    }


    /// Merge two or more sequences
///
/// 
///
/// 
                /// 
    /// The `interleave` [optArg](/api/java/optarg) controls how the sequences will be merged:
    /// 
    /// * `true`: results will be mixed together; this is the fastest setting, but ordering of elements is not guaranteed. (This is the default.)
    /// * `false`: input sequences will be appended to one another, left to right.
    /// * `"field_name"`: a string will be taken as the name of a field to perform a merge-sort on. The input sequences must be ordered _before_ being passed to `union`.
    /// * function: the `interleave` optArg can take a function whose argument is the current row, and whose return value is a string to take as a field name, as with the `"field_name"` setting described above.
    /// 
    /// __Example:__ Construct a stream of all heroes.
    /// 
    /// ```java
    /// r.table("marvel").union(r.table("dc")).run(conn);
    /// ```
    /// 
    /// __Example:__ Combine four arrays into one.
    /// 
    /// ```java
    /// r.expr(r.array(1, 2)).union(r.array(3, 4), r.array(5, 6), r.array(7, 8, 9)).run(conn);
    /// 
    /// // Result:
    /// [1, 2, 3, 4, 5, 6, 7, 8, 9]
    /// ```
    /// 
    /// __Example:__ Create a [changefeed][cf] from the first example.
    /// 
    /// ```java
    /// r.table("marvel").union(r.table("dc")).changes().run(conn);
    /// ```
    /// 
    /// Now, when any heroes are added, modified or deleted from either table, a change notification will be sent out.
    /// 
    /// [cf]: /docs/changefeeds/java
    /// 
    /// __Example:__ Merge-sort the tables of heroes, ordered by name.
    /// 
    /// ```java
    /// r.table("marvel").orderBy("name")
    ///  .union(r.table("dc").orderBy("name")).optArg("interleave", "name")
    ///  .run(conn);
    /// ```

    pub fn union<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("union", args)
    }


    /// Select a given number of elements from a sequence with uniform random distribution
///
/// 
///
/// Selection is done without replacement.
                /// 
    /// If the sequence has less than the requested number of elements (i.e., calling `sample(10)` on a sequence with only five elements), `sample` will return the entire sequence in a random order.
    /// 
    /// __Example:__ Select 3 random heroes.
    /// 
    /// ```java
    /// r.table("marvel").sample(3).run(conn);
    /// ```

    pub fn sample<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("sample", args)
    }


    /// 
///
/// 
///
/// 
                /// Takes a stream and partitions it into multiple groups based on the
    /// fields or functions provided.
    /// 
    /// Two options are available via [optArg](/api/java/optarg): `index` can be the name of an index to group on (in place of a field. The `multi` flag, a boolean (default `false`), allows single documents to be assigned to multiple groups, similar to the behavior of [multi-indexes](/docs/secondary-indexes/). When `multi` is `true` and the grouping value is an array, documents will be placed in each group that corresponds to the elements of the array. If the array is empty the row will be ignored.
    /// 
    /// The data returned by `group` will be a `List<GroupedResult>`:
    /// 
    /// ```java
    /// public class GroupedResult<G,V> {
    ///     public final G group;
    ///     public final List<V> values;
    /// 
    ///     public GroupedResult(G group, List<V> values){
    ///         this.group = group;
    ///         this.values = values;
    ///     }
    /// }
    /// ```
    /// 
    /// Suppose that the table `games` has the following data:
    /// 
    /// ```json
    /// [
    ///     {"id": 2, "player": "Bob", "points": 15, "type": "ranked"},
    ///     {"id": 5, "player": "Alice", "points": 7, "type": "free"},
    ///     {"id": 11, "player": "Bob", "points": 10, "type": "free"},
    ///     {"id": 12, "player": "Alice", "points": 2, "type": "free"}
    /// ]
    /// ```
    /// 
    /// __Example:__ Group games by player.
    /// 
    /// ```java
    /// r.table("games").group("player").run(conn);
    /// ```
    /// 
    /// To show the returned data, we'll use JSON representation again, with `group` and `values` as the fields corresponding to the elements in each `GroupedResult`.
    /// 
    /// ```json
    /// [
    ///     {
    ///         "group": "Alice",
    ///         "values": [
    ///             {"id": 5, "player": "Alice", "points": 7, "type": "free"},
    ///             {"id": 12, "player": "Alice", "points": 2, "type": "free"}
    ///         ]
    ///     },
    ///     {
    ///         "group": "Bob",
    ///         "values": [
    ///             {"id": 2, "player": "Bob", "points": 15, "type": "ranked"},
    ///             {"id": 11, "player": "Bob", "points": 10, "type": "free"}
    ///         ]
    ///     }
    /// ]
    /// ```
    /// 
    /// <!-- stop -->
    /// 
    /// Commands chained after `group` will be called on each of these grouped
    /// sub-streams, producing grouped data.
    /// 
    /// __Example:__ What is each player's best game?
    /// 
    /// ```java
    /// r.table("games").group("player").max("points").run(conn);
    /// ```
    /// 
    /// ```json
    /// [
    ///     {
    ///         "group": "Alice",
    ///         "values": {"id": 5, "player": "Alice", "points": 7, "type": "free"}
    ///     },
    ///     {
    ///         "group": "Bob",
    ///         "values": {"id": 2, "player": "Bob", "points": 15, "type": "ranked"}
    ///     }
    /// ]
    /// ```
    /// 
    /// Commands chained onto grouped data will operate on each grouped datum,
    /// producing more grouped data.
    /// 
    /// __Example:__ What is the maximum number of points scored by each player?
    /// 
    /// ```java
    /// r.table("games").group("player").max("points").g("points").run(conn);
    /// ```
    /// 
    /// ```json
    /// [
    ///     {
    ///         "group": "Alice",
    ///         "values": 7
    ///     },
    ///     {
    ///         "group": "Bob",
    ///         "values": 15
    ///     }
    /// ]
    /// ```
    /// 
    /// You can also group by more than one field.
    /// 
    /// __Example:__ What is the maximum number of points scored by each
    /// player for each game type?
    /// 
    /// ```java
    /// r.table("games").group("player", "type").max("points").g("points").run(conn);
    /// ```
    /// 
    /// ```json
    /// [
    ///     {
    ///         "group": ["Alice", "free"],
    ///         "values": 7
    ///     }
    ///     {
    ///         "group": ["Bob", "free"],
    ///         "values": 10,
    ///     },
    ///     {
    ///         "group": ["Bob", "ranked"],
    ///         "values": 15,
    ///     }
    /// ]
    /// ```
    /// 
    /// You can also group by a function.
    /// 
    /// __Example:__ What is the maximum number of points scored by each
    /// player for each game type?
    /// 
    /// 
    /// ```java
    /// r.table("games").group(
    ///     game -> game.pluck("player", "type")
    /// ).max("points").g("points").run(conn);
    /// ```
    /// 
    /// ```json
    /// [
    ///     {
    ///         "group": {"player": "Alice", "type": "free"},
    ///         "values": 7
    ///     },
    ///     {
    ///         "group": {"player": "Bob", "type": "free"},
    ///         "values": 10
    ///     },
    ///     {
    ///         "group": {"player": "Bob", "type": "ranked"},
    ///         "values": 15
    ///     }
    /// ]
    /// ```
    /// 
    /// Using a function, you can also group by date on a ReQL [date field](/docs/dates-and-times/javascript/).
    /// 
    /// __Example:__ How many matches have been played this year by month?
    /// 
    /// ```java
    /// r.table("matches").group(
    ///     match -> r.array(match.g("date").year(), match.g("date").month())
    /// ).count().run(conn);
    /// ```
    /// 
    /// ```json
    /// [
    ///     {
    ///         "group": [2014, 2],
    ///         "values": 2
    ///     },
    ///     {
    ///         "group": [2014, 3],
    ///         "values": 2
    ///     },
    ///     {
    ///         "group": [2014, 4],
    ///         "values": 1
    ///     },
    ///     {
    ///         "group": [2014, 5],
    ///         "values": 3
    ///     }
    /// ]
    /// ```
    /// 
    /// You can also group on an index (primary key or secondary).
    /// 
    /// __Example:__ What is the maximum number of points scored by game type?
    /// 
    /// 
    /// ```java
    /// r.table("games").group().optArg("index", "type")
    ///  .max("points").g("points").run(conn);
    /// ```
    /// 
    /// ```json
    /// [
    ///     {
    ///         "group": "free",
    ///         "values": 10
    ///     },
    ///     {
    ///         "group": "ranked",
    ///         "values": 15
    ///     }
    /// ]
    /// ```
    /// 
    /// # Organizing by value with **multi** #
    /// 
    /// Suppose that the table `games2` has the following data:
    /// 
    /// ```json
    /// [
    ///     { "id": 1, "matches": {"a": [1, 2, 3], "b": [4, 5, 6]} },
    ///     { "id": 2, "matches": {"b": [100], "c": [7, 8, 9]} },
    ///     { "id": 3, "matches": {"a": [10, 20], "c": [70, 80]} }
    /// ]
    /// ```
    /// 
    /// Using the `multi` option we can group data by match A, B or C.
    /// 
    /// ```java
    /// r.table("games2").group(
    ///     row -> row.g("matches").keys()
    /// ).optArg("multi", true).run(conn);
    /// ```
    /// 
    /// ```json
    /// [
    ///     {
    ///         "group": "a",
    ///         "values": [ <id 1>, <id 3> ]
    ///     },
    ///     {
    ///         "group": "b",
    ///         "values": [ <id 1>, <id 2> ]
    ///     },
    ///     {
    ///         "group": "c",
    ///         "values": [ <id 2>, <id 3> ]
    ///     }
    /// ]
    /// ```
    /// 
    /// (The full result set is abbreviated in the figure; `<id 1>, <id 2>` and `<id 3>` would be the entire documents matching those keys.)
    /// 
    /// __Example:__ Use [map](/api/java/map) and [sum](/api/java/sum) to get the total points scored for each match.
    /// 
    /// ```java
    /// r.table("games2").group(
    ///     row -> row.g("matches").keys()
    /// ).optArg("multi", true).ungroup().map(
    ///     doc -> r.hashMap("match", doc.g("group")).with(
    ///         "total", doc.g("reduction").sum(
    ///             set -> set.g("matches").bracket(doc.g("group")).sum()
    ///         )
    ///     )
    /// ).run(conn);
    /// ```
    /// 
    /// ```json
    /// [
    ///     { "match": "a", "total": 36 },
    ///     { "match": "b", "total": 115 },
    ///     { "match": "c", "total": 174 }
    /// ]
    /// ```
    /// 
    /// The inner `sum` adds the scores by match within each document; the outer `sum` adds those results together for a total across all the documents.
    /// 
    /// # Ungrouping #
    /// 
    /// If you want to operate on all the groups rather than operating on each
    /// group (e.g. if you want to order the groups by their reduction), you
    /// can use [ungroup](/api/java/ungroup/) to turn a grouped stream or
    /// grouped data into an array of objects representing the groups.
    /// 
    /// The format of the array returned by `ungroup` is the same as the
    /// default native format of grouped data in the JavaScript driver and
    /// Data Explorer.
    /// 
    /// __Example:__ Ungrouping grouped data.
    /// 
    /// ```java
    /// r.table('games').group('player').max('points')['points'].ungroup().run(conn);
    /// ```
    /// 
    /// ```json
    /// [
    ///     {
    ///         "group": "Alice",
    ///         "reduction": 7
    ///     },
    ///     {
    ///         "group": "Bob",
    ///         "reduction": 15
    ///     }
    /// ]
    /// ```
    /// 
    /// Ungrouping is useful e.g. for ordering grouped data, or for inserting
    /// grouped data into a table.
    /// 
    /// __Example:__ What is the maximum number of points scored by each
    /// player, with the highest scorers first?
    /// 
    /// ```java
    /// r.table("games").group("player").max("points").g("points").ungroup()
    ///  .orderBy(r.desc("reduction")).run(conn);
    /// ```
    /// 
    /// ```json
    /// [
    ///     {
    ///         "group": "Bob",
    ///         "reduction": 15
    ///     },
    ///     {
    ///         "group": "Alice",
    ///         "reduction": 7
    ///     }
    /// ]
    /// ```
    /// 
    /// 
    /// # Implementation Details #
    /// 
    /// When grouped data are returned to the client, they are transformed
    /// into a client-specific native type.  (Something similar is done with
    /// [times](/docs/dates-and-times/).)  In Java, grouped data are
    /// transformed into an `List`.  If you instead want to receive the raw
    /// pseudotype from the server, you can specify `group_format: "raw"` as an optional
    /// argument to `run`:
    /// 
    /// __Example:__ Get back the raw `GROUPED_DATA` pseudotype.
    /// 
    /// ```java
    /// r.table("games").group("player").avg("points").run(conn).optArg("group_format", "raw");
    /// ```
    /// 
    /// ```json
    /// {
    ///     "$reql_type$": "GROUPED_DATA",
    ///     "data": [
    ///         ["Alice", 4.5],
    ///         ["Bob", 12.5]
    ///     ]
    /// }
    /// ```
    /// 
    /// You might also want to use the [ungroup](/api/java/ungroup/)
    /// command (see above), which will turn the grouped data into an array of
    /// objects on the server.
    /// 
    /// 
    /// # Performance Details #
    /// 
    /// If you run a query that returns a grouped stream, it will be
    /// automatically converted to grouped data before being sent back to you
    /// (there is currently no efficient way to stream groups from RethinkDB).
    /// This grouped data is subject to the array size limit, by default 100,000 elements (see [run](/api/java/run) for details on how to use the `array_limit` argument to change this).
    /// 
    /// In general, operations on grouped streams will be efficiently
    /// distributed, and operations on grouped data won't be.  You can figure
    /// out what you're working with by putting `typeOf` on the end of your
    /// query.  Below are efficient and inefficient examples.
    /// 
    /// __Example:__ Efficient operation.
    /// 
    /// ```java
    /// // r.table("games").group("player").typeOf().run(conn);
    /// // Returns "GROUPED_STREAM"
    /// r.table("games").group("player").min("points").run(conn); // EFFICIENT
    /// ```
    /// 
    /// __Example:__ Inefficient operation.
    /// 
    /// ```java
    /// // r.table("games").group("player").orderBy("score").typeOf().run(conn);
    /// // Returns "GROUPED_DATA"
    /// r.table("games").group("player").orderBy("score").nth(0).run(conn); // INEFFICIENT
    /// ```
    /// 
    /// What does it mean to be inefficient here?  When operating on grouped
    /// data rather than a grouped stream, *all* of the data has to be
    /// available on the node processing the query.  This means that the
    /// operation will only use one server's resources, and will require
    /// memory proportional to the size of the grouped data it's operating
    /// on.  (In the case of the [orderBy](/api/java/order_by/) in the inefficient example, that
    /// means memory proportional **to the size of the table**.)  The array
    /// limit is also enforced for grouped data, so the `orderBy` example
    /// would fail for tables with more than 100,000 rows without changing the `arrayLimit` option to `run`.
    /// 
    /// # More Examples #
    /// 
    /// __Example:__ What is the maximum number of points scored by each
    /// player in free games?
    /// 
    /// ```java
    /// r.table("games").filter(
    ///     game -> game.g("type").eq("free")
    /// ).group("player").max("points").g("points").run(conn);
    /// ```
    /// 
    /// ```json
    /// [
    ///     {
    ///         "group": "Alice",
    ///         "values": 7
    ///     },
    ///     {
    ///         "group": "Bob",
    ///         "values": 10
    ///     }
    /// ]
    /// ```
    /// 
    /// __Example:__ What is each player's highest even and odd score?
    /// 
    /// ```java
    /// r.table("games").group(
    ///     "name", game -> game.g("points").mod(2)
    /// ).max("points").g("points").run(conn);
    /// ```
    /// 
    /// ```json
    /// [
    ///     {
    ///         "group": ["Alice", 1],
    ///         "values": 7
    ///     },
    ///     {
    ///         "group": ["Bob", 0],
    ///         "values": 10
    ///     },
    ///     {
    ///         "group": ["Bob", 1],
    ///         "values": 15
    ///     }
    /// ]
    /// ```

    pub fn group(&self) -> Client {
        cmd("group")
    }


    /// 
///
/// 
///
/// 
                /// Takes a grouped stream or grouped data and turns it into an array of
    /// objects representing the groups.  Any commands chained after `ungroup`
    /// will operate on this array, rather than operating on each group
    /// individually.  This is useful if you want to e.g. order the groups by
    /// the value of their reduction.
    /// 
    /// The format of the array returned by `ungroup` is the same as the
    /// default native format of grouped data in the JavaScript driver and
    /// Data Explorer.
    /// 
    /// Suppose that the table `games` has the following data:
    /// 
    /// ```json
    /// [
    ///     {"id": 2, "player": "Bob", "points": 15, "type": "ranked"},
    ///     {"id": 5, "player": "Alice", "points": 7, "type": "free"},
    ///     {"id": 11, "player": "Bob", "points": 10, "type": "free"},
    ///     {"id": 12, "player": "Alice", "points": 2, "type": "free"}
    /// ]
    /// ```
    /// 
    /// __Example:__ What is the maximum number of points scored by each
    /// player, with the highest scorers first?
    /// 
    /// ```java
    /// r.table("games").group("player").max("points").g("points").ungroup()
    ///  .orderBy(r.desc("reduction")).run(conn);
    /// ```
    /// 
    /// <!-- stop -->
    /// 
    /// The result:
    /// 
    /// ```json
    /// [
    ///     {
    ///         "group": "Bob",
    ///         "reduction": 15
    ///     },
    ///     {
    ///         "group": "Alice",
    ///         "reduction": 7
    ///     }
    /// ]
    /// ```
    /// 
    /// __Example:__ Select one random player and all their games.
    /// 
    /// ```java
    /// r.table("games").group("player").ungroup().sample(1).run(conn);
    /// ```
    /// 
    /// Result:
    /// 
    /// ```json
    /// [
    ///     {
    ///         "group": "Bob",
    ///         "reduction": 15
    ///     },
    ///     {
    ///         "group": "Alice",
    ///         "reduction": 7
    ///     }
    /// ]
    /// ```
    /// 
    /// Note that if you didn't call `ungroup`, you would instead select one
    /// random game from each player:
    /// 
    /// ```java
    /// r.table("games").group("player").sample(1).run(conn);
    /// ```
    /// 
    /// Result: (Note this is a JSON representation of a `List<GroupedResult>`; see the [group](/api/java/group) documentation for more details.)
    /// 
    /// ```json
    /// [
    ///     {
    ///         "group": "Alice",
    ///         "values": [
    ///             {"id": 5, "player": "Alice", "points": 7, "type": "free"}
    ///         ]
    ///     },
    ///     {
    ///         "group": "Bob",
    ///         "values": [
    ///             {"id": 11, "player": "Bob", "points": 10, "type": "free"}
    ///         ]
    ///     }
    /// [
    /// ```
    /// 
    /// __Example:__ Finding the arithmetic mode of an array of values:
    /// 
    /// ```java
    /// r.expr(r.array([1,2,2,2,3,3])).group(
    ///   row -> row
    /// ).count().ungroup().orderBy("reduction").nth(-1).bracket("group")
    /// ```
    /// 
    /// Result:
    /// 
    /// ```json
    /// 2
    /// ```
    /// 
    /// 
    /// __Example:__ Types!
    /// 
    /// ```java
    /// r.table('games').group('player').typeOf().run(conn); // Returns "GROUPED_STREAM"
    /// r.table('games').group('player').ungroup().typeOf().run(conn); // Returns "ARRAY"
    /// r.table('games').group('player').avg('points').run(conn); // Returns "GROUPED_DATA"
    /// r.table('games').group('player').avg('points').ungroup().run(conn); // Returns "ARRAY"
    /// ```

    pub fn ungroup(&self) -> Client {
        cmd("ungroup")
    }


    /// Produce a single value from a sequence through repeated application of a reduction function
///
/// 
///
/// 
                /// 
    /// The reduction function may be called on:
    /// 
    /// - two elements of the sequence
    /// - one element of the sequence and one result of a previous reduction
    /// - two results of previous reductions
    /// 
    /// The reduction function can be called on the results of two previous reductions because the `reduce` command is distributed and parallelized across shards and CPU cores. A common mistaken when using the `reduce` command is to suppose that the reduction is executed from left to right. Read the [map-reduce in RethinkDB](/docs/map-reduce/) article to see an example.
    /// 
    /// If the sequence is empty, the server will produce a `ReqlRuntimeError` that can be caught with `default_`. If the sequence has only one element, the first element will be returned.
    /// 
    /// __Example:__ Return the number of documents in the table `posts`.
    /// 
    /// ```java
    /// r.table("posts").map(doc -> 1).reduce(
    ///     (left, right) -> left.add(right)
    /// ).default_(0).run(conn);
    /// ```
    /// 
    /// 
    /// A shorter way to execute this query is to use [count](/api/java/count).
    /// 
    /// 
    /// __Example:__ Suppose that each `post` has a field `comments` that is an array of
    /// comments.  
    /// Return the number of comments for all posts.
    /// 
    /// ```java
    /// r.table("posts").map(doc -> doc.g("comments").count()).reduce(
    ///     (left, right) -> left.add(right)
    /// ).default_(0).run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ Suppose that each `post` has a field `comments` that is an array of
    /// comments.  
    /// Return the maximum number comments per post.
    /// 
    /// ```java
    /// r.table("posts").map(doc -> doc.g("comments").count()).reduce(
    ///     (left, right) -> r.branch(left.gt(right), left, right)
    /// ).default_(0).run(conn);
    /// ```
    /// 
    /// A shorter way to execute this query is to use [max](/api/java/max).

    pub fn reduce<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("reduce", args)
    }


    /// Apply a function to a sequence in order, maintaining state via an accumulator
///
/// 
///
/// The `fold` command returns either a single value or a new sequence.
                /// 
    /// In its first form, `fold` operates like [reduce][rd], returning a value by applying a combining function to each element in a sequence. The combining function takes two parameters: the previous reduction result (the accumulator) and the current element. However, `fold` has the following differences from `reduce`:
    /// 
    /// * it is guaranteed to proceed through the sequence from first element to last.
    /// * it passes an initial base value to the function with the first element in place of the previous reduction result.
    /// 
    /// {% apibody %}
    /// combiningFunction(accumulator | base, element) &rarr; newAccumulator
    /// {% endapibody %}
    /// 
    /// In its second form, `fold` operates like [concat_map][cm], returning a new sequence rather than a single value. When an `emit` function is provided, `fold` will:
    /// 
    /// * proceed through the sequence in order and take an initial base value, as above.
    /// * for each element in the sequence, call both the combining function and a separate emitting function. The emitting function takes three parameters: the previous reduction result (the accumulator), the current element, and the output from the combining function (the new value of the accumulator).
    /// 
    /// If provided, the emitting function must return a list.
    /// 
    /// {% apibody %}
    /// emit(previousAccumulator, element, accumulator) &rarr; array
    /// {% endapibody %}
    /// 
    /// A `finalEmit` function may also be provided, which will be called at the end of the sequence. It takes a single parameter: the result of the last reduction through the iteration (the accumulator), or the original base value if the input sequence was empty. This function must return a list, which will be appended to `fold`'s output stream.
    /// 
    /// {% apibody %}
    /// finalEmit(accumulator | base) &rarr; array
    /// {% endapibody %}
    /// 
    /// [rd]: /api/java/reduce/
    /// [cm]: /api/java/concat_map/
    /// 
    /// __Example:__ Concatenate words from a list.
    /// 
    /// ```java
    /// r.table("words").orderBy("id").fold("",
    ///     (acc, word) -> acc.add(r.branch(r.eq(acc, ""), "", ", ")).add(word)
    /// ).run(conn);
    /// ```
    /// 
    /// (This example could be implemented with `reduce`, but `fold` will preserve the order when `words` is a RethinkDB table or other stream, which is not guaranteed with `reduce`.)
    /// 
    /// __Example:__ Return every other row in a table.
    /// 
    /// ```java
    /// r.table("even_things").fold(0,
    ///     (acc, row) -> r.add(acc, 1)
    /// ).optArg("emit",
    ///     (acc, row, new_acc) -> r.branch(r.mod(new_acc, 2).eq(0),
    ///                                     r.array(row), r.array())
    /// ).run(conn);
    /// ```
    /// 
    /// The first function increments the accumulator each time it's called, starting at `0`; the second function, the emitting function, alternates between returning a single-item list containing the current row or an empty list. The `fold` command will return a concatenated list of each emitted value.
    /// 
    /// __Example:__ Compute a five-day running average for a weight tracker.
    /// 
    /// ```java
    /// r.table("tracker").filter(
    ///     r.hashMap("name", "bob")
    /// ).orderBy("date").g("weight").fold(r.array(),
    ///     (acc, row) -> r.add(r.array(row), acc).limit(5)
    /// ).optArg("emit",
    ///     (acc, row, new_acc) -> r.branch(new_acc.size().eq(5),
    ///                                     r.array(new_acc.avg()),
    ///                                     r.array())
    /// ).run(conn);
    /// ```

    pub fn fold<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("fold", args)
    }


    /// Counts the number of elements in a sequence or key/value pairs in an object, or returns the size of a string or binary object
///
/// 
///
/// 
                /// 
    /// When `count` is called on a sequence with a predicate value or function, it returns the number of elements in the sequence equal to that value or where the function returns `true`. On a [binary](/api/java/binary) object, `count` returns the size of the object in bytes; on strings, `count` returns the string's length. This is determined by counting the number of Unicode codepoints in the string, counting combining codepoints separately.
    /// 
    /// __Example:__ Count the number of users.
    /// 
    /// ```java
    /// r.table("users").count().run(conn);
    /// ```
    /// 
    /// __Example:__ Count the number of 18 year old users.
    /// 
    /// ```java
    /// r.table("users").g("age").count(18).run(conn);
    /// ```
    /// 
    /// __Example:__ Count the number of users over 18.
    /// 
    /// ```java
    /// r.table("users").g("age").count(age -> age.gt(18)).run(conn);
    /// ```
    /// 
    /// Alternatively:
    /// 
    /// ```java
    /// r.table("users").count(user -> user.g("age").gt(18)).run(conn);
    /// ```
    /// 
    /// __Example:__ Return the length of a Unicode string.
    /// 
    /// ```java
    /// r.expr("こんにちは").count().run(conn);
    /// // returns: 5
    /// ```

    pub fn count(&self) -> Client {
        cmd("count")
    }


    /// Sums all the elements of a sequence
///
/// 
///
/// If called with a field name,
                /// sums all the values of that field in the sequence, skipping elements
    /// of the sequence that lack that field.  If called with a function,
    /// calls that function on every element of the sequence and sums the
    /// results, skipping elements of the sequence where that function returns
    /// `null` or a non-existence error.
    /// 
    /// Returns `0` when called on an empty sequence.
    /// 
    /// __Example:__ What's 3 + 5 + 7?
    /// 
    /// ```java
    /// r.expr(r.array(3, 5, 7)).sum().run(conn);
    /// ```
    /// 
    /// __Example:__ How many points have been scored across all games?
    /// 
    /// ```java
    /// r.table("games").sum("points").run(conn);
    /// ```
    /// 
    /// __Example:__ How many points have been scored across all games,
    /// counting bonus points?
    /// 
    /// ```java
    /// r.table("games").sum(
    ///     game -> game.g("points").add(game.g("bonus_points"))
    /// ).run(conn);
    /// ```

    pub fn sum(&self) -> Client {
        cmd("sum")
    }


    /// Averages all the elements of a sequence
///
/// 
///
/// If called with a field name,
                /// averages all the values of that field in the sequence, skipping
    /// elements of the sequence that lack that field.  If called with a
    /// function, calls that function on every element of the sequence and
    /// averages the results, skipping elements of the sequence where that
    /// function returns `null` or a non-existence error.
    /// 
    /// Produces a non-existence error when called on an empty sequence.  You
    /// can handle this case with `default`.
    /// 
    /// __Example:__ What's the average of 3, 5, and 7?
    /// 
    /// ```java
    /// r.expr(r.array(3, 5, 7)).avg().run(conn);
    /// ```
    /// 
    /// __Example:__ What's the average number of points scored in a game?
    /// 
    /// ```java
    /// r.table("games").avg("points").run(conn);
    /// ```
    /// 
    /// __Example:__ What's the average number of points scored in a game,
    /// counting bonus points?
    /// 
    /// ```java
    /// r.table("games").avg(
    ///     game -> game.g("points").add(game.g("bonus_points"))
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ What's the average number of points scored in a game?
    /// (But return `null` instead of raising an error if there are no games where
    /// points have been scored.)
    /// 
    /// ```java
    /// r.table("games").avg("points").default_(null).run(conn);
    /// ```

    pub fn avg(&self) -> Client {
        cmd("avg")
    }


    /// Finds the minimum element of a sequence
///
/// 
///
/// 
                /// 
    /// The `min` command can be called with:
    /// 
    /// * a **field name**, to return the element of the sequence with the smallest value in that field;
    /// * a **function**, to apply the function to every element within the sequence and return the element which returns the smallest value from the function, ignoring any elements where the function produces a non-existence error;
    /// * an **index** (the primary key or a secondary index) via [optArg](/api/java/optarg), to return the element of the sequence with the smallest value in that index.
    /// 
    /// For more information on RethinkDB's sorting order, read the section in [ReQL data types](/docs/data-types/#sorting-order).
    /// 
    /// Calling `min` on an empty sequence will throw a non-existence error; this can be handled using the [default_](/api/java/default/) command.
    /// 
    /// __Example:__ Return the minimum value in the list `[3, 5, 7]`.
    /// 
    /// ```java
    /// r.expr(r.array(3, 5, 7)).min().run(conn);
    /// ```
    /// 
    /// __Example:__ Return the user who has scored the fewest points.
    /// 
    /// ```java
    /// r.table("users").min("points").run(conn);
    /// ```
    /// 
    /// __Example:__ The same as above, but using a secondary index on the `points` field.
    /// 
    /// ```java
    /// r.table("users").min().optArg("index", "points").run(conn);
    /// ```
    /// 
    /// __Example:__ Return the user who has scored the fewest points, adding in bonus points from a separate field using a function.
    /// 
    /// ```java
    /// r.table("users").min(
    ///     user -> user.g("points").add(user.g("bonus_points"))
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Return the smallest number of points any user has ever scored. This returns the value of that `points` field, not a document.
    /// 
    /// ```java
    /// r.table("users").min("points").g("points").run(conn);
    /// ```
    /// 
    /// __Example:__ Return the user who has scored the fewest points, but add a default `None` return value to prevent an error if no user has ever scored points.
    /// 
    /// ```java
    /// r.table("users").min("points").default_(null).run(conn);
    /// ```

    pub fn min(&self) -> Client {
        cmd("min")
    }


    /// Finds the maximum element of a sequence
///
/// 
///
/// 
                /// 
    /// The `max` command can be called with:
    /// 
    /// * a **field name**, to return the element of the sequence with the largest value in that field;
    /// * a **function**, to apply the function to every element within the sequence and return the element which returns the largest value from the function, ignoring any elements where the function produces a non-existence error;
    /// * an **index** (the primary key or a secondary index) via [optArg](/api/java/optarg), to return the element of the sequence with the largest value in that index.
    /// 
    /// For more information on RethinkDB's sorting order, read the section in [ReQL data types](/docs/data-types/#sorting-order).
    /// 
    /// Calling `max` on an empty sequence will throw a non-existence error; this can be handled using the [default_](/api/java/default/) command.
    /// 
    /// __Example:__ Return the maximum value in the list `[3, 5, 7]`.
    /// 
    /// ```java
    /// r.expr(r.array(3, 5, 7)).max().run(conn);
    /// ```
    /// 
    /// __Example:__ Return the user who has scored the most points.
    /// 
    /// ```java
    /// r.table("users").max("points").run(conn);
    /// ```
    /// 
    /// __Example:__ The same as above, but using a secondary index on the `points` field.
    /// 
    /// ```java
    /// r.table("users").max().optArg("index", "points").run(conn);
    /// ```
    /// 
    /// __Example:__ Return the user who has scored the most points, adding in bonus points from a separate field using a function.
    /// 
    /// ```java
    /// r.table("users").max(
    ///     user -> user.g("points").add(user.g("bonus_points"))
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Return the highest number of points any user has ever scored. This returns the value of that `points` field, not a document.
    /// 
    /// ```java
    /// r.table("users").max("points").g("points").run(conn);
    /// ```
    /// 
    /// __Example:__ Return the user who has scored the most points, but add a default `None` return value to prevent an error if no user has ever scored points.
    /// 
    /// ```java
    /// r.table("users").max("points").default_(null).run(conn);
    /// ```

    pub fn max(&self) -> Client {
        cmd("max")
    }


    /// Removes duplicates from elements in a sequence
///
/// 
///
/// 
                /// 
    /// The `distinct` command can be called on any sequence or table with an index.
    /// 
    /// {% infobox %}
    /// While `distinct` can be called on a table without an index, the only effect will be to convert the table into a stream; the content of the stream will not be affected.
    /// {% endinfobox %}
    /// 
    /// __Example:__ Which unique villains have been vanquished by Marvel heroes?
    /// 
    /// ```java
    /// r.table("marvel").concatMap(
    ///     hero -> hero.g("villain_list")
    /// ).distinct().run(conn);
    /// ```
    /// 
    /// __Example:__ Topics in a table of messages have a secondary index on them, and more than one message can have the same topic. What are the unique topics in the table?
    /// 
    /// ```java
    /// r.table("messages").distinct().optArg("index", "topics").run(conn);
    /// ```
    /// 
    /// The above structure is functionally identical to:
    /// 
    /// ```java
    /// r.table("messages").g("topics").distinct().run(conn);
    /// ```
    /// 
    /// However, the first form (passing the index as an argument to `distinct`) is faster, and won't run into array limit issues since it's returning a stream.

    pub fn distinct(&self) -> Client {
        cmd("distinct")
    }


    /// 
///
/// 
///
/// 
                /// When called with values, returns `true` if a sequence contains all the
    /// specified values.  When called with predicate functions, returns `true`
    /// if for each predicate there exists at least one element of the stream
    /// where that predicate returns `true`.
    /// 
    /// Values and predicates may be mixed freely in the argument list.
    /// 
    /// __Example:__ Has Iron Man ever fought Superman?
    /// 
    /// ```java
    /// r.table("marvel").get("ironman").g("opponents").contains("superman").run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ Has Iron Man ever defeated Superman in battle?
    /// 
    /// ```java
    /// r.table("marvel").get("ironman").g("battles").contains(
    ///     battle -> battle.g("winner").eq("ironman").and(
    ///               battle.g("loser").eq("superman"))
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Return all heroes who have fought _both_ Loki and the Hulk.
    /// 
    /// ```java
    /// r.table("marvel").filter(
    ///     hero -> hero.g("opponents").contains("loki", "hulk")
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Use `contains` with a predicate function to simulate an `or`. Return the Marvel superheroes who live in Detroit, Chicago or Hoboken.
    /// 
    /// ```java
    /// r.table("marvel").filter(
    ///     hero -> r.expr(r.array("Detroit", "Chicago", "Hoboken"))
    ///              .contains(hero.g("city"))
    /// 
    /// ).run(conn);
    /// ```

    pub fn contains<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("contains", args)
    }


    /// 
///
/// 
///
/// 
                /// Plucks out one or more attributes from either an object or a sequence of objects
    /// (projection).
    /// 
    /// __Example:__ We just need information about IronMan's reactor and not the rest of the
    /// document.
    /// 
    /// ```java
    /// r.table("marvel").get("IronMan").pluck("reactorState", "reactorPower").run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ For the hero beauty contest we only care about certain qualities.
    /// 
    /// ```java
    /// r.table("marvel").pluck("beauty", "muscleTone", "charm").run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ Pluck can also be used on nested objects.
    /// 
    /// ```java
    /// // JSON equivalent:
    /// //   { "abilities": { "damage": true, "mana_cost": true }, "weapons": true }
    /// r.table("marvel").pluck(
    ///     r.hashMap("abilities",
    ///         r.hashMap("damage", true).with("mana_cost", true))
    ///     .with("weapons", true)
    /// ).run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ The nested syntax can quickly become overly verbose, so there's a shorthand for it.
    /// 
    /// ```java
    /// // JSON equivalent:
    /// //   { "abilities": [ "damage", "mana cost" ] }, "weapons"
    /// r.table("marvel")
    ///  .pluck(r.hashMap("abilities", r.array("damage", "mana_cost")), "weapons")
    ///  .run(conn);
    /// ```
    /// 
    /// For more information read the [nested field documentation](/docs/nested-fields/).

    pub fn pluck<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("pluck", args)
    }


    /// 
///
/// 
///
/// 
                /// The opposite of pluck; takes an object or a sequence of objects, and returns them with
    /// the specified fields or paths removed.
    /// 
    /// __Example:__ Since we don't need it for this computation we'll save bandwidth and leave
    /// out the list of IronMan's romantic conquests.
    /// 
    /// ```java
    /// r.table("marvel").get("IronMan").without("personalVictoriesList").run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ Without their prized weapons, our enemies will quickly be vanquished.
    /// 
    /// ```java
    /// r.table("enemies").without("weapons").run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ Nested objects can be used to remove the damage subfield from the weapons and abilities fields.
    /// 
    /// ```java
    /// r.table("marvel").without(
    ///     r.hashMap("weapons", r.hashMap("damage", true))
    ///      .with("abilities", r.hashMap("damage", true))
    /// ).run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ The nested syntax can quickly become overly verbose so there's a shorthand for it.
    /// 
    /// ```java
    /// r.table("marvel")
    ///  .without(r.hashMap("weapons", "damage").with("abilities", "damage")).run(conn);
    /// ```
    /// 

    pub fn without<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("without", args)
    }


    /// Merge two or more objects together to construct a new object with properties from all
///
/// 
///
/// When there is a conflict between field names, preference is given to fields in the rightmost object in the argument list. `merge` also accepts a function that returns an object, which will be used similarly to a [map](/api/java/map/) function.
                /// 
    /// __Example:__ Equip Thor for battle.
    /// 
    /// ```java
    /// r.table("marvel").get("thor")
    ///  .merge(r.table("equipment").get("hammer"),
    ///         r.table("equipment").get("pimento_sandwich"))
    ///  .run(conn);
    /// ```
    /// 
    /// __Example:__ Equip every hero for battle, using a function to retrieve their weapons.
    /// 
    /// ```java
    /// r.table("marvel").merge(
    ///     hero -> r.hashMap("weapons", r.table("weapons").get(hero.g("weapon_id")))
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Use `merge` to join each blog post with its comments.
    /// 
    /// Note that the sequence being merged&mdash;in this example, the comments&mdash;must be coerced from a selection to an array. Without `coerceTo` the operation will throw an error ("Expected type DATUM but found SELECTION").
    /// 
    /// ```java
    /// r.table("posts").merge(
    ///     post -> r.hashMap("comments", r.table("comments").getAll(post.g("id"))
    ///                       .optArg("index", "post_id").coerceTo("array"))
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Merge can be used recursively to modify sub-objects within objects.
    /// 
    /// ```java
    /// r.expr(r.hashMap("weapons", r.hashMap("spectacular graviton beam",
    ///     r.hashMap("dmg", 10).with("cooldown", 20))))
    ///  .merge(r.hashMap("weapons", r.hashMap("spectacular graviton beam",
    ///     r.hashMap("dmg", 10))))
    ///  .run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ To replace a nested object with another object you can use the [literal](/api/java/literal) term.
    /// 
    /// ```java
    /// r.expr(r.hashMap("weapons", r.hashMap("spectacular graviton beam",
    ///     r.hashMap("dmg", 10).with("cooldown", 20))))
    ///  .merge(r.hashMap("weapons", r.literal(r.hashMap("repulsor rays",
    ///     r.hashMap("dmg", 3).with("cooldown", 0)))))
    ///  .run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ `literal` can be used to remove keys from an object as well.
    /// 
    /// ```java
    /// r.expr(r.hashMap("weapons", r.hashMap("spectacular graviton beam",
    ///     r.hashMap("dmg", 10).with("cooldown", 20))))
    ///  .merge(r.hashMap("weapons", r.hashMap("spectacular graviton beam",
    ///     r.literal())))
    ///  .run(conn);
    /// ```

    pub fn merge<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("merge", args)
    }


    /// Append a value to an array
///
/// 
///
/// 
                /// 
    /// __Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.
    /// 
    /// ```java
    /// r.table("marvel").get("IronMan").g("equipment").append("newBoots").run(conn);
    /// ```
    /// 
    /// 

    pub fn append<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("append", args)
    }


    /// Prepend a value to an array
///
/// 
///
/// 
                /// 
    /// __Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.
    /// 
    /// ```java
    /// r.table("marvel").get("IronMan").g("equipment").prepend("newBoots").run(conn);
    /// ```
    /// 
    /// 

    pub fn prepend<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("prepend", args)
    }


    /// Remove the elements of one array from another array
///
/// 
///
/// 
                /// 
    /// __Example:__ Retrieve Iron Man's equipment list without boots.
    /// 
    /// ```java
    /// r.table("marvel").get("IronMan").g("equipment").difference(r.array("Boots"))
    ///  .run(conn);
    /// ```
    /// 
    /// __Example:__ Remove Iron Man's boots from his equipment.
    /// 
    /// ```java
    /// r.table("marvel").get("IronMan").g("equipment").update(
    ///     doc -> r.hashMap("equipment",
    ///                      doc.g("equipment").difference(r.array("Boots")))
    /// ).run(conn);
    /// ```
    /// 
    /// 

    pub fn difference<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("difference", args)
    }


    /// Add a value to an array and return it as a set (an array with distinct values)
///
/// 
///
/// 
                /// 
    /// __Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.
    /// 
    /// ```java
    /// r.table("marvel").get("IronMan").g("equipment").setInsert("newBoots").run(conn);
    /// ```
    /// 
    /// 

    pub fn set_insert<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("set_insert", args)
    }


    /// Perform a set intersection of two arrays, returning an array with all unique items from both
///
/// 
///
/// 
                /// 
    /// __Example:__ Retrieve Iron Man's equipment list with the addition of some new boots and an arc reactor.
    /// 
    /// ```java
    /// r.table("marvel").get("IronMan").g("equipment")
    ///  .setUnion(r.array("newBoots", "arc_reactor")).run(conn);
    /// ```
    /// 

    pub fn set_union<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("set_union", args)
    }


    /// 
///
/// 
///
/// 
                /// Intersect two arrays returning values that occur in both of them as a set (an array with
    /// distinct values).
    /// 
    /// __Example:__ Check which pieces of equipment Iron Man has from a fixed list.
    /// 
    /// ```java
    /// r.table("marvel").get("IronMan").g("equipment")
    ///  .setIntersection(r.array("newBoots", "arc_reactor")).run(conn);
    /// ```
    /// 

    pub fn set_intersection<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("set_intersection", args)
    }


    /// 
///
/// 
///
/// 
                /// Remove the elements of one array from another and return them as a set (an array with
    /// distinct values).
    /// 
    /// __Example:__ Check which pieces of equipment Iron Man has, excluding a fixed list.
    /// 
    /// ```java
    /// r.table("marvel").get("IronMan").g("equipment")
    ///  .setDifference(r.array("newBoots", "arc_reactor")).run(conn);
    /// ```
    /// 
    /// 

    pub fn set_difference<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("set_difference", args)
    }


    /// Get a single field from an object
///
/// 
///
/// If called on a sequence, gets that field from every object in the sequence, skipping objects that lack it.
                /// 
    /// {% infobox %}
    /// Under most circumstances, you'll want to use [getField](/api/java/get_field) (or its shorthand `g`) or [nth](/api/java/nth) rather than `bracket`. The `bracket` term may be useful in situations where you are unsure of the data type returned by the term you are calling `bracket` on.
    /// {% endinfobox %}
    /// 
    /// __Example:__ What was Iron Man's first appearance in a comic?
    /// 
    /// ```java
    /// r.table("marvel").get("IronMan").bracket("firstAppearance").run(conn);
    /// // more idiomatically:
    /// r.table("marvel").get("IronMan").g("firstAppearance").run(conn);
    /// ```
    /// 
    /// <!-- stop -->
    /// 
    /// The `bracket` command also accepts integer arguments as array offsets, like the [nth](/api/java/nth) command.
    /// 
    /// __Example:__ Get the fourth element in a sequence. (The first element is position `0`, so the fourth element is position `3`.)
    /// 
    /// ```java
    /// r.expr(r.array(10, 20, 30, 40, 50)).bracket(3).run(conn);
    /// // more idiomatically:
    /// r.expr(r.array(10, 20, 30, 40, 50)).nth(3).run(conn);
    /// 
    /// 40
    /// ```

    pub fn bracket<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("bracket", args)
    }


    /// Get a single field from an object
///
/// 
///
/// If called on a sequence, gets that field from every
                /// object in the sequence, skipping objects that lack it.
    /// 
    /// You may use either `getField` or its shorthand, `g`.
    /// 
    /// __Example:__ What was Iron Man's first appearance in a comic?
    /// 
    /// ```java
    /// r.table("marvel").get("IronMan").g("firstAppearance").run(conn);
    /// ```

    pub fn get_field<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("get_field", args)
    }


    /// Test if an object has one or more fields
///
/// 
///
/// An object has a field if it has that key and the key has a non-null value. For instance, the object `{'a': 1,'b': 2,'c': null}` has the fields `a` and `b`.
                /// 
    /// When applied to a single object, `hasFields` returns `true` if the object has the fields and `false` if it does not. When applied to a sequence, it will return a new sequence (an array or stream) containing the elements that have the specified fields.
    /// 
    /// __Example:__ Return the players who have won games.
    /// 
    /// ```java
    /// r.table("players").hasFields("games_won").run(conn);
    /// ```
    /// 
    /// __Example:__ Return the players who have *not* won games. To do this, use `hasFields` with [not](/api/java/not), wrapped with [filter](/api/java/filter).
    /// 
    /// ```java
    /// r.table("players").filter(
    ///     row -> row.hasFields("games_won").not()
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Test if a specific player has won any games.
    /// 
    /// ```java
    /// r.table("players").get("b5ec9714-837e-400c-aa74-dbd35c9a7c4c")
    ///  .hasFields("games_won").run(conn);
    /// ```
    /// 
    /// **Nested Fields**
    /// 
    /// `hasFields` lets you test for nested fields in objects. If the value of a field is itself a set of key/value pairs, you can test for the presence of specific keys.
    /// 
    /// __Example:__ In the `players` table, the `games_won` field contains one or more fields for kinds of games won:
    /// 
    /// ```json
    /// {
    ///     "games_won": {
    ///         "playoffs": 2,
    ///         "championships": 1
    ///     }
    /// }
    /// ```
    /// 
    /// Return players who have the "championships" field.
    /// 
    /// ```java
    /// r.table("players")
    ///  .hasFields(r.hashMap("games_won", r.hashMap("championships", true)))
    ///  .run(conn);
    /// ```
    /// 
    /// Note that `true` in the example above is testing for the existence of `championships` as a field, not testing to see if the value of the `championships` field is set to `true`. There's a more convenient shorthand form available. (See [pluck](/api/java/pluck) for more details on this.)
    /// 
    /// ```java
    /// r.table("players").hasFields(r.hashMap("games_won", "championships")).run(conn);
    /// ```

    pub fn has_fields<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("has_fields", args)
    }


    /// Insert a value in to an array at a given index
///
/// 
///
/// Returns the modified array.
                /// 
    /// __Example:__ Hulk decides to join the avengers.
    /// 
    /// ```java
    /// r.expr(r.array("Iron Man", "Spider-Man")).insertAt(1, "Hulk").run(conn);
    /// ```
    /// 
    /// 

    pub fn insert_at<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("insert_at", args)
    }


    /// Insert several values into an array at the given index
///
/// 
///
/// Returns the modified array.
                /// 
    /// __Example:__ Hulk and Thor decide to join the Avengers.
    /// 
    /// ```java
    /// r.expr(r.array("Iron Man", "Spider-Man"))
    ///  .spliceAt(1, r.array("Hulk", "Thor")).run(conn);
    /// ```
    /// 

    pub fn splice_at<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("splice_at", args)
    }


    /// Remove one or more elements from an array at a given index
///
/// 
///
/// Returns the modified array. (Note: `deleteAt` operates on arrays, not documents; to delete documents, see the [delete](/api/java/delete) command.)
                /// 
    /// If only `offset` is specified, `deleteAt` removes the element at that index. If both `offset` and `endOffset` are specified, `deleteAt` removes the range of elements between `offset` and `endOffset`, inclusive of `offset` but not inclusive of `endOffset`.
    /// 
    /// If `endOffset` is specified, it must not be less than `offset`. Both `offset` and `endOffset` must be within the array's bounds (i.e., if the array has 10 elements, an `offset` or `endOffset` of 10 or higher is invalid).
    /// 
    /// By using a negative `offset` you can delete from the end of the array. `-1` is the last element in the array, `-2` is the second-to-last element, and so on. You may specify a negative `endOffset`, although just as with a positive value, this will not be inclusive. The range `(2,-1)` specifies the third element through the next-to-last element.
    /// 
    /// __Example:__ Delete the second element of an array.
    /// 
    /// ```java
    /// r.expr(r.array("a", "b", "c", "d", "e", "f")).deleteAt(1).run(conn);
    /// 
    /// // Result:
    /// ["a", "c", "d", "e", "f"]
    /// ```
    /// 
    /// __Example:__ Delete the second and third elements of an array.
    /// 
    /// ```java
    /// r.expr(r.array("a", "b", "c", "d", "e", "f")).deleteAt(1, 3).run(conn);
    /// 
    /// // Result:
    /// ["a", "d", "e", "f"]
    /// ```
    /// 
    /// __Example:__ Delete the next-to-last element of an array.
    /// 
    /// ```java
    /// r.expr(r.array("a", "b", "c", "d", "e", "f")).deleteAt(-2).run(conn);
    /// 
    /// // Result:
    /// ["a", "b", "c", "d", "f"]
    /// ```
    /// 
    /// __Example:__ Delete a comment on a post.
    /// 
    /// Given a post document such as:
    /// 
    /// ```json
    /// {
    ///     "id": "4cf47834-b6f9-438f-9dec-74087e84eb63",
    ///     "title": "Post title",
    ///     "author": "Bob",
    ///     "comments": [
    ///         { "author": "Agatha", "text": "Comment 1" },
    ///         { "author": "Fred", "text": "Comment 2" }
    ///     ]
    /// }
    /// ```
    /// 
    /// The second comment can be deleted by using `update` and `deleteAt` together.
    /// 
    /// ```java
    /// r.table("posts").get("4cf47834-b6f9-438f-9dec-74087e84eb63").update(
    ///     row -> r.hashMap("comments", row.g("comments").deleteAt(1)
    /// ).run(conn);
    /// ```

    pub fn delete_at<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("delete_at", args)
    }


    /// Change a value in an array at a given index
///
/// 
///
/// Returns the modified array.
                /// 
    /// __Example:__ Bruce Banner hulks out.
    /// 
    /// ```java
    /// r.expr(r.array("Iron Man", "Bruce", "Spider-Man")).changeAt(1, "Hulk")
    ///  .run(conn);
    /// ```

    pub fn change_at<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("change_at", args)
    }


    /// Return an array containing all of an object's keys
///
/// 
///
/// Note that the keys will be sorted as described in [ReQL data types](/docs/data-types/#sorting-order) (for strings, lexicographically).
                /// 
    /// __Example:__ Get all the keys from a table row.
    /// 
    /// ```java
    /// // row: { "id": 1, "mail": "fred@example.com", "name": "fred" }
    /// 
    /// r.table("users").get(1).keys().run(conn);
    /// 
    /// // Result:
    /// [ "id", "mail", "name" ]
    /// ```

    pub fn keys(&self) -> Client {
        cmd("keys")
    }


    /// Return an array containing all of an object's values
///
/// 
///
/// `values()` guarantees the values will come out in the same order as [keys](/api/java/keys).
                /// 
    /// __Example:__ Get all of the values from a table row.
    /// 
    /// 
    /// ```java
    /// // row: { "id": 1, "mail": "fred@example.com", "name": "fred" }
    /// 
    /// r.table("users").get(1).values().run(conn);
    /// 
    /// // Result:
    /// [ 1, "fred@example.com", "fred" ]
    /// ```

    pub fn values(&self) -> Client {
        cmd("values")
    }


    /// Replace an object in a field instead of merging it with an existing object in a `merge` or `update` operation
///
/// 
///
/// Using `literal` with no arguments in a `merge` or `update` operation will remove the corresponding field.
                /// 
    /// Assume your users table has this structure:
    /// 
    /// ```json
    /// [
    ///     {
    ///         "id": 1,
    ///         "name": "Alice",
    ///         "data": {
    ///             "age": 18,
    ///             "city": "Dallas"
    ///         }
    ///     }       
    ///     ...
    /// ]
    /// ```
    /// 
    /// Using `update` to modify the `data` field will normally merge the nested documents:
    /// 
    /// ```java
    /// r.table("users").get(1)
    ///  .update(r.hashMap("data", r.hashMap("age", 19).with("job", "Engineer")))
    ///  .run(conn);
    /// 
    /// // Result:
    /// {
    ///     "id": 1,
    ///     "name": "Alice",
    ///     "data": {
    ///         "age": 19,
    ///         "city": "Dallas",
    ///         "job": "Engineer"
    ///     }
    /// }       
    /// ```
    /// 
    /// That will preserve `city` and other existing fields. But to replace the entire `data` document with a new object, use `literal`.
    /// 
    /// __Example:__ Replace one nested document with another rather than merging the fields.
    /// 
    /// ```java
    /// r.table("users").get(1)
    ///  .update(r.hashMap("data", r.literal(r.hashMap("age", 19).with("job", "Engineer"))))
    ///  .run(conn);
    /// 
    /// // Result:
    /// {
    ///     "id": 1,
    ///     "name": "Alice",
    ///     "data": {
    ///         "age": 19,
    ///         "job": "Engineer"
    ///     }
    /// }       
    /// ```
    /// 
    /// __Example:__ Use `literal` to remove a field from a document.
    /// 
    /// ```java
    /// r.table("users").get(1).merge(r.hashMap("data", r.literal())).run(conn);
    /// 
    /// // Result:
    /// {
    ///     "id": 1,
    ///     "name": "Alice"
    /// }
    /// ```

    pub fn literal(&self) -> Client {
        cmd("literal")
    }


    /// 
///
/// 
///
/// 
                /// Creates an object from a list of key-value pairs, where the keys must
    /// be strings.  `r.object(A, B, C, D)` is equivalent to
    /// `r.expr([[A, B], [C, D]]).coerce_to('OBJECT')`.
    /// 
    /// __Example:__ Create a simple object.
    /// 
    /// ```java
    /// r.object("id", 5, "data", r.array("foo", "bar")).run(conn);
    /// 
    /// // Result:
    /// { "data": ["foo", "bar"], "id": 5}
    /// ```

    pub fn object<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("object", args)
    }


    /// Match a string against a regular expression
///
/// 
///
/// If there is a match, returns an object with the fields:
                /// 
    /// - `str`: The matched string
    /// - `start`: The matched string's start
    /// - `end`: The matched string's end
    /// - `groups`: The capture groups defined with parentheses
    /// 
    /// If no match is found, returns `null`.
    /// 
    /// <!-- break -->
    /// 
    /// Accepts [RE2 syntax][re2]. You can enable case-insensitive matching by prefixing the regular expression with `(?i)`. See the linked RE2 documentation for more flags.
    /// 
    /// [re2]: https://github.com/google/re2/wiki/Syntax
    /// 
    /// The `match` command does not support backreferences.
    /// 
    /// __Example:__ Get all users whose name starts with "A". Because `null` evaluates to `false` in
    /// [filter](/api/java/filter/), you can use the result of `match` for the predicate.
    /// 
    /// 
    /// ```java
    /// r.table("users").filter(doc -> doc.g("name").match("^A")).run(conn);
    /// ```
    /// 
    /// __Example:__ Get all users whose name ends with "n."
    /// 
    /// ```java
    /// r.table("users").filter(doc -> doc.g("name").match("n$")).run(conn);
    /// ```
    /// __Example:__ Get all users whose name contains "li."
    /// 
    /// ```java
    /// r.table("users").filter(doc -> doc.g("name").match("li")).run(conn);
    /// ```
    /// 
    /// __Example:__ Get all users whose name is "John," performing a case-insensitive search.
    /// 
    /// ```java
    /// r.table("users").filter(doc -> doc.g("name").match("(?i)^john$")).run(conn);
    /// ```
    /// 
    /// __Example:__ Retrieve the domain of a basic email.
    /// 
    /// ```java
    /// r.expr("name@domain.com").match(".*@(.*)").run(conn);
    /// ```
    /// 
    /// Result:
    /// 
    /// ```json
    /// {
    ///     "start": 0,
    ///     "end": 20,
    ///     "str": "name@domain.com",
    ///     "groups": [
    ///         {
    ///             "end": 17,
    ///             "start": 7,
    ///             "str": "domain.com"
    ///         }
    ///     ]
    /// }
    /// ```
    /// 
    /// You can then retrieve only the domain with [g()](/api/java/get_field) and [nth](/api/java/nth).
    /// 
    /// ```java
    /// r.expr("name@domain.com").match(".*@(.*)").g("groups").nth(0)
    ///  .g("str").run(conn);
    /// ```
    /// 
    /// Returns `domain.com`.
    /// 
    /// 
    /// __Example:__ A failure to parse out the domain name will return `null`.
    /// 
    /// ```java
    /// r.expr("name[at]domain.com").match(".*@(.*)").run(conn);
    /// ```

    pub fn match_<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("match_", args)
    }


    /// Split a string into substrings
///
/// 
///
/// With no arguments, will split on whitespace; when called with a string as the first argument, will split using that string as a separator. A maximum number of splits can also be specified. (To specify `max_splits` while still splitting on whitespace, use `null` as the separator argument.)
                /// 
    /// Mimics the behavior of Python's `string.split` in edge cases, except
    /// for splitting on the empty string, which instead produces an array of
    /// single-character strings.
    /// 
    /// __Example:__ Split on whitespace.
    /// 
    /// ```java
    /// r.expr("foo  bar bax").split().run(conn);
    /// ```
    /// 
    /// Result:
    /// 
    /// ```json
    /// ["foo", "bar", "bax"]
    /// ```
    /// 
    /// __Example:__ Split the entries in a CSV file.
    /// 
    /// ```java
    /// r.expr("12,37,,22,").split(",").run(conn);
    /// ```
    /// 
    /// Result:
    /// 
    /// ```json
    /// ["12", "37", "", "22", ""]
    /// ```
    /// 
    /// __Example:__ Split a string into characters.
    /// 
    /// ```java
    /// r.expr("mlucy").split("").run(conn);
    /// ```
    /// 
    /// Result:
    /// 
    /// ```json
    /// ["m", "l", "u", "c", "y"]
    /// ```
    /// 
    /// __Example:__ Split the entries in a CSV file, but only at most 3
    /// times.
    /// 
    /// ```java
    /// r.expr("12,37,,22,").split(",", 3).run(conn);
    /// ```
    /// 
    /// Result:
    /// 
    /// ```json
    /// ["12", "37", "", "22,"]
    /// ```
    /// 
    /// __Example:__ Split on whitespace at most once (i.e. get the first word).
    /// 
    /// ```java
    /// r.expr("foo  bar bax").split(null, 1).run(conn);
    /// ```
    /// 
    /// Result:
    /// 
    /// ```json
    /// ["foo", "bar bax"]
    /// ```

    pub fn split(&self) -> Client {
        cmd("split")
    }


    /// Uppercases a string
///
/// 
///
/// 
                /// 
    /// __Example:__
    /// 
    /// ```java
    /// r.expr("Sentence about LaTeX.").upcase().run(conn);
    /// ```
    /// 
    /// Result:
    /// 
    /// ```
    /// "SENTENCE ABOUT LATEX."
    /// ```
    /// 
    /// __Note:__ `upcase` and `downcase` only affect ASCII characters.

    pub fn upcase(&self) -> Client {
        cmd("upcase")
    }


    /// Lowercase a string
///
/// 
///
/// 
                /// 
    /// __Example:__
    /// 
    /// ```java
    /// r.expr("Sentence about LaTeX.").downcase().run(conn);
    /// ```
    /// 
    /// Result:
    /// 
    /// ```
    /// "sentence about latex."
    /// ```
    /// 
    /// __Note:__ `upcase` and `downcase` only affect ASCII characters.

    pub fn downcase(&self) -> Client {
        cmd("downcase")
    }


    /// Sum two or more numbers, or concatenate two or more strings or arrays
///
/// 
///
/// 
                /// 
    /// The `add` command can be called in either prefix or infix form; both forms are equivalent. Note that ReQL will not perform type coercion. You cannot, for example, `add` a string and a number together.
    /// 
    /// __Example:__ It's as easy as 2 + 2 = 4.
    /// 
    /// ```java
    /// r.expr(2).add(2).run(conn);
    /// 
    /// // Result:
    /// 4
    /// ```
    /// 
    /// __Example:__ Concatenate strings.
    /// 
    /// ```java
    /// r.expr("foo").add("bar", "baz").run(conn);
    /// 
    /// // Result:
    /// "foobarbaz"
    /// ```
    /// 
    /// 
    /// __Example:__ Concatenate arrays.
    /// 
    /// ```java
    /// r.expr(["foo", "bar"]).add(["buzz"]).run(conn);
    /// 
    /// // Result:
    /// [ "foo", "bar", "buzz" ]
    /// ```
    /// 
    /// 
    /// __Example:__ Create a date one year from now.
    /// 
    /// ```java
    /// r.now().add(365*24*60*60).run(conn);
    /// ```
    /// 
    /// __Example:__ Use [args](/api/java/args) with `add` to sum multiple values.
    /// 
    /// ```java
    /// int[] vals = { 10, 20, 30 };
    /// r.add(r.args(vals)).run(conn);
    /// 
    /// // Result:
    /// 60
    /// ```
    /// 
    /// __Example:__ Concatenate an array of strings with `args`.
    /// 
    /// ```java
    /// String[] vals = { "foo", "bar", "buzz" };
    /// r.add(r.args(vals)).run(conn);
    /// 
    /// // Result:
    /// "foobarbuzz"
    /// ```

    pub fn add<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("add", args)
    }


    /// Subtract two numbers
///
/// 
///
/// 
                /// 
    /// __Example:__ It's as easy as 2 - 2 = 0.
    /// 
    /// ```java
    /// r.expr(2).sub(2).run(conn);
    /// ```
    /// 
    /// __Example:__ Create a date one year ago today.
    /// 
    /// ```java
    /// r.now().sub(365*24*60*60);
    /// ```
    /// 
    /// __Example:__ Retrieve how many seconds elapsed between today and `date`.
    /// 
    /// ```java
    /// r.now().sub(date);
    /// ```

    pub fn sub<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("sub", args)
    }


    /// Multiply two numbers, or make a periodic array
///
/// 
///
/// 
                /// 
    /// __Example:__ It's as easy as 2 * 2 = 4.
    /// 
    /// ```java
    /// r.expr(2).mul(2).run(conn);
    /// ```
    /// 
    /// __Example:__ Arrays can be multiplied by numbers as well.
    /// 
    /// ```java
    /// r.expr(["This", "is", "the", "song", "that", "never", "ends."]).mul(100).run(conn);
    /// ```
    /// 

    pub fn mul<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("mul", args)
    }


    /// Divide two numbers
///
/// 
///
/// 
                /// 
    /// __Example:__ It's as easy as 2 / 2 = 1.
    /// 
    /// ```java
    /// r.expr(2).div(2).run(conn);
    /// ```
    /// 

    pub fn div<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("div", args)
    }


    /// 
///
/// 
///
/// 

    pub fn mod_<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("mod_", args)
    }


    /// Compute the logical "and" of one or more values
///
/// 
///
/// 
                /// 
    /// The `and` command can be used as an infix operator after its first argument (`r.expr(true).and(false)`) or given all of its arguments as parameters (`r.and(true,false)`).
    /// 
    /// Calling `and` with zero arguments will return `true`.
    /// 
    /// __Example:__ Return whether both `a` and `b` evaluate to true.
    /// 
    /// ```java
    /// boolean a = true;
    /// boolean b = false;
    /// r.expr(a).and(b).run(conn);
    /// 
    /// // Result:
    /// false
    /// ```
    /// 
    /// __Example:__ Return whether all of `x`, `y` and `z` evaluate to true.
    /// 
    /// ```java
    /// boolean x = true;
    /// boolean y = true;
    /// boolean z = true;
    /// 
    /// r.and(x, y, z).run(conn);
    /// 
    /// // Result:
    /// true
    /// ```

    pub fn and<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("and", args)
    }


    /// Compute the logical "or" of one or more values
///
/// 
///
/// 
                /// 
    /// The `or` command can be used as an infix operator after its first argument (`r.expr(true).or(false)`) or given all of its arguments as parameters (`r.or(true,false)`).
    /// 
    /// Calling `or` with zero arguments will return `false`.
    /// 
    /// __Example:__ Return whether either `a` or `b` evaluate to true.
    /// 
    /// ```java
    /// boolean a = true;
    /// boolean b = false;
    /// r.expr(a).or(b).run(conn);
    /// 
    /// // Result:
    /// true
    /// ```
    /// 
    /// __Example:__ Return whether any of `x`, `y` or `z` evaluate to true.
    /// 
    /// ```java
    /// boolean x = false;
    /// boolean y = false;
    /// boolean z = false;
    /// r.or(x, y, z).run(conn);
    /// 
    /// // Result:
    /// false
    /// ```
    /// 
    /// __Note:__ When using `or` inside a `filter` predicate to test the values of fields that may not exist on the documents being tested, you should use the `default` command with those fields so they explicitly return `false`.
    /// 
    /// ```java
    /// r.table("posts").filter(row ->
    ///     row.g("category").default("foo").eq("article").
    ///     or(row.g("genre").default("foo").eq("mystery"))
    /// ).run(conn);
    /// ```

    pub fn or<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("or", args)
    }


    /// Test if two or more values are equal
///
/// 
///
/// 
                /// 
    /// __Example:__ See if a user's `role` field is set to `administrator`. 
    /// 
    /// ```java
    /// r.table("users").get(1).g("role").eq("administrator").run(conn);
    /// ```
    /// 
    /// __Example:__ See if three variables contain equal values.
    /// 
    /// ```java
    /// r.eq(a, b, c).run(conn);
    /// ```

    pub fn eq<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("eq", args)
    }


    /// Test if two or more values are not equal
///
/// 
///
/// 
                /// 
    /// __Example:__ See if a user's `role` field is not set to `administrator`. 
    /// 
    /// ```java
    /// r.table("users").get(1).g("role").ne("administrator").run(conn);
    /// ```
    /// 
    /// __Example:__ See if three variables do not contain equal values.
    /// 
    /// ```java
    /// r.ne(a, b, c).run(conn);
    /// ```

    pub fn ne<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("ne", args)
    }


    /// Compare values, testing if the left-hand value is greater than the right-hand
///
/// 
///
/// 
                /// 
    /// __Example:__ Test if a player has scored more than 10 points.
    /// 
    /// ```java
    /// r.table("players").get(1).g("score").gt(10).run(conn);
    /// ```
    /// 
    /// __Example:__ Test if variables are ordered from lowest to highest, with no values being equal to one another.
    /// 
    /// ```java
    /// int a = 10;
    /// int b = 20;
    /// int c = 15;
    /// r.gt(a, b, c).run(conn);
    /// ```
    /// 
    /// This is the equivalent of the following:
    /// 
    /// ```java
    /// r.gt(a, b).and(r.gt(b, c)).run(conn);
    /// ```

    pub fn gt<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("gt", args)
    }


    /// Compare values, testing if the left-hand value is greater than or equal to the right-hand
///
/// 
///
/// 
                /// 
    /// __Example:__ Test if a player has scored 10 points or more.
    /// 
    /// ```java
    /// r.table("players").get(1).g("score").ge(10).run(conn);
    /// ```
    /// 
    /// __Example:__ Test if variables are ordered from lowest to highest.
    /// 
    /// ```java
    /// int a = 10;
    /// int b = 20;
    /// int c = 15;
    /// r.ge(a, b, c).run(conn);
    /// ```
    /// 
    /// This is the equivalent of the following:
    /// 
    /// ```java
    /// r.ge(a, b).and(r.ge(b, c)).run(conn);
    /// ```

    pub fn ge<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("ge", args)
    }


    /// Compare values, testing if the left-hand value is less than the right-hand
///
/// 
///
/// 
                /// 
    /// __Example:__ Test if a player has scored less than 10 points.
    /// 
    /// ```java
    /// r.table("players").get(1).g("score").lt(10).run(conn);
    /// ```
    /// 
    /// __Example:__ Test if variables are ordered from highest to lowest, with no values being equal to one another.
    /// 
    /// ```java
    /// int a = 20;
    /// int b = 10;
    /// int c = 15;
    /// r.lt(a, b, c).run(conn);
    /// ```
    /// 
    /// This is the equivalent of the following:
    /// 
    /// ```java
    /// r.lt(a, b).and(r.lt(b, c)).run(conn);
    /// ```

    pub fn lt<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("lt", args)
    }


    /// Compare values, testing if the left-hand value is less than or equal to the right-hand
///
/// 
///
/// 
                /// 
    /// __Example:__ Test if a player has scored 10 points or less.
    /// 
    /// ```java
    /// r.table("players").get(1).g("score").le(10).run(conn);
    /// ```
    /// 
    /// __Example:__ Test if variables are ordered from highest to lowest.
    /// 
    /// ```java
    /// int a = 20;
    /// int b = 10;
    /// int c = 15;
    /// r.le(a, b, c).run(conn);
    /// ```
    /// 
    /// This is the equivalent of the following:
    /// 
    /// ```java
    /// r.le(a, b).and(r.le(b, c)).run(conn);
    /// ```

    pub fn le<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("le", args)
    }


    /// Compute the logical inverse (not) of an expression
///
/// 
///
/// 
                /// 
    /// `not` can be called either via method chaining, immediately after an expression that evaluates as a boolean value, or by passing the expression as a parameter to `not`. All values that are not `false` or `null` will be converted to `true`.
    /// 
    /// __Example:__ Not true is false.
    /// 
    /// ```java
    /// r(true).not().run(conn);
    /// r.not(true).run(conn);
    /// ```
    /// 
    /// These evaluate to `false`.
    /// 
    /// __Example:__ Return all the users that do not have a "flag" field.
    /// 
    /// ```java
    /// r.table("users").filter(user -> user.hasFields("flag").not()).run(conn);
    /// ```
    /// 
    /// __Example:__ As above, but prefix-style.
    /// 
    /// ```java
    /// r.table("users").filter(user -> r.not(user.hasFields("flag")).run(conn);
    /// ```

    pub fn not(&self) -> Client {
        cmd("not")
    }


    /// Generate a random number between given (or implied) bounds
///
/// 
///
/// `random` takes zero, one or two arguments, and can also take an [optArg](/api/java/optarg) of `float`.
                /// 
    /// - With __zero__ arguments, the result will be a floating-point number in the range `[0,1)` (from 0 up to but not including 1).
    /// - With __one__ argument _x,_ the result will be in the range `[0,x)`, and will be integer unless `.optArg("float", true)` is given as an option. Specifying a floating point number without the `float` option will raise an error.
    /// - With __two__ arguments _x_ and _y,_ the result will be in the range `[x,y)`, and will be integer unless `.optArg("float", true)` is given as an option.  If _x_ and _y_ are equal an error will occur, unless the floating-point option has been specified, in which case _x_ will be returned. Specifying a floating point number without the `float` option will raise an error.
    /// 
    /// Note: The last argument given will always be the 'open' side of the range, but when generating a floating-point number, the 'open' side may be less than the 'closed' side.
    /// 
    /// __Example:__ Generate a random number in the range `[0,1)`
    /// 
    /// ```java
    /// r.random().run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ Generate a random integer in the range `[0,100)`
    /// 
    /// ```java
    /// r.random(100).run(conn);
    /// r.random(0, 100).run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ Generate a random number in the range `(-2.24,1.59]`
    /// 
    /// ```java
    /// r.random(1.59, -2.24).optArg("float", true).run(conn)
    /// ```
    /// 

    pub fn random(&self) -> Client {
        cmd("random")
    }


    /// Rounds the given value to the nearest whole integer
///
/// 
///
/// 
                /// 
    /// For example, values of 1.0 up to but not including 1.5 will return 1.0, similar to [floor][]; values of 1.5 up to 2.0 will return 2.0, similar to [ceil][].
    /// 
    /// [floor]: /api/java/floor/
    /// [ceil]:  /api/java/ceil/
    /// 
    /// __Example:__ Round 12.345 to the nearest integer.
    /// 
    /// ```java
    /// r.round(12.345).run(conn);
    /// 
    /// // Result:
    /// 12.0
    /// ```
    /// 
    /// The `round` command can also be chained after an expression.
    /// 
    /// __Example:__ Round -12.345 to the nearest integer.
    /// 
    /// ```java
    /// r.expr(-12.345).round().run(conn);
    /// 
    /// // Result:
    /// -12.0
    /// ```
    /// 
    /// __Example:__ Return Iron Man's weight, rounded to the nearest integer.
    /// 
    /// ```java
    /// r.table("superheroes").get("ironman").g("weight").round().run(conn);
    /// ```

    pub fn round(&self) -> Client {
        cmd("round")
    }


    /// Rounds the given value up, returning the smallest integer greater than or equal to the given value (the value's ceiling)
///
/// 
///
/// 
                /// 
    /// __Example:__ Return the ceiling of 12.345.
    /// 
    /// ```java
    /// r.ceil(12.345).run(conn);
    /// 
    /// // Result:
    /// 13.0
    /// ```
    /// 
    /// The `ceil` command can also be chained after an expression.
    /// 
    /// __Example:__ Return the ceiling of -12.345.
    /// 
    /// ```java
    /// r.expr(-12.345).ceil().run(conn);
    /// 
    /// // Result:
    /// -12.0
    /// ```
    /// 
    /// __Example:__ Return Iron Man's weight, rounded up with `ceil`.
    /// 
    /// ```java
    /// r.table("superheroes").get("ironman").g("weight").ceil().run(conn);
    /// ```

    pub fn ceil(&self) -> Client {
        cmd("ceil")
    }


    /// Rounds the given value down, returning the largest integer value less than or equal to the given value (the value's floor)
///
/// 
///
/// 
                /// 
    /// __Example:__ Return the floor of 12.345.
    /// 
    /// ```java
    /// r.floor(12.345).run(conn);
    /// 
    /// // Result:
    /// 12.0
    /// ```
    /// 
    /// The `floor` command can also be chained after an expression.
    /// 
    /// __Example:__ Return the floor of -12.345.
    /// 
    /// ```java
    /// r.expr(-12.345).floor().run(conn);
    /// 
    /// // Result:
    /// -13.0
    /// ```
    /// 
    /// __Example:__ Return Iron Man's weight, rounded down with `floor`.
    /// 
    /// ```java
    /// r.table("superheroes").get("ironman").g("weight").floor().run(conn);
    /// ```

    pub fn floor(&self) -> Client {
        cmd("floor")
    }


    /// Return a time object representing the current time in UTC
///
/// 
///
/// The command now() is computed once when the server receives the query, so multiple instances of r.now() will always return the same time inside a query.
                /// 
    /// __Example:__ Add a new user with the time at which he subscribed.
    /// 
    /// ```java
    /// r.table("users").insert(
    ///     r.hashMap("name", "John")
    ///      .with("subscription_date", r.now())
    /// ).run(conn);
    /// ```
    /// 

    pub fn now(&self) -> Client {
        cmd("now")
    }


    /// Create a time object for a specific time
///
/// 
///
/// 
                /// 
    /// A few restrictions exist on the arguments:
    /// 
    /// - `year` is an integer between 1400 and 9,999.
    /// - `month` is an integer between 1 and 12.
    /// - `day` is an integer between 1 and 31.
    /// - `hour` is an integer.
    /// - `minutes` is an integer.
    /// - `seconds` is a double. Its value will be rounded to three decimal places
    /// (millisecond-precision).
    /// - `timezone` can be `'Z'` (for UTC) or a string with the format `±[hh]:[mm]`.
    /// 
    /// 
    /// __Example:__ Update the birthdate of the user "John" to November 3rd, 1986 UTC.
    /// 
    /// ```java
    /// r.table("user").get("John").update(
    ///     r.hashMap("birthdate", r.time(1986, 11, 3, 'Z'))
    /// ).run(conn);
    /// ```

    pub fn time<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("time", args)
    }


    /// Create a time object based on seconds since epoch
///
/// 
///
/// The first argument is a double and
                /// will be rounded to three decimal places (millisecond-precision).
    /// 
    /// __Example:__ Update the birthdate of the user "John" to November 3rd, 1986.
    /// 
    /// ```java
    /// r.table("user").get("John").update(
    ///     r.hashMap(birthdate, r.epochTime(531360000))
    /// ).run(conn);
    /// ```

    pub fn epoch_time<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("epoch_time", args)
    }


    /// Create a time object based on an ISO 8601 date-time string (e
///
/// 
///
/// g. '2013-01-01T01:01:01+00:00'). RethinkDB supports all valid ISO 8601 formats except for week dates. Read more about the ISO 8601 format at [Wikipedia](http://en.wikipedia.org/wiki/ISO_8601).
                /// 
    /// If you pass an ISO 8601 string without a time zone, you must specify the time zone with the `default_timezone` [optArg](/api/java/optarg).
    /// 
    /// __Example:__ Update the time of John's birth.
    /// 
    /// ```java
    /// r.table("user").get("John").update(
    ///     r.hashMap("birth", r.iso8601("1986-11-03T08:30:00-07:00"))
    /// ).run(conn);
    /// ```
    /// 
    /// 

    pub fn iso8601<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("iso8601", args)
    }


    /// Return a new time object with a different timezone
///
/// 
///
/// While the time stays the same, the results returned by methods such as hours() will change since they take the timezone into account. The timezone argument has to be of the ISO 8601 format.
                /// 
    /// __Example:__ Hour of the day in San Francisco (UTC/GMT -8, without daylight saving time).
    /// 
    /// ```java
    /// r.now().inTimezone("-08:00").hours().run(conn);
    /// ```
    /// 
    /// 

    pub fn in_timezone<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("in_timezone", args)
    }


    /// Return the timezone of the time object
///
/// 
///
/// 
                /// 
    /// __Example:__ Return all the users in the "-07:00" timezone.
    /// 
    /// ```java
    /// r.table("users").filter(
    ///     user -> user.g("subscriptionDate").timezone().eq("-07:00")
    /// ).run(conn);
    /// ```
    /// 
    /// 

    pub fn timezone(&self) -> Client {
        cmd("timezone")
    }


    /// Return whether a time is between two other times
///
/// 
///
/// 
                /// 
    /// By default, this is inclusive of the start time and exclusive of the end time. Use the [optArgs](/api/java/optarg) `left_bound` and `right_bound` to explicitly include (`closed`) or exclude (`open`) that endpoint of the range.
    /// 
    /// __Example:__ Retrieve all the posts that were posted between December 1st, 2013
    /// (inclusive) and December 10th, 2013 (exclusive).
    /// 
    /// ```java
    /// r.table("posts").filter(
    ///     row -> row.g("date").during(r.time(2013, 12, 1, "Z"), r.time(2013, 12, 10, "Z"))
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Retrieve all the posts that were posted between December 1st, 2013
    /// (exclusive) and December 10th, 2013 (inclusive).
    /// 
    /// ```java
    /// r.table("posts").filter(
    ///     row -> row.g("date")
    ///         .during(r.time(2013, 12, 1, "Z"), r.time(2013, 12, 10, "Z"))
    ///         .optArg("left_bound", "open").optArg("right_bound", "closed")
    /// ).run(conn);
    /// ```
    /// 

    pub fn during<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("during", args)
    }


    /// Return a new [OffsetDateTime][odt] object only based on the day, month and year (ie
///
/// 
///
/// the same day at 00:00).
                /// 
    /// [odt]: https://docs.oracle.com/javase/8/docs/api/java/time/OffsetDateTime.html
    /// 
    /// __Example:__ Retrieve all the users whose birthday is today.
    /// 
    /// ```java
    /// r.table("users").filter(
    ///     user -> user.g("birthdate").date().eq(r.now().date())
    /// ).run(conn);
    /// ```
    /// 
    /// <!-- stop -->
    /// 
    /// Note that the [now][] command always returns UTC time, so the comparison may fail if `user.g("birthdate")` isn't also in UTC. You can use the [inTimezone][itz] command to adjust for this:
    /// 
    /// ```js
    /// r.table("users").filter(
    ///     user -> user.g("birthdate").date().eq(r.now().inTimezone("-08:00").date())
    /// ).run(conn);
    /// ```
    /// 
    /// [now]: /api/java/now/
    /// [itz]: /api/java/in_timezone/

    pub fn date(&self) -> Client {
        cmd("date")
    }


    /// Return the number of seconds elapsed since the beginning of the day stored in the time object
///
/// 
///
/// 
                /// 
    /// __Example:__ Retrieve posts that were submitted before noon.
    /// 
    /// ```java
    /// r.table("posts").filter(
    ///     post -> post.g("date").timeOfDay().le(12*60*60)
    /// ).run(conn);
    /// ```

    pub fn time_of_day(&self) -> Client {
        cmd("time_of_day")
    }


    /// Return the year of a time object
///
/// 
///
/// 
                /// 
    /// __Example:__ Retrieve all the users born in 1986.
    /// 
    /// ```java
    /// r.table("users").filter(
    ///     user -> user.g("birthdate").year().eq(1986)
    /// }).run(conn);
    /// ```

    pub fn year(&self) -> Client {
        cmd("year")
    }


    /// Return the month of a time object as a number between 1 and 12
///
/// 
///
/// For your convenience, the terms `r.january`, `r.february`, etc. are defined and map to the appropriate integer.
                /// 
    /// __Example:__ Retrieve all the users who were born in November.
    /// 
    /// ```java
    /// r.table("users").filter(row -> row.g("birthdate").month().eq(11)).run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ Retrieve all the users who were born in September.
    /// 
    /// ```java
    /// r.table("users").filter(
    ///     row -> row.g("birthdate").month().eq(r.september())
    /// ).run(conn);
    /// ```
    /// 

    pub fn month(&self) -> Client {
        cmd("month")
    }


    /// Return the day of a time object as a number between 1 and 31
///
/// 
///
/// 
                /// 
    /// __Example:__ Return the users born on the 24th of any month.
    /// 
    /// ```java
    /// r.table("users").filter(
    ///     row -> row.g("birthdate").day().eq(24)
    /// ).run(conn);
    /// ```
    /// 
    /// 

    pub fn day(&self) -> Client {
        cmd("day")
    }


    /// Return the day of week of a time object as a number between 1 and 7 (following ISO 8601 standard)
///
/// 
///
/// For your convenience, the terms r.monday, r.tuesday, etc. are defined and map to the appropriate integer.
                /// 
    /// __Example:__ Return today's day of week.
    /// 
    /// ```java
    /// r.now().dayOfWeek().run(conn);
    /// ```
    /// 
    /// __Example:__ Retrieve all the users who were born on a Tuesday.
    /// 
    /// ```java
    /// r.table("users").filter(
    ///     row -> row.g("birthdate").dayOfWeek().eq(r.tuesday())
    /// ).run(conn);
    /// ```
    /// 

    pub fn day_of_week(&self) -> Client {
        cmd("day_of_week")
    }


    /// Return the day of the year of a time object as a number between 1 and 366 (following ISO 8601 standard)
///
/// 
///
/// 
                /// 
    /// __Example:__ Retrieve all the users who were born the first day of a year.
    /// 
    /// ```java
    /// r.table("users").filter(
    ///     row -> row.g("birthdate").dayOfYear().eq(1)
    /// ).run(conn);
    /// ```
    /// 
    /// 

    pub fn day_of_year(&self) -> Client {
        cmd("day_of_year")
    }


    /// Return the hour in a time object as a number between 0 and 23
///
/// 
///
/// 
                /// 
    /// __Example:__ Return all the posts submitted after midnight and before 4am.
    /// 
    /// ```java
    /// r.table("posts").filter(post -> post.g("date").hours().lt(4)).run(conn);
    /// ```
    /// 

    pub fn hours(&self) -> Client {
        cmd("hours")
    }


    /// Return the minute in a time object as a number between 0 and 59
///
/// 
///
/// 
                /// 
    /// __Example:__ Return all the posts submitted during the first 10 minutes of every hour.
    /// 
    /// ```java
    /// r.table("posts").filter(post -> post.g("date").minutes().lt(10)).run(conn);
    /// ```
    /// 
    /// 

    pub fn minutes(&self) -> Client {
        cmd("minutes")
    }


    /// Return the seconds in a time object as a number between 0 and 59
///
/// 
///
/// 999 (double precision).
                /// 
    /// __Example:__ Return the post submitted during the first 30 seconds of every minute.
    /// 
    /// ```java
    /// r.table("posts").filter(post -> post.g("date").seconds().lt(30)).run(conn);
    /// ```
    /// 

    pub fn seconds(&self) -> Client {
        cmd("seconds")
    }


    /// Convert a time object to a string in ISO 8601 format
///
/// 
///
/// 
                /// 
    /// __Example:__ Return the current ISO 8601 time.
    /// 
    /// ```java
    /// r.now().toIso8601().run(conn);
    /// 
    /// // Result:
    /// "2015-04-20T18:37:52.690+00:00"
    /// ```
    /// 

    pub fn to_iso8601<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("to_iso8601", args)
    }


    /// Convert a time object to its epoch time
///
/// 
///
/// 
                /// 
    /// __Example:__ Return the current time in seconds since the Unix Epoch with millisecond-precision.
    /// 
    /// ```java
    /// r.now().toEpochTime().run(conn);
    /// ```
    /// 
    /// 

    pub fn to_epoch_time(&self) -> Client {
        cmd("to_epoch_time")
    }


    /// Take one or more values as arguments and return an array
///
/// 
///
/// (Technically, return a [List][] object.)
                /// 
    /// [List]: https://docs.oracle.com/javase/8/docs/api/java/util/List.html
    /// 
    /// __Example:__ Create an array.
    /// 
    /// ```java
    /// r.expr(r.array(10, 20, 30)).run(conn);
    /// ```
    /// 
    /// This is a ReQL equivalent to:
    /// 
    /// ```java
    /// List<Integer> myArray = Arrays.asList(10, 20, 30);
    /// ```

    pub fn array<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("array", args)
    }


    /// Take a key/value pair, with extra key/value pairs optionally specified by chaining one or more `with(key, value)` terms after `hashMap`, and return an object
///
/// 
///
/// 
                /// 
    /// `hashMap` is a convenience provided by the RethinkDB Java driver, and is not actually a ReQL term. It returns a `MapObject`, a RethinkDB-provided class that inherits from `Map<Object,Object>`. You can use `hashMap` outside the context of a ReQL query.
    /// 
    /// 
    /// __Example:__ Create a hashmap.
    /// 
    /// ```java
    /// import com.rethinkdb.model.MapObject;
    /// 
    /// MapObject newData = r.hashMap("user", "fred")
    ///     .with("email", "fred@example.com")
    ///     .with("id", 101)
    ///     .with("admin", true);
    /// ```
    /// 
    /// This creates the object (in JSON):
    /// 
    /// ```json
    /// {
    ///     "admin": true,
    ///     "email": "fred@example.com",
    ///     "id": 101,
    ///     "user": "fred"
    /// }
    /// ```

    pub fn hashmap<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("hashmap", args)
    }


    /// `r
///
/// 
///
/// args` is a special term that's used to splice an array of arguments
                /// into another term.  This is useful when you want to call a variadic
    /// term such as [getAll](/api/java/get_all/) with a set of arguments produced at runtime.
    /// 
    /// Note that `args` evaluates all its arguments before passing them into the parent term, even if the parent term otherwise allows lazy evaluation.
    /// 
    /// __Example:__ Get Alice and Bob from the table `people`.
    /// 
    /// ```java
    /// r.table("people").getAll("Alice", "Bob").run(conn);
    /// // or
    /// r.table("people").getAll(r.args(r.array("Alice", "Bob"))).run(conn);
    /// ```
    /// 
    /// __Example:__ Get all of Alice's children from the table `people`.
    /// 
    /// ```java
    /// // r.table("people").get("Alice") returns (in JSON)
    /// // { "id": "Alice", "children": ["Bob, "Carol"] }
    /// r.table("people").getAll(r.args(r.table("people").get("Alice").g("children"))).run(conn);
    /// ```

    pub fn args<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("args", args)
    }


    /// Encapsulate binary data within a query
///
/// 
///
/// 
                /// 
    /// The type of data `binary` accepts depends on the client language. In Java, it expects a parameter of `byte[]` type (or ReQL queries that return binary data).
    /// 
    /// Binary objects returned to the client in Java will also be `byte[]` types. This can be changed with the `binary_format` [optArg](/api/java/optarg) provided to [run](/api/java/run) to return "raw" objects.
    /// 
    /// Only a limited subset of ReQL commands may be chained after `binary`:
    /// 
    /// * [coerceTo](/api/java/coerce_to/) can coerce `binary` objects to `string` types
    /// * [count](/api/java/count/) will return the number of bytes in the object
    /// * [slice](/api/java/slice/) will treat bytes like array indexes (i.e., `slice(10,20)` will return bytes 10&ndash;19)
    /// * [typeOf](/api/java/type_of) returns `PTYPE<BINARY>`
    /// * [info](/api/java/info) will return information on a binary object.
    /// 
    /// __Example:__ Save an avatar image to a existing user record.
    /// 
    /// ```java
    /// import java.nio.file.*;
    /// 
    /// Path path = Paths.get("./defaultAvatar.png");
    /// byte[] avatarImage = Files.readAllBytes(path);
    /// r.table("users").get(100).update(r.hashMap("avatar", avatarImage));
    /// ```
    /// 
    /// __Example:__ Get the size of an existing avatar image.
    /// 
    /// ```java
    /// r.table("users").get(100)("avatar").count().run(conn);
    /// 
    /// // Result:
    /// 14156
    /// ```
    /// 
    /// Read more details about RethinkDB's binary object support: [Storing binary objects](/docs/storing-binary/).

    pub fn binary<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("binary", args)
    }


    /// Call an anonymous function using return values from other ReQL commands or queries as arguments
///
/// 
///
/// 
                /// 
    /// The last argument to `do_` (or, in some forms, the only argument) is an expression or an anonymous function which receives values from either the previous arguments or from prefixed commands chained before `do_`. The `do_` command is essentially a single-element [map](/api/java/map/), letting you map a function over just one document. This allows you to bind a query result to a local variable within the scope of `do_`, letting you compute the result just once and reuse it in a complex expression or in a series of ReQL commands.
    /// 
    /// Arguments passed to the `do_` function must be basic data types, and cannot be streams or selections. (Read about [ReQL data types](/docs/data-types/).) While the arguments will all be evaluated before the function is executed, they may be evaluated in any order, so their values should not be dependent on one another. The type of `do_`'s result is the type of the value returned from the function or last expression.
    /// 
    /// __Example:__ Compute a golfer's net score for a game.
    /// 
    /// ```java
    /// r.table("players").get("86be93eb-a112-48f5-a829-15b2cb49de1d").do_(
    ///     player -> player.g("gross_score").sub(player.g("course_handicap"))
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Return the best scoring player in a two-player golf match.
    /// 
    /// ```java
    /// r.do_(r.table("players").get(id1), r.table("players").get(id2),
    ///     (player1, player2) -> r.branch(
    ///         player1.g("gross_score").lt(player2.g("gross_score")),
    ///         player1,
    ///         player2
    ///     )
    /// ).run(conn);
    /// 
    /// ```
    /// 
    /// Note that `branch`, the ReQL conditional command, must be used instead of `if`. See the `branch` [documentation](/api/java/branch) for more.
    /// 
    /// __Example:__ Take different actions based on the result of a ReQL [insert](/api/java/insert) command.
    /// 
    /// ```java
    /// import com.rethinkdb.model.MapObject;
    /// 
    /// MapObject newData = r.hashMap("id", 100)
    ///     .with("name", "Agatha")
    ///     .with("gross_score", 57)
    ///     .with("course_handicap", 4);
    /// 
    /// r.table("players").insert(newData).do_(doc ->
    ///     r.branch(doc.g("inserted").ne(0),
    ///         r.table("log").insert(
    ///             r.hashMap("time", r.now())
    ///                .with("response", doc)
    ///                .with("result", "ok")),
    ///         r.table("log").insert(
    ///             r.hashMap("time", r.now())
    ///                .with("response", doc)
    ///                .with("result", "error"))
    ///     )
    /// ).run(conn);
    /// ```

    pub fn do_<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("do_", args)
    }


    /// Perform a branching conditional equivalent to `if-then-else`
///
/// 
///
/// 
                /// 
    /// The `branch` command takes 2n+1 arguments: pairs of conditional expressions and commands to be executed if the conditionals return any value but `false` or `null` (i.e., "truthy" values), with a final "else" command to be evaluated if all of the conditionals are `false` or `null`.
    /// 
    /// <!-- break -->
    /// 
    /// You may call `branch` infix style on the first test. (See the second example for an illustration.)
    /// 
    /// ```
    /// r.branch(test1, val1, test2, val2, elseval)
    /// ```
    /// 
    /// is the equivalent of the Java statement
    /// 
    /// ```java
    /// if (test1) {
    ///     return val1;
    /// } else if (test2) {
    ///     return val2;
    /// } else {
    ///     return elseval;
    /// }
    /// ```
    /// 
    /// __Example:__ Test the value of x.
    /// 
    /// ```java
    /// int x = 10;
    /// r.branch(r.expr(x).gt(5), "big", "small").run(conn);
    /// 
    /// // Result:
    /// "big"
    /// ```
    /// 
    /// __Example:__ As above, infix-style.
    /// 
    /// ```java
    /// int x = 10;
    /// r.expr(x).gt(5).branch("big", "small").run(conn);
    /// 
    /// // Result:
    /// "big"
    /// ```
    /// 
    /// __Example:__ Categorize heroes by victory counts.
    /// 
    /// ```java
    /// r.table("marvel").map(hero -> r.branch(
    ///     hero.g("victories").gt(100),
    ///     hero.g("name").add(" is a superhero"),
    ///     hero.g("victories").gt(10),
    ///     hero.g("name").add(" is a hero"),
    ///     hero.g("name").add(" is very nice")
    /// )).run(conn);
    /// 
    /// ```
    /// 
    /// If the documents in the table `marvel` are:
    /// 
    /// ```json
    /// [
    ///     { "name": "Iron Man", "victories": 214 },
    ///     { "name": "Jubilee", "victories": 49 },
    ///     { "name": "Slava", "victories": 5 }
    /// ]
    /// ```
    /// 
    /// The results will be:
    /// 
    /// ```json
    /// [
    ///     "Iron Man is a superhero",
    ///     "Jubilee is a hero",
    ///     "Slava is very nice"
    /// ]
    /// ```

    pub fn branch<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("branch", args)
    }


    /// Loop over a sequence, evaluating the given write query for each element
///
/// 
///
/// 
                /// 
    /// __Example:__ Now that our heroes have defeated their villains, we can safely remove them from the villain table.
    /// 
    /// ```java
    /// r.table("marvel").forEach(
    ///     hero -> r.table("villains").get(hero.g("villainDefeated")).delete()
    /// ).run(conn);
    /// ```

    pub fn for_each<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("for_each", args)
    }


    /// Generate a stream of sequential integers in a specified range
///
/// 
///
/// 
                /// 
    /// `range` takes 0, 1 or 2 arguments:
    /// 
    /// * With no arguments, `range` returns an "infinite" stream from 0 up to and including the maximum integer value;
    /// * With one argument, `range` returns a stream from 0 up to but not including the end value;
    /// * With two arguments, `range` returns a stream from the start value up to but not including the end value.
    /// 
    /// Note that the left bound (including the implied left bound of 0 in the 0- and 1-argument form) is always closed and the right bound is always open: the start value will always be included in the returned range and the end value will *not* be included in the returned range.
    /// 
    /// Any specified arguments must be integers, or a `ReqlRuntimeError` will be thrown. If the start value is equal or to higher than the end value, no error will be thrown but a zero-element stream will be returned.
    /// 
    /// __Example:__ Return a four-element range of `[0, 1, 2, 3]`.
    /// 
    /// ```java
    /// r.range(4).run(conn);
    /// ```
    /// 
    /// Result (shown as JSON):
    /// 
    /// ```json
    /// [0, 1, 2, 3]
    /// ```
    /// 
    /// <!-- stop -->
    /// 
    /// You can also use the [limit](/api/java/limit) command with the no-argument variant to achieve the same result in this case:
    /// 
    /// ```java
    /// r.range().limit(4).run(conn);
    /// ```
    /// 
    /// __Example:__ Return a range from -5 through 5.
    /// 
    /// ```java
    /// r.range(-5, 6).run(conn);
    /// ```
    /// 
    /// ```json
    /// [-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5]
    /// ```

    pub fn range(&self) -> Client {
        cmd("range")
    }


    /// Throw a runtime error
///
/// 
///
/// If called with no arguments inside the second argument to `default`, re-throw the current error.
                /// 
    /// __Example:__ Iron Man can't possibly have lost a battle:
    /// 
    /// ```java
    /// r.table("marvel").get("IronMan").do_(
    ///     ironman -> r.branch(
    ///         ironman.g("victories").lt(ironman.g("battles")),
    ///         r.error("impossible code path"),
    ///         ironman
    ///     )
    /// ).run(conn);
    /// ```

    pub fn error<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("error", args)
    }


    /// Provide a default value in case of non-existence errors
///
/// 
///
/// The `default` command evaluates its first argument (the value it's chained to). If that argument returns `null` or a non-existence error is thrown in evaluation, then `default` returns its second argument. The second argument is usually a default value, but it can be a function that returns a value.
                /// 
    /// __Example:__ Suppose we want to retrieve the titles and authors of the table `posts`.
    /// In the case where the author field is missing or `null`, we want to retrieve the string
    /// `Anonymous`.
    /// 
    /// ```java
    /// r.table("posts").map(post ->
    ///     r.hashMap("title", post.g("title"))
    ///         .with("author", post.g("author").default_("Anonymous"))
    /// ).run(conn);
    /// ```
    /// 
    /// <!-- stop -->
    /// 
    /// We can rewrite the previous query with `r.branch` too.
    /// 
    /// ```java
    /// r.table("posts").map(post ->
    ///     r.branch(
    ///         post.hasFields("author"),
    ///         r.hashMap("title", post.g("title"))
    ///             .with("author", post.g("author")),
    ///         r.hashMap("title", post.g("title"))
    ///             .with("author", "Anonymous")
    ///     )
    /// ).run(conn);
    /// ```
    /// 
    /// 
    /// __Example:__ The `default` command can also be used to filter documents. Suppose we want to retrieve all our users who are not grown-ups or whose age is unknown (i.e., the field `age` is missing or equals `null`). We can do it with this query:
    /// 
    /// ```java
    /// r.table("users").filter(
    ///     user -> user.g("age").lt(18).default_(true)
    /// ).run(conn);
    /// ```
    /// 
    /// One more way to write the previous query is to set the age to be `-1` when the
    /// field is missing.
    /// 
    /// ```java
    /// r.table("users").filter(
    ///     user -> user.g("age").default_(-1).lt(18)
    /// ).run(conn);
    /// ```
    /// 
    /// Another way to do the same query is to use [hasFields](/api/java/has_fields/).
    /// 
    /// ```java
    /// r.table("users").filter(
    ///     user -> user.hasFields("age").not().or(user.g("age").lt(18))
    /// ).run(conn);
    /// ```
    /// 
    /// The body of every [filter](/api/java/filter/) is wrapped in an implicit `.default_(false)`. You can overwrite
    /// the value `false` with the `default` [optArg](/api/java/optarg) to `filter`, so the previous query can also be
    /// written like this.
    /// 
    /// ```java
    /// r.table("users").filter(
    ///     user -> user.g("age").lt(18).default_(true)
    /// ).optArg("default", true).run(conn);
    /// 
    /// ```
    /// 

    pub fn default<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("default", args)
    }


    /// Construct a ReQL JSON object from a native object
///
/// 
///
/// 
                /// 
    /// The native object can be any Java primitive type, as well as Array, List, Map, LocalDateTime, ZonedDateTime, OffsetDateTime, and POJOs ("plain old Java objects") whose classes are public and whose numeric properties are `Long` instead of `Integer`.
    /// 
    /// If the native object is of type `bytes[]`, then `expr` will return a binary object. See [binary](/api/java/binary) for more information.
    /// 
    /// __Example:__ Objects wrapped with `expr` can then be manipulated by ReQL API functions.
    /// 
    /// ```java
    /// import com.rethinkdb.model.MapObject;
    /// 
    /// // Create object { "a": "b" }
    /// MapObject newData = r.hashMap("a", "b");
    /// 
    /// // merge with { "b": [1, 2, 3] }
    /// r.expr(newData).merge(r.hashMap("b", r.array(1, 2, 3))).run(conn);
    /// ```

    pub fn expr<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("expr", args)
    }


    /// Create a JavaScript expression
///
/// 
///
/// 
                /// 
    /// You may use the `timeout` [optArg](/api/java/optarg) to specify a number of seconds before `r.js` times out. The default value is 5 seconds.
    /// 
    /// {% infobox %}
    /// Whenever possible, you should use native ReQL commands rather than `r.js` for better performance.
    /// {% endinfobox %}
    /// 
    /// __Example:__ Concatenate two strings using JavaScript.
    /// 
    /// ```java
    /// r.js("'str1' + 'str2'").run(conn);
    /// ```
    /// 
    /// __Example:__ Select all documents where the 'magazines' field is greater than 5 by running JavaScript on the server.
    /// 
    /// ```java
    /// r.table("marvel").filter(
    ///     r.js('(function (row) { return row.magazines.length > 5; })')
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ You may also specify a timeout in seconds (defaults to 5).
    /// 
    /// ```java
    /// r.js("while (true) {}").optArg("timeout", 1.3).run(conn);
    /// ```
    /// 

    pub fn js<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("js", args)
    }


    /// Convert a value of one type into another
///
/// 
///
/// 
                /// 
    /// * a sequence, selection or object can be coerced to an array
    /// * a sequence, selection or an array of key-value pairs can be coerced to an object
    /// * a string can be coerced to a number
    /// * any datum (single value) can be coerced to to a string
    /// * a binary object can be coerced to a string and vice-versa
    /// 
    /// __Example:__ Coerce a stream to an array to store its output in a field. (A stream cannot be stored in a field directly.)
    /// 
    /// ```java
    /// r.table("posts").map(post -> post.merge(
    ///     r.hashMap("comments",
    ///               r.table("comments").getAll(post.g("id")).optArg("index", "post_id")
    ///               .coerceTo("array"))
    /// )).run(conn);
    /// ```
    /// 
    /// __Example:__ Coerce an array of key-value pairs into an object.
    /// 
    /// 
    /// ```java
    /// r.expr(r.array(r.array("name", "Ironman"), r.array("victories", 2000)))
    ///  .coerceTo("object").run(conn);
    /// ```
    /// 
    /// __Note:__ To coerce a list of key-value pairs like `["name", "Ironman", "victories", 2000]` to an object, use the [object](/api/java/object) command.
    /// 
    /// __Example:__ Coerce a number to a string.
    /// 
    /// ```java
    /// r.expr(1).coerceTo("string").run(conn);
    /// ```

    pub fn coerce_to<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("coerce_to", args)
    }


    /// Gets the type of a ReQL query's return value
///
/// 
///
/// 
                /// 
    /// The type will be returned as a string:
    /// 
    /// * `ARRAY`
    /// * `BOOL`
    /// * `DB`
    /// * `FUNCTION`
    /// * `GROUPED_DATA`
    /// * `GROUPED_STREAM`
    /// * `MAXVAL`
    /// * `MINVAL`
    /// * `NULL`
    /// * `NUMBER`
    /// * `OBJECT`
    /// * `PTYPE<BINARY>`
    /// * `PTYPE<GEOMETRY>`
    /// * `PTYPE<TIME>`
    /// * `SELECTION<ARRAY>`
    /// * `SELECTION<OBJECT>`
    /// * `SELECTION<STREAM>`
    /// * `STREAM`
    /// * `STRING`
    /// * `TABLE_SLICE`
    /// * `TABLE`
    /// 
    /// Read the article on [ReQL data types](/docs/data-types/) for a more detailed discussion. Note that some possible return values from `typeOf` are internal values, such as `MAXVAL`, and unlikely to be returned from queries in standard practice.
    /// 
    /// __Example:__ Get the type of a string.
    /// 
    /// ```java
    /// r.expr("foo").typeOf().run(conn);
    /// // result: "STRING"
    /// ```

    pub fn type_of(&self) -> Client {
        cmd("type_of")
    }


    /// Get information about a ReQL value
///
/// 
///
/// 
                /// 
    /// __Example:__ Get information about a table such as primary key, or cache size.
    /// 
    /// ```java
    /// r.table("marvel").info().run(conn);
    /// ```

    pub fn info(&self) -> Client {
        cmd("info")
    }


    /// Parse a JSON string on the server
///
/// 
///
/// 
                /// 
    /// __Example:__ Send an array to the server.
    /// 
    /// ```java
    /// r.json("[1,2,3]").run(conn);
    /// ```

    pub fn json<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("json", args)
    }


    /// Convert a ReQL value or object to a JSON string
///
/// 
///
/// You may use either `toJsonString` or `toJson`.
                /// 
    /// __Example:__ Get a ReQL document as a JSON string.
    /// 
    /// ```java
    /// r.table("hero").get(1).toJson().run(conn)
    /// ```
    /// 
    /// Returned data:
    /// 
    /// ```json
    /// '{"id": 1, "name": "Batman", "city": "Gotham", "powers": ["martial arts", "cinematic entrances"]}'
    /// ```

    pub fn to_json<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("to_json", args)
    }


    /// Retrieve data from the specified URL over HTTP
///
/// 
///
/// The return type depends on the `resultFormat` option, which checks the `Content-Type` of the response by default.
                /// 
    /// __Example:__ Perform an HTTP `GET` and store the result in a table.
    /// 
    /// ```java
    /// r.table("posts").insert(r.http("http://httpbin.org/get")).run(conn);
    /// ```
    /// 
    /// <!-- stop -->
    /// 
    /// See [the tutorial](/docs/external-api-access/) on `r.http` for more examples on how to use this command.
    /// 
    /// # Options #
    /// 
    /// These options are specified with the [optArg](/api/java/optarg) command.
    /// 
    /// ## General Options ##
    /// 
    /// * `timeout`: timeout period in seconds to wait before aborting the connect (default `30`).
    /// * `attempts`: number of retry attempts to make after failed connections (default `5`).
    /// * `redirects`: number of redirect and location headers to follow (default `1`).
    /// * `verify`: if `true`, verify the server's SSL certificate (default `true`).
    /// * `resultFormat`: string specifying the format to return results in. One of the following:
    ///     * `text`: always return a string.
    ///     * `json`: parse the result as JSON, raising an error on failure.
    ///     * `jsonp`: parse the result as [Padded JSON][jsonp].
    ///     * `binary`: return a binary object.
    ///     * `auto`: parse the result based on its `Content-Type` (the default):
    ///         * `application/json`: as `json`
    ///         * `application/json-p`, `text/json-p`, `text/javascript`: as `jsonp`
    ///         * `audio/*`, `video/*`, `image/*`, `application/octet-stream`: as `binary`
    ///         * anything else: as `text`
    /// 
    /// [jsonp]: https://en.wikipedia.org/wiki/JSONP
    /// 
    /// ## Request Options
    /// 
    /// * `method`: HTTP method to use for the request. One of `GET`, `POST`, `PUT`, `PATCH`, `DELETE` or `HEAD`. Default: `GET`.
    /// * `auth`: object giving authentication, with the following fields:
    ///     * `type`: `basic` (default) or `digest`
    ///     * `user`: username
    ///     * `pass`: password in plain text
    /// * `params`: hashMap or object specifying URL parameters to append to the URL as encoded key/value pairs. `{ "query": "banana", "limit": 2 }` will be appended as `?query=banana&limit=2`. Default: no parameters.
    /// * `header`: Extra header lines to include. The value may be an array of strings or an object. Default: `Accept-Encoding: deflate;q=1, gzip;q=0.5` and `User-Agent: RethinkDB/<VERSION>`.
    /// * `data`: Data to send to the server on a `POST`, `PUT`, `PATCH`, or `DELETE` request. For `POST` requests, data may be either an object (which will be written to the body as form-encoded key/value pairs) or a string; for all other requests, data will be serialized as JSON and placed in the request body, sent as `Content-Type: application/json`. Default: no data will be sent.
    /// 
    /// __Example:__ Perform multiple requests with different parameters.
    /// 
    /// ```java
    /// r.expr(r.array(1, 2, 3)).map(
    ///     i -> r.http("http://httpbin.org/get")
    ///           .optArg("params", r.hashMap("user", i))
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Perform a `PUT` request for each item in a table.
    /// 
    /// ```java
    /// r.table("data").map(
    ///     row -> r.http("http://httpbin.org/put")
    ///             .optArg("method", "PUT")
    ///             .optArg("data", row)
    /// ).run(conn);
    /// ```
    /// 
    /// __Example:__ Perform a `POST` request with accompanying data.
    /// 
    /// Using form-encoded data:
    /// 
    /// ```java
    /// r.http("http://httpbin.org/post").optArg("method", "POST")
    ///  .optArg("data", r.hashMap("player", "Bob").with("game", "tic tac toe"))
    ///  .run(conn);
    /// ```
    /// 
    /// Using JSON data:
    /// 
    /// ```java
    /// r.http("http://httpbin.org/post").optArg("method", "POST")
    ///  .optArg("data", r.expr(value).coerceTo("string"))
    ///  .optArg("header", r.hashMap("Content-Type", "application/json"))
    ///  .run(conn);
    /// ```
    /// 
    /// ## Pagination
    /// 
    /// `r.http` supports depagination, which will request multiple pages in a row and aggregate the results into a stream.  The use of this feature is controlled by the [optArgs](/api/java/optarg) `page` and `page_limit`.  Either none or both of these arguments must be provided.
    /// 
    /// * `page`: This option may specify either a built-in pagination strategy (see below), or a function to provide the next URL and/or `params` to request.
    /// * `page_limit`: An integer specifying the maximum number of requests to issue using the `page` functionality.  This is to prevent overuse of API quotas, and must be specified with `page`.
    ///     * `-1`: no limit
    ///     * `0`: no requests will be made, an empty stream will be returned
    ///     * `n`: `n` requests will be made
    /// 
    /// At the moment, the only built-in strategy is `link-next`, which is equivalent to `info -> info.g("header").g("link").g("rel='next'").default_(null)`.
    /// 
    /// __Example:__ Perform a GitHub search and collect up to 3 pages of results.
    /// 
    /// ```java
    /// r.http("https://api.github.com/search/code?q=addClass+user:mozilla")
    ///  .optArg("page", "link-next").optArg("page_limit", 3)
    ///  .run(conn);
    /// ```
    /// 
    /// As a function, `page` takes one parameter, an object of the format:
    /// 
    /// ```js
    /// {
    ///     "params": object,  // the URL parameters used in the last request
    ///     "header": object,  // the headers of the last response as key/value pairs
    ///     "body": value      // the body of the last response in the format
    /// }                      //   specified by `resultFormat`
    /// ```
    /// 
    /// The `header` field will be a parsed version of the header with fields lowercased, like so:
    /// 
    /// ```json
    /// {
    ///     "content-length": "1024",
    ///     "content-type": "application/json",
    ///     "date": "Thu, 1 Jan 1970 00:00:00 GMT",
    ///     "link": {
    ///         "rel=\"last\"": "http://example.com/?page=34",
    ///         "rel=\"next\"": "http://example.com/?page=2"
    ///     }
    /// }
    /// ```
    /// 
    /// The `page` function may return a string corresponding to the next URL to request, `null` indicating that there is no more to get, or an object of the format:
    /// 
    /// ```js
    /// {
    ///     "url": string,    // the next URL to request, or null for no more pages
    ///     "params": object  // new URL parameters to use, will be merged with the
    /// }                     //   previous request's params
    /// ```
    /// 
    /// __Example:__ Perform depagination with a custom `page` function.
    /// 
    /// ```java
    /// r.http("example.com/pages")
    ///  .optArg("page", info -> info.g("body").g("meta").g("next").default_(null))
    ///  .optArg("page_limit", 5)
    ///  .run(conn);
    /// ```
    /// 
    /// # Learn more
    /// 
    /// See [the tutorial](/docs/external-api-access/) on `r.http` for more examples on how to use this command.

    pub fn http<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("http", args)
    }


    /// Return a UUID (universally unique identifier), a string that can be used as a unique ID
///
/// 
///
/// If a string is passed to `uuid` as an argument, the UUID will be deterministic, derived from the string's SHA-1 hash.
                /// 
    /// RethinkDB's UUIDs are standards-compliant. Without the optional argument, a version 4 random UUID will be generated; with that argument, a version 5 UUID will be generated, using a fixed namespace UUID of `91461c99-f89d-49d2-af96-d8e2e14e9b58`. For more information, read [Wikipedia's UUID article][uu].
    /// 
    /// [uu]: https://en.wikipedia.org/wiki/Universally_unique_identifier
    /// 
    /// __Example:__ Generate a UUID.
    /// 
    /// ```java
    /// r.uuid().run(conn);
    /// // returns "27961a0e-f4e8-4eb3-bf95-c5203e1d87b9"
    /// ```
    /// 
    /// __Example:__ Generate a UUID based on a string.
    /// 
    /// ```java
    /// r.uuid("slava@example.com").run(conn);
    /// // returns "90691cbc-b5ea-5826-ae98-951e30fc3b2d"
    /// ```

    pub fn uuid(&self) -> Client {
        cmd("uuid")
    }


    /// Construct a circular line or polygon
///
/// 
///
/// A circle in RethinkDB is a polygon or line *approximating* a circle of a given radius around a given center, consisting of a specified number of vertices (default 32).
                /// 
    /// The center may be specified either by two floating point numbers, the latitude (&minus;90 to 90) and longitude (&minus;180 to 180) of the point on a perfect sphere (see [Geospatial support](/docs/geo-support/) for more information on ReQL's coordinate system), or by a point object. The radius is a floating point number whose units are meters by default, although that may be changed with the `unit` argument.
    /// 
    /// Optional arguments that can be specified with [optArg](/api/java/optarg) are:
    /// 
    /// * `num_vertices`: the number of vertices in the polygon or line. Defaults to 32.
    /// * `geo_system`: the reference ellipsoid to use for geographic coordinates. Possible values are `WGS84` (the default), a common standard for Earth's geometry, or `unit_sphere`, a perfect sphere of 1 meter radius.
    /// * `unit`: Unit for the radius distance. Possible values are `m` (meter, the default), `km` (kilometer), `mi` (international mile), `nm` (nautical mile), `ft` (international foot).
    /// * `fill`: if `true` (the default) the circle is filled, creating a polygon; if `false` the circle is unfilled (creating a line).
    /// 
    /// 
    /// 
    /// __Example:__ Define a circle.
    /// 
    /// ```java
    /// r.table("geo").insert(
    ///     r.hashMap("id", 300)
    ///      .with("name", "Hayes Valley")
    ///      .with("neighborhood", r.circle(r.array(-122.423246, 37.779388), 1000))
    /// ).run(conn);
    /// ```

    pub fn circle<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("circle", args)
    }


    /// Compute the distance between a point and another geometry object
///
/// 
///
/// At least one of the geometry objects specified must be a point.
                /// 
    /// Optional arguments available with `distance` are:
    /// 
    /// * `geo_system`: the reference ellipsoid to use for geographic coordinates. Possible values are `WGS84` (the default), a common standard for Earth's geometry, or `unit_sphere`, a perfect sphere of 1 meter radius.
    /// * `unit`: Unit to return the distance in. Possible values are `m` (meter, the default), `km` (kilometer), `mi` (international mile), `nm` (nautical mile), `ft` (international foot).
    /// 
    /// If one of the objects is a polygon or a line, the point will be projected onto the line or polygon assuming a perfect sphere model before the distance is computed (using the model specified with `geo_system`). As a consequence, if the polygon or line is extremely large compared to Earth's radius and the distance is being computed with the default WGS84 model, the results of `distance` should be considered approximate due to the deviation between the ellipsoid and spherical models.
    /// 
    /// 
    /// __Example:__ Compute the distance between two points on the Earth in kilometers.
    /// 
    /// ```java
    /// r.distance(
    ///     r.point(-122.423246,37.779388),
    ///     r.point(-117.220406,32.719464)
    /// ).optArg("unit", "km").run(conn);
    /// 
    /// // Result:
    /// 734.1252496021841
    /// ```

    pub fn distance<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("distance", args)
    }


    /// Convert a Line object into a Polygon object
///
/// 
///
/// If the last point does not specify the same coordinates as the first point, `polygon` will close the polygon by connecting them.
                /// 
    /// Longitude (&minus;180 to 180) and latitude (&minus;90 to 90) of vertices are plotted on a perfect sphere. See [Geospatial support](/docs/geo-support/) for more information on ReQL's coordinate system.
    /// 
    /// If the last point does not specify the same coordinates as the first point, `polygon` will close the polygon by connecting them. You cannot directly construct a polygon with holes in it using `polygon`, but you can use [polygonSub](/api/java/polygon_sub) to use a second polygon within the interior of the first to define a hole.
    /// 
    /// 
    /// __Example:__ Create a line object and then convert it to a polygon.
    /// 
    /// ```java
    /// r.table("geo").insert(
    ///     r.hashMap("id", 201)
    ///      .with("rectangle", r.line(
    ///         r.array(-122.423246,37.779388),
    ///         r.array(-122.423246,37.329898),
    ///         r.array(-121.886420,37.329898),
    ///         r.array(-121.886420,37.779388)))
    /// ).run(conn);
    /// 
    /// r.table("geo").get(201).update(
    ///     r.hashMap("rectangle", row -> row.g("rectangle").fill())
    /// ).optArg("non_atomic", true).run(conn);
    /// ```

    pub fn fill(&self) -> Client {
        cmd("fill")
    }


    /// Convert a [GeoJSON](http://geojson
///
/// 
///
/// org) object to a ReQL geometry object.
                /// 
    /// RethinkDB only allows conversion of GeoJSON objects which have ReQL equivalents: `Point`, `LineString`, and `Polygon`. `MultiPoint`, `MultiLineString`, and `MultiPolygon` are not supported. (You could, however, store multiple points, lines and polygons in an array and use a geospatial multi index with them.)
    /// 
    /// Only longitude/latitude coordinates are supported. GeoJSON objects that use Cartesian coordinates, specify an altitude, or specify their own coordinate reference system will be rejected.
    /// 
    /// __Example:__ Convert a GeoJSON object to a ReQL geometry object.
    /// 
    /// ```java
    /// import com.rethinkdb.model.Geojson;
    /// 
    /// // GeoJSON object:
    /// //      {
    /// //          "type": "Point",
    /// //          "coordinates": [ -122.423246, 37.779388 ]
    /// //      }
    /// Geojson geo = r.hashMap("type, "Point")
    ///                .with("coordinates", r.array(-122.423246, 37.779388));
    /// 
    /// r.table("geo").insert(
    ///     r.hashMap("id", "sfo")
    ///      .with("name", "San Francisco")
    ///      .with("location", r.geojson(geo))
    /// ).run(conn);
    /// ```

    pub fn geojson<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("geojson", args)
    }


    /// Convert a ReQL geometry object to a [GeoJSON](http://geojson
///
/// 
///
/// org) object.
                /// 
    /// __Example:__ Convert a ReQL geometry object to a GeoJSON object.
    /// 
    /// ```java
    /// r.table("geo").get("sfo")("location").toGeojson().run(conn);
    /// 
    /// // Result:
    /// {
    ///     "type": "Point",
    ///     "coordinates": [ -122.423246, 37.779388 ]
    /// }
    /// ```

    pub fn to_geojson(&self) -> Client {
        cmd("to_geojson")
    }


    /// Get all documents where the given geometry object intersects the geometry object of the requested geospatial index
///
/// 
///
/// 
                /// 
    /// The `index` [optarg](/api/java/optarg) is mandatory. This command returns the same results as `row -> row.g(index).intersects(geometry)`. The total number of results is limited to the array size limit which defaults to 100,000, but can be changed with the `array_limit` option to [run](/api/java/run).
    /// 
    /// __Example:__ Which of the locations in a list of parks intersect `circle1`?
    /// 
    /// ```java
    /// import com.rethinkdb.gen.ast.Circle;
    /// 
    /// Circle circle1 = r.circle(r.array(-117.220406, 32.719464), 10)
    ///                   .optArg("unit", "mi");
    /// 
    /// r.table("parks").getIntersecting(circle1).optArg("index", "area").run(conn);
    /// ```

    pub fn get_intersecting<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("get_intersecting", args)
    }


    /// Return a list of documents closest to a specified point based on a geospatial index, sorted in order of increasing distance
///
/// 
///
/// 
                /// 
    /// The `index` [optArg](/api/java/optarg) is mandatory. Optional arguments are:
    /// 
    /// * `max_results`: the maximum number of results to return (default 100).
    /// * `unit`: Unit for the distance. Possible values are `m` (meter, the default), `km` (kilometer), `mi` (international mile), `nm` (nautical mile), `ft` (international foot).
    /// * `max_dist`: the maximum distance from an object to the specified point (default 100 km).
    /// * `geo_system`: the reference ellipsoid to use for geographic coordinates. Possible values are `WGS84` (the default), a common standard for Earth's geometry, or `unit_sphere`, a perfect sphere of 1 meter radius.
    /// 
    /// The return value will be an array of two-item objects with the keys `dist` and `doc`, set to the distance between the specified point and the document (in the units specified with `unit`, defaulting to meters) and the document itself, respectively. The array will be sorted by the values of `dist`.
    /// 
    /// __Example:__ Return a list of the closest 25 enemy hideouts to the secret base.
    /// 
    /// ```java
    /// import com.rethinkdb.gen.ast.Point;
    /// 
    /// Point secretBase = r.point(-122.422876,37.777128);
    /// 
    /// r.table("hideouts")
    ///  .getNearest(secretBase)
    ///  .optArg("index", "location")
    ///  .optArg("max_results", 25)
    ///  .run(conn);
    /// ```
    /// 
    /// <!-- stop -->
    /// 
    /// {% infobox %}
    /// If you wish to find all points within a certain radius of another point, it's often faster to use [getIntersecting][gi] with [circle][c], as long as the approximation of a circle that `circle` generates is sufficient.
    /// 
    /// [gi]: /api/java/get_intersecting/
    /// [c]:  /api/java/circle/
    /// {% endinfobox %}

    pub fn get_nearest<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("get_nearest", args)
    }


    /// Tests whether a geometry object is completely contained within another
///
/// 
///
/// When applied to a sequence of geometry objects, `includes` acts as a [filter](/api/java/filter), returning a sequence of objects from the sequence that include the argument.
                /// 
    /// 
    /// __Example:__ Is a point included within a 2000-meter circle?
    /// 
    /// ```java
    /// Object point1 = r.point(-117.220406,32.719464);
    /// Object point2 = r.point(-117.206201,32.725186);
    /// 
    /// r.circle(point1, 2000).includes(point2).run(conn);
    /// 
    /// // Result:
    /// true
    /// ```
    /// 
    /// __Example:__ Which of the locations in a list of parks include a given circle?
    /// 
    /// ```java
    /// import com.rethinkdb.gen.ast.Circle;
    /// 
    /// Circle circle1 = r.circle(r.array(-117.220406, 32.719464), 10)
    ///                   .optArg("unit", "mi");
    /// 
    /// r.table("parks").g("area").includes(circle1).run(conn);
    /// ```
    /// 
    /// {% infobox %}
    /// The `includes` command cannot take advantage of a geospatial [secondary index](/docs/secondary-indexes/javascript). If you're working with large data sets, consider using an index and [getIntersecting](/api/javascript/get_intersecting) before `includes` to narrow down the initial result set.
    /// {% endinfobox %}
    /// 
    /// __Example:__ Rewrite the previous example with `getIntersecting`.
    /// 
    /// ```java
    /// Circle circle1 = r.circle(r.array(-117.220406, 32.719464), 10)
    ///                   .optArg("unit", "mi");
    /// 
    /// r.table("parks").getIntersecting(circle1)
    ///  .optArg("index", "area").g("area")
    ///  .includes(circle1).run(conn);
    /// ```

    pub fn includes<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("includes", args)
    }


    /// Tests whether two geometry objects intersect with one another
///
/// 
///
/// When applied to a sequence of geometry objects, `intersects` acts as a [filter](/api/java/filter), returning a sequence of objects from the sequence that intersect with the argument.
                /// 
    /// __Example:__ Is `point2` within a 2000-meter circle around `point1`?
    /// 
    /// ```java
    /// import com.rethinkdb.gen.ast.Point;
    /// 
    /// Point point1 = r.point(-117.220406,32.719464);
    /// Point point2 = r.point(-117.206201,32.725186);
    /// 
    /// r.circle(point1, 2000).intersects(point2).run(conn);
    /// 
    /// // Result:
    /// true
    /// ```
    /// 
    /// __Example:__ Which of the locations in a list of parks intersect a given circle?
    /// 
    /// ```java
    /// r.table("parks").g("area")
    ///  .intersects(r.circle(r.array(-117.220406, 32.719464), 10).optArg("unit", "mi"))
    ///  .run(conn);
    /// ```
    /// 
    /// {% infobox %}
    /// The `intersects` command cannot take advantage of a geospatial [secondary index](/docs/secondary-indexes/java). If you're working with large data sets, you should consider using an index and the [getIntersecting](/api/java/get_intersecting) command instead of `intersects`.
    /// {% endinfobox %}

    pub fn intersects<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("intersects", args)
    }


    /// Construct a geometry object of type Line
///
/// 
///
/// The line can be specified in one of two ways:
                /// 
    /// * Two or more two-item arrays, specifying latitude and longitude numbers of the line's vertices;
    /// * Two or more [Point](/api/java/point) objects specifying the line's vertices.
    /// 
    /// <!-- break -->
    /// 
    /// Longitude (&minus;180 to 180) and latitude (&minus;90 to 90) of vertices are plotted on a perfect sphere. See [Geospatial support](/docs/geo-support/) for more information on ReQL's coordinate system.
    /// 
    /// __Example:__ Define a line.
    /// 
    /// ```java
    /// r.table("geo").insert(
    ///     r.hashMap("id", 101)
    ///      .with("route", r.line(r.array(-122.423246, 37.779388),
    ///                            r.array(-121.88642, 37.329898)))
    /// ).run(conn);
    /// ```

    pub fn line<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("line", args)
    }


    /// Construct a geometry object of type Point
///
/// 
///
/// The point is specified by two floating point numbers, the longitude (&minus;180 to 180) and latitude (&minus;90 to 90) of the point on a perfect sphere. See [Geospatial support](/docs/geo-support/) for more information on ReQL's coordinate system.
                /// 
    /// __Example:__ Define a point.
    /// 
    /// ```java
    /// r.table("geo").insert(
    ///     r.hashMap("id", 1)
    ///      .with("name", "San Francisco")
    ///      .with("location", r.point(-122.423246, 37.779388))
    /// ).run(conn);
    /// ```

    pub fn point<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("point", args)
    }


    /// Construct a geometry object of type Polygon
///
/// 
///
/// The Polygon can be specified in one of two ways:
                /// 
    /// * Three or more two-item arrays, specifying latitude and longitude numbers of the polygon's vertices;
    /// * Three or more [Point](/api/java/point) objects specifying the polygon's vertices.
    /// 
    /// <!-- break -->
    /// 
    /// Longitude (&minus;180 to 180) and latitude (&minus;90 to 90) of vertices are plotted on a perfect sphere. See [Geospatial support](/docs/geo-support/) for more information on ReQL's coordinate system.
    /// 
    /// If the last point does not specify the same coordinates as the first point, `polygon` will close the polygon by connecting them. You cannot directly construct a polygon with holes in it using `polygon`, but you can use [polygonSub](/api/java/polygon_sub) to use a second polygon within the interior of the first to define a hole.
    /// 
    /// 
    /// __Example:__ Define a polygon.
    /// 
    /// ```java
    /// r.table("geo").insert(
    ///     r.hashMap("id", 101)
    ///      .with("rectangle", r.polygon(
    ///         r.array(-122.423246, 37.779388),
    ///         r.array(-122.423246, 37.329898),
    ///         r.array(-121.88642, 37.329898),
    ///         r.array(-121.88642, 37.779388))
    ///     )
    /// ).run(conn);
    /// ```

    pub fn polygon<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("polygon", args)
    }


    /// Use `polygon2` to "punch out" a hole in `polygon1`
///
/// 
///
/// `polygon2` must be completely contained within `polygon1` and must have no holes itself (it must not be the output of `polygonSub` itself).
                /// 
    /// 
    /// __Example:__ Define a polygon with a hole punched in it.
    /// 
    /// ```java
    /// import com.rethinkdb.gen.ast.Polygon;
    /// 
    /// Polygon outerPolygon = r.polygon(
    ///     [-122.4,37.7],
    ///     [-122.4,37.3],
    ///     [-121.8,37.3],
    ///     [-121.8,37.7]
    /// );
    /// Polygon innerPolygon = r.polygon(
    ///     [-122.3,37.4],
    ///     [-122.3,37.6],
    ///     [-122.0,37.6],
    ///     [-122.0,37.4]
    /// );
    /// outerPolygon.polygonSub(inner_polygon).run(conn);
    /// ```

    pub fn polygon_sub<T: ToArg>(&self, args: T) -> Client {
        cmd_with_args("polygon_sub", args)
    }


    /// Grant or deny access permissions for a user account, globally or on a per-database or per-table basis
///
/// 
///
/// 
                /// 
    /// There are four different permissions that can be granted to an account:
    /// 
    /// * `read` allows reading the data in tables.
    /// * `write` allows modifying data, including inserting, replacing/updating, and deleting.
    /// * `connect` allows a user to open HTTP connections via the [http][] command. This permission can only be granted in global scope.
    /// * `config` allows users to create/drop [secondary indexes][si] on a table and changing the cluster configuration; to create and drop tables, if granted on a database; and to create and drop databases, if granted globally.
    /// 
    /// [si]: /docs/secondary-indexes/
    /// [http]: /api/java/http
    /// 
    /// Permissions may be granted on a global scope, or granted for a specific table or database. The scope is defined by calling `grant` on its own (e.g., `r.grant()`, on a table (`r.table().grant()`), or on a database (`r.db().grant()`).
    /// 
    /// The `grant` command returns an object of the following form:
    /// 
    /// ```json
    /// {
    ///     "granted": 1,
    ///     "permissions_changes": [
    ///         {
    ///             "new_val": { new permissions },
    ///             "old_val": { original permissions }
    ///         }
    ///     ]
    /// ```
    /// 
    /// The `granted` field will always be `1`, and the `permissions_changes` list will have one object, describing the new permissions values and the old values they were changed from (which may be `null`).
    /// 
    /// Permissions that are not defined on a local scope will be inherited from the next largest scope. For example, a write operation on a table will first check if `write` permissions are explicitly set to `true` or `false` for that table and account combination; if they are not, the `write` permissions for the database will be used if those are explicitly set; and if neither table nor database permissions are set for that account, the global `write` permissions for that account will be used.
    /// 
    /// __Note:__ For all accounts other than the special, system-defined `admin` account, permissions that are not explicitly set in any scope will effectively be `false`. When you create a new user account by inserting a record into the [system table][st], that account will have _no_ permissions until they are explicitly granted.
    /// 
    /// [st]: /docs/system-tables/#users
    /// 
    /// For a full description of permissions, read [Permissions and user accounts][pa].
    /// 
    /// [pa]: /docs/permissions-and-accounts/
    /// 
    /// __Example:__ Grant the `chatapp` user account read and write permissions on the `users` database.
    /// 
    /// ```java
    /// r.db("users").grant("chatapp", r.hashMap("read", true).with("write", true)).run(conn);
    /// ```
    /// 
    /// Return:
    /// 
    /// ```json
    /// {
    ///     "granted": 1,
    ///     "permissions_changes": [
    ///         {
    ///             "new_val": { "read": true, "write": true },
    ///             "old_val": { null }
    ///         }
    ///     ]
    /// ```
    /// 
    /// __Example:__ Deny write permissions from the `chatapp` account for the `admin` table.
    /// 
    /// ```java
    /// r.db("users").table("admin").grant("chatapp", r.hashMap("write", false)).run(conn);
    /// ```
    /// 
    /// This will override the `write: true` permissions granted in the first example, but for this table only. Other tables in the `users` database will inherit from the database permissions.
    /// 
    /// __Example:__ Delete a table-level permission for the `chatapp` account.
    /// 
    /// ```java
    /// r.db("users").table("admin").grant("chatapp", r.hashMap("write", null)).run(conn);
    /// ```
    /// 
    /// By specifying `None`, the table scope `write` permission is removed, and will again inherit from the next highest scope (database or global).
    /// 
    /// __Example:__ Grant `chatapp` the ability to use HTTP connections.
    /// 
    /// ```java
    /// r.grant("chatapp", r.hashMap("connect", true)).run(conn);
    /// ```
    /// 
    /// This grant can only be given on a global level.
    /// 
    /// 
    /// __Example:__ Grant a `monitor` account read-only access to all databases.
    /// 
    /// ```java
    /// r.grant("monitor", r.hashMap("read", true)).run(conn);
    /// ```

    pub fn grant(&self) -> Client {
        cmd("grant")
    }


    /// Query (read and/or update) the configurations for individual tables or databases
///
/// 
///
/// 
                /// 
    /// The `config` command is a shorthand way to access the `table_config` or `db_config` [System tables](/docs/system-tables/#configuration-tables). It will return the single row from the system that corresponds to the database or table configuration, as if [get](/api/java/get) had been called on the system table with the UUID of the database or table in question.
    /// 
    /// __Example:__ Get the configuration for the `users` table.
    /// 
    /// ```java
    /// r.table("users").config().run(conn);
    /// ```
    /// 
    /// <!-- stop -->
    /// Result:
    /// 
    /// ```json
    /// {
    ///     "id": "31c92680-f70c-4a4b-a49e-b238eb12c023",
    ///     "name": "users",
    ///     "db": "superstuff",
    ///     "primary_key": "id",
    ///     "shards": [
    ///         {
    ///             "primary_replica": "a", 
    ///             "replicas": ["a", "b"],
    ///             "nonvoting_replicas": []
    ///         },
    ///         {
    ///             "primary_replica": "d",
    ///             "replicas": ["c", "d"],
    ///             "nonvoting_replicas": []
    ///         }
    ///     ],
    ///     "indexes": [],
    ///     "write_acks": "majority",
    ///     "durability": "hard"
    /// }
    /// ```
    /// 
    /// __Example:__ Change the write acknowledgement requirement of the `users` table.
    /// 
    /// ```java
    /// r.table("users").config().update(r.hashMap("write_acks", "single")).run(conn);
    /// ```

    pub fn config(&self) -> Client {
        cmd("config")
    }


    /// Rebalances the shards of a table
///
/// 
///
/// When called on a database, all the tables in that database will be rebalanced.
                /// 
    /// The `rebalance` command operates by measuring the distribution of primary keys within a table and picking split points that will give each shard approximately the same number of documents. It won't change the number of shards within a table, or change any other configuration aspect for the table or the database.
    /// 
    /// A table will lose availability temporarily after `rebalance` is called; use the [wait](/api/java/wait) command to wait for the table to become available again, or [status](/api/java/status) to check if the table is available for writing.
    /// 
    /// RethinkDB automatically rebalances tables when the number of shards are increased, and as long as your documents have evenly distributed primary keys&mdash;such as the default UUIDs&mdash;it is rarely necessary to call `rebalance` manually. Cases where `rebalance` may need to be called include:
    /// 
    /// * Tables with unevenly distributed primary keys, such as incrementing integers
    /// * Changing a table's primary key type
    /// * Increasing the number of shards on an empty table, then using non-UUID primary keys in that table
    /// 
    /// The [web UI](/docs/administration-tools/) (and the [info](/api/java/info) command) can be used to tell you when a table's shards need to be rebalanced.
    /// 
    /// The return value of `rebalance` is an object with two fields:
    /// 
    /// * `rebalanced`: the number of tables rebalanced.
    /// * `status_changes`: a list of new and old table status values. Each element of the list will be an object with two fields:
    ///     * `old_val`: The table's [status](/api/java/status) value before `rebalance` was executed. 
    ///     * `new_val`: The table's `status` value after `rebalance` was executed. (This value will almost always indicate the table is unavailable.)
    /// 
    /// See the [status](/api/java/status) command for an explanation of the objects returned in the `old_val` and `new_val` fields.
    /// 
    /// __Example:__ Rebalance a table.
    /// 
    /// ```java
    /// > r.table("superheroes").rebalance().run(conn);
    /// ```
    /// 
    /// <!-- stop -->
    /// 
    /// Result:
    /// 
    /// ```json
    /// {
    ///   "rebalanced": 1,
    ///   "status_changes": [
    ///     {
    ///       "old_val": {
    ///         "db": "database",
    ///         "id": "5cb35225-81b2-4cec-9eef-bfad15481265",
    ///         "name": "superheroes",
    ///         "shards": [
    ///           {
    ///             "primary_replica": "jeeves",
    ///             "replicas": [
    ///               {
    ///                 "server": "jeeves",
    ///                 "state": "ready"
    ///               }
    ///             ]
    ///           },
    ///           {
    ///             "primary_replica": "jeeves",
    ///             "replicas": [
    ///               {
    ///                 "server": "jeeves",
    ///                 "state": "ready"
    ///               }
    ///             ]
    ///           }
    ///         ],
    ///         "status": {
    ///           "all_replicas_ready": true,
    ///           "ready_for_outdated_reads": true,
    ///           "ready_for_reads": true,
    ///           "ready_for_writes": true
    ///         }
    ///       },
    ///       "new_val": {
    ///         "db": "database",
    ///         "id": "5cb35225-81b2-4cec-9eef-bfad15481265",
    ///         "name": "superheroes",
    ///         "shards": [
    ///           {
    ///             "primary_replica": "jeeves",
    ///             "replicas": [
    ///               {
    ///                 "server": "jeeves",
    ///                 "state": "transitioning"
    ///               }
    ///             ]
    ///           },
    ///           {
    ///             "primary_replica": "jeeves",
    ///             "replicas": [
    ///               {
    ///                 "server": "jeeves",
    ///                 "state": "transitioning"
    ///               }
    ///             ]
    ///           }
    ///         ],
    ///         "status": {
    ///           "all_replicas_ready": false,
    ///           "ready_for_outdated_reads": false,
    ///           "ready_for_reads": false,
    ///           "ready_for_writes": false
    ///         }
    ///       }
    /// 
    ///     }
    ///   ]
    /// }
    /// ```

    pub fn rebalance(&self) -> Client {
        cmd("rebalance")
    }


    /// Reconfigure a table's sharding and replication
///
/// 
///
/// Pass the following options using [optArg](/api/java/optarg/):
                /// 
    /// * `shards`: the number of shards, an integer from 1-64. Required.
    /// * `replicas`: either an integer or a mapping object. Required.
    ///     * If `replicas` is an integer, it specifies the number of replicas per shard. Specifying more replicas than there are servers will return an error.
    ///     * If `replicas` is an object, it specifies key-value pairs of server tags and the number of replicas to assign to those servers: `{tag1: 2, tag2: 4, tag3: 2, ...}`. For more information about server tags, read [Administration tools](/docs/administration-tools/).
    /// * `primary_replica_tag`: the primary server specified by its server tag. Required if `replicas` is an object; the tag must be in the object. This must *not* be specified if `replicas` is an integer.
    /// * `dry_run`: if `true` the generated configuration will not be applied to the table, only returned.
    /// * `nonvoting_replica_tags`: replicas with these server tags will be added to the `nonvoting_replicas` list of the resulting configuration. (See [failover](/docs/failover) for details about non-voting replicas.)
    /// 
    /// * `emergency_repair`: Used for the Emergency Repair mode. See the separate section below.
    /// 
    /// The return value of `reconfigure` is an object with three fields:
    /// 
    /// * `reconfigured`: the number of tables reconfigured. This will be `0` if `dry_run` is `true`.
    /// * `config_changes`: a list of new and old table configuration values. Each element of the list will be an object with two fields:
    ///     * `old_val`: The table's [config](/api/java/config) value before `reconfigure` was executed. 
    ///     * `new_val`: The table's `config` value after `reconfigure` was executed.
    /// * `status_changes`: a list of new and old table status values. Each element of the list will be an object with two fields:
    ///     * `old_val`: The table's [status](/api/java/status) value before `reconfigure` was executed.
    ///     * `new_val`: The table's `status` value after `reconfigure` was executed.
    /// 
    /// For `config_changes` and `status_changes`, see the [config](/api/java/config) and [status](/api/java/status) commands for an explanation of the objects returned in the `old_val` and `new_val` fields.
    /// 
    /// A table will lose availability temporarily after `reconfigure` is called; use the [wait](/api/java/wait) command to wait for the table to become available again, or [status](/api/java/status) to check if the table is available for writing.
    /// 
    /// **Note:** Whenever you call `reconfigure`, the write durability will be set to `hard` and the write acknowledgments will be set to `majority`; these can be changed by using the `config` command on the table.
    /// 
    /// If `reconfigure` is called on a database, all the tables in the database will have their configurations affected. The return value will be an array of the objects described above, one per table.
    /// 
    /// Read [Sharding and replication](/docs/sharding-and-replication/) for a complete discussion of the subject, including advanced topics.
    /// 
    /// __Example:__ Reconfigure a table.
    /// 
    /// ```java
    /// r.table("superheroes").reconfigure().optArg("shards", 2).optArg("replicas", 1).run(conn);
    /// ```
    /// 
    /// <!-- stop -->
    /// 
    /// Result:
    /// 
    /// ```json
    /// {
    ///   "reconfigured": 1,
    ///   "config_changes": [
    ///     {
    ///       "new_val": {
    ///         "id": "31c92680-f70c-4a4b-a49e-b238eb12c023",
    ///         "name": "superheroes",
    ///         "db": "superstuff",
    ///         "primary_key": "id",
    ///         "shards": [
    ///           {
    ///             "primary_replica": "jeeves",
    ///             "replicas": ["jeeves", "alfred"],
    ///             "nonvoting_replicas": []
    ///           },
    ///           {
    ///             "primary_replica": "alfred",
    ///             "replicas": ["jeeves", "alfred"],
    ///             "nonvoting_replicas": []
    ///           }
    ///         ],
    ///         "indexes": [],
    ///         "write_acks": "majority",
    ///         "durability": "hard"
    ///       },
    ///       "old_val": {
    ///         "id": "31c92680-f70c-4a4b-a49e-b238eb12c023",
    ///         "name": "superheroes",
    ///         "db": "superstuff",
    ///         "primary_key": "id",
    ///         "shards": [
    ///             "primary_replica": "alfred",
    ///             "replicas": ["jeeves", "alfred"],
    ///             "nonvoting_replicas": []
    ///         ],
    ///         "indexes": [],
    ///         "write_acks": "majority",
    ///         "durability": "hard"
    ///       }
    ///     }
    ///   ],
    ///   "status_changes": [
    ///     {
    ///       "new_val": (status object),
    ///       "old_val": (status object)
    ///     }
    ///   ]
    /// }
    /// ```
    /// 
    /// __Example:__ Reconfigure a table, specifying replicas by server tags.
    /// 
    /// ```java
    /// r.table("superheroes").reconfigure().optArg("shards", 2).optArg("replicas", r.hashMap("wooster", 1).with("wayne", 1)).optArg("primary_replica_tag", "wooster").run(conn)
    /// 
    /// {
    ///   "reconfigured": 1,
    ///   "config_changes": [
    ///     {
    ///       "new_val": {
    ///         "id": "31c92680-f70c-4a4b-a49e-b238eb12c023",
    ///         "name": "superheroes",
    ///         "db": "superstuff",
    ///         "primary_key": "id",
    ///         "shards": [
    ///           {
    ///             "primary_replica": "jeeves",
    ///             "replicas": ["jeeves", "alfred"],
    ///             "nonvoting_replicas": []
    ///           },
    ///           {
    ///             "primary_replica": "alfred",
    ///             "replicas": ["jeeves", "alfred"],
    ///             "nonvoting_replicas": []
    ///           }
    ///         ],
    ///         "indexes": [],
    ///         "write_acks": "majority",
    ///         "durability": "hard"
    ///       },
    ///       "old_val": {
    ///         "id": "31c92680-f70c-4a4b-a49e-b238eb12c023",
    ///         "name": "superheroes",
    ///         "db": "superstuff",
    ///         "primary_key": "id",
    ///         "shards": [
    ///             "primary_replica": "alfred",
    ///             "replicas": ["jeeves", "alfred"],
    ///             "nonvoting_replicas": []
    ///         ],
    ///         "indexes": [],
    ///         "write_acks": "majority",
    ///         "durability": "hard"
    ///       }
    ///     }
    ///   ],
    ///   "status_changes": [
    ///     {
    ///       "new_val": (status object),
    ///       "old_val": (status object)
    ///     }
    ///   ]
    /// }
    /// ```
    /// 
    /// # Emergency Repair mode #
    /// 
    /// RethinkDB supports automatic failover when more than half of the voting replicas for each shard of a table are still available (see the [Failover][fail] documentation for more details). However, if half or more of the voting replicas for a shard are lost, failover will not happen automatically, leaving two options:
    /// 
    /// [fail]: /docs/failover/
    /// 
    /// * Bring enough of the missing servers back online to allow automatic failover
    /// * Use emergency repair mode to reconfigure the table
    /// 
    /// The `emergency_repair` argument is effectively a different command; when it is specified, no other arguments to `reconfigure` are allowed except for `dry_run`. When it's executed, each shard of the table is examined and classified into one of three categories:
    /// 
    /// * **Healthy:** more than half of the shard's voting replicas are still available.
    /// * **Repairable:** the shard is not healthy, but has at least one replica, whether voting or non-voting, available.
    /// * **Beyond repair:** the shard has no replicas available.
    /// 
    /// For each repairable shard, `emergency_repair` will convert all unavailable voting replicas into non-voting replicas. If all the voting replicas were removed, an arbitrarily-chosen available non-voting replica will be converted into a voting replica. After this operation, all of the shard's available replicas will be voting replicas.
    /// 
    /// Specify `emergency_repair` with one of two string options:
    /// 
    /// * `unsafe_rollback`: shards that are beyond repair will be left alone.
    /// * `unsafe_rollback_or_erase`: a shard that is beyond repair will be destroyed and recreated on an available server that holds another shard for that table.
    /// 
    /// The return value of `reconfigure` in emergency repair mode is the same as before. Examine the `config_changes` field to see the old and new configuration settings for the table. As in the normal mode, if you specify `emergency_repair` with `dry_run: true`, the table will not actually be reconfigured.
    /// 
    /// __Note:__ `emergency_repair` may only be used on individual tables, not on databases. It cannot be used after the `db` command.
    /// 
    /// {% infobox alert %}
    /// **The emergency repair mode is extremely dangerous.** It bypasses normal safeguards that prevent data loss and invalidates the [consistency guarantees](/docs/consistency/) that RethinkDB normally provides, and can easily lose data in either mode&mdash;in `unsafe_rollback_or_erase` mode it could lose *all* of a shard's data.
    /// {% endinfobox %}
    /// 
    /// __Example:__ Perform an emergency repair on a table.
    /// 
    /// ```java
    /// r.table("superheroes").reconfigure().optArg("emergency_repair", "unsafe_rollback").run(conn);
    /// ```

    pub fn reconfigure(&self) -> Client {
        cmd("reconfigure")
    }


    /// Return the status of a table
///
/// 
///
/// 
                /// 
    /// The return value is an object providing information about the table's shards, replicas and replica readiness states. For a more complete discussion of the object fields, read about the `table_status` table in [System tables](/docs/system-tables/#status-tables).
    /// 
    /// * `id`: the UUID of the table.
    /// * `name`: the table's name.
    /// * `db`: the database the table is in.
    /// * `status`: the subfields in this field indicate whether all shards of the table are ready to accept the given type of query: `outdated_reads`, `reads` and `writes`. The `all_replicas_ready` field indicates whether all backfills have finished.
    /// * `shards`: one entry for each shard in `table_config`. Each shard's object has the following fields:
    /// 	* `primary_replicas`: a list of zero or more servers acting as primary replicas for the table.
    /// 	* `replicas`: a list of all servers acting as a replica for that shard. The `state` field may be one of the following: `ready`, `transitioning`, `backfilling`, `disconnected`, `waiting_for_primary`, or `waiting_for_quorum`.
    /// 
    /// __Example:__ Get a table's status.
    /// 
    /// ```java
    /// r.table("superheroes").status().run(conn);
    /// ```
    /// 
    /// <!-- stop -->
    /// 
    /// Result:
    /// 
    /// ```json
    /// {
    ///   "db": "database",
    ///   "id": "5cb35225-81b2-4cec-9eef-bfad15481265",
    ///   "name": "superheroes",
    ///   "shards": [
    ///     {
    ///       "primary_replicas": ["jeeves"],
    ///       "replicas": [
    ///         {
    ///           "server": "jeeves",
    ///           "state": "ready"
    ///         }
    ///       ]
    ///     },
    ///     {
    ///       "primary_replicas": ["jeeves"],
    ///       "replicas": [
    ///         {
    ///           "server": "jeeves",
    ///           "state": "ready"
    ///         }
    ///       ]
    ///     }
    ///   ],
    ///   "status": {
    ///     "all_replicas_ready": true,
    ///     "ready_for_outdated_reads": true,
    ///     "ready_for_reads": true,
    ///     "ready_for_writes": true
    ///   }
    /// }
    /// ```

    pub fn status(&self) -> Client {
        cmd("status")
    }


    /// Wait for a table or all the tables in a database to be ready
///
/// 
///
/// A table may be temporarily unavailable after creation, rebalancing or reconfiguring. The `wait` command blocks until the given table (or database) is fully up to date.
                /// 
    /// The `wait` command takes two optional arguments using [optArg](/api/java/optarg/):
    /// 
    /// 
    /// * `wait_for`: a string indicating a table [status](/api/java/status) to wait on before returning, one of `ready_for_outdated_reads`, `ready_for_reads`, `ready_for_writes`, or `all_replicas_ready`. The default is `all_replicas_ready`. 
    /// * `timeout`: a number indicating maximum time, in seconds, to wait for the table to be ready. If this value is exceeded, a `ReqlRuntimeError` will be thrown. A value of `0` means no timeout. The default is `0` (no timeout).
    /// 
    /// The return value is an object consisting of a single field, `ready`. The value is an integer indicating the number of tables waited for. It will always be `1` when `wait` is called on a table, and the total number of tables when called on a database.
    /// 
    /// {% infobox %}
    /// Versions of RethinkDB prior to 2.3 allowed `wait` to be called without a table or database specified. This is no longer valid; `wait` requires explicit selection of a database or table.
    /// {% endinfobox %}
    /// 
    /// __Example:__ Wait on a table to be ready.
    /// 
    /// ```java
    /// r.table("superheroes").wait().run(conn);
    /// ```
    /// 
    /// Result:
    /// 
    /// ```json
    /// { "ready": 1 }
    /// ```

    pub fn wait(&self) -> Client {
        cmd("wait")
    }

}
