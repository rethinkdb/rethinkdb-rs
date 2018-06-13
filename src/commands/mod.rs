
            
            // AUTO GENERATED
            // Manual changes made to this file will be overwritten by the build script.
            // Edit `build/commands.rs` instead...
            // @generated

            mod io;
            mod util;
            mod args;

            use Connection;
            use {Config, Client, IntoArg, Result};
            use ql2::proto::{Term, Term_TermType as Type};
        
            impl Client {

                /// Create a new ReQL client
                ///
                /// By convention, the variable binding holding a ReQL client is called `r`.
                ///
                /// __Example__: Create your client.
                ///
                /// ```reql
                /// let r = Client::new();
                /// ```

                pub fn new() -> Client {
                    util::new_client()
                }

                #[doc(hidden)]
                pub fn set_term(&mut self, term: Result<Term>) {
                    self.term = term;
                }

                #[doc(hidden)]
                pub fn term(&self) -> Result<&Term> {
                    match self.term {
                        Ok(ref term) => Ok(term),
                        Err(ref error) => Err(error.clone()),
                    }
                }

                /// Specify optional arguments to a ReQL command
                ///
                /// Normally, you should use the `args!()` macro to pass arguments to a command that
                /// also takes optional arguments. If the command takes at least one argument, you
                /// don't need to call `with_args`. However, some commands like [delete](struct.Client.html#method.delete)
                /// do not have any required arguments but yet they have optional ones. That's when `with_args` comes in.
                /// The `args` macro is provided by the `reql-macros` crate. NB: That crate
                /// requires the nightly compiler. See its docs for more details.
                ///
                /// __Example__: Delete all documents from the table `comments` without waiting for the operation to be flushed to
                /// disk.
                ///
                /// ```rust,ignore
                /// # #![feature(proc_macro)]
                /// # #![feature(proc_macro_non_items)]
                /// # #![allow(unused_must_use)]
                /// # extern crate reql;
                /// # extern crate reql_macros;
                /// # use reql_macros::args;
                /// # fn main() {
                /// # use reql::Client;
                /// # let r = Client::new();
                /// r.table("comments").delete().with_args(args!({durability: "soft"}));
                /// # }
                /// ```

                pub fn with_args<T: IntoArg>(&self, args: T) -> Client {
                    util::with_args(self, args)
                }

                
                /// Create a new connection to the database server
///
/// <img src="https://raw.githubusercontent.com/rethinkdb/docs/master/_jekyll/_images/api_illustrations/connect_javascript.png" class="api_command_illustration" />
///
/// Accepts the following
///
/// options:
/// 
/// - `host`: the host to connect to (default `localhost`).
/// - `port`: the port to connect on (default `28015`).
/// - `db`: the default database (default `test`).
/// - `user`: the user account to connect as (default `admin`).
/// - `password`: the password for the user account to connect as (default `''`, empty).
/// - `timeout`: timeout period in seconds for the connection to be opened (default `20`).
/// - `ssl`: a hash of options to support SSL connections (default `null`). Currently, there is only one option available, and if the `ssl` option is specified, this key is required:
///     - `ca`: a list of [Node.js](http://nodejs.org) `Buffer` objects containing SSL CA certificates.
/// 
/// If the connection cannot be established, a `ReqlDriverError` will be passed to the callback instead of a connection.
/// 
/// <!-- break -->
/// 
/// The returned connection object will have two properties on it containing the connection's port and address:
/// 
/// ```javascript
/// conn.clientPort;
/// conn.clientAddress;
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
/// ```javascript
/// r.connect({
///     db: 'marvel'
/// }, function(err, conn) {
///     // ...
/// });
/// ```
/// 
/// If no callback is provided, a promise will be returned.
/// 
/// ```javascript
/// var promise = r.connect({db: 'marvel'});
/// ```
/// 
/// __Example:__ Open a new connection to the database.
/// 
/// ```javascript
/// r.connect({
///     host: 'localhost',
///     port: 28015,
///     db: 'marvel'
/// }, function(err, conn) {
///     // ...
/// });
/// ```
/// 
/// Alternatively, you can use promises.
/// 
/// ```javascript
/// var p = r.connect({
///     host: 'localhost',
///     port: 28015,
///     db: 'marvel'
/// });
/// p.then(function(conn) {
///     // ...
/// }).error(function(error) {
///     // ...
/// });
/// ```
/// 
/// __Example:__ Open a new connection to the database, specifying a user/password combination for authentication.
/// 
/// ```javascript
/// r.connect({
///     host: 'localhost',
///     port: 28015,
///     db: 'marvel',
///     user: 'herofinder',
///     password: 'metropolis'
/// }, function(err, conn) {
///     // ...
/// });
/// ```
/// 
/// __Example:__ Open a new connection to the database using an SSL proxy.
/// 
/// ```javascript
/// var fs = require('fs');
/// fs.readFile('/path/to/cert', function (err, caCert) {
///     if (!err) {
///         r.connect({
///             host: 'localhost',
///             port: 28015,
///             db: 'marvel',
///             authKey: 'hunter2',
///             ssl: {
///                 ca: caCert
///             }
///         }, function(err, conn) {
///             // ...
///         });
///     } else {
///         console.log(err);
///     }
/// });
/// ```

                pub fn connect<'a>(&self, cfg: Config<'a>) -> Result<Connection> {
                    io::connect(self, cfg)
                }
            

                /// Turn a query into a changefeed, an infinite stream of objects representing changes to the query's results as they occur
///
/// A changefeed may return changes to a table or an individual document (a "point" changefeed). Commands such as `filter` or `map` may be used before the `changes` command to transform or filter the output, and many commands that operate on sequences can be chained after `changes`.
///
/// 
/// There are six optional arguments to `changes`.
/// 
/// * `squash`: Controls how change notifications are batched. Acceptable values are `true`, `false` and a numeric value:
///     * `true`: When multiple changes to the same document occur before a batch of notifications is sent, the changes are "squashed" into one change. The client receives a notification that will bring it fully up to date with the server.
///     * `false`: All changes will be sent to the client verbatim. This is the default.
///     * `n`: A numeric value (floating point). Similar to `true`, but the server will wait `n` seconds to respond in order to squash as many changes together as possible, reducing network traffic. The first batch will always be returned immediately.
/// * `changefeedQueueSize`: the number of changes the server will buffer between client reads before it starts dropping changes and generates an error (default: 100,000).
/// * `includeInitial`: if `true`, the changefeed stream will begin with the current contents of the table or selection being monitored. These initial results will have `new_val` fields, but no `old_val` fields. The initial results may be intermixed with actual changes, as long as an initial result for the changed document has already been given. If an initial result for a document has been sent and a change is made to that document that would move it to the unsent part of the result set (e.g., a changefeed monitors the top 100 posters, the first 50 have been sent, and poster 48 has become poster 52), an "uninitial" notification will be sent, with an `old_val` field but no `new_val` field.
/// * `includeStates`: if `true`, the changefeed stream will include special status documents consisting of the field `state` and a string indicating a change in the feed's state. These documents can occur at any point in the feed between the notification documents described below. If `includeStates` is `false` (the default), the status documents will not be sent.
/// * `includeOffsets`: if `true`, a changefeed stream on an `orderBy.limit` changefeed will include `old_offset` and `new_offset` fields in status documents that include `old_val` and `new_val`. This allows applications to maintain ordered lists of the stream's result set. If `old_offset` is set and not `null`, the element at `old_offset` is being deleted; if `new_offset` is set and not `null`, then `new_val` is being inserted at `new_offset`. Setting `includeOffsets` to `true` on a changefeed that does not support it will raise an error.
/// * `includeTypes`: if `true`, every result on a changefeed will include a `type` field with a string that indicates the kind of change the result represents: `add`, `remove`, `change`, `initial`, `uninitial`, `state`. Defaults to `false`.
/// 
/// There are currently two states:
/// 
/// * `{state: 'initializing'}` indicates the following documents represent initial values on the feed rather than changes. This will be the first document of a feed that returns initial values.
/// * `{state: 'ready'}` indicates the following documents represent changes. This will be the first document of a feed that does *not* return initial values; otherwise, it will indicate the initial values have all been sent.
/// 
/// {% infobox %}
/// Starting with RethinkDB 2.2, state documents will *only* be sent if the `includeStates` option is `true`, even on point changefeeds. Initial values will only be sent if `includeInitial` is `true`. If `includeStates` is `true` and `includeInitial` is false, the first document on the feed will be `{state: 'ready'}`.
/// {% endinfobox %}
/// 
/// If the table becomes unavailable, the changefeed will be disconnected, and a runtime exception will be thrown by the driver.
/// 
/// Changefeed notifications take the form of a two-field object:
/// 
/// ```javascript
/// {
///     "old_val": <document before change>,
///     "new_val": <document after change>
/// }
/// ```
/// 
/// When `includeTypes` is `true`, there will be three fields:
/// 
/// ```javascript
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
/// Certain document transformation commands can be chained before changefeeds. For more information, read the [discussion of changefeeds](/docs/changefeeds/javascript/) in the "Query language" documentation.
/// 
/// __Note:__ Changefeeds ignore the `read_mode` flag to `run`, and always behave as if it is set to `single` (i.e., the values they return are in memory on the primary replica, but have not necessarily been written to disk yet). For more details read [Consistency guarantees](/docs/consistency).
/// {% endinfobox %}
/// 
/// The server will buffer up to `changefeedQueueSize` elements (default 100,000). If the buffer limit is hit, early changes will be discarded, and the client will receive an object of the form `{error: "Changefeed cache over array size limit, skipped X elements."}` where `X` is the number of elements skipped.
/// 
/// Commands that operate on streams (such as [filter](/api/javascript/filter/) or [map](/api/javascript/map/)) can usually be chained after `changes`.  However, since the stream produced by `changes` has no ending, commands that need to consume the entire stream before returning (such as [reduce](/api/javascript/reduce/) or [count](/api/javascript/count/)) cannot.
/// 
/// __Example:__ Subscribe to the changes on a table.
/// 
/// Start monitoring the changefeed in one client:
/// 
/// ```javascript
/// r.table('games').changes().run(conn, function(err, cursor) {
///   cursor.each(console.log);
/// });
/// ```
/// 
/// As these queries are performed in a second client, the first
/// client would receive and print the following objects:
/// 
/// ```javascript
/// > r.table('games').insert({id: 1}).run(conn, callback);
/// {old_val: null, new_val: {id: 1}}
/// 
/// > r.table('games').get(1).update({player1: 'Bob'}).run(conn, callback);
/// {old_val: {id: 1}, new_val: {id: 1, player1: 'Bob'}}
/// 
/// > r.table('games').get(1).replace({id: 1, player1: 'Bob', player2: 'Alice'}).run(conn, callback);
/// {old_val: {id: 1, player1: 'Bob'},
///  new_val: {id: 1, player1: 'Bob', player2: 'Alice'}}
/// 
/// > r.table('games').get(1).delete().run(conn, callback)
/// {old_val: {id: 1, player1: 'Bob', player2: 'Alice'}, new_val: null}
/// 
/// > r.tableDrop('games').run(conn, callback);
/// ReqlRuntimeError: Changefeed aborted (table unavailable)
/// ```
/// 
/// __Example:__ Return all the changes that increase a player's score.
/// 
/// ```javascript
/// r.table('test').changes().filter(
///   r.row('new_val')('score').gt(r.row('old_val')('score'))
/// ).run(conn, callback)
/// ```
/// 
/// __Example:__ Return all the changes to a specific player's score that increase it past 10.
/// 
/// ```javascript
/// r.table('test').get(1).filter(r.row('score').gt(10)).changes().run(conn, callback)
/// ```
/// 
/// __Example:__ Return all the inserts on a table.
/// 
/// ```javascript
/// r.table('test').changes().filter(r.row('old_val').eq(null)).run(conn, callback)
/// ```
/// 
/// __Example:__ Return all the changes to game 1, with state notifications and initial values.
/// 
/// ```javascript
/// r.table('games').get(1).changes({includeInitial: true, includeStates: true}).run(conn, callback);
/// // Result returned on changefeed
/// {state: 'initializing'}
/// {new_val: {id: 1, score: 12, arena: 'Hobbiton Field'}}
/// {state: 'ready'}
/// {
/// 	old_val: {id: 1, score: 12, arena: 'Hobbiton Field'},
/// 	new_val: {id: 1, score: 14, arena: 'Hobbiton Field'}
/// }
/// {
/// 	old_val: {id: 1, score: 14, arena: 'Hobbiton Field'},
/// 	new_val: {id: 1, score: 17, arena: 'Hobbiton Field', winner: 'Frodo'}
/// }
/// ```
/// 
/// __Example:__ Return all the changes to the top 10 games. This assumes the presence of a `score` secondary index on the `games` table.
/// 
/// ```javascript
/// r.table('games').orderBy(
///     { index: r.desc('score') }
/// ).limit(10).changes().run(conn, callback);
/// ```
/// 
/// __Example:__ Maintain the state of an array based on a changefeed.
/// 
/// ```javascript
/// r.table('data').changes(
///     {includeInitial: true, includeOffsets: true}
/// ).run(conn, function (err, change) {
///     // delete item at old_offset before inserting at new_offset
///     if (change.old_offset != null) {
///         myArray.splice(change.old_offset, 1);
///     }
///     if (change.new_offset != null) {
///         myArray.splice(change.new_offset, 0, change.new_val);
///     }
/// });
/// ```
/// 
/// (This is a simplistic implementation; for a more sophisticated treatment, see the `applyChange` function in Horizon's [client/src/ast.js][ast] source.)
/// 
/// [ast]: https://github.com/rethinkdb/horizon/blob/next/client/src/ast.js

                pub fn changes(&self) -> Client {
                    util::make_cmd::<Client>(self, "changes", Some(Type::CHANGES), None)
                }
            

                /// Create a database
///
/// A RethinkDB database is a collection of tables, similar to
///
/// relational databases.
/// 
/// If successful, the command returns an object with two fields:
/// 
/// * `dbs_created`: always `1`.
/// * `config_changes`: a list containing one object with two fields, `old_val` and `new_val`:
///     * `old_val`: always `null`.
///     * `new_val`: the database's new [config](/api/javascript/config) value.
/// 
/// If a database with the same name already exists, the command throws `ReqlRuntimeError`.
/// 
/// Note: Only alphanumeric characters and underscores are valid for the database name.
/// 
/// __Example:__ Create a database named 'superheroes'.
/// 
/// ```javascript
/// > r.dbCreate('superheroes').run(conn, callback);
/// // Result passed to callback
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

                pub fn db_create<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "db_create", Some(Type::DB_CREATE), Some(args))
                }
            

                /// Drop a database
///
/// The database, all its tables, and corresponding data will be deleted.
///
/// 
/// If successful, the command returns an object with two fields:
/// 
/// * `dbs_dropped`: always `1`.
/// * `tables_dropped`: the number of tables in the dropped database.
/// * `config_changes`: a list containing one two-field object, `old_val` and `new_val`:
///     * `old_val`: the database's original [config](/api/javascript/config) value.
///     * `new_val`: always `null`.
/// 
/// If the given database does not exist, the command throws `ReqlRuntimeError`.
/// 
/// __Example:__ Drop a database named 'superheroes'.
/// 
/// ```javascript
/// > r.dbDrop('superheroes').run(conn, callback);
/// // Result passed to callback
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

                pub fn db_drop<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "db_drop", Some(Type::DB_DROP), Some(args))
                }
            

                /// List all database names in the system
///
/// The result is a list of strings.
///
/// 
/// __Example:__ List all databases.
/// 
/// ```javascript
/// r.dbList().run(conn, callback)
/// ```

                pub fn db_list(&self) -> Client {
                    util::make_cmd::<Client>(self, "db_list", Some(Type::DB_LIST), None)
                }
            

                /// Create a table
///
/// <img src="https://raw.githubusercontent.com/rethinkdb/docs/master/_jekyll/_images/api_illustrations/table_create_javascript.png" class="api_command_illustration" />
///
/// A RethinkDB table is a collection of JSON documents.
///
/// 
/// If successful, the command returns an object with two fields:
/// 
/// * `tables_created`: always `1`.
/// * `config_changes`: a list containing one two-field object, `old_val` and `new_val`:
///     * `old_val`: always `null`.
///     * `new_val`: the table's new [config](/api/javascript/config) value.
/// 
/// If a table with the same name already exists, the command throws `ReqlOpFailedError`.
/// 
/// {% infobox %}
/// __Note:__ Only alphanumeric characters and underscores are valid for the table name.
/// 
/// Invoking `tableCreate` without specifying a database using [db](/api/javascript/db/) creates a table in the database specified in [connect](/api/javascript/connect/), or `test` if no database was specified.
/// {% endinfobox %}
/// 
/// When creating a table you can specify the following options:
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
/// ```javascript
/// > r.db('heroes').tableCreate('dc_universe').run(conn, callback);
/// // Result passed to callback
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
/// ```javascript
/// r.db('test').tableCreate('dc_universe', {primaryKey: 'name'}).run(conn, callback);
/// ```
/// 
/// __Example:__ Create a table set up for two shards and three replicas per shard. This requires three available servers.
/// 
/// ```javascript
/// r.db('test').tableCreate('dc_universe', {shards: 2, replicas: 3}).run(conn, callback);
/// ```
/// 
/// Read [Sharding and replication](/docs/sharding-and-replication/) for a complete discussion of the subject, including advanced topics.

                pub fn table_create<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "table_create", Some(Type::TABLE_CREATE), Some(args))
                }
            

                /// Drop a table from a database
///
/// The table and all its data will be deleted.
///
/// 
/// If successful, the command returns an object with two fields:
/// 
/// * `tables_dropped`: always `1`.
/// * `config_changes`: a list containing one two-field object, `old_val` and `new_val`:
///     * `old_val`: the dropped table's [config](/api/javascript/config) value.
///     * `new_val`: always `null`.
/// 
/// If the given table does not exist in the database, the command throws `ReqlRuntimeError`.
/// 
/// __Example:__ Drop a table named 'dc_universe'.
/// 
/// ```javascript
/// > r.db('test').tableDrop('dc_universe').run(conn, callback);
/// // Result passed to callback
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

                pub fn table_drop<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "table_drop", Some(Type::TABLE_DROP), Some(args))
                }
            

                /// List all table names in a database
///
/// The result is a list of strings.
///
/// 
/// __Example:__ List all tables of the 'test' database.
/// 
/// ```javascript
/// r.db('test').tableList().run(conn, callback)
/// ```
/// 

                pub fn table_list(&self) -> Client {
                    util::make_cmd::<Client>(self, "table_list", Some(Type::TABLE_LIST), None)
                }
            

                /// Create a new secondary index on a table
///
/// Secondary indexes improve the speed of many read queries at the slight cost of increased storage space and decreased write performance. For more information about secondary indexes, read the article "[Using secondary indexes in RethinkDB](/docs/secondary-indexes/)."
///
/// 
/// RethinkDB supports different types of secondary indexes:
/// 
/// - *Simple indexes* based on the value of a single field.
/// - *Compound indexes* based on multiple fields.
/// - *Multi indexes* based on arrays of values, created when the `multi` optional argument is `true`.
/// - *Geospatial indexes* based on indexes of geometry objects, created when the `geo` optional argument is true.
/// - Indexes based on *arbitrary expressions*.
/// 
/// The `indexFunction` can be an anonymous function or a binary representation obtained from the `function` field of [indexStatus](/api/javascript/index_status). The function must be deterministic, and so cannot use a subquery or the `r.js` command.
/// 
/// If successful, `createIndex` will return an object of the form `{"created": 1}`. If an index by that name already exists on the table, a `ReqlRuntimeError` will be thrown.
/// 
/// {% infobox %}
/// Note that an index may not be immediately available after creation. If your application needs to use indexes immediately after creation, use the [indexWait](/api/javascript/index_wait) command to ensure the indexes are ready before use.
/// {% endinfobox %}
/// 
/// __Example:__ Create a simple index based on the field `postId`.
/// 
/// ```javascript
/// r.table('comments').indexCreate('postId').run(conn, callback)
/// ```
/// 
/// __Example:__ Create a geospatial index based on the field `location`.
/// 
/// ```javascript
/// r.table('places').indexCreate('location', {geo: true}).run(conn, callback)
/// ```
/// 
/// A geospatial index field should contain only geometry objects. It will work with geometry ReQL terms ([getIntersecting](/api/javascript/get_intersecting/) and [getNearest](/api/javascript/get_nearest/)) as well as index-specific terms ([indexStatus](/api/javascript/index_status), [indexWait](/api/javascript/index_wait), [indexDrop](/api/javascript/index_drop) and [indexList](/api/javascript/index_list)). Using terms that rely on non-geometric ordering such as [getAll](/api/javascript/get_all/), [orderBy](/api/javascript/order_by/) and [between](/api/javascript/between/) will result in an error.
/// 
/// __Example:__ Create a simple index based on the nested field `author > name`.
/// 
/// ```javascript
/// r.table('comments').indexCreate('authorName', r.row("author")("name")).run(conn, callback)
/// ```
/// 
/// 
/// __Example:__ Create a compound index based on the fields `postId` and `date`.
/// 
/// ```javascript
/// r.table('comments').indexCreate('postAndDate', [r.row("postId"), r.row("date")]).run(conn, callback)
/// ```
/// 
/// __Example:__ Create a multi index based on the field `authors`.
/// 
/// ```javascript
/// r.table('posts').indexCreate('authors', {multi: true}).run(conn, callback)
/// ```
/// 
/// __Example:__ Create a geospatial multi index based on the field `towers`.
/// 
/// ```javascript
/// r.table('networks').indexCreate('towers', {multi: true, geo: true}).run(conn, callback)
/// ```
/// 
/// __Example:__ Create an index based on an arbitrary expression.
/// 
/// ```javascript
/// r.table('posts').indexCreate('authors', function(doc) {
///     return r.branch(
///         doc.hasFields("updatedAt"),
///         doc("updatedAt"),
///         doc("createdAt")
///     )
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ Create a new secondary index based on an existing one.
/// 
/// ```javascript
/// r.table('posts').indexStatus('authors').nth(0)('function').run(conn, function (func) {
///     r.table('newPosts').indexCreate('authors', func).run(conn, callback);
/// });
/// ```
/// 
/// __Example:__ Rebuild an outdated secondary index on a table.
/// 
/// ```javascript
/// r.table('posts').indexStatus('oldIndex').nth(0).do(function(oldIndex) {
///   return r.table('posts').indexCreate('newIndex', oldIndex("function")).do(function() {
///     return r.table('posts').indexWait('newIndex').do(function() {
///       return r.table('posts').indexRename('newIndex', 'oldIndex', {overwrite: true})
///     })
///   })
/// })
/// ```

                pub fn index_create<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "index_create", Some(Type::INDEX_CREATE), Some(args))
                }
            

                /// Delete a previously created secondary index of this table
///
/// 
/// __Example:__ Drop a secondary index named 'code_name'.
/// 
/// ```javascript
/// r.table('dc').indexDrop('code_name').run(conn, callback)
/// ```
/// 
/// 

                pub fn index_drop<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "index_drop", Some(Type::INDEX_DROP), Some(args))
                }
            

                /// List all the secondary indexes of this table
///
/// 
/// __Example:__ List the available secondary indexes for this table.
/// 
/// ```javascript
/// r.table('marvel').indexList().run(conn, callback)
/// ```
/// 

                pub fn index_list(&self) -> Client {
                    util::make_cmd::<Client>(self, "index_list", Some(Type::INDEX_LIST), None)
                }
            

                /// Rename an existing secondary index on a table
///
/// If the optional argument `overwrite` is specified as `true`, a previously existing index with the new name will be deleted and the index will be renamed. If `overwrite` is `false` (the default) an error will be raised if the new index name already exists.
///
/// 
/// The return value on success will be an object of the format `{renamed: 1}`, or `{renamed: 0}` if the old and new names are the same.
/// 
/// An error will be raised if the old index name does not exist, if the new index name is already in use and `overwrite` is `false`, or if either the old or new index name are the same as the primary key field name.
/// 
/// __Example:__ Rename an index on the comments table.
/// 
/// ```javascript
/// r.table('comments').indexRename('postId', 'messageId').run(conn, callback)
/// ```

                pub fn index_rename<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "index_rename", Some(Type::INDEX_RENAME), Some(args))
                }
            

                /// Get the status of the specified indexes on this table, or the status
/// of all indexes on this table if no indexes are specified
///
/// 
/// The result is an array where for each index, there will be an object like this one:
/// 
/// ```javascript
/// {
///     index: <indexName>,
///     ready: true,
///     function: <binary>,
///     multi: <bool>,
///     geo: <bool>,
///     outdated: <bool>
/// }
/// ```
/// 
/// or this one:
/// 
/// ```javascript
/// {
///     index: <indexName>,
///     ready: false,
///     progress: <float>,
///     function: <binary>,
///     multi: <bool>,
///     geo: <bool>,
///     outdated: <bool>
/// }
/// ```
/// 
/// The `multi` field will be `true` or `false` depending on whether this index was created as a multi index; the `geo` field will be `true` or `false` depending on whether this index was created as a geospatial index. See [indexCreate](/api/javascript/index_create/) for details. The `outdated` field will be true if the index is outdated in the current version of RethinkDB and needs to be rebuilt. The `progress` field is a float between `0` and `1`, indicating how far along the server is in constructing indexes after the most recent change to the table that would affect them. (`0` indicates no such indexes have been constructed; `1` indicates all of them have.)
/// 
/// The `function` field is a binary object containing an opaque representation of the secondary index (including the `multi` argument if specified). It can be passed as the second argument to [indexCreate](/api/javascript/index_create/) to create a new index with the same function; see `indexCreate` for more information.
/// 
/// __Example:__ Get the status of all the indexes on `test`:
/// 
/// ```javascript
/// r.table('test').indexStatus().run(conn, callback)
/// ```
/// 
/// __Example:__ Get the status of the `timestamp` index:
/// 
/// ```javascript
/// r.table('test').indexStatus('timestamp').run(conn, callback)
/// ```
/// 
/// __Example:__ Save the binary representation of the index:
/// 
/// ```javascript
/// var func;
/// r.table('test').indexStatus('timestamp').run(conn, function (err, res) {
///     func = res[0].function;
/// });
/// ```

                pub fn index_status(&self) -> Client {
                    util::make_cmd::<Client>(self, "index_status", Some(Type::INDEX_STATUS), None)
                }
            

                /// Wait for the specified indexes on this table to be ready, or for all
/// indexes on this table to be ready if no indexes are specified
///
/// 
/// The result is an array containing one object for each table index:
/// 
/// ```javascript
/// {
///     index: <indexName>,
///     ready: true,
///     function: <binary>,
///     multi: <bool>,
///     geo: <bool>,
///     outdated: <bool>
/// }
/// ```
/// 
/// See the [indexStatus](/api/javascript/index_status) documentation for a description of the field values.
/// 
/// __Example:__ Wait for all indexes on the table `test` to be ready:
/// 
/// ```javascript
/// r.table('test').indexWait().run(conn, callback)
/// ```
/// 
/// __Example:__ Wait for the index `timestamp` to be ready:
/// 
/// ```javascript
/// r.table('test').indexWait('timestamp').run(conn, callback)
/// ```

                pub fn index_wait(&self) -> Client {
                    util::make_cmd::<Client>(self, "index_wait", Some(Type::INDEX_WAIT), None)
                }
            

                /// Insert documents into a table
///
/// <img src="https://raw.githubusercontent.com/rethinkdb/docs/master/_jekyll/_images/api_illustrations/insert_javascript.png" class="api_command_illustration" />
///
/// Accepts a single document or an array of
///
/// documents.
/// 
/// The optional arguments are:
/// 
/// - `durability`: possible values are `hard` and `soft`. This option will override the table or query's durability setting (set in [run](/api/javascript/run/)). In soft durability mode RethinkDB will acknowledge the write immediately after receiving and caching it, but before the write has been committed to disk.
/// - `returnChanges`:
///     - `true`: return a `changes` array consisting of `old_val`/`new_val` objects describing the changes made, only including the documents actually updated.
///     - `false`: do not return a `changes` array (the default).
///     - `"always"`: behave as `true`, but include all documents the command tried to update whether or not the update was successful. (This was the behavior of `true` pre-2.0.)
/// - `conflict`: Determine handling of inserting documents with the same primary key as existing entries. There are three built-in methods: `"error"`, `"replace"` or `"update"`; alternatively, you may provide a conflict resolution function.
///     - `"error"`: Do not insert the new document and record the conflict as an error. This is the default.
///     - `"replace"`: [Replace](/api/javascript/replace/) the old document in its entirety with the new one.
///     - `"update"`: [Update](/api/javascript/update/) fields of the old document with fields from the new one.
///     - `function (id, oldDoc, newDoc) { return resolvedDoc }`: a function that receives the id, old and new documents as arguments and returns a document which will be inserted in place of the conflicted one.
/// 
/// If `returnChanges` is set to `true` or `"always"`, the `changes` array will follow the same order as the inserted documents. Documents in `changes` for which an error occurs (such as a key conflict) will have a third field, `error`, with an explanation of the error.
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
/// ```javascript
/// r.table("posts").insert({
///     id: 1,
///     title: "Lorem ipsum",
///     content: "Dolor sit amet"
/// }).run(conn, callback)
/// ```
/// 
/// <!-- stop -->
/// 
/// The result will be:
/// 
/// ```javascript
/// {
///     deleted: 0,
///     errors: 0,
///     inserted: 1,
///     replaced: 0,
///     skipped: 0,
///     unchanged: 0
/// }
/// ```
/// 
/// 
/// __Example:__ Insert a document without a defined primary key into the table `posts` where the
/// primary key is `id`.
/// 
/// ```javascript
/// r.table("posts").insert({
///     title: "Lorem ipsum",
///     content: "Dolor sit amet"
/// }).run(conn, callback)
/// ```
/// 
/// RethinkDB will generate a primary key and return it in `generated_keys`.
/// 
/// ```javascript
/// {
///     deleted: 0,
///     errors: 0,
///     generated_keys: [
///         "dd782b64-70a7-43e4-b65e-dd14ae61d947"
///     ],
///     inserted: 1,
///     replaced: 0,
///     skipped: 0,
///     unchanged: 0
/// }
/// ```
/// 
/// Retrieve the document you just inserted with:
/// 
/// ```javascript
/// r.table("posts").get("dd782b64-70a7-43e4-b65e-dd14ae61d947").run(conn, callback)
/// ```
/// 
/// And you will get back:
/// 
/// ```javascript
/// {
///     id: "dd782b64-70a7-43e4-b65e-dd14ae61d947",
///     title: "Lorem ipsum",
///     content: "Dolor sit amet",
/// }
/// ```
/// 
/// 
/// __Example:__ Insert multiple documents into the table `users`.
/// 
/// ```javascript
/// r.table("users").insert([
///     {id: "william", email: "william@rethinkdb.com"},
///     {id: "lara", email: "lara@rethinkdb.com"}
/// ]).run(conn, callback)
/// ```
/// 
/// 
/// __Example:__ Insert a document into the table `users`, replacing the document if it already exists.  
/// 
/// ```javascript
/// r.table("users").insert(
///     {id: "william", email: "william@rethinkdb.com"},
///     {conflict: "replace"}
/// ).run(conn, callback)
/// ```
/// 
/// __Example:__ Copy the documents from `posts` to `postsBackup`.
/// 
/// ```javascript
/// r.table("postsBackup").insert(r.table("posts")).run(conn, callback)
/// ```
/// 
/// 
/// __Example:__ Get back a copy of the inserted document (with its generated primary key).
/// 
/// ```javascript
/// r.table("posts").insert(
///     {title: "Lorem ipsum", content: "Dolor sit amet"},
///     {returnChanges: true}
/// ).run(conn, callback)
/// ```
/// 
/// The result will be
/// 
/// ```javascript
/// {
///     deleted: 0,
///     errors: 0,
///     generated_keys: [
///         "dd782b64-70a7-43e4-b65e-dd14ae61d947"
///     ],
///     inserted: 1,
///     replaced: 0,
///     skipped: 0,
///     unchanged: 0,
///     changes: [
///         {
///             old_val: null,
///             new_val: {
///                 id: "dd782b64-70a7-43e4-b65e-dd14ae61d947",
///                 title: "Lorem ipsum",
///                 content: "Dolor sit amet"
///             }
///         }
///     ]
/// }
/// ```
/// 
/// __Example:__ Provide a resolution function that concatenates memo content in case of conflict.
/// 
/// ```javascript
/// // assume newMemos is a list of memo documents to insert
/// r.table('memos').insert(newMemos, {conflict: function(id, oldDoc, newDoc) {
///     return newDoc.merge(
///         {content: oldDoc('content').add("\n").add(newDoc('content'))}
///     );
/// }}).run(conn, callback)
/// ```

                pub fn insert<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "insert", Some(Type::INSERT), Some(args))
                }
            

                /// Update JSON documents in a table
///
/// Accepts a JSON document, a ReQL expression, or a combination of the two.
///
/// 
/// The optional arguments are:
/// 
/// - `durability`: possible values are `hard` and `soft`. This option will override the table or query's durability setting (set in [run](/api/javascript/run/)). In soft durability mode RethinkDB will acknowledge the write immediately after receiving it, but before the write has been committed to disk.
/// - `returnChanges`:
///     - `true`: return a `changes` array consisting of `old_val`/`new_val` objects describing the changes made, only including the documents actually updated.
///     - `false`: do not return a `changes` array (the default).
///     - `"always"`: behave as `true`, but include all documents the command tried to update whether or not the update was successful. (This was the behavior of `true` pre-2.0.)
/// - `nonAtomic`: if set to `true`, executes the update and distributes the result to replicas in a non-atomic fashion. This flag is required to perform non-deterministic updates, such as those that require reading data from another table.
/// 
/// Update returns an object that contains the following attributes:
/// 
/// - `replaced`: the number of documents that were updated.
/// - `unchanged`: the number of documents that would have been modified except the new value was the same as the old value.
/// - `skipped`: the number of documents that were skipped because the document didn't exist.
/// - `errors`: the number of errors encountered while performing the update.
/// - `first_error`: If errors were encountered, contains the text of the first error.
/// - `deleted` and `inserted`: 0 for an update operation.
/// - `changes`: if `returnChanges` is set to `true`, this will be an array of objects, one for each objected affected by the `update` operation. Each object will have two keys: `{new_val: <new value>, old_val: <old value>}`.
/// 
/// {% infobox alert %}
/// RethinkDB write operations will only throw exceptions if errors occur before any writes. Other errors will be listed in `first_error`, and `errors` will be set to a non-zero count. To properly handle errors with this term, code must both handle exceptions and check the `errors` return value!
/// {% endinfobox %}
/// 
/// __Example:__ Update the status of the post with `id` of `1` to `published`.
/// 
/// ```javascript
/// r.table("posts").get(1).update({status: "published"}).run(conn, callback)
/// ```
/// 
/// __Example:__ Update the status of all posts to `published`.
/// 
/// ```javascript
/// r.table("posts").update({status: "published"}).run(conn, callback)
/// ```
/// 
/// __Example:__ Update the status of all the posts written by William.
/// 
/// ```javascript
/// r.table("posts").filter({author: "William"}).update({status: "published"}).run(conn, callback)
/// ```
/// 
/// {% infobox alert %}
/// Note that `filter`, `getAll` and similar operations do _not_ execute in an atomic fashion with `update`. Read [Consistency guarantees](/docs/consistency) for more details. Also, see the example for conditional updates below for a solution using `branch` in an `update` clause.
/// {% endinfobox %}
/// 
/// __Example:__ Increment the field `view` of the post with `id` of `1`.
/// This query will throw an error if the field `views` doesn't exist.
/// 
/// ```javascript
/// r.table("posts").get(1).update({
///     views: r.row("views").add(1)
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ Increment the field `view` of the post with `id` of `1`.
/// If the field `views` does not exist, it will be set to `0`.
/// 
/// ```javascript
/// r.table("posts").get(1).update({
///     views: r.row("views").add(1).default(0)
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ Perform a conditional update.  
/// If the post has more than 100 views, set the `type` of a post to `hot`, else set it to `normal`.
/// 
/// ```javascript
/// r.table("posts").get(1).update(function(post) {
///     return r.branch(
///         post("views").gt(100),
///         {type: "hot"},
///         {type: "normal"}
///     )
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ Update the field `numComments` with the result of a sub-query. Because this update is not atomic, you must pass the `nonAtomic` flag.
/// 
/// ```javascript
/// r.table("posts").get(1).update({
///     numComments: r.table("comments").filter({idPost: 1}).count()
/// }, {
///     nonAtomic: true
/// }).run(conn, callback)
/// ```
/// 
/// If you forget to specify the `nonAtomic` flag, you will get a `ReqlRuntimeError`:
/// 
/// ```text
/// ReqlRuntimeError: Could not prove function deterministic.  Maybe you want to use the non_atomic flag? 
/// ```
/// 
/// __Example:__ Update the field `numComments` with a random value between 0 and 100. This update cannot be proven deterministic because of `r.js` (and in fact is not), so you must pass the `nonAtomic` flag.
/// 
/// ```javascript
/// r.table("posts").get(1).update({
///     num_comments: r.js("Math.floor(Math.random()*100)")
/// }, {
///     nonAtomic: true
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ Update the status of the post with `id` of `1` using soft durability.
/// 
/// ```javascript
/// r.table("posts").get(1).update({status: "published"}, {durability: "soft"}).run(conn, callback)
/// ```
/// 
/// __Example:__ Increment the field `views` and return the values of the document before and after the update operation.
/// 
/// ```javascript
/// r.table("posts").get(1).update({
///     views: r.row("views").add(1)
/// }, {
///     returnChanges: true
/// }).run(conn, callback)
/// ```
/// 
/// The result will now include a `changes` field:
/// 
/// ```javascript
/// {
///     deleted: 0,
///     errors: 0,
///     inserted: 0,
///     changes: [
///         {
///             new_val: {
///                 id: 1,
///                 author: "Julius_Caesar",
///                 title: "Commentarii de Bello Gallico",
///                 content: "Aleas jacta est",
///                 views: 207
///             },
///             old_val: {
///                 id: 1,
///                 author: "Julius_Caesar",
///                 title: "Commentarii de Bello Gallico",
///                 content: "Aleas jacta est",
///                 views: 206
///             }
///         }
///     ],
///     replaced: 1,
///     skipped: 0,
///     unchanged: 0
/// }
/// ```
/// 
/// 
/// ## Updating nested fields ##
/// 
/// The `update` command supports RethinkDB's [nested field][nf] syntax to update subdocuments. Consider a user table with contact information in this format:
/// 
/// [nf]: /docs/nested-fields/javascript
/// 
/// ```javascript
/// {
/// 	id: 10001,
/// 	name: "Bob Smith",
/// 	contact: {
/// 		phone: {
/// 			work: "408-555-1212",
/// 			home: "408-555-1213",
/// 			cell: "408-555-1214"
/// 		},
/// 		email: {
/// 			work: "bob@smith.com",
/// 			home: "bobsmith@example.com",
/// 			other: "bobbys@moosecall.net"
/// 		},
/// 		im: {
/// 			skype: "Bob Smith",
/// 			aim: "bobmoose",
/// 			icq: "nobodyremembersicqnumbers"
/// 		}
/// 	},
/// 	notes: [
/// 		{
/// 			date: r.time(2014,1,1,'Z'),
/// 			from: "John Doe",
/// 			subject: "My name is even more boring than Bob's"
/// 		},
/// 		{
/// 			date: r.time(2014,2,2,'Z'),
/// 			from: "Bob Smith Sr",
/// 			subject: "Happy Second of February"
/// 		}
/// 	]
/// }
/// ```
/// 
/// __Example:__ Update Bob Smith's cell phone number.
/// 
/// ```javascript
/// r.table("users").get(10001).update(
///     {contact: {phone: {cell: "408-555-4242"}}}
/// ).run(conn, callback)
/// ```
/// 
/// __Example:__ Add another note to Bob Smith's record.
/// 
/// ```javascript
/// var newNote = {
///     date: r.now(),
///     from: "Inigo Montoya",
///     subject: "You killed my father"
/// };
/// r.table("users").get(10001).update(
///     {notes: r.row("notes").append(newNote)}
/// ).run(conn, callback)
/// ```
/// 
/// This will fail if the `notes` field does not exist in the document. To perform this as an "upsert" (update or insert), use the [default][] command to ensure the field is initialized as an empty list.
/// 
/// [default]: /api/javascript/default/
/// 
/// ```javascript
/// r.table("users").get(10001).update(
///     {notes: r.row("notes").default([]).append(newNote)}
/// ).run(conn, callback)
/// ```
/// 
/// __Example:__ Send a note to every user with an ICQ number.
/// 
/// ```javascript
/// var icqNote = {
///     date: r.now(),
///     from: "Admin",
///     subject: "Welcome to the future"
/// };
/// r.table("users").filter(
///     r.row.hasFields({contact: {im: "icq"}})
/// ).update(
///     {notes: r.row("notes").append(icqNote)}
/// ).run(conn, callback)
/// ```
/// 
/// __Example:__ Replace all of Bob's IM records. Normally, `update` will merge nested documents together; to replace the entire `"im"` document, use the [literal][] command.
/// 
/// [literal]: /api/javascript/literal/
/// 
/// ```javascript
/// r.table('users').get(10001).update(
///     {contact: {im: r.literal({aim: "themoosemeister"})}}
/// ).run(conn, callback)
/// ```

                pub fn update<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "update", Some(Type::UPDATE), Some(args))
                }
            

                /// Replace documents in a table
///
/// <img src="https://raw.githubusercontent.com/rethinkdb/docs/master/_jekyll/_images/api_illustrations/replace.png" class="api_command_illustration" />
///
/// Accepts a JSON document or a ReQL expression,
///
/// and replaces the original document with the new one. The new document must
/// have the same primary key as the original document.
/// 
/// The `replace` command can be used to both insert and delete documents. If
/// the "replaced" document has a primary key that doesn't exist in the table,
/// the document will be inserted; if an existing document is replaced with
/// `null`, the document will be deleted. Since `update` and `replace` operations
/// are performed atomically, this allows atomic inserts and deletes as well.
/// 
/// The optional arguments are:
/// 
/// - `durability`: possible values are `hard` and `soft`. This option will override
///   the table or query's durability setting (set in [run](/api/javascript/run/)).
///   In soft durability mode RethinkDB will acknowledge the write immediately after
///   receiving it, but before the write has been committed to disk.
/// - `returnChanges`:
///     - `true`: return a `changes` array consisting of `old_val`/`new_val` objects
///       describing the changes made, only including the documents actually
///       updated.
///     - `false`: do not return a `changes` array (the default).
///     - `"always"`: behave as `true`, but include all documents the command tried
///       to update whether or not the update was successful. (This was the behavior
///       of `true` pre-2.0.)
/// - `nonAtomic`: if set to `true`, executes the replacement and distributes the
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
///   object will have two keys: `{new_val: <new value>, old_val: <old value>}`.
/// 
/// {% infobox alert %}
/// RethinkDB write operations will only throw exceptions if errors occur before any writes. Other errors will be listed in `first_error`, and `errors` will be set to a non-zero count. To properly handle errors with this term, code must both handle exceptions and check the `errors` return value!
/// {% endinfobox %}
/// 
/// __Example:__ Replace the document with the primary key `1`.
/// 
/// ```javascript
/// r.table("posts").get(1).replace({
///     id: 1,
///     title: "Lorem ipsum",
///     content: "Aleas jacta est",
///     status: "draft"
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ Remove the field `status` from all posts.
/// 
/// ```javascript
/// r.table("posts").replace(function(post) {
///     return post.without("status")
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ Remove all the fields that are not `id`, `title` or `content`.
/// 
/// ```javascript
/// r.table("posts").replace(function(post) {
///     return post.pluck("id", "title", "content")
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ Replace the document with the primary key `1` using soft durability.
/// 
/// ```javascript
/// r.table("posts").get(1).replace({
///     id: 1,
///     title: "Lorem ipsum",
///     content: "Aleas jacta est",
///     status: "draft"
/// }, {
///     durability: "soft"
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ Replace the document with the primary key `1` and return the values of the document before
/// and after the replace operation.
/// 
/// ```javascript
/// r.table("posts").get(1).replace({
///     id: 1,
///     title: "Lorem ipsum",
///     content: "Aleas jacta est",
///     status: "published"
/// }, {
///     returnChanges: true
/// }).run(conn, callback)
/// ```
/// 
/// The result will have two fields `old_val` and `new_val`.
/// 
/// ```javascript
/// {
///     deleted: 0,
///     errors: 0,
///     inserted: 0,
///     changes: [
///         {
///             new_val: {
///                 id:1,
///                 title: "Lorem ipsum"
///                 content: "Aleas jacta est",
///                 status: "published",
///             },
///             old_val: {
///                 id:1,
///                 title: "Lorem ipsum"
///                 content: "TODO",
///                 status: "draft",
///                 author: "William",
///             }
///         }
///     ],
///     replaced: 1,
///     skipped: 0,
///     unchanged: 0
/// }
/// ```

                pub fn replace<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "replace", Some(Type::REPLACE), Some(args))
                }
            

                /// Delete one or more documents from a table
///
/// <img src="https://raw.githubusercontent.com/rethinkdb/docs/master/_jekyll/_images/api_illustrations/delete-vector.png" class="api_command_illustration" />
///
/// 
/// The optional arguments are:
/// 
/// - `durability`: possible values are `hard` and `soft`. This option will override the
/// table or query's durability setting (set in [run](/api/javascript/run/)).  
/// In soft durability mode RethinkDB will acknowledge the write immediately after
/// receiving it, but before the write has been committed to disk.
/// - `returnChanges`:
///     - `true`: return a `changes` array consisting of `old_val`/`new_val` objects describing the changes made, only including the documents actually updated.
///     - `false`: do not return a `changes` array (the default).
///     - `"always"`: behave as `true`, but include all documents the command tried to update whether or not the update was successful. (This was the behavior of `true` pre-2.0.)
/// 
/// 
/// Delete returns an object that contains the following attributes:
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
/// ```javascript
/// r.table("comments").get("7eab9e63-73f1-4f33-8ce4-95cbea626f59").delete().run(conn, callback)
/// ```
/// 
/// 
/// __Example:__ Delete all documents from the table `comments`.
/// 
/// ```javascript
/// r.table("comments").delete().run(conn, callback)
/// ```
/// 
/// 
/// __Example:__ Delete all comments where the field `idPost` is `3`.
/// 
/// ```javascript
/// r.table("comments").filter({idPost: 3}).delete().run(conn, callback)
/// ```
/// 
/// 
/// __Example:__ Delete a single document from the table `comments` and return its value.
/// 
/// ```javascript
/// r.table("comments").get("7eab9e63-73f1-4f33-8ce4-95cbea626f59").delete({returnChanges: true}).run(conn, callback)
/// ```
/// 
/// The result look like:
/// 
/// ```javascript
/// {
///     deleted: 1,
///     errors: 0,
///     inserted: 0,
///     changes: [
///         {
///             new_val: null,
///             old_val: {
///                 id: "7eab9e63-73f1-4f33-8ce4-95cbea626f59",
///                 author: "William",
///                 comment: "Great post",
///                 idPost: 3
///             }
///         }
///     ],
///     replaced: 0,
///     skipped: 0,
///     unchanged: 0
/// }
/// ```
/// 
/// 
/// __Example:__ Delete all documents from the table `comments` without waiting for the
/// operation to be flushed to disk.
/// 
/// ```javascript
/// r.table("comments").delete({durability: "soft"}).run(conn, callback)
/// ```

                pub fn delete(&self) -> Client {
                    util::make_cmd::<Client>(self, "delete", Some(Type::DELETE), None)
                }
            

                /// `sync` ensures that writes on a given table are written to permanent storage
///
/// Queries
///
/// that specify soft durability (`{durability: 'soft'}`) do not give such guarantees, so
/// `sync` can be used to ensure the state of these queries. A call to `sync` does not return
/// until all previous writes to the table are persisted.
/// 
/// If successful, the operation returns an object: `{synced: 1}`.
/// 
/// __Example:__ After having updated multiple heroes with soft durability, we now want to wait
/// until these changes are persisted.
/// 
/// ```javascript
/// r.table('marvel').sync().run(conn, callback)
/// ```
/// 
/// 

                pub fn sync(&self) -> Client {
                    util::make_cmd::<Client>(self, "sync", Some(Type::SYNC), None)
                }
            

                /// Reference a database
///
/// 
/// The `db` command is optional. If it is not present in a query, the query will run against the default database for the connection, specified in the `db` argument to [connect](/api/javascript/connect).
/// 
/// __Example:__ Explicitly specify a database for a query.
/// 
/// ```javascript
/// r.db('heroes').table('marvel').run(conn, callback)
/// ```
/// 

                pub fn db<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "db", Some(Type::DB), Some(args))
                }
            

                /// Return all documents in a table
///
/// Other commands may be chained after `table` to return a subset of documents (such as [get](/api/javascript/get/) and [filter](/api/javascript/filter/)) or perform further processing.
///
/// 
/// __Example:__ Return all documents in the table 'marvel' of the default database.
/// 
/// ```javascript
/// r.table('marvel').run(conn, callback)
/// ```
/// 
/// __Example:__ Return all documents in the table 'marvel' of the database 'heroes'.
/// 
/// ```javascript
/// r.db('heroes').table('marvel').run(conn, callback)
/// ```
/// 
/// There are two optional arguments.
/// 
/// * `readMode`: One of three possible values affecting the consistency guarantee for the table read:
///     * `single` returns values that are in memory (but not necessarily written to disk) on the primary replica. This is the default.
///     * `majority` will only return values that are safely committed on disk on a majority of replicas. This requires sending a message to every replica on each read, so it is the slowest but most consistent.
///     * `outdated` will return values that are in memory on an arbitrarily-selected replica. This is the fastest but least consistent.
/// * `identifierFormat`: possible values are `name` and `uuid`, with a default of `name`. If set to `uuid`, then [system tables](/docs/system-tables/) will refer to servers, databases and tables by UUID rather than name. (This only has an effect when used with system tables.)
/// 
/// __Example:__ Allow potentially out-of-date data in exchange for faster reads.
/// 
/// ```javascript
/// r.db('heroes').table('marvel', {readMode: 'outdated'}).run(conn, callback)
/// ```

                pub fn table<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "table", Some(Type::TABLE), Some(args))
                }
            

                /// Get a document by primary key
///
/// 
/// If no document exists with that primary key, `get` will return `null`.
/// 
/// __Example:__ Find a document by UUID.
/// 
/// ```javascript
/// r.table('posts').get('a9849eef-7176-4411-935b-79a6e3c56a74').run(conn, callback);
/// ```
/// 
/// __Example:__ Find a document and merge another document with it.
/// 
/// ```javascript
/// r.table('heroes').get(3).merge(
///     { powers: ['invisibility', 'speed'] }
/// ).run(conn, callback);
/// ```
/// 
/// ___Example:__ Subscribe to a document's [changefeed](/docs/changefeeds/javascript).
/// 
/// ```javascript
/// r.table('heroes').get(3).changes().run(conn, callback);
/// ```

                pub fn get<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "get", Some(Type::GET), Some(args))
                }
            

                /// Get all documents where the given value matches the value of the requested index
///
/// <img src="https://raw.githubusercontent.com/rethinkdb/docs/master/_jekyll/_images/api_illustrations/get-all.png" class="api_command_illustration" />
///
/// 
/// __Example:__ Secondary index keys are not guaranteed to be unique so we cannot query via [get](/api/javascript/get/) when using a secondary index.
/// 
/// ```javascript
/// r.table('marvel').getAll('man_of_steel', {index:'code_name'}).run(conn, callback)
/// ```
/// 
/// __Example:__ Without an index argument, we default to the primary index. While `get` will either return the document or `null` when no document with such a primary key value exists, this will return either a one or zero length stream.
/// 
/// ```javascript
/// r.table('dc').getAll('superman').run(conn, callback)
/// ```
/// 
/// __Example:__ You can get multiple documents in a single call to `getAll`.
/// 
/// ```javascript
/// r.table('dc').getAll('superman', 'ant man').run(conn, callback)
/// ```
/// 
/// {% infobox %}
/// __Note:__ `getAll` does not perform any de-duplication. If you pass the same key more than once, the same document will be returned multiple times.
/// {% endinfobox %}
/// 
/// __Example:__ You can use [args](/api/javascript/args/) with `getAll` to retrieve multiple documents whose keys are in a list. This uses `getAll` to get a list of female superheroes, coerces that to an array, and then gets a list of villains who have those superheroes as enemies.
/// 
/// ```javascript
/// r.do(
///     r.table('heroes').getAll('f', {index: 'gender'})('id').coerceTo('array'),
///     function(heroines) {
///         return r.table('villains').getAll(r.args(heroines));
///     }
/// ).run(conn, callback)
/// ```
/// 
/// Calling `getAll` with zero arguments&mdash;which could happen in this example if the `heroines` list had no elements&mdash;will return nothing, i.e., a zero length stream.
/// 
/// Secondary indexes can be used in extremely powerful ways with `getAll` and other commands; read the full article on [secondary indexes](/docs/secondary-indexes) for examples using boolean operations, `contains` and more.

                pub fn get_all<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "get_all", Some(Type::GET_ALL), Some(args))
                }
            

                /// Get all documents between two keys
///
/// Accepts three optional arguments: `index`, `leftBound`, and `rightBound`. If `index` is set to the name of a secondary index, `between` will return all documents where that index's value is in the specified range (it uses the primary key by default). `leftBound` or `rightBound` may be set to `open` or `closed` to indicate whether or not to include that endpoint of the range (by default, `leftBound` is closed and `rightBound` is open).
///
/// 
/// You may also use the special constants `r.minval` and `r.maxval` for boundaries, which represent "less than any index key" and "more than any index key" respectively. For instance, if you use `r.minval` as the lower key, then `between` will return all documents whose primary keys (or indexes) are less than the specified upper key.
/// 
/// If you use arrays as indexes (compound indexes), they will be sorted using [lexicographical order][lo]. Take the following range as an example:
/// 
/// ```text
/// [[1, "c"] ... [5, "e"]]
/// ```
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
/// ```javascript
/// r.table('marvel').between(10, 20).run(conn, callback);
/// ```
/// 
/// __Example:__ Find all users with primary key >= 10 and <= 20 (an interval closed on both sides).
/// 
/// ```javascript
/// r.table('marvel').between(10, 20, {rightBound: 'closed'}).run(conn, callback);
/// ```
/// 
/// __Example:__ Find all users with primary key < 20.
/// 
/// ```javascript
/// r.table('marvel').between(r.minval, 20).run(conn, callback);
/// ```
/// 
/// __Example:__ Find all users with primary key > 10.
/// 
/// ```javascript
/// r.table('marvel').between(10, r.maxval, {leftBound: 'open'}).run(conn, callback);
/// ```
/// 
/// __Example:__ Between can be used on secondary indexes too. Just pass an optional index argument giving the secondary index to query.
/// 
/// ```javascript
/// r.table('dc').between('dark_knight', 'man_of_steel', {index: 'code_name'}).run(conn, callback);
/// ```
/// 
/// __Example:__ Get all users whose full name is between "John Smith" and "Wade Welles."
/// 
/// ```javascript
/// r.table("users").between(["Smith", "John"], ["Welles", "Wade"],
///   {index: "full_name"}).run(conn, callback);
/// ```
/// 
/// __Example:__ Get the top 10 ranked teams in order.
/// 
/// ```javascript
/// r.table("teams").orderBy({index: "rank"}).between(1, 11).run(conn, callback);
/// ```
/// 
/// __Note:__ When `between` is chained after [orderBy](/api/javascript/order_by), both commands must use the same index; `between` will default to the index `orderBy` is using, so in this example `"rank"` is automatically being used by `between`. Trying to specify another index will result in a `ReqlRuntimeError`.
/// 
/// __Example:__ Subscribe to a [changefeed](/docs/changefeeds/javascript) of teams ranked in the top 10.
/// 
/// ```javascript
/// r.table("teams").between(1, 11, {index: "rank"}).changes().run(conn, callback);
/// ```
/// 
/// {% infobox %}
/// The `between` command works with secondary indexes on date fields, but will not work with unindexed date fields. To test whether a date value is between two other dates, use the [during](/api/javascript/during) command, not `between`.
/// 
/// Secondary indexes can be used in extremely powerful ways with `between` and other commands; read the full article on [secondary indexes](/docs/secondary-indexes) for examples using boolean operations, `contains` and more.
/// 
/// RethinkDB uses byte-wise ordering for `between` and does not support Unicode collations; non-ASCII characters will be sorted by UTF-8 codepoint.
/// {% endinfobox %}

                pub fn between<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "between", Some(Type::BETWEEN), Some(args))
                }
            

                /// Return all the elements in a sequence for which the given predicate is true
///
/// The return value of `filter` will be the same as the input (sequence, stream, or array). Documents can be filtered in a variety of ways&mdash;ranges, nested values, boolean conditions, and the results of anonymous functions.
///
/// 
/// By default, `filter` will silently skip documents with missing fields: if the predicate tries to access a field that doesn't exist (for instance, the predicate `{age: 30}` applied to a document with no `age` field), that document will not be returned in the result set, and no error will be generated. This behavior can be changed with the `default` optional argument.
/// 
/// * If `default` is set to `true`, documents with missing fields will be returned rather than skipped.
/// * If `default` is set to `r.error()`, an `ReqlRuntimeError` will be thrown when a document with a missing field is tested.
/// * If `default` is set to `false` (the default), documents with missing fields will be skipped.
/// 
/// {% infobox %}
/// __Note:__ `filter` does not use secondary indexes. For retrieving documents via secondary indexes, consider [getAll](/api/javascript/get_all/), [between](/api/javascript/between/) and [eqJoin](/api/javascript/eq_join/).
/// {% endinfobox %}
/// 
/// ## Basic predicates ##
/// 
/// __Example:__ Get all users who are 30 years old.
/// 
/// 
/// ```javascript
/// r.table('users').filter({age: 30}).run(conn, callback);
/// ```
/// 
/// The predicate `{age: 30}` selects documents in the `users` table with an `age` field whose value is `30`. Documents with an `age` field set to any other value *or* with no `age` field present are skipped.
/// 
/// <!-- stop -->
/// 
/// While the `{field: value}` style of predicate is useful for exact matches, a more general way to write a predicate is to use the [row](/api/javascript/row) command with a comparison operator such as [eq](/api/javascript/eq) or [gt](/api/javascript/gt), or to use an anonymous function that returns `true` or `false`.
/// 
/// ```javascript
/// r.table('users').filter(r.row("age").eq(30)).run(conn, callback);
/// ```
/// 
/// In this case, the predicate `r.row("age").eq(30)` returns `true` if the field `age` is equal to 30. You can write this predicate as an anonymous function instead:
/// 
/// ```javascript
/// r.table('users').filter(function (user) {
///     return user("age").eq(30);
/// }).run(conn, callback);
/// ```
/// 
/// Predicates to `filter` are evaluated on the server, and must use ReQL expressions. You cannot use standard JavaScript comparison operators such as `==`, `<`/`>` and `||`/`&&`.
/// 
/// Also, predicates must evaluate document fields. They cannot evaluate [secondary indexes](/docs/secondary-indexes/).
/// 
/// __Example:__ Get all users who are more than 18 years old.
/// 
/// ```javascript
/// r.table("users").filter(r.row("age").gt(18)).run(conn, callback)
/// ```
/// 
/// 
/// __Example:__ Get all users who are less than 18 years old and more than 13 years old.
/// 
/// ```javascript
/// r.table("users").filter(
///     r.row("age").lt(18).and(r.row("age").gt(13))
/// ).run(conn, callback);
/// ```
/// 
/// 
/// __Example:__ Get all users who are more than 18 years old or have their parental consent.
/// 
/// ```javascript
/// r.table("users").filter(
///     r.row("age").ge(18).or(r.row("hasParentalConsent"))
/// ).run(conn, callback);
/// ```
/// 
/// ## More complex predicates ##
/// 
/// __Example:__ Retrieve all users who subscribed between January 1st, 2012
/// (included) and January 1st, 2013 (excluded).
/// 
/// ```javascript
/// r.table("users").filter(function (user) {
///     return user("subscriptionDate").during(
///         r.time(2012, 1, 1, 'Z'), r.time(2013, 1, 1, 'Z'));
/// }).run(conn, callback);
/// ```
/// 
/// __Example:__ Retrieve all users who have a gmail account (whose field `email` ends with `@gmail.com`).
/// 
/// ```javascript
/// r.table("users").filter(function (user) {
///     return user("email").match("@gmail.com$");
/// }).run(conn, callback);
/// ```
/// 
/// __Example:__ Filter based on the presence of a value in an array.
/// 
/// Given this schema for the `users` table:
/// 
/// ```javascript
/// {
///     name: String
///     placesVisited: [String]
/// }
/// ```
/// 
/// Retrieve all users whose field `placesVisited` contains `France`.
/// 
/// ```javascript
/// r.table("users").filter(function(user) {
///     return user("placesVisited").contains("France")
/// }).run( conn, callback)
/// ```
/// 
/// __Example:__ Filter based on nested fields.
/// 
/// Given this schema for the `users` table:
/// 
/// ```javascript
/// {
///     id: String
///     name: {
///         first: String,
///         middle: String,
///         last: String
///     }
/// }
/// ```
/// 
/// Retrieve all users named "William Adama" (first name "William", last name
/// "Adama"), with any middle name.
/// 
/// 
/// ```javascript
/// r.table("users").filter({
///     name: {
///         first: "William",
///         last: "Adama"
///     }
/// }).run(conn, callback)
/// ```
/// 
/// If you want an exact match for a field that is an object, you will have to use `r.literal`.
/// 
/// Retrieve all users named "William Adama" (first name "William", last name
/// "Adama"), and who do not have a middle name.
/// 
/// ```javascript
/// r.table("users").filter(r.literal({
///     name: {
///         first: "William",
///         last: "Adama"
///     }
/// })).run(conn, callback)
/// ```
/// 
/// You may rewrite these with anonymous functions.
/// 
/// ```javascript
/// r.table("users").filter(function(user) {
///     return user("name")("first").eq("William")
///         .and(user("name")("last").eq("Adama"));
/// }).run(conn, callback);
/// 
/// r.table("users").filter(function(user) {
///     return user("name").eq({
///         first: "William",
///         last: "Adama"
///     });
/// }).run(conn, callback);
/// ```
/// 
/// ## Handling missing fields ##
/// 
/// By default, documents missing fields tested by the `filter` predicate are skipped. In the previous examples, users without an `age` field are not returned. By passing the optional `default` argument to `filter`, you can change this behavior.
/// 
/// __Example:__ Get all users less than 18 years old or whose `age` field is missing.
/// 
/// ```javascript
/// r.table("users").filter(
///     r.row("age").lt(18), {default: true}
/// ).run(conn, callback);
/// ```
/// 
/// __Example:__ Get all users more than 18 years old. Throw an error if a
/// document is missing the field `age`.
/// 
/// ```javascript
/// r.table("users").filter(
///     r.row("age").gt(18), {default: r.error()}
/// ).run(conn, callback);
/// ```
/// 
/// __Example:__ Get all users who have given their phone number (all the documents whose field `phoneNumber` exists and is not `null`).
/// 
/// ```javascript
/// r.table('users').filter(function (user) {
///     return user.hasFields('phoneNumber');
/// }).run(conn, callback);
/// ```
/// 
/// __Example:__ Get all users with an "editor" role or an "admin" privilege.
/// 
/// ```javascript
/// r.table('users').filter(function (user) {
///     return (user('role').eq('editor').default(false).
///         or(user('privilege').eq('admin').default(false)));
/// }).run(conn, callback);
/// ```
/// 
/// Instead of using the `default` optional argument to `filter`, we have to use default values on the fields within the `or` clause. Why? If the field on the left side of the `or` clause is missing from a document&mdash;in this case, if the user doesn't have a `role` field&mdash;the predicate will generate an error, and will return `false` (or the value the `default` argument is set to) without evaluating the right side of the `or`. By using `.default(false)` on the fields, each side of the `or` will evaluate to either the field's value or `false` if the field doesn't exist.

                pub fn filter<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "filter", Some(Type::FILTER), Some(args))
                }
            

                /// Returns an inner join of two sequences
///
/// 
/// The returned sequence represents an intersection of the left-hand sequence and the right-hand sequence: each row of the left-hand sequence will be compared with each row of the right-hand sequence to find all pairs of rows which satisfy the predicate. Each matched pair of rows of both sequences are combined into a result row. In most cases, you will want to follow the join with [zip](/api/javascript/zip) to combine the left and right results.
/// 
/// {% infobox %}
/// Note that `innerJoin` is slower and much less efficient than using [eqJoin](/api/javascript/eq_join/) or [concatMap](/api/javascript/concat_map/) with [getAll](/api/javascript/get_all/). You should avoid using `innerJoin` in commands when possible.
/// {% endinfobox %}
/// 
/// __Example:__ Return a list of all matchups between Marvel and DC heroes in which the DC hero could beat the Marvel hero in a fight.
/// 
/// ```javascript
/// r.table('marvel').innerJoin(r.table('dc'), function(marvelRow, dcRow) {
///     return marvelRow('strength').lt(dcRow('strength'))
/// }).zip().run(conn, callback)
/// ```
/// 
/// <!-- stop -->
/// 
/// (Compare this to an [outerJoin](/api/javascript/outer_join) with the same inputs and predicate, which would return a list of *all* Marvel heroes along with any DC heroes with a higher strength.)

                pub fn inner_join<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "inner_join", Some(Type::INNER_JOIN), Some(args))
                }
            

                /// Returns a left outer join of two sequences
///
/// The returned sequence represents a union of the left-hand sequence and the right-hand sequence: all documents in the left-hand sequence will be returned, each matched with a document in the right-hand sequence if one satisfies the predicate condition. In most cases, you will want to follow the join with [zip](/api/javascript/zip) to combine the left and right results.
///
/// 
/// 
/// {% infobox %}
/// Note that `outerJoin` is slower and much less efficient than using [concatMap](/api/javascript/concat_map/) with [getAll](/api/javascript/get_all). You should avoid using `outerJoin` in commands when possible.
/// {% endinfobox %}
/// 
/// __Example:__ Return a list of all Marvel heroes, paired with any DC heroes who could beat them in a fight.
/// 
/// ```javascript
/// r.table('marvel').outerJoin(r.table('dc'), function(marvelRow, dcRow) {
///     return marvelRow('strength').lt(dcRow('strength'))
/// }).run(conn, callback)
/// ```
/// 
/// (Compare this to an [innerJoin](/api/javascript/inner_join) with the same inputs and predicate, which would return a list only of the matchups in which the DC hero has the higher strength.)

                pub fn outer_join<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "outer_join", Some(Type::OUTER_JOIN), Some(args))
                }
            

                /// <img alt="Data Modeling Illustration" class="api_command_illustration" src="https://raw
///
/// githubusercontent.com/rethinkdb/docs/master/_jekyll/_images/api_illustrations/table-joins.png" />
///
/// 
/// Join tables using a field or function on the left-hand sequence matching primary keys or secondary indexes on the right-hand table. `eqJoin` is more efficient than other ReQL join types, and operates much faster. Documents in the result set consist of pairs of left-hand and right-hand documents, matched when the field on the left-hand side exists and is non-null and an entry with that field's value exists in the specified index on the right-hand side.
/// 
/// The result set of `eqJoin` is a stream or array of objects. Each object in the returned set will be an object of the form `{ left: <left-document>, right: <right-document> }`, where the values of `left` and `right` will be the joined documents. Use the <code><a href="/api/javascript/zip/">zip</a></code> command to merge the `left` and `right` fields together.
/// 
/// The results from `eqJoin` are, by default, not ordered. The optional `ordered: true` parameter will cause `eqJoin` to order the output based on the left side input stream. (If there are multiple matches on the right side for a document on the left side, their order is not guaranteed even if `ordered` is `true`.) Requiring ordered results can significantly slow down `eqJoin`, and in many circumstances this ordering will not be required. (See the first example, in which ordered results are obtained by using `orderBy` after `eqJoin`.)
/// 
/// Suppose the players table contains these documents:
/// 
/// ```javascript
/// [
///     { id: 1, player: 'George', gameId: 1 },
///     { id: 2, player: 'Agatha', gameId: 3 },
///     { id: 3, player: 'Fred', gameId: 2 },
///     { id: 4, player: 'Marie', gameId: 2 },
///     { id: 5, player: 'Earnest', gameId: 1 },
///     { id: 6, player: 'Beth', gameId: 3 }
/// ]
/// ```
/// 
/// The games table contains these documents:
/// 
/// ```javascript
/// [
///     { id: 1, field: 'Little Delving' },
///     { id: 2, field: 'Rushock Bog' },
///     { id: 3, field: 'Bucklebury' }
/// ]
/// ```
/// 
/// __Example:__ Match players with the games they've played against one another.
/// 
/// Join these tables using `gameId` on the player table and `id` on the games table:
/// 
/// ```javascript
/// r.table('players').eqJoin('gameId', r.table('games')).run(conn, callback)
/// ```
/// 
/// This will return a result set such as the following:
/// 
/// ```javascript
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
/// ```javascript
/// r.table('players').eqJoin('gameId', r.table('games')).without({right: "id"}).zip().orderBy('gameId').run(conn, callback)
/// 
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
/// ```javascript
/// r.table('players').eqJoin('cityId', r.table('arenas'), {index: 'cityId'}).run(conn, callback)
/// ```
/// 
/// __Example:__ Use a nested key as the join field. Suppose the documents in the players table were structured like this:
/// 
/// ```javascript
/// { id: 1, player: 'George', game: {id: 1} },
/// { id: 2, player: 'Agatha', game: {id: 3} },
/// ...
/// ```
/// 
/// Simply specify the field using the `row` command instead of a string.
/// 
/// ```javascript
/// r.table('players').eqJoin(r.row('game')('id'), r.table('games')).without({right: 'id'}).zip()
/// 
/// [
///     { "field": "Little Delving", "game": { "id": 1 }, "id": 5, "player": "Earnest" },
///     { "field": "Little Delving", "game": { "id": 1 }, "id": 1, "player": "George" },
///     ...
/// ]
/// ```
/// 
/// __Example:__ Use a function instead of a field to join on a more complicated expression. Suppose the players have lists of favorite games ranked in order in a field such as `favorites: [3, 2, 1]`. Get a list of players and their top favorite:
/// 
/// ```javascript
/// r.table('players').eqJoin(function (player) {
///     return player('favorites').nth(0)
/// }, r.table('games')).without([{left: ['favorites', 'gameId', 'id']}, {right: 'id'}]).zip()
/// ```
/// 
/// Result:
/// 
/// ```javascript
/// [
/// 	{ "field": "Rushock Bog", "name": "Fred" },
/// 	{ "field": "Little Delving", "name": "George" },
/// 	...
/// ]
/// ```

                pub fn eq_join<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "eq_join", Some(Type::EQ_JOIN), Some(args))
                }
            

                /// Used to 'zip' up the result of a join by merging the 'right' fields into 'left' fields of each member of the sequence
///
/// 
/// __Example:__ 'zips up' the sequence by merging the left and right fields produced by a join.
/// 
/// ```javascript
/// r.table('marvel').eqJoin('main_dc_collaborator', r.table('dc'))
///     .zip().run(conn, callback)
/// ```
/// 
/// 

                pub fn zip(&self) -> Client {
                    util::make_cmd::<Client>(self, "zip", Some(Type::ZIP), None)
                }
            

                /// Transform each element of one or more sequences by applying a mapping function to them
///
/// If `map` is run with two or more sequences, it will iterate for as many items as there are in the shortest sequence.
///
/// 
/// Note that `map` can only be applied to sequences, not single values. If you wish to apply a function to a single value/selection (including an array), use the [do](/api/javascript/do) command.
/// 
/// __Example:__ Return the first five squares.
/// 
/// ```javascript
/// r.expr([1, 2, 3, 4, 5]).map(function (val) {
///     return val.mul(val);
/// }).run(conn, callback);
/// // Result passed to callback
/// [1, 4, 9, 16, 25]
/// ```
/// 
/// __Example:__ Sum the elements of three sequences.
/// 
/// ```javascript
/// var sequence1 = [100, 200, 300, 400];
/// var sequence2 = [10, 20, 30, 40];
/// var sequence3 = [1, 2, 3, 4];
/// r.map(sequence1, sequence2, sequence3, function (val1, val2, val3) {
///     return val1.add(val2).add(val3);
/// }).run(conn, callback);
/// // Result passed to callback
/// [111, 222, 333, 444]
/// ```
/// 
/// __Example:__ Rename a field when retrieving documents using `map` and [merge](/api/javascript/merge/).
/// 
/// This example renames the field `id` to `userId` when retrieving documents from the table `users`.
/// 
/// ```javascript
/// r.table('users').map(function (doc) {
///     return doc.merge({userId: doc('id')}).without('id');
/// }).run(conn, callback);
/// ```
/// 
/// Note that in this case, [row](/api/javascript/row) may be used as an alternative to writing an anonymous function, as it returns the same value as the function parameter receives:
/// 
/// ```javascript
/// r.table('users').map(
///     r.row.merge({userId: r.row('id')}).without('id');
/// }).run(conn, callback);
/// ```
/// 
/// 
/// __Example:__ Assign every superhero an archenemy.
/// 
/// ```javascript
/// r.table('heroes').map(r.table('villains'), function (hero, villain) {
///     return hero.merge({villain: villain});
/// }).run(conn, callback);
/// ```

                pub fn map<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "map", Some(Type::MAP), Some(args))
                }
            

                /// Plucks one or more attributes from a sequence of objects, filtering out any objects in the sequence that do not have the specified fields
///
/// Functionally, this is identical to [hasFields](/api/javascript/has_fields/) followed by [pluck](/api/javascript/pluck/) on a sequence.
///
/// 
/// __Example:__ Get a list of users and their posts, excluding any users who have not made any posts.
/// 
/// Existing table structure:
/// 
/// ```javascript
/// [
///     { 'id': 1, 'user': 'bob', 'email': 'bob@foo.com', 'posts': [ 1, 4, 5 ] },
///     { 'id': 2, 'user': 'george', 'email': 'george@foo.com' },
///     { 'id': 3, 'user': 'jane', 'email': 'jane@foo.com', 'posts': [ 2, 3, 6 ] }
/// ]
/// ```
/// 
/// Command and output:
/// 
/// ```javascript
/// > r.table('users').withFields('id', 'user', 'posts').run(conn, callback)
/// // Result passed to callback
/// [
///     { 'id': 1, 'user': 'bob', 'posts': [ 1, 4, 5 ] },
///     { 'id': 3, 'user': 'jane', 'posts': [ 2, 3, 6 ] }
/// ]
/// ```
/// 
/// __Example:__ Use the [nested field syntax](/docs/nested-fields/) to get a list of users with cell phone numbers in their contacts.
/// 
/// ```javascript
/// r.table('users').withFields('id', 'user', {contact: {phone: "work"}).run(conn, callback)
/// ```

                pub fn with_fields<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "with_fields", Some(Type::WITH_FIELDS), Some(args))
                }
            

                /// Concatenate one or more elements into a single sequence using a mapping function
///
/// 
/// `concatMap` works in a similar fashion to [map](/api/javascript/map/), applying the given function to each element in a sequence, but it will always return a single sequence. If the mapping function returns a sequence, `map` would produce a sequence of sequences:
/// 
/// ```javascript
/// r.expr([1, 2, 3]).map(function(x) { return [x, x.mul(2)] }).run(conn, callback)
/// ```
/// 
/// Result:
/// 
/// ```javascript
/// [[1, 2], [2, 4], [3, 6]]
/// ```
/// 
/// Whereas `concatMap` with the same mapping function would merge those sequences into one:
/// 
/// ```javascript
/// r.expr([1, 2, 3]).concatMap(function(x) { return [x, x.mul(2)] }).run(conn, callback)
/// ```
/// 
/// Result:
/// 
/// ```javascript
/// [1, 2, 2, 4, 3, 6]
/// ```
/// 
/// The return value, array or stream, will be the same type as the input.
/// 
/// __Example:__ Construct a sequence of all monsters defeated by Marvel heroes. The field "defeatedMonsters" is an array of one or more monster names.
/// 
/// ```javascript
/// r.table('marvel').concatMap(function(hero) {
///     return hero('defeatedMonsters')
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ Simulate an [eqJoin](/api/javascript/eq_join/) using `concatMap`. (This is how ReQL joins are implemented internally.)
/// 
/// ```javascript
/// r.table("posts").concatMap(function(post) {
/// 	return r.table("comments").getAll(
/// 		post("id"),
/// 		{ index:"postId" }
/// 	).map(function(comment) {
/// 		return { left: post, right: comment }
/// 	})
/// }).run(conn, callback)
/// ```

                pub fn concat_map<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "concat_map", Some(Type::CONCAT_MAP), Some(args))
                }
            

                /// Sort the sequence by document values of the given key(s)
///
/// To specify
///
/// the ordering, wrap the attribute with either `r.asc` or `r.desc`
/// (defaults to ascending).
/// 
/// __Note:__ RethinkDB uses byte-wise ordering for `orderBy` and does not support Unicode collations; non-ASCII characters will be sorted by UTF-8 codepoint. For more information on RethinkDB's sorting order, read the section in [ReQL data types](/docs/data-types/#sorting-order).
/// 
/// Sorting without an index requires the server to hold the sequence in
/// memory, and is limited to 100,000 documents (or the setting of the `arrayLimit` option for [run](/api/javascript/run)). Sorting with an index can
/// be done on arbitrarily large tables, or after a [between](/api/javascript/between/) command
/// using the same index. This applies to both secondary indexes and the primary key (e.g., `{index: 'id'}`).
/// 
/// Sorting functions passed to `orderBy` must be deterministic. You cannot, for instance, order rows using the [random](/api/javascript/random/) command. Using a non-deterministic function with `orderBy` will raise a `ReqlQueryLogicError`.
/// 
/// __Example:__ Order all the posts using the index `date`.   
/// 
/// ```javascript
/// r.table('posts').orderBy({index: 'date'}).run(conn, callback);
/// ```
/// 
/// <!-- stop -->
/// 
/// The index must either be the primary key or have been previously created with [indexCreate](/api/javascript/index_create/).
/// 
/// ```javascript
/// r.table('posts').indexCreate('date').run(conn, callback);
/// ```
/// 
/// You can also select a descending ordering:
/// 
/// ```javascript
/// r.table('posts').orderBy({index: r.desc('date')}).run(conn, callback);
/// ```
/// 
/// __Example:__ Order a sequence without an index.
/// 
/// ```javascript
/// r.table('posts').get(1)('comments').orderBy('date').run(conn, callback);
/// ```
/// 
/// You can also select a descending ordering:
/// 
/// ```javascript
/// r.table('posts').get(1)('comments').orderBy(r.desc('date')).run(conn, callback);
/// ```
/// 
/// If you're doing ad-hoc analysis and know your table won't have more then 100,000
/// elements (or you've changed the setting of the `array_limit` option for [run](/api/javascript/run)) you can run `orderBy` without an index:
/// 
/// ```javascript
/// r.table('small_table').orderBy('date').run(conn, callback);
/// ```
/// 
/// __Example:__ You can efficiently order using multiple fields by using a
/// [compound index](http://www.rethinkdb.com/docs/secondary-indexes/javascript/).
/// 
/// Order by date and title.
/// 
/// ```javascript
/// r.table('posts').orderBy({index: 'dateAndTitle'}).run(conn, callback);
/// ```
/// 
/// The index must either be the primary key or have been previously created with [indexCreate](/api/javascript/index_create/).
/// 
/// ```javascript
/// r.table('posts').indexCreate('dateAndTitle', [r.row('date'), r.row('title')]).run(conn, callback);
/// ```
/// 
/// _Note_: You cannot specify multiple orders in a compound index. See [issue #2306](https://github.com/rethinkdb/rethinkdb/issues/2306)
/// to track progress.
/// 
/// __Example:__ If you have a sequence with fewer documents than the `arrayLimit`, you can order it
/// by multiple fields without an index.
/// 
/// ```javascript
/// r.table('small_table').orderBy('date', r.desc('title')).run(conn, callback);
/// ```
/// 
/// __Example:__ Notice that an index ordering always has highest
/// precedence. The following query orders posts by date, and if multiple
/// posts were published on the same date, they will be ordered by title.
/// 
/// ```javascript
/// r.table('post').orderBy('title', {index: 'date'}).run(conn, callback);
/// ```
/// 
/// __Example:__ Use [nested field](/docs/cookbook/javascript/#filtering-based-on-nested-fields) syntax to sort on fields from subdocuments. (You can also create indexes on nested fields using this syntax with `indexCreate`.)
/// 
/// ```javascript
/// r.table('user').orderBy(r.row('group')('id')).run(conn, callback);
/// ```
/// 
/// __Example:__ You can efficiently order data on arbitrary expressions using indexes.
/// 
/// ```javascript
/// r.table('posts').orderBy({index: 'votes'}).run(conn, callback);
/// ```
/// 
/// The index must have been previously created with [indexCreate](/api/javascript/index_create/).
/// 
/// ```javascript
/// r.table('posts').indexCreate('votes', function(post) {
///     return post('upvotes').sub(post('downvotes'))
/// }).run(conn, callback);
/// ```
/// 
/// __Example:__ If you have a sequence with fewer documents than the `arrayLimit`, you can order it with an arbitrary function directly.
/// 
/// ```javascript
/// r.table('small_table').orderBy(function(doc) {
///     return doc('upvotes').sub(doc('downvotes'))
/// }).run(conn, callback);
/// ```
/// 
/// You can also select a descending ordering:
/// 
/// ```javascript
/// r.table('small_table').orderBy(r.desc(function(doc) {
///     return doc('upvotes').sub(doc('downvotes'))
/// })).run(conn, callback);
/// ```
/// 
/// __Example:__ Ordering after a `between` command can be done as long as the same index is being used.
/// 
/// ```javascript
/// r.table('posts').between(r.time(2013, 1, 1, '+00:00'), r.time(2013, 1, 1, '+00:00'), {index: 'date'})
///     .orderBy({index: 'date'}).run(conn, callback);
/// ```
/// 

                pub fn order_by<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "order_by", Some(Type::ORDER_BY), Some(args))
                }
            

                /// Skip a number of elements from the head of the sequence
///
/// 
/// __Example:__ Here in conjunction with [orderBy](/api/javascript/order_by/) we choose to ignore the most successful heroes.
/// 
/// ```javascript
/// r.table('marvel').orderBy('successMetric').skip(10).run(conn, callback)
/// ```

                pub fn skip<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "skip", Some(Type::SKIP), Some(args))
                }
            

                /// End the sequence after the given number of elements
///
/// 
/// __Example:__ Only so many can fit in our Pantheon of heroes.
/// 
/// ```javascript
/// r.table('marvel').orderBy('belovedness').limit(10).run(conn, callback)
/// ```
/// 
/// 

                pub fn limit<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "limit", Some(Type::LIMIT), Some(args))
                }
            

                /// Return the elements of a sequence within the specified range
///
/// 
/// `slice` returns the range between `startOffset` and `endOffset`. If only `startOffset` is specified, `slice` returns the range from that index to the end of the sequence. Specify `leftBound` or `rightBound` as `open` or `closed` to indicate whether to include that endpoint of the range by default: `closed` returns that endpoint, while `open` does not. By default, `leftBound` is closed and `rightBound` is open, so the range `(10,13)` will return the tenth, eleventh and twelfth elements in the sequence.
/// 
/// If `endOffset` is past the end of the sequence, all elements from `startOffset` to the end of the sequence will be returned. If `startOffset` is past the end of the sequence or `endOffset` is less than `startOffset`, a zero-element sequence will be returned.
/// 
/// Negative `startOffset` and `endOffset` values are allowed with arrays; in that case, the returned range counts back from the array's end. That is, the range `(-2)` returns the last two elements, and the range of `(2,-1)` returns the second element through the next-to-last element of the range. An error will be raised on a negative `startOffset` or `endOffset` with non-arrays. (An `endOffset` of &minus;1 *is* allowed with a stream if `rightBound` is closed; this behaves as if no `endOffset` was specified.)
/// 
/// If `slice` is used with a [binary](/api/javascript/binary) object, the indexes refer to byte positions within the object. That is, the range `(10,20)` will refer to the 10th byte through the 19th byte.
/// 
/// With a string, `slice` behaves similarly, with the indexes referring to Unicode codepoints. String indexes start at `0`. (Note that [combining codepoints][cc] are counted separately.)
/// 
/// [cc]: https://en.wikipedia.org/wiki/Combining_character
/// 
/// __Example:__ Return the fourth, fifth and sixth youngest players. (The youngest player is at index 0, so those are elements 3&ndash;5.)
/// 
/// ```javascript
/// r.table('players').orderBy({index: 'age'}).slice(3,6).run(conn, callback);
/// ```
/// 
/// __Example:__ Return all but the top three players who have a red flag.
/// 
/// ```javascript
/// r.table('players').filter({flag: 'red'}).orderBy(r.desc('score')).slice(3).run(conn, callback);
/// ```
/// 
/// __Example:__ Return holders of tickets `X` through `Y`, assuming tickets are numbered sequentially. We want to include ticket `Y`.
/// 
/// ```javascript
/// r.table('users').orderBy('ticket').slice(x, y, {right_bound: 'closed'}).run(conn, callback);
/// ```
/// 
/// __Example:__ Return the elements of an array from the second through two from the end (that is, not including the last two).
/// 
/// ```javascript
/// r.expr([0,1,2,3,4,5]).slice(2,-2).run(conn, callback);
/// // Result passed to callback
/// [2,3]
/// ```
/// 
/// __Example:__ Return the third through fifth characters of a string.
/// 
/// ```javascript
/// r.expr("rutabaga").slice(2,5).run(conn, callback);
/// // Result passed to callback
/// "tab"
/// ```

                pub fn slice<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "slice", Some(Type::SLICE), Some(args))
                }
            

                /// Get the *nth* element of a sequence, counting from zero
///
/// If the argument is negative, count from the last element.
///
/// 
/// __Example:__ Select the second element in the array.
/// 
/// ```javascript
/// r.expr([1,2,3]).nth(1).run(conn, callback)
/// r.expr([1,2,3])(1).run(conn, callback)
/// ```
/// 
/// __Example:__ Select the bronze medalist from the competitors.
/// 
/// ```javascript
/// r.table('players').orderBy({index: r.desc('score')}).nth(3).run(conn, callback)
/// ```
/// 
/// __Example:__ Select the last place competitor.
/// 
/// ```javascript
/// r.table('players').orderBy({index: r.desc('score')}).nth(-1).run(conn, callback)
/// ```

                pub fn nth<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "nth", Some(Type::NTH), Some(args))
                }
            

                /// Get the indexes of an element in a sequence
///
/// If the argument is a predicate, get the indexes of all elements matching it.
///
/// 
/// __Example:__ Find the position of the letter 'c'.
/// 
/// ```javascript
/// r.expr(['a','b','c']).offsetsOf('c').run(conn, callback)
/// ```
/// 
/// __Example:__ Find the popularity ranking of invisible heroes.
/// 
/// ```javascript
/// r.table('marvel').union(r.table('dc')).orderBy('popularity').offsetsOf(
///     r.row('superpowers').contains('invisibility')
/// ).run(conn, callback)
/// ```

                pub fn offsets_of<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "offsets_of", Some(Type::OFFSETS_OF), Some(args))
                }
            

                /// Test if a sequence is empty
///
/// 
/// __Example:__ Are there any documents in the marvel table?
/// 
/// ```javascript
/// r.table('marvel').isEmpty().run(conn, callback)
/// ```

                pub fn is_empty(&self) -> Client {
                    util::make_cmd::<Client>(self, "is_empty", Some(Type::IS_EMPTY), None)
                }
            

                /// Merge two or more sequences
///
/// 
/// The optional `interleave` argument controls how the sequences will be merged:
/// 
/// * `true`: results will be mixed together; this is the fastest setting, but ordering of elements is not guaranteed. (This is the default.)
/// * `false`: input sequences will be appended to one another, left to right.
/// * `"field_name"`: a string will be taken as the name of a field to perform a merge-sort on. The input sequences must be ordered _before_ being passed to `union`.
/// * function: the `interleave` argument can take a function whose argument is the current row, and whose return value is a string to take as a field name, as with the `"field_name"` setting described above.
/// 
/// __Example:__ Construct a stream of all heroes.
/// 
/// ```javascript
/// r.table('marvel').union(r.table('dc')).run(conn, callback);
/// ```
/// 
/// __Example:__ Combine four arrays into one.
/// 
/// ```javascript
/// r.expr([1, 2]).union([3, 4], [5, 6], [7, 8, 9]).run(conn, callback)
/// // Result passed to callback
/// [1, 2, 3, 4, 5, 6, 7, 8, 9]
/// ```
/// 
/// __Example:__ Create a [changefeed][cf] from the first example.
/// 
/// ```javascript
/// r.table('marvel').union(r.table('dc')).changes().run(conn, callback);
/// ```
/// 
/// Now, when any heroes are added, modified or deleted from either table, a change notification will be sent out.
/// 
/// [cf]: /docs/changefeeds/javascript
/// 
/// __Example:__ Merge-sort the tables of heroes, ordered by name.
/// 
/// ```javascript
/// r.table('marvel').order_by('name').union(
///     r.table('dc').order_by('name'), {interleave: 'name'}
/// ).run(conn, callback);
/// ```

                pub fn union<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "union", Some(Type::UNION), Some(args))
                }
            

                /// Select a given number of elements from a sequence with uniform random distribution
///
/// Selection is done without replacement.
///
/// 
/// If the sequence has less than the requested number of elements (i.e., calling `sample(10)` on a sequence with only five elements), `sample` will return the entire sequence in a random order.
/// 
/// __Example:__ Select 3 random heroes.
/// 
/// ```javascript
/// r.table('marvel').sample(3).run(conn, callback)
/// ```

                pub fn sample<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "sample", Some(Type::SAMPLE), Some(args))
                }
            

                /// Takes a stream and partitions it into multiple groups based on the
/// fields or functions provided
///
/// <img src="https://raw.githubusercontent.com/rethinkdb/docs/master/_jekyll/_images/api_illustrations/group.png" class="api_command_illustration" />
///
/// 
/// With the `multi` flag single documents can be assigned to multiple groups, similar to the behavior of [multi-indexes](/docs/secondary-indexes/javascript). When `multi` is `true` and the grouping value is an array, documents will be placed in each group that corresponds to the elements of the array. If the array is empty the row will be ignored.
/// 
/// Suppose that the table `games` has the following data:
/// 
/// ```javascript
/// [
///     {id: 2, player: "Bob", points: 15, type: "ranked"},
///     {id: 5, player: "Alice", points: 7, type: "free"},
///     {id: 11, player: "Bob", points: 10, type: "free"},
///     {id: 12, player: "Alice", points: 2, type: "free"}
/// ]
/// ```
/// 
/// __Example:__ Group games by player.
/// 
/// ```javascript
/// > r.table('games').group('player').run(conn, callback)
/// 
/// // Result passed to callback
/// [
///     {
///         group: "Alice",
///         reduction: [
///             {id: 5, player: "Alice", points: 7, type: "free"},
///             {id: 12, player: "Alice", points: 2, type: "free"}
///         ]
///     },
///     {
///         group: "Bob",
///         reduction: [
///             {id: 2, player: "Bob", points: 15, type: "ranked"},
///             {id: 11, player: "Bob", points: 10, type: "free"}
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
/// ```javascript
/// > r.table('games').group('player').max('points').run(conn, callback)
/// 
/// // Result passed to callback
/// [
///     {
///         group: "Alice",
///         reduction: {id: 5, player: "Alice", points: 7, type: "free"}
///     },
///     {
///         group: "Bob",
///         reduction: {id: 2, player: "Bob", points: 15, type: "ranked"}
///     }
/// ]
/// ```
/// 
/// Commands chained onto grouped data will operate on each grouped datum,
/// producing more grouped data.
/// 
/// __Example:__ What is the maximum number of points scored by each player?
/// 
/// ```javascript
/// > r.table('games').group('player').max('points')('points').run(conn, callback)
/// 
/// // Result passed to callback
/// [
///     {
///         group: "Alice",
///         reduction: 7
///     },
///     {
///         group: "Bob",
///         reduction: 15
///     }
/// ]
/// ```
/// 
/// You can also group by more than one field.
/// 
/// __Example:__ What is the maximum number of points scored by each
/// player for each game type?
/// 
/// ```javascript
/// > r.table('games').group('player', 'type').max('points')('points').run(conn, callback)
/// 
/// // Result passed to callback
/// [
///     {
///         group: ["Alice", "free"],
///         reduction: 7
///     }
///     {
///         group: ["Bob", "free"],
///         reduction: 10,
///     },
///     {
///         group: ["Bob", "ranked"],
///         reduction: 15,
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
/// ```javascript
/// > r.table('games')
///     .group(function(game) {
///         return game.pluck('player', 'type')
///     }).max('points')('points').run(conn, callback)
/// 
/// // Result passed to callback
/// [
///     {
///         group: {"player": "Alice", "type": "free"},
///         reduction: 7
///     },
///     {
///         group: {"player": "Bob", "type": "free"},
///         reduction: 10
///     },
///     {
///         group: {"player": "Bob", "type": "ranked"},
///         reduction: 15
///     }
/// ]
/// ```
/// 
/// Using a function, you can also group by date on a ReQL [date field](/docs/dates-and-times/javascript/).
/// 
/// __Example:__ How many matches have been played this year by month?
/// 
/// ```javascript
/// > r.table('matches').group(
///       [r.row('date').year(), r.row('date').month()]
///   ).count().run(conn, callback)
/// 
/// // Result passed to callback
/// [
///     {
///         group: [2014, 2],
///         reduction: 2
///     },
///     {
///         group: [2014, 3],
///         reduction: 2
///     },
///     {
///         group: [2014, 4],
///         reduction: 1
///     },
///     {
///         group: [2014, 5],
///         reduction: 3
///     }
/// ]
/// ```
/// 
/// You can also group on an index (primary key or secondary).
/// 
/// __Example:__ What is the maximum number of points scored by game type?
/// 
/// 
/// ```javascript
/// > r.table('games').group({index:'type'}).max('points')('points').run(conn, callback)
/// 
/// // Result passed to callback
/// [
///     {
///         group: "free",
///         reduction: 10
///     },
///     {
///         group: "ranked",
///         reduction: 15
///     }
/// ]
/// ```
/// 
/// # Organizing by value with **multi** #
/// 
/// Suppose that the table `games2` has the following data:
/// 
/// ```javascript
/// [
///     { id: 1, matches: {'a': [1, 2, 3], 'b': [4, 5, 6]} },
///     { id: 2, matches: {'b': [100], 'c': [7, 8, 9]} },
///     { id: 3, matches: {'a': [10, 20], 'c': [70, 80]} }
/// ]
/// ```
/// 
/// Using the `multi` option we can group data by match A, B or C.
/// 
/// ```javascript
/// r.table('games2').group(r.row('matches').keys(), {multi: true}).run(conn, callback);
/// // Result passed to callback
/// [
///     {
///         group: "a",
///         reduction: [ <id 1>, <id 3> ]
///     },
///     {
///         group: "b",
///         reduction: [ <id 1>, <id 2> ]
///     },
///     {
///         group: "c",
///         reduction: [ <id 2>, <id 3> ]
///     }
/// ]
/// ```
/// 
/// (The full result set is abbreviated in the figure; `<id 1>, <id 2>` and `<id 3>` would be the entire documents matching those keys.)
/// 
/// __Example:__ Use [map](/api/javascript/map) and [sum](/api/javascript/sum) to get the total points scored for each match.
/// 
/// ```javascript
/// r.table('games2').group(r.row('matches').keys(), {multi: true}).ungroup().map(
///     function (doc) {
///         return { match: doc('group'), total: doc('reduction').sum(
///             function (set) {
///                 return set('matches')(doc('group')).sum();
///             }
///         )};
///     }
/// ).run(conn, callback);
/// // Result passed to callback
/// [
///     { match: "a", total: 36 },
///     { match: "b", total: 115 },
///     { match: "c", total: 174 }
/// ]
/// ```
/// 
/// The inner `sum` adds the scores by match within each document; the outer `sum` adds those results together for a total across all the documents.
/// 
/// # Ungrouping #
/// 
/// If you want to operate on all the groups rather than operating on each
/// group (e.g. if you want to order the groups by their reduction), you
/// can use [ungroup](/api/javascript/ungroup/) to turn a grouped stream or
/// grouped data into an array of objects representing the groups.
/// 
/// __Example:__ Ungrouping grouped data.
/// 
/// ```javascript
/// > r.table('games').group('player').max('points')('points').ungroup().run(conn, callback)
/// 
/// // Result passed to callback
/// [
///     {
///         group: "Alice",
///         reduction: 7
///     },
///     {
///         group: "Bob",
///         reduction: 15
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
/// ```javascript
/// > r.table('games')
///    .group('player').max('points')('points')
///    .ungroup().orderBy(r.desc('reduction')).run(conn, callback)
/// 
/// // Result passed to callback
/// [
///     {
///         group: "Bob",
///         reduction: 15
///     },
///     {
///         group: "Alice",
///         reduction: 7
///     }
/// ]
/// ```
/// 
/// 
/// # Implementation Details #
/// 
/// When grouped data are returned to the client, they are transformed
/// into a client-specific native type.  (Something similar is done with
/// [times](/docs/dates-and-times/).)  In JavaScript, grouped data are
/// transformed into an `Array`.  If you instead want to receive the raw
/// pseudotype from the server, you can specify `groupFormat: 'raw'` as an optional
/// argument to `run`:
/// 
/// __Example:__ Get back the raw `GROUPED_DATA` pseudotype.
/// 
/// ```javascript
/// > r.table('games').group('player').avg('points').run(conn, {groupFormat:'raw'}, callback)
/// 
/// // Result passed to callback
/// {
///     $reql_type$: "GROUPED_DATA",
///     data: [
///         ["Alice", 4.5],
///         ["Bob", 12.5]
///     ]
/// }
/// ```
/// 
/// Not passing the `group_format` flag would return:
/// 
/// ```javascript
/// [
///     {
///         group: "Alice":
///         reduction: 4.5
///     },
///     {
///         group: "Bob"
///         reduction: 12.5
///     }
/// ]
/// ```
/// 
/// 
/// You might also want to use the [ungroup](/api/javascript/ungroup/)
/// command (see above), which will turn the grouped data into an array of
/// objects on the server.
/// 
/// 
/// # Performance Details #
/// 
/// If you run a query that returns a grouped stream, it will be
/// automatically converted to grouped data before being sent back to you
/// (there is currently no efficient way to stream groups from RethinkDB).
/// This grouped data is subject to the array size limit, by default 100,000 elements (see [run](/api/javascript/run) for details on how to use the `arrayLimit` option to change this).
/// 
/// In general, operations on grouped streams will be efficiently
/// distributed, and operations on grouped data won't be.  You can figure
/// out what you're working with by putting `typeOf` on the end of your
/// query.  Below are efficient and inefficient examples.
/// 
/// __Example:__ Efficient operation.
/// 
/// ```javascript
/// // r.table('games').group('player').typeOf().run(conn, callback)
/// // Returns "GROUPED_STREAM"
/// r.table('games').group('player').min('points').run(conn, callback) // EFFICIENT
/// ```
/// 
/// __Example:__ Inefficient operation.
/// 
/// ```javascript
/// // r.table('games').group('player').orderBy('score').typeOf().run(conn, callback)
/// // Returns "GROUPED_DATA"
/// r.table('games').group('player').orderBy('score').nth(0).run(conn, callback) // INEFFICIENT
/// ```
/// 
/// What does it mean to be inefficient here?  When operating on grouped
/// data rather than a grouped stream, *all* of the data has to be
/// available on the node processing the query.  This means that the
/// operation will only use one server's resources, and will require
/// memory proportional to the size of the grouped data it's operating
/// on.  (In the case of the [orderBy](/api/javascript/order_by/) in the inefficient example, that
/// means memory proportional **to the size of the table**.)  The array
/// limit is also enforced for grouped data, so the `orderBy` example
/// would fail for tables with more than 100,000 rows without changing the `arrayLimit` option to `run`.
/// 
/// # More Examples #
/// 
/// __Example:__ What is the maximum number of points scored by each
/// player in free games?
/// 
/// ```javascript
/// > r.table('games').filter( r.row('type').eq('free'))
///     .group('player').max('points')('points')
///     .run(conn, callback)
/// 
/// // Result passed to callback
/// [
///     {
///         group: "Alice":
///         reduction: 7
///     },
///     {
///         group: "Bob",
///         reduction: 10
///     }
/// ]
/// ```
/// 
/// __Example:__ What is each player's highest even and odd score?
/// 
/// ```javascript
/// r.table('games')
///     .group('name', function(game) {
///         return game('points').mod(2)
///     }).max('points')('points').run(conn, callback)
/// 
/// // Result passed to callback
/// [
///     {
///         group: ["Alice", 1]
///         reduction: 7,
///     },
///     {
///         group: ["Bob", 0],
///         reduction: 10
///     },
///     {
///         group: ["Bob", 1],
///         reduction: 15
///     }
/// ]
/// ```

                pub fn group(&self) -> Client {
                    util::make_cmd::<Client>(self, "group", Some(Type::GROUP), None)
                }
            

                /// Takes a grouped stream or grouped data and turns it into an array of
/// objects representing the groups
///
/// Any commands chained after `ungroup`
///
/// will operate on this array, rather than operating on each group
/// individually.  This is useful if you want to e.g. order the groups by
/// the value of their reduction.
/// 
/// The format of the array returned by `ungroup` is the same as the
/// default native format of grouped data in the javascript driver and
/// data explorer.
/// 
/// Suppose that the table `games` has the following data:
/// 
/// ```javascript
/// [
///     {id: 2, player: "Bob", points: 15, type: "ranked"},
///     {id: 5, player: "Alice", points: 7, type: "free"},
///     {id: 11, player: "Bob", points: 10, type: "free"},
///     {id: 12, player: "Alice", points: 2, type: "free"}
/// ]
/// ```
/// 
/// __Example:__ What is the maximum number of points scored by each
/// player, with the highest scorers first?
/// 
/// ```javascript
/// r.table('games')
///    .group('player').max('points')('points')
///    .ungroup().orderBy(r.desc('reduction')).run(conn, callback)
/// ```
/// 
/// <!-- stop -->
/// 
/// Result:
/// 
/// ```javascript
/// [
///     {
///         group: "Bob",
///         reduction: 15
///     },
///     {
///         group: "Alice",
///         reduction: 7
///     }
/// ]
/// ```
/// 
/// __Example:__ Select one random player and all their games.
/// 
/// ```javascript
/// r.table('games').group('player').ungroup().sample(1).run(conn, callback)
/// ```
/// 
/// Result:
/// 
/// ```javascript
/// [
///     {
///         group: "Bob",
///         reduction: [
///             {id: 2, player: "Bob", points: 15, type: "ranked"},
///             {id: 11, player: "Bob", points: 10, type: "free"},
/// 
///         ]
///     }
/// 
/// ]
/// ```
/// 
/// Note that if you didn't call `ungroup`, you would instead select one
/// random game from each player:
/// 
/// ```javascript
/// r.table('games').group('player').sample(1).run(conn, callback)
/// ```
/// 
/// Result:
/// 
/// ```javascript
/// [
///     {
///         group: "Alice",
///         reduction: [
///             {id: 5, player: "Alice", points: 7, type: "free"}
///         ]
///     },
///     {
///         group: "Bob",
///         reduction: [
///             {id: 11, player: "Bob", points: 10, type: "free"}
///         ]
///     }
/// }
/// ```
/// 
/// __Example:__ Finding the arithmetic mode of an array of values:
/// 
/// ```js
/// r.expr([1,2,2,2,3,3]).group(r.row).count().ungroup().orderBy('reduction').nth(-1)('group').run(conn, callback)
/// ```
/// 
/// Result:
/// 
/// ```json
/// 2
/// ```
/// 
/// __Example:__ Types!
/// 
/// ```javascript
/// r.table('games').group('player').typeOf().run(conn, callback) // Returns "GROUPED_STREAM"
/// r.table('games').group('player').ungroup().typeOf().run(conn, callback) // Returns "ARRAY"
/// r.table('games').group('player').avg('points').run(conn, callback) // Returns "GROUPED_DATA"
/// r.table('games').group('player').avg('points').ungroup().run(conn, callback) // Returns "ARRAY"
/// ```

                pub fn ungroup(&self) -> Client {
                    util::make_cmd::<Client>(self, "ungroup", Some(Type::UNGROUP), None)
                }
            

                /// Produce a single value from a sequence through repeated application of a reduction function
///
/// 
/// The reduction function can be called on:
/// 
/// - two elements of the sequence
/// - one element of the sequence and one result of a previous reduction
/// - two results of previous reductions
/// 
/// The reduction function can be called on the results of two previous reductions because the
/// `reduce` command is distributed and parallelized across shards and CPU cores. A common
/// mistaken when using the `reduce` command is to suppose that the reduction is executed
/// from left to right. Read the [map-reduce in RethinkDB](/docs/map-reduce/) article to
/// see an example.
/// 
/// If the sequence is empty, the server will produce a `ReqlRuntimeError` that can be
/// caught with `default`.  
/// If the sequence has only one element, the first element will be returned.
/// 
/// __Example:__ Return the number of documents in the table `posts`.
/// 
/// ```javascript
/// r.table("posts").map(function(doc) {
///     return 1;
/// }).reduce(function(left, right) {
///     return left.add(right);
/// }).default(0).run(conn, callback);
/// ```
/// 
/// A shorter way to execute this query is to use [count](/api/javascript/count).
/// 
/// 
/// __Example:__ Suppose that each `post` has a field `comments` that is an array of
/// comments.  
/// Return the number of comments for all posts.
/// 
/// ```javascript
/// r.table("posts").map(function(doc) {
///     return doc("comments").count();
/// }).reduce(function(left, right) {
///     return left.add(right);
/// }).default(0).run(conn, callback);
/// ```
/// 
/// 
/// 
/// __Example:__ Suppose that each `post` has a field `comments` that is an array of
/// comments.  
/// Return the maximum number comments per post.
/// 
/// ```javascript
/// r.table("posts").map(function(doc) {
///     return doc("comments").count();
/// }).reduce(function(left, right) {
///     return r.branch(
///         left.gt(right),
///         left,
///         right
///     );
/// }).default(0).run(conn, callback);
/// ```
/// 
/// A shorter way to execute this query is to use [max](/api/javascript/max).

                pub fn reduce<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "reduce", Some(Type::REDUCE), Some(args))
                }
            

                /// Apply a function to a sequence in order, maintaining state via an accumulator
///
/// The `fold` command returns either a single value or a new sequence.
///
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
/// In its second form, `fold` operates like [concatMap][cm], returning a new sequence rather than a single value. When an `emit` function is provided, `fold` will:
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
/// [rd]: /api/javascript/reduce/
/// [cm]: /api/javascript/concat_map/
/// 
/// __Example:__ Concatenate words from a list.
/// 
/// ```javascript
/// r.table('words').orderBy('id').fold('', function (acc, word) {
///     return acc.add(r.branch(acc.eq(''), '', ', ')).add(word);
/// }).run(conn, callback);
/// ```
/// 
/// (This example could be implemented with `reduce`, but `fold` will preserve the order when `words` is a RethinkDB table or other stream, which is not guaranteed with `reduce`.)
/// 
/// __Example:__ Return every other row in a table.
/// 
/// ```javascript
/// r.table('even_things').fold(0, function(acc, row) {
///     return acc.add(1);
/// }, {emit:
///     function (acc, row, new_acc) {
///         return r.branch(new_acc.mod(2).eq(0), [row], []);
///     }
/// }).run(conn, callback);
/// ```
/// 
/// The first function increments the accumulator each time it's called, starting at `0`; the second function, the emitting function, alternates between returning a single-item list containing the current row or an empty list. The `fold` command will return a concatenated list of each emitted value.
/// 
/// __Example:__ Compute a five-day running average for a weight tracker.
/// 
/// ```javascript
/// r.table('tracker').filter({name: 'bob'}).orderBy('date')('weight').fold(
///     [],
///     function (acc, row) { return r.expr([row]).add(acc).limit(5); },
///     {emit:
///         function (acc, row, newAcc) {
///             return r.branch(newAcc.length().eq(5), [newAcc.avg()], []);
///         }
///     }
/// ).run(conn, callback);
/// ```

                pub fn fold<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "fold", Some(Type::FOLD), Some(args))
                }
            

                /// Counts the number of elements in a sequence or key/value pairs in an object, or returns the size of a string or binary object
///
/// 
/// When `count` is called on a sequence with a predicate value or function, it returns the number of elements in the sequence equal to that value or where the function returns `true`. On a [binary](/api/javascript/binary) object, `count` returns the size of the object in bytes; on strings, `count` returns the string's length. This is determined by counting the number of Unicode codepoints in the string, counting combining codepoints separately.
/// 
/// __Example:__ Count the number of users.
/// 
/// ```javascript
/// r.table('users').count().run(conn, callback);
/// ```
/// 
/// __Example:__ Count the number of 18 year old users.
/// 
/// ```javascript
/// r.table('users')('age').count(18).run(conn, callback);
/// ```
/// 
/// __Example:__ Count the number of users over 18.
/// 
/// ```javascript
/// r.table('users')('age').count(function(age) { 
///     return age.gt(18)
/// }).run(conn, callback);
/// ```
/// 
/// ```javascript
/// r.table('users').count(function(user) {
///     return user('age').gt(18)
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ Return the length of a Unicode string.
/// 
/// ```javascript
/// r.expr("こんにちは").count().run(conn, callback);
/// // Result passed to callback
/// 5
/// ```

                pub fn count(&self) -> Client {
                    util::make_cmd::<Client>(self, "count", Some(Type::COUNT), None)
                }
            

                /// Sums all the elements of a sequence
///
/// If called with a field name,
///
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
/// ```javascript
/// r.expr([3, 5, 7]).sum().run(conn, callback)
/// ```
/// 
/// __Example:__ How many points have been scored across all games?
/// 
/// ```javascript
/// r.table('games').sum('points').run(conn, callback)
/// ```
/// 
/// __Example:__ How many points have been scored across all games,
/// counting bonus points?
/// 
/// ```javascript
/// r.table('games').sum(function(game) {
///     return game('points').add(game('bonus_points'))
/// }).run(conn, callback)
/// ```

                pub fn sum(&self) -> Client {
                    util::make_cmd::<Client>(self, "sum", Some(Type::SUM), None)
                }
            

                /// Averages all the elements of a sequence
///
/// If called with a field name,
///
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
/// ```javascript
/// r.expr([3, 5, 7]).avg().run(conn, callback)
/// ```
/// 
/// __Example:__ What's the average number of points scored in a game?
/// 
/// ```javascript
/// r.table('games').avg('points').run(conn, callback)
/// ```
/// 
/// __Example:__ What's the average number of points scored in a game,
/// counting bonus points?
/// 
/// ```javascript
/// r.table('games').avg(function(game) {
///     return game('points').add(game('bonus_points'))
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ What's the average number of points scored in a game?
/// (But return `null` instead of raising an error if there are no games where
/// points have been scored.)
/// 
/// ```javascript
/// r.table('games').avg('points').default(null).run(conn, callback)
/// ```

                pub fn avg(&self) -> Client {
                    util::make_cmd::<Client>(self, "avg", Some(Type::AVG), None)
                }
            

                /// Finds the minimum element of a sequence
///
/// 
/// The `min` command can be called with:
/// 
/// * a **field name**, to return the element of the sequence with the smallest value in that field;
/// * an **index** (the primary key or a secondary index), to return the element of the sequence with the smallest value in that index;
/// * a **function**, to apply the function to every element within the sequence and return the element which returns the smallest value from the function, ignoring any elements where the function produces a non-existence error.
/// 
/// For more information on RethinkDB's sorting order, read the section in [ReQL data types](/docs/data-types/#sorting-order).
/// 
/// Calling `min` on an empty sequence will throw a non-existence error; this can be handled using the [default](/api/javascript/default/) command.
/// 
/// __Example:__ Return the minimum value in the list `[3, 5, 7]`.
/// 
/// ```javascript
/// r.expr([3, 5, 7]).min().run(conn, callback);
/// ```
/// 
/// __Example:__ Return the user who has scored the fewest points.
/// 
/// ```javascript
/// r.table('users').min('points').run(conn, callback);
/// ```
/// 
/// __Example:__ The same as above, but using a secondary index on the `points` field.
/// 
/// ```javascript
/// r.table('users').min({index: 'points'}).run(conn, callback);
/// ```
/// 
/// __Example:__ Return the user who has scored the fewest points, adding in bonus points from a separate field using a function.
/// 
/// ```javascript
/// r.table('users').min(function(user) {
///     return user('points').add(user('bonusPoints'));
/// }).run(conn, callback);
/// ```
/// 
/// __Example:__ Return the smallest number of points any user has ever scored. This returns the value of that `points` field, not a document.
/// 
/// ```javascript
/// r.table('users').min('points')('points').run(conn, callback);
/// ```
/// 
/// __Example:__ Return the user who has scored the fewest points, but add a default `null` return value to prevent an error if no user has ever scored points.
/// 
/// ```javascript
/// r.table('users').min('points').default(null).run(conn, callback);
/// ```

                pub fn min(&self) -> Client {
                    util::make_cmd::<Client>(self, "min", Some(Type::MIN), None)
                }
            

                /// Finds the maximum element of a sequence
///
/// 
/// The `max` command can be called with:
/// 
/// * a **field name**, to return the element of the sequence with the largest value in that field;
/// * an **index** (the primary key or a secondary index), to return the element of the sequence with the largest value in that index;
/// * a **function**, to apply the function to every element within the sequence and return the element which returns the largest value from the function, ignoring any elements where the function produces a non-existence error.
/// 
/// For more information on RethinkDB's sorting order, read the section in [ReQL data types](/docs/data-types/#sorting-order).
/// 
/// Calling `max` on an empty sequence will throw a non-existence error; this can be handled using the [default](/api/javascript/default/) command.
/// 
/// __Example:__ Return the maximum value in the list `[3, 5, 7]`.
/// 
/// ```javascript
/// r.expr([3, 5, 7]).max().run(conn, callback);
/// ```
/// 
/// __Example:__ Return the user who has scored the most points.
/// 
/// ```javascript
/// r.table('users').max('points').run(conn, callback);
/// ```
/// 
/// __Example:__ The same as above, but using a secondary index on the `points` field.
/// 
/// ```javascript
/// r.table('users').max({index: 'points'}).run(conn, callback);
/// ```
/// 
/// __Example:__ Return the user who has scored the most points, adding in bonus points from a separate field using a function.
/// 
/// ```javascript
/// r.table('users').max(function(user) {
///     return user('points').add(user('bonusPoints'));
/// }).run(conn, callback);
/// ```
/// 
/// __Example:__ Return the highest number of points any user has ever scored. This returns the value of that `points` field, not a document.
/// 
/// ```javascript
/// r.table('users').max('points')('points').run(conn, callback);
/// ```
/// 
/// __Example:__ Return the user who has scored the most points, but add a default `null` return value to prevent an error if no user has ever scored points.
/// 
/// ```javascript
/// r.table('users').max('points').default(null).run(conn, callback);
/// ```

                pub fn max(&self) -> Client {
                    util::make_cmd::<Client>(self, "max", Some(Type::MAX), None)
                }
            

                /// Removes duplicates from elements in a sequence
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
/// ```javascript
/// r.table('marvel').concatMap(function(hero) {
///     return hero('villainList')
/// }).distinct().run(conn, callback)
/// ```
/// 
/// __Example:__ Topics in a table of messages have a secondary index on them, and more than one message can have the same topic. What are the unique topics in the table?
/// 
/// ```javascript
/// r.table('messages').distinct({index: 'topics'}).run(conn, callback)
/// ```
/// 
/// The above structure is functionally identical to:
/// 
/// ```javascript
/// r.table('messages')('topics').distinct().run(conn, callback)
/// ```
/// 
/// However, the first form (passing the index as an argument to `distinct`) is faster, and won't run into array limit issues since it's returning a stream.

                pub fn distinct(&self) -> Client {
                    util::make_cmd::<Client>(self, "distinct", Some(Type::DISTINCT), None)
                }
            

                /// When called with values, returns `true` if a sequence contains all the
/// specified values
///
/// When called with predicate functions, returns `true`
///
/// if for each predicate there exists at least one element of the stream
/// where that predicate returns `true`.
/// 
/// Values and predicates may be mixed freely in the argument list.
/// 
/// __Example:__ Has Iron Man ever fought Superman?
/// 
/// ```javascript
/// r.table('marvel').get('ironman')('opponents').contains('superman').run(conn, callback);
/// ```
/// 
/// __Example:__ Has Iron Man ever defeated Superman in battle?
/// 
/// ```javascript
/// r.table('marvel').get('ironman')('battles').contains(function (battle) {
///     return battle('winner').eq('ironman').and(battle('loser').eq('superman'));
/// }).run(conn, callback);
/// ```
/// 
/// __Example:__ Return all heroes who have fought _both_ Loki and the Hulk.
/// 
/// ```javascript
/// r.table('marvel').filter(function (hero) {
///   return hero('opponents').contains('loki', 'hulk');
/// }).run(conn, callback);
/// ```
/// 
/// __Example:__ Use `contains` with a predicate function to simulate an `or`. Return the Marvel superheroes who live in Detroit, Chicago or Hoboken.
/// 
/// ```javascript
/// r.table('marvel').filter(function(hero) {
///     return r.expr(['Detroit', 'Chicago', 'Hoboken']).contains(hero('city'))
/// }).run(conn, callback);
/// ```

                pub fn contains<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "contains", Some(Type::CONTAINS), Some(args))
                }
            

                /// Plucks out one or more attributes from either an object or a sequence of objects
/// (projection)
///
/// <img src="https://raw.githubusercontent.com/rethinkdb/docs/master/_jekyll/_images/api_illustrations/pluck.png" class="api_command_illustration" />
///
/// 
/// __Example:__ We just need information about IronMan's reactor and not the rest of the
/// document.
/// 
/// ```javascript
/// r.table('marvel').get('IronMan').pluck('reactorState', 'reactorPower').run(conn, callback)
/// ```
/// 
/// 
/// __Example:__ For the hero beauty contest we only care about certain qualities.
/// 
/// ```javascript
/// r.table('marvel').pluck('beauty', 'muscleTone', 'charm').run(conn, callback)
/// ```
/// 
/// 
/// __Example:__ Pluck can also be used on nested objects.
/// 
/// ```javascript
/// r.table('marvel').pluck({'abilities' : {'damage' : true, 'mana_cost' : true}, 'weapons' : true}).run(conn, callback)
/// ```
/// 
/// 
/// __Example:__ The nested syntax can quickly become overly verbose so there's a shorthand for it.
/// 
/// ```javascript
/// r.table('marvel').pluck({'abilities' : ['damage', 'mana_cost']}, 'weapons').run(conn, callback)
/// ```
/// 
/// For more information read the [nested field documentation](/docs/nested-fields/).

                pub fn pluck<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "pluck", Some(Type::PLUCK), Some(args))
                }
            

                /// The opposite of pluck; takes an object or a sequence of objects, and returns them with
/// the specified paths removed
///
/// 
/// __Example:__ Since we don't need it for this computation we'll save bandwidth and leave
/// out the list of IronMan's romantic conquests.
/// 
/// ```javascript
/// r.table('marvel').get('IronMan').without('personalVictoriesList').run(conn, callback)
/// ```
/// 
/// 
/// __Example:__ Without their prized weapons, our enemies will quickly be vanquished.
/// 
/// ```javascript
/// r.table('enemies').without('weapons').run(conn, callback)
/// ```
/// 
/// 
/// __Example:__ Nested objects can be used to remove the damage subfield from the weapons and abilities fields.
/// 
/// ```javascript
/// r.table('marvel').without({'weapons' : {'damage' : true}, 'abilities' : {'damage' : true}}).run(conn, callback)
/// ```
/// 
/// 
/// __Example:__ The nested syntax can quickly become overly verbose so there's a shorthand for it.
/// 
/// ```javascript
/// r.table('marvel').without({'weapons':'damage', 'abilities':'damage'}).run(conn, callback)
/// ```
/// 

                pub fn without<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "without", Some(Type::WITHOUT), Some(args))
                }
            

                /// Merge two or more objects together to construct a new object with properties from all
///
/// When there is a conflict between field names, preference is given to fields in the rightmost object in the argument list. `merge` also accepts a subquery function that returns an object, which will be used similarly to a [map](/api/javascript/map/) function.
///
/// 
/// __Example:__ Equip Thor for battle.
/// 
/// ```javascript
/// r.table('marvel').get('thor').merge(
///     r.table('equipment').get('hammer'),
///     r.table('equipment').get('pimento_sandwich')
/// ).run(conn, callback)
/// ```
/// 
/// __Example:__ Equip every hero for battle, using a subquery function to retrieve their weapons.
/// 
/// ```javascript
/// r.table('marvel').merge(function (hero) {
///     return { weapons: r.table('weapons').get(hero('weaponId')) };
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ Use `merge` to join each blog post with its comments.
/// 
/// Note that the sequence being merged&mdash;in this example, the comments&mdash;must be coerced from a selection to an array. Without `coerceTo` the operation will throw an error ("Expected type DATUM but found SELECTION").
/// 
/// ```javascript
/// r.table('posts').merge(function (post) {
///     return {
///         comments: r.table('comments').getAll(post('id'),
///             {index: 'postId'}).coerceTo('array')
///     }
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ Merge can be used recursively to modify object within objects.
/// 
/// ```javascript
/// r.expr({weapons : {spectacular_graviton_beam : {dmg : 10, cooldown : 20}}}).merge(
///     {weapons : {spectacular_graviton_beam : {dmg : 10}}}).run(conn, callback)
/// ```
/// 
/// 
/// __Example:__ To replace a nested object with another object you can use the literal keyword.
/// 
/// ```javascript
/// r.expr({weapons : {spectacular_graviton_beam : {dmg : 10, cooldown : 20}}}).merge(
///     {weapons : r.literal({repulsor_rays : {dmg : 3, cooldown : 0}})}).run(conn, callback)
/// ```
/// 
/// 
/// __Example:__ Literal can be used to remove keys from an object as well.
/// 
/// ```javascript
/// r.expr({weapons : {spectacular_graviton_beam : {dmg : 10, cooldown : 20}}}).merge(
///     {weapons : {spectacular_graviton_beam : r.literal()}}).run(conn, callback)
/// ```
/// 

                pub fn merge<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "merge", Some(Type::MERGE), Some(args))
                }
            

                /// Append a value to an array
///
/// 
/// __Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.
/// 
/// ```javascript
/// r.table('marvel').get('IronMan')('equipment').append('newBoots').run(conn, callback)
/// ```
/// 
/// 

                pub fn append<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "append", Some(Type::APPEND), Some(args))
                }
            

                /// Prepend a value to an array
///
/// 
/// __Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.
/// 
/// ```javascript
/// r.table('marvel').get('IronMan')('equipment').prepend('newBoots').run(conn, callback)
/// ```
/// 
/// 

                pub fn prepend<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "prepend", Some(Type::PREPEND), Some(args))
                }
            

                /// Remove the elements of one array from another array
///
/// 
/// __Example:__ Retrieve Iron Man's equipment list without boots.
/// 
/// ```javascript
/// r.table('marvel').get('IronMan')('equipment')
///   .difference(['Boots'])
///   .run(conn, callback)
/// ```
/// 
/// __Example:__ Remove Iron Man's boots from his equipment.
/// 
/// ```javascript
/// r.table('marvel').get('IronMan')
///   .update({
///     equipment: r.row('equipment').difference(['Boots'])
///   })
///   .run(conn, callback)
/// ```
/// 
/// 

                pub fn difference<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "difference", Some(Type::DIFFERENCE), Some(args))
                }
            

                /// Add a value to an array and return it as a set (an array with distinct values)
///
/// 
/// __Example:__ Retrieve Iron Man's equipment list with the addition of some new boots.
/// 
/// ```javascript
/// r.table('marvel').get('IronMan')('equipment').setInsert('newBoots').run(conn, callback)
/// ```
/// 
/// 

                pub fn set_insert<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "set_insert", Some(Type::SET_INSERT), Some(args))
                }
            

                /// Add a several values to an array and return it as a set (an array with distinct values)
///
/// 
/// __Example:__ Retrieve Iron Man's equipment list with the addition of some new boots and an arc reactor.
/// 
/// ```javascript
/// r.table('marvel').get('IronMan')('equipment').setUnion(['newBoots', 'arc_reactor']).run(conn, callback)
/// ```
/// 

                pub fn set_union<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "set_union", Some(Type::SET_UNION), Some(args))
                }
            

                /// Intersect two arrays returning values that occur in both of them as a set (an array with
/// distinct values)
///
/// 
/// __Example:__ Check which pieces of equipment Iron Man has from a fixed list.
/// 
/// ```javascript
/// r.table('marvel').get('IronMan')('equipment').setIntersection(['newBoots', 'arc_reactor']).run(conn, callback)
/// ```
/// 

                pub fn set_intersection<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "set_intersection", Some(Type::SET_INTERSECTION), Some(args))
                }
            

                /// Remove the elements of one array from another and return them as a set (an array with
/// distinct values)
///
/// 
/// __Example:__ Check which pieces of equipment Iron Man has, excluding a fixed list.
/// 
/// ```javascript
/// r.table('marvel').get('IronMan')('equipment').setDifference(['newBoots', 'arc_reactor']).run(conn, callback)
/// ```
/// 
/// 

                pub fn set_difference<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "set_difference", Some(Type::SET_DIFFERENCE), Some(args))
                }
            

                /// Get a single field from an object
///
/// If called on a sequence, gets that field from every object in the sequence, skipping objects that lack it.
///
/// 
/// __Example:__ What was Iron Man's first appearance in a comic?
/// 
/// ```javascript
/// r.table('marvel').get('IronMan')('firstAppearance').run(conn, callback)
/// ```
/// 
/// <!-- stop -->
/// 
/// The `()` command also accepts integer arguments as array offsets, like the [nth](/api/javascript/nth) command.
/// 
/// __Example:__ Get the fourth element in a sequence. (The first element is position `0`, so the fourth element is position `3`.)
/// 
/// ```javascript
/// r.expr([10, 20, 30, 40, 50])(3)
/// 
/// 40
/// ```

                pub fn bracket<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "bracket", Some(Type::BRACKET), Some(args))
                }
            

                /// Get a single field from an object
///
/// If called on a sequence, gets that field from every
///
/// object in the sequence, skipping objects that lack it.
/// 
/// __Example:__ What was Iron Man's first appearance in a comic?
/// 
/// ```javascript
/// r.table('marvel').get('IronMan').getField('firstAppearance').run(conn, callback)
/// ```

                pub fn get_field<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "get_field", Some(Type::GET_FIELD), Some(args))
                }
            

                /// Test if an object has one or more fields
///
/// An object has a field if it has that key and the key has a non-null value. For instance, the object `{'a': 1,'b': 2,'c': null}` has the fields `a` and `b`.
///
/// 
/// When applied to a single object, `hasFields` returns `true` if the object has the fields and `false` if it does not. When applied to a sequence, it will return a new sequence (an array or stream) containing the elements that have the specified fields.
/// 
/// __Example:__ Return the players who have won games.
/// 
/// ```javascript
/// r.table('players').hasFields('games_won').run(conn, callback)
/// ```
/// 
/// __Example:__ Return the players who have *not* won games. To do this, use `hasFields` with [not](/api/javascript/not), wrapped with [filter](/api/javascript/filter).
/// 
/// ```javascript
/// r.table('players').filter(
///     r.row.hasFields('games_won').not()
/// ).run(conn, callback)
/// ```
/// 
/// __Example:__ Test if a specific player has won any games.
/// 
/// ```javascript
/// r.table('players').get('b5ec9714-837e-400c-aa74-dbd35c9a7c4c'
///     ).hasFields('games_won').run(conn, callback)
/// ```
/// 
/// **Nested Fields**
/// 
/// `hasFields` lets you test for nested fields in objects. If the value of a field is itself a set of key/value pairs, you can test for the presence of specific keys.
/// 
/// __Example:__ In the `players` table, the `games_won` field contains one or more fields for kinds of games won:
/// 
/// ```javascript
/// {
///     games_won: {
///         playoffs: 2,
///         championships: 1
///     }
/// }
/// ```
/// 
/// Return players who have the "championships" field.
/// 
/// ```javascript
/// r.table('players').hasFields({'games_won': {'championships': true}}).run(conn, callback)
/// ```
/// 
/// Note that `true` in the example above is testing for the existence of `championships` as a field, not testing to see if the value of the `championships` field is set to `true`. There's a more convenient shorthand form available. (See [pluck](/api/javascript/pluck) for more details on this.)
/// 
/// ```javascript
/// r.table('players').hasFields({'games_won': 'championships'}
///     ).run(conn, callback)
/// ```

                pub fn has_fields<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "has_fields", Some(Type::HAS_FIELDS), Some(args))
                }
            

                /// Insert a value in to an array at a given index
///
/// Returns the modified array.
///
/// 
/// __Example:__ Hulk decides to join the avengers.
/// 
/// ```javascript
/// r.expr(["Iron Man", "Spider-Man"]).insertAt(1, "Hulk").run(conn, callback)
/// ```
/// 
/// 

                pub fn insert_at<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "insert_at", Some(Type::INSERT_AT), Some(args))
                }
            

                /// Insert several values in to an array at a given index
///
/// Returns the modified array.
///
/// 
/// __Example:__ Hulk and Thor decide to join the avengers.
/// 
/// ```javascript
/// r.expr(["Iron Man", "Spider-Man"]).spliceAt(1, ["Hulk", "Thor"]).run(conn, callback)
/// ```
/// 

                pub fn splice_at<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "splice_at", Some(Type::SPLICE_AT), Some(args))
                }
            

                /// Remove one or more elements from an array at a given index
///
/// Returns the modified array. (Note: `deleteAt` operates on arrays, not documents; to delete documents, see the [delete](/api/javascript/delete) command.)
///
/// 
/// If only `offset` is specified, `deleteAt` removes the element at that index. If both `offset` and `endOffset` are specified, `deleteAt` removes the range of elements between `offset` and `endOffset`, inclusive of `offset` but not inclusive of `endOffset`.
/// 
/// If `endOffset` is specified, it must not be less than `offset`. Both `offset` and `endOffset` must be within the array's bounds (i.e., if the array has 10 elements, an `offset` or `endOffset` of 10 or higher is invalid).
/// 
/// By using a negative `offset` you can delete from the end of the array. `-1` is the last element in the array, `-2` is the second-to-last element, and so on. You may specify a negative `endOffset`, although just as with a positive value, this will not be inclusive. The range `(2,-1)` specifies the third element through the next-to-last element.
/// 
/// __Example:__ Delete the second element of an array.
/// 
/// ```javascript
/// > r(['a','b','c','d','e','f']).deleteAt(1).run(conn, callback)
/// // result passed to callback
/// ['a', 'c', 'd', 'e', 'f']
/// ```
/// 
/// __Example:__ Delete the second and third elements of an array.
/// 
/// ```javascript
/// > r(['a','b','c','d','e','f']).deleteAt(1,3).run(conn, callback)
/// // result passed to callback
/// ['a', 'd', 'e', 'f']
/// ```
/// 
/// __Example:__ Delete the next-to-last element of an array.
/// 
/// ```javascript
/// > r(['a','b','c','d','e','f']).deleteAt(-2).run(conn, callback)
/// // result passed to callback
/// ['a', 'b', 'c', 'd', 'f']
/// ```
/// 
/// __Example:__ Delete a comment on a post.
/// 
/// Given a post document such as:
/// 
/// ```javascript
/// {
///     id: '4cf47834-b6f9-438f-9dec-74087e84eb63',
///     title: 'Post title',
///     author: 'Bob',
///     comments: [
///         { author: 'Agatha', text: 'Comment 1' },
///         { author: 'Fred', text: 'Comment 2' }
///     ]
/// }
/// ```
/// 
/// The second comment can be deleted by using `update` and `deleteAt` together.
/// 
/// ```javascript
/// r.table('posts').get('4cf47834-b6f9-438f-9dec-74087e84eb63').update({
///     comments: r.row('comments').deleteAt(1)
/// }).run(conn, callback)
/// ```

                pub fn delete_at<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "delete_at", Some(Type::DELETE_AT), Some(args))
                }
            

                /// Change a value in an array at a given index
///
/// Returns the modified array.
///
/// 
/// __Example:__ Bruce Banner hulks out.
/// 
/// ```javascript
/// r.expr(["Iron Man", "Bruce", "Spider-Man"]).changeAt(1, "Hulk").run(conn, callback)
/// ```

                pub fn change_at<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "change_at", Some(Type::CHANGE_AT), Some(args))
                }
            

                /// Return an array containing all of an object's keys
///
/// Note that the keys will be sorted as described in [ReQL data types](/docs/data-types/#sorting-order) (for strings, lexicographically).
///
/// 
/// __Example:__ Get all the keys from a table row.
/// 
/// ```javascript
/// // row: { id: 1, mail: "fred@example.com", name: "fred" }
/// 
/// r.table('users').get(1).keys().run(conn, callback);
/// // Result passed to callback
/// [ "id", "mail", "name" ]
/// ```

                pub fn keys(&self) -> Client {
                    util::make_cmd::<Client>(self, "keys", Some(Type::KEYS), None)
                }
            

                /// Return an array containing all of an object's values
///
/// `values()` guarantees the values will come out in the same order as [keys](/api/javascript/keys).
///
/// 
/// __Example:__ Get all of the values from a table row.
/// 
/// ```javascript
/// // row: { id: 1, mail: "fred@example.com", name: "fred" }
/// 
/// r.table('users').get(1).values().run(conn, callback);
/// // Result passed to callback
/// [ 1, "fred@example.com", "fred" ]
/// ```

                pub fn values(&self) -> Client {
                    util::make_cmd::<Client>(self, "values", Some(Type::VALUES), None)
                }
            

                /// Replace an object in a field instead of merging it with an existing object in a `merge` or `update` operation
///
/// Using `literal` with no arguments in a `merge` or `update` operation will remove the corresponding field.
///
/// 
/// Assume your users table has this structure:
/// 
/// ```javascript
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
/// ```javascript
/// r.table('users').get(1).update({ data: { age: 19, job: 'Engineer' } }).run(conn, callback)
/// 
/// // Result passed to callback
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
/// ```javascript
/// r.table('users').get(1).update({ data: r.literal({ age: 19, job: 'Engineer' }) }).run(conn, callback)
/// 
/// // Result passed to callback
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
/// ```javascript
/// r.table('users').get(1).merge({ data: r.literal() }).run(conn, callback)
/// 
/// // Result passed to callback
/// {
///     "id": 1,
///     "name": "Alice"
/// }
/// ```

                pub fn literal(&self) -> Client {
                    util::make_cmd::<Client>(self, "literal", Some(Type::LITERAL), None)
                }
            

                /// Matches against a regular expression
///
/// <img src="https://raw.githubusercontent.com/rethinkdb/docs/master/_jekyll/_images/api_illustrations/match.png" class="api_command_illustration" />
///
/// If there is a match, returns an object with the fields:
///
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
/// [filter](/api/javascript/filter/), you can just use the result of `match` for the predicate.
/// 
/// 
/// ```javascript
/// r.table('users').filter(function(doc){
///     return doc('name').match("^A")
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ Get all users whose name ends with "n".
/// 
/// ```javascript
/// r.table('users').filter(function(doc){
///     return doc('name').match("n$")
/// }).run(conn, callback)
/// ```
/// __Example:__ Get all users whose name has "li" in it
/// 
/// ```javascript
/// r.table('users').filter(function(doc){
///     return doc('name').match("li")
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ Get all users whose name is "John" with a case-insensitive search.
/// 
/// ```javascript
/// r.table('users').filter(function(doc){
///     return doc('name').match("(?i)^john$")
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ Get all users whose name is composed of only characters between "a" and "z".
/// 
/// ```javascript
/// r.table('users').filter(function(doc){
///     return doc('name').match("(?i)^[a-z]+$")
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ Get all users where the zipcode is a string of 5 digits.
/// 
/// ```javascript
/// r.table('users').filter(function(doc){
///     return doc('zipcode').match("\\d{5}")
/// }).run(conn, callback)
/// ```
/// 
/// 
/// __Example:__ Retrieve the domain of a basic email
/// 
/// ```javascript
/// r.expr("name@domain.com").match(".*@(.*)").run(conn, callback)
/// ```
/// 
/// Result:
/// 
/// ```javascript
/// {
///     start: 0,
///     end: 20,
///     str: "name@domain.com",
///     groups: [
///         {
///             end: 17,
///             start: 7,
///             str: "domain.com"
///         }
///     ]
/// }
/// ```
/// 
/// You can then retrieve only the domain with the [\(\)](/api/javascript/get_field) selector and [nth](/api/javascript/nth).
/// 
/// ```javascript
/// r.expr("name@domain.com").match(".*@(.*)")("groups").nth(0)("str").run(conn, callback)
/// ```
/// 
/// Returns `'domain.com'`
/// 
/// 
/// __Example:__ Fail to parse out the domain and returns `null`.
/// 
/// ```javascript
/// r.expr("name[at]domain.com").match(".*@(.*)").run(conn, callback)
/// ```

                pub fn match_<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "match_", Some(Type::MATCH), Some(args))
                }
            

                /// Splits a string into substrings
///
/// <img src="https://raw.githubusercontent.com/rethinkdb/docs/master/_jekyll/_images/api_illustrations/split.png" class="api_command_illustration" />
///
/// Splits on whitespace when called
///
/// with no arguments.  When called with a separator, splits on that
/// separator.  When called with a separator and a maximum number of
/// splits, splits on that separator at most `max_splits` times.  (Can be
/// called with `null` as the separator if you want to split on whitespace
/// while still specifying `max_splits`.)
/// 
/// Mimics the behavior of Python's `string.split` in edge cases, except
/// for splitting on the empty string, which instead produces an array of
/// single-character strings.
/// 
/// __Example:__ Split on whitespace.
/// 
/// ```javascript
/// r.expr("foo  bar bax").split().run(conn, callback)
/// ```
/// 
/// Result:
/// 
/// ```javascript
/// ["foo", "bar", "bax"]
/// ```
/// 
/// __Example:__ Split the entries in a CSV file.
/// 
/// ```javascript
/// r.expr("12,37,,22,").split(",").run(conn, callback)
/// ```
/// 
/// Result:
/// 
/// ```javascript
/// ["12", "37", "", "22", ""]
/// ```
/// 
/// __Example:__ Split a string into characters.
/// 
/// ```javascript
/// r.expr("mlucy").split("").run(conn, callback)
/// ```
/// 
/// Result:
/// 
/// ```javascript
/// ["m", "l", "u", "c", "y"]
/// ```
/// 
/// __Example:__ Split the entries in a CSV file, but only at most 3
/// times.
/// 
/// ```javascript
/// r.expr("12,37,,22,").split(",", 3).run(conn, callback)
/// ```
/// 
/// Result:
/// 
/// ```javascript
/// ["12", "37", "", "22,"]
/// ```
/// 
/// __Example:__ Split on whitespace at most once (i.e. get the first word).
/// 
/// ```javascript
/// r.expr("foo  bar bax").split(null, 1).run(conn, callback)
/// ```
/// 
/// Result:
/// 
/// ```javascript
/// ["foo", "bar bax"]
/// ```

                pub fn split(&self) -> Client {
                    util::make_cmd::<Client>(self, "split", Some(Type::SPLIT), None)
                }
            

                /// Uppercases a string
///
/// 
/// __Example:__
/// 
/// ```javascript
/// r.expr("Sentence about LaTeX.").upcase().run(conn, callback)
/// ```
/// 
/// Result:
/// 
/// ```javascript
/// "SENTENCE ABOUT LATEX."
/// ```
/// 
/// __Note:__ `upcase` and `downcase` only affect ASCII characters.

                pub fn upcase(&self) -> Client {
                    util::make_cmd::<Client>(self, "upcase", Some(Type::UPCASE), None)
                }
            

                /// Lowercases a string
///
/// 
/// __Example:__
/// 
/// ```javascript
/// r.expr("Sentence about LaTeX.").downcase().run(conn, callback)
/// ```
/// 
/// Result:
/// 
/// ```javascript
/// "sentence about latex."
/// ```
/// 
/// __Note:__ `upcase` and `downcase` only affect ASCII characters.

                pub fn downcase(&self) -> Client {
                    util::make_cmd::<Client>(self, "downcase", Some(Type::DOWNCASE), None)
                }
            

                /// Sum two or more numbers, or concatenate two or more strings or arrays
///
/// 
/// The `add` command can be called in either prefix or infix form; both forms are equivalent. Note that ReQL will not perform type coercion. You cannot, for example, `add` a string and a number together.
/// 
/// __Example:__ It's as easy as 2 + 2 = 4.
/// 
/// ```javascript
/// > r.expr(2).add(2).run(conn, callback)
/// // result passed to callback
/// 4
/// ```
/// 
/// __Example:__ Concatenate strings.
/// 
/// ```javascript
/// > r.expr("foo").add("bar", "baz").run(conn, callback)
/// // result passed to callback
/// "foobarbaz"
/// ```
/// 
/// 
/// __Example:__ Concatenate arrays.
/// 
/// ```javascript
/// > r.expr(["foo", "bar"]).add(["buzz"]).run(conn, callback)
/// // result passed to callback
/// [ "foo", "bar", "buzz" ]
/// ```
/// 
/// 
/// __Example:__ Create a date one year from now.
/// 
/// ```javascript
/// r.now().add(365*24*60*60).run(conn, callback)
/// ```
/// 
/// __Example:__ Use [args](/api/javascript/args) with `add` to sum multiple values.
/// 
/// ```javascript
/// > vals = [10, 20, 30];
/// > r.add(r.args(vals)).run(conn, callback);
/// // result passed to callback
/// 60
/// ```
/// 
/// __Example:__ Concatenate an array of strings with `args`.
/// 
/// ```javascript
/// > vals = ['foo', 'bar', 'buzz'];
/// > r.add(r.args(vals)).run(conn, callback);
/// // result passed to callback
/// "foobarbuzz"
/// ```

                pub fn add<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "add", Some(Type::ADD), Some(args))
                }
            

                /// Subtract two numbers
///
/// 
/// __Example:__ It's as easy as 2 - 2 = 0.
/// 
/// ```javascript
/// r.expr(2).sub(2).run(conn, callback)
/// ```
/// 
/// __Example:__ Create a date one year ago today.
/// 
/// ```javascript
/// r.now().sub(365*24*60*60)
/// ```
/// 
/// __Example:__ Retrieve how many seconds elapsed between today and `date`.
/// 
/// ```javascript
/// r.now().sub(date)
/// ```

                pub fn sub<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "sub", Some(Type::SUB), Some(args))
                }
            

                /// Multiply two numbers, or make a periodic array
///
/// 
/// __Example:__ It's as easy as 2 * 2 = 4.
/// 
/// ```javascript
/// r.expr(2).mul(2).run(conn, callback)
/// ```
/// 
/// __Example:__ Arrays can be multiplied by numbers as well.
/// 
/// ```javascript
/// r.expr(["This", "is", "the", "song", "that", "never", "ends."]).mul(100).run(conn, callback)
/// ```
/// 

                pub fn mul<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "mul", Some(Type::MUL), Some(args))
                }
            

                /// Divide two numbers
///
/// 
/// __Example:__ It's as easy as 2 / 2 = 1.
/// 
/// ```javascript
/// r.expr(2).div(2).run(conn, callback)
/// ```
/// 

                pub fn div<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "div", Some(Type::DIV), Some(args))
                }
            

                
                pub fn mod_<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "mod_", Some(Type::MOD), Some(args))
                }
            

                /// Compute the logical "and" of one or more values
///
/// 
/// The `and` command can be used as an infix operator after its first argument (`r.expr(true).and(false)`) or given all of its arguments as parameters (`r.and(true,false)`).
/// 
/// Calling `and` with zero arguments will return `true`.
/// 
/// __Example:__ Return whether both `a` and `b` evaluate to true.
/// 
/// ```javascript
/// var a = true, b = false;
/// r.expr(a).and(b).run(conn, callback);
/// // result passed to callback
/// false
/// ```
/// 
/// __Example:__ Return whether all of `x`, `y` and `z` evaluate to true.
/// 
/// ```javascript
/// var x = true, y = true, z = true;
/// r.and(x, y, z).run(conn, callback);
/// // result passed to callback
/// true
/// ```

                pub fn and<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "and", Some(Type::AND), Some(args))
                }
            

                /// Compute the logical "or" of one or more values
///
/// 
/// The `or` command can be used as an infix operator after its first argument (`r.expr(true).or(false)`) or given all of its arguments as parameters (`r.or(true,false)`).
/// 
/// Calling `or` with zero arguments will return `false`.
/// 
/// __Example:__ Return whether either `a` or `b` evaluate to true.
/// 
/// ```javascript
/// var a = true, b = false;
/// r.expr(a).or(b).run(conn, callback);
/// // result passed to callback
/// true
/// ```
/// 
/// __Example:__ Return whether any of `x`, `y` or `z` evaluate to true.
/// 
/// ```javascript
/// var x = false, y = false, z = false;
/// r.or(x, y, z).run(conn, callback);
/// // result passed to callback
/// false
/// ```
/// 
/// __Note:__ When using `or` inside a `filter` predicate to test the values of fields that may not exist on the documents being tested, you should use the `default` command with those fields so they explicitly return `false`.
/// 
/// ```javascript
/// r.table('posts').filter(
///     r.row('category').default('foo').eq('article').
///     or(r.row('genre').default('foo').eq('mystery'))
/// ).run(conn, callback);
/// ```

                pub fn or<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "or", Some(Type::OR), Some(args))
                }
            

                /// Test if two or more values are equal
///
/// 
/// __Example:__ See if a user's `role` field is set to `administrator`. 
/// 
/// ```javascript
/// r.table('users').get(1)('role').eq('administrator').run(conn, callback);
/// ```
/// 
/// __Example:__ See if three variables contain equal values.
/// 
/// ```javascript
/// r.eq(a, b, c).run(conn, callback);
/// ```

                pub fn eq<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "eq", Some(Type::EQ), Some(args))
                }
            

                /// Test if two or more values are not equal
///
/// 
/// __Example:__ See if a user's `role` field is not set to `administrator`. 
/// 
/// ```javascript
/// r.table('users').get(1)('role').ne('administrator').run(conn, callback);
/// ```
/// 
/// __Example:__ See if three variables do not contain equal values.
/// 
/// ```javascript
/// r.ne(a, b, c).run(conn, callback);
/// ```

                pub fn ne<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "ne", Some(Type::NE), Some(args))
                }
            

                /// Compare values, testing if the left-hand value is greater than the right-hand
///
/// 
/// __Example:__ Test if a player has scored more than 10 points.
/// 
/// ```javascript
/// r.table('players').get(1)('score').gt(10).run(conn, callback);
/// ```
/// 
/// __Example:__ Test if variables are ordered from lowest to highest, with no values being equal to one another.
/// 
/// ```javascript
/// var a = 10, b = 20, c = 15;
/// r.gt(a, b, c).run(conn, callback);
/// ```
/// 
/// This is the equivalent of the following:
/// 
/// ```javascript
/// r.gt(a, b).and(r.gt(b, c)).run(conn, callback);
/// ```

                pub fn gt<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "gt", Some(Type::GT), Some(args))
                }
            

                /// Compare values, testing if the left-hand value is greater than or equal to the right-hand
///
/// 
/// __Example:__ Test if a player has scored 10 points or more.
/// 
/// ```javascript
/// r.table('players').get(1)('score').ge(10).run(conn, callback);
/// ```
/// 
/// __Example:__ Test if variables are ordered from lowest to highest.
/// 
/// ```javascript
/// var a = 10, b = 20, c = 15;
/// r.ge(a, b, c).run(conn, callback);
/// ```
/// 
/// This is the equivalent of the following:
/// 
/// ```javascript
/// r.ge(a, b).and(r.ge(b, c)).run(conn, callback);
/// ```

                pub fn ge<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "ge", Some(Type::GE), Some(args))
                }
            

                /// Compare values, testing if the left-hand value is less than the right-hand
///
/// 
/// __Example:__ Test if a player has scored less than 10 points.
/// 
/// ```javascript
/// r.table('players').get(1)('score').lt(10).run(conn, callback);
/// ```
/// 
/// __Example:__ Test if variables are ordered from highest to lowest, with no values being equal to one another.
/// 
/// ```javascript
/// var a = 20, b = 10,c = 15;
/// r.lt(a, b, c).run(conn, callback);
/// ```
/// 
/// This is the equivalent of the following:
/// 
/// ```javascript
/// r.lt(a, b).and(r.lt(b, c)).run(conn, callback);
/// ```

                pub fn lt<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "lt", Some(Type::LT), Some(args))
                }
            

                /// Compare values, testing if the left-hand value is less than or equal to the right-hand
///
/// 
/// __Example:__ Test if a player has scored 10 points or less.
/// 
/// ```javascript
/// r.table('players').get(1)('score').le(10).run(conn, callback);
/// ```
/// 
/// __Example:__ Test if variables are ordered from highest to lowest.
/// 
/// ```javascript
/// var a = 20, b = 10, c = 15;
/// r.le(a, b, c).run(conn, callback);
/// ```
/// 
/// This is the equivalent of the following:
/// 
/// ```javascript
/// r.le(a, b).and(r.le(b, c)).run(conn, callback);
/// ```

                pub fn le<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "le", Some(Type::LE), Some(args))
                }
            

                /// Compute the logical inverse (not) of an expression
///
/// 
/// `not` can be called either via method chaining, immediately after an expression that evaluates as a boolean value, or by passing the expression as a parameter to `not`. All values that are not `false` or `null` will be converted to `true`.
/// 
/// __Example:__ Not true is false.
/// 
/// ```javascript
/// r(true).not().run(conn, callback)
/// r.not(true).run(conn, callback)
/// ```
/// 
/// These evaluate to `false`.
/// 
/// __Example:__ Return all the users that do not have a "flag" field.
/// 
/// ```javascript
/// r.table('users').filter(function(user) {
///     return user.hasFields('flag').not()
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ As above, but prefix-style.
/// 
/// ```javascript
/// r.table('users').filter(function(user) {
///     return r.not(user.hasFields('flag'))
/// }).run(conn, callback)
/// ```

                pub fn not(&self) -> Client {
                    util::make_cmd::<Client>(self, "not", Some(Type::NOT), None)
                }
            

                /// Generate a random number between given (or implied) bounds
///
/// `random` takes zero, one or two arguments.
///
/// 
/// - With __zero__ arguments, the result will be a floating-point number in the range `[0,1)` (from 0 up to but not including 1).
/// - With __one__ argument _x,_ the result will be in the range `[0,x)`, and will be integer unless `{float: true}` is given as an option. Specifying a floating point number without the `float` option will raise an error.
/// - With __two__ arguments _x_ and _y,_ the result will be in the range `[x,y)`, and will be integer unless `{float: true}` is given as an option.  If _x_ and _y_ are equal an error will occur, unless the floating-point option has been specified, in which case _x_ will be returned. Specifying a floating point number without the `float` option will raise an error.
/// 
/// Note: The last argument given will always be the 'open' side of the range, but when generating a floating-point number, the 'open' side may be less than the 'closed' side.
/// 
/// __Example:__ Generate a random number in the range `[0,1)`
/// 
/// ```javascript
/// r.random().run(conn, callback)
/// ```
/// 
/// 
/// __Example:__ Generate a random integer in the range `[0,100)`
/// 
/// ```javascript
/// r.random(100).run(conn, callback)
/// r.random(0, 100).run(conn, callback)
/// ```
/// 
/// 
/// __Example:__ Generate a random number in the range `(-2.24,1.59]`
/// 
/// ```javascript
/// r.random(1.59, -2.24, {float: true}).run(conn, callback)
/// ```
/// 

                pub fn random(&self) -> Client {
                    util::make_cmd::<Client>(self, "random", Some(Type::RANDOM), None)
                }
            

                /// Rounds the given value to the nearest whole integer
///
/// 
/// For example, values of 1.0 up to but not including 1.5 will return 1.0, similar to [floor][]; values of 1.5 up to 2.0 will return 2.0, similar to [ceil][].
/// 
/// [floor]: /api/javascript/floor/
/// [ceil]:  /api/javascript/ceil/
/// 
/// __Example:__ Round 12.345 to the nearest integer.
/// 
/// ```javascript
/// r.round(12.345).run(conn, callback);
/// // Result passed to callback
/// 12.0
/// ```
/// 
/// The `round` command can also be chained after an expression.
/// 
/// __Example:__ Round -12.345 to the nearest integer.
/// 
/// ```javascript
/// r.expr(-12.345).round().run(conn, callback);
/// // Result passed to callback
/// -12.0
/// ```
/// 
/// __Example:__ Return Iron Man's weight, rounded to the nearest integer.
/// 
/// ```javascript
/// r.table('superheroes').get('ironman')('weight').round().run(conn, callback);
/// ```

                pub fn round(&self) -> Client {
                    util::make_cmd::<Client>(self, "round", Some(Type::ROUND), None)
                }
            

                /// Rounds the given value up, returning the smallest integer value greater than or equal to the given value (the value's ceiling)
///
/// 
/// __Example:__ Return the ceiling of 12.345.
/// 
/// ```javascript
/// r.ceil(12.345).run(conn, callback);
/// // Result passed to callback
/// 13.0
/// ```
/// 
/// The `ceil` command can also be chained after an expression.
/// 
/// __Example:__ Return the ceiling of -12.345.
/// 
/// ```javascript
/// r.expr(-12.345).ceil().run(conn, callback);
/// // Result passed to callback
/// -12.0
/// ```
/// 
/// __Example:__ Return Iron Man's weight, rounded up with `ceil`.
/// 
/// ```javascript
/// r.table('superheroes').get('ironman')('weight').ceil().run(conn, callback);
/// ```

                pub fn ceil(&self) -> Client {
                    util::make_cmd::<Client>(self, "ceil", Some(Type::CEIL), None)
                }
            

                /// Rounds the given value down, returning the largest integer value less than or equal to the given value (the value's floor)
///
/// 
/// __Example:__ Return the floor of 12.345.
/// 
/// ```javascript
/// r.floor(12.345).run(conn, callback);
/// // Result passed to callback
/// 12.0
/// ```
/// 
/// The `floor` command can also be chained after an expression.
/// 
/// __Example:__ Return the floor of -12.345.
/// 
/// ```javascript
/// r.expr(-12.345).floor().run(conn, callback);
/// // Result passed to callback
/// -13.0
/// ```
/// 
/// __Example:__ Return Iron Man's weight, rounded down with `floor`.
/// 
/// ```javascript
/// r.table('superheroes').get('ironman')('weight').floor().run(conn, callback);
/// ```

                pub fn floor(&self) -> Client {
                    util::make_cmd::<Client>(self, "floor", Some(Type::FLOOR), None)
                }
            

                /// Return a time object representing the current time in UTC
///
/// The command now() is computed once when the server receives the query, so multiple instances of r.now() will always return the same time inside a query.
///
/// 
/// __Example:__ Add a new user with the time at which he subscribed.
/// 
/// ```javascript
/// r.table("users").insert({
///     name: "John",
///     subscription_date: r.now()
/// }).run(conn, callback)
/// ```
/// 

                pub fn now(&self) -> Client {
                    util::make_cmd::<Client>(self, "now", Some(Type::NOW), None)
                }
            

                /// Create a time object for a specific time
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
/// ```javascript
/// r.table("user").get("John").update({birthdate: r.time(1986, 11, 3, 'Z')}).run(conn, callback)
/// ```

                pub fn time<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "time", Some(Type::TIME), Some(args))
                }
            

                /// Create a time object based on seconds since epoch
///
/// The first argument is a double and
///
/// will be rounded to three decimal places (millisecond-precision).
/// 
/// __Example:__ Update the birthdate of the user "John" to November 3rd, 1986.
/// 
/// ```javascript
/// r.table("user").get("John").update({birthdate: r.epochTime(531360000)}).run(conn, callback)
/// ```

                pub fn epoch_time<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "epoch_time", Some(Type::EPOCH_TIME), Some(args))
                }
            

                /// Create a time object based on an ISO 8601 date-time string (e
///
/// g. '2013-01-01T01:01:01+00:00'). RethinkDB supports all valid ISO 8601 formats except for week dates. Read more about the ISO 8601 format at [Wikipedia](http://en.wikipedia.org/wiki/ISO_8601).
///
/// 
/// If you pass an ISO 8601 string without a time zone, you must specify the time zone with the `defaultTimezone` argument.
/// 
/// __Example:__ Update the time of John's birth.
/// 
/// ```javascript
/// r.table("user").get("John").update({birth: r.ISO8601('1986-11-03T08:30:00-07:00')}).run(conn, callback)
/// ```
/// 
/// 

                pub fn iso8601<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "iso8601", Some(Type::ISO8601), Some(args))
                }
            

                /// Return a new time object with a different timezone
///
/// While the time stays the same, the results returned by methods such as hours() will change since they take the timezone into account. The timezone argument has to be of the ISO 8601 format.
///
/// 
/// __Example:__ Hour of the day in San Francisco (UTC/GMT -8, without daylight saving time).
/// 
/// ```javascript
/// r.now().inTimezone('-08:00').hours().run(conn, callback)
/// ```
/// 
/// 

                pub fn in_timezone<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "in_timezone", Some(Type::IN_TIMEZONE), Some(args))
                }
            

                /// Return the timezone of the time object
///
/// 
/// __Example:__ Return all the users in the "-07:00" timezone.
/// 
/// ```javascript
/// r.table("users").filter( function(user) {
///     return user("subscriptionDate").timezone().eq("-07:00")
/// })
/// ```
/// 
/// 

                pub fn timezone(&self) -> Client {
                    util::make_cmd::<Client>(self, "timezone", Some(Type::TIMEZONE), None)
                }
            

                /// Return whether a time is between two other times
///
/// 
/// By default, this is inclusive of the start time and exclusive of the end time. Set `leftBound` and `rightBound` to explicitly include (`closed`) or exclude (`open`) that endpoint of the range.
/// 
/// __Example:__ Retrieve all the posts that were posted between December 1st, 2013
/// (inclusive) and December 10th, 2013 (exclusive).
/// 
/// ```javascript
/// r.table("posts").filter(
///     r.row('date').during(r.time(2013, 12, 1, "Z"), r.time(2013, 12, 10, "Z"))
/// ).run(conn, callback)
/// ```
/// 
/// 
/// __Example:__ Retrieve all the posts that were posted between December 1st, 2013
/// (exclusive) and December 10th, 2013 (inclusive).
/// 
/// ```javascript
/// r.table("posts").filter(
///   r.row('date').during(r.time(2013, 12, 1, "Z"), r.time(2013, 12, 10, "Z"), {leftBound: "open", rightBound: "closed"})
/// ).run(conn, callback)
/// ```
/// 

                pub fn during<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "during", Some(Type::DURING), Some(args))
                }
            

                /// Return a new time object only based on the day, month and year (ie
///
/// the same day at 00:00).
///
/// 
/// __Example:__ Retrieve all the users whose birthday is today.
/// 
/// ```javascript
/// r.table("users").filter(function(user) {
///     return user("birthdate").date().eq(r.now().date())
/// }).run(conn, callback)
/// ```
/// 
/// <!-- stop -->
/// 
/// Note that the [now][] command always returns UTC time, so the comparison may fail if `user("birthdate")` isn't also in UTC. You can use the [inTimezone][itz] command to adjust for this:
/// 
/// ```javascript
/// r.table("users").filter(function(user) {
///     return user("birthdate").date().eq(r.now().inTimezone("-08:00").date())
/// }).run(conn, callback)
/// ```
/// 
/// [now]: /api/javascript/now/
/// [itz]: /api/javascript/in_timezone/

                pub fn date(&self) -> Client {
                    util::make_cmd::<Client>(self, "date", Some(Type::DATE), None)
                }
            

                /// Return the number of seconds elapsed since the beginning of the day stored in the time object
///
/// 
/// __Example:__ Retrieve posts that were submitted before noon.
/// 
/// ```javascript
/// r.table("posts").filter(
///     r.row("date").timeOfDay().le(12*60*60)
/// ).run(conn, callback)
/// ```
/// 
/// 
/// 

                pub fn time_of_day(&self) -> Client {
                    util::make_cmd::<Client>(self, "time_of_day", Some(Type::TIME_OF_DAY), None)
                }
            

                /// Return the year of a time object
///
/// 
/// __Example:__ Retrieve all the users born in 1986.
/// 
/// ```javascript
/// r.table("users").filter(function(user) {
///     return user("birthdate").year().eq(1986)
/// }).run(conn, callback)
/// ```

                pub fn year(&self) -> Client {
                    util::make_cmd::<Client>(self, "year", Some(Type::YEAR), None)
                }
            

                /// Return the month of a time object as a number between 1 and 12
///
/// For your convenience, the terms r.january, r.february etc. are defined and map to the appropriate integer.
///
/// 
/// __Example:__ Retrieve all the users who were born in November.
/// 
/// ```javascript
/// r.table("users").filter(
///     r.row("birthdate").month().eq(11)
/// )
/// ```
/// 
/// 
/// __Example:__ Retrieve all the users who were born in November.
/// 
/// ```javascript
/// r.table("users").filter(
///     r.row("birthdate").month().eq(r.november)
/// )
/// ```
/// 

                pub fn month(&self) -> Client {
                    util::make_cmd::<Client>(self, "month", Some(Type::MONTH), None)
                }
            

                /// Return the day of a time object as a number between 1 and 31
///
/// 
/// __Example:__ Return the users born on the 24th of any month.
/// 
/// ```javascript
/// r.table("users").filter(
///     r.row("birthdate").day().eq(24)
/// ).run(conn, callback)
/// ```
/// 
/// 

                pub fn day(&self) -> Client {
                    util::make_cmd::<Client>(self, "day", Some(Type::DAY), None)
                }
            

                /// Return the day of week of a time object as a number between 1 and 7 (following ISO 8601 standard)
///
/// For your convenience, the terms r.monday, r.tuesday etc. are defined and map to the appropriate integer.
///
/// 
/// __Example:__ Return today's day of week.
/// 
/// ```javascript
/// r.now().dayOfWeek().run(conn, callback)
/// ```
/// 
/// __Example:__ Retrieve all the users who were born on a Tuesday.
/// 
/// ```javascript
/// r.table("users").filter(
///     r.row("birthdate").dayOfWeek().eq(r.tuesday)
/// )
/// ```
/// 

                pub fn day_of_week(&self) -> Client {
                    util::make_cmd::<Client>(self, "day_of_week", Some(Type::DAY_OF_WEEK), None)
                }
            

                /// Return the day of the year of a time object as a number between 1 and 366 (following ISO 8601 standard)
///
/// 
/// __Example:__ Retrieve all the users who were born the first day of a year.
/// 
/// ```javascript
/// r.table("users").filter(
///     r.row("birthdate").dayOfYear().eq(1)
/// )
/// ```
/// 
/// 

                pub fn day_of_year(&self) -> Client {
                    util::make_cmd::<Client>(self, "day_of_year", Some(Type::DAY_OF_YEAR), None)
                }
            

                /// Return the hour in a time object as a number between 0 and 23
///
/// 
/// __Example:__ Return all the posts submitted after midnight and before 4am.
/// 
/// ```javascript
/// r.table("posts").filter(function(post) {
///     return post("date").hours().lt(4)
/// })
/// ```
/// 

                pub fn hours(&self) -> Client {
                    util::make_cmd::<Client>(self, "hours", Some(Type::HOURS), None)
                }
            

                /// Return the minute in a time object as a number between 0 and 59
///
/// 
/// __Example:__ Return all the posts submitted during the first 10 minutes of every hour.
/// 
/// ```javascript
/// r.table("posts").filter(function(post) {
///     return post("date").minutes().lt(10)
/// })
/// ```
/// 
/// 

                pub fn minutes(&self) -> Client {
                    util::make_cmd::<Client>(self, "minutes", Some(Type::MINUTES), None)
                }
            

                /// Return the seconds in a time object as a number between 0 and 59
///
/// 999 (double precision).
///
/// 
/// __Example:__ Return the post submitted during the first 30 seconds of every minute.
/// 
/// ```javascript
/// r.table("posts").filter(function(post) {
///     return post("date").seconds().lt(30)
/// })
/// ```
/// 

                pub fn seconds(&self) -> Client {
                    util::make_cmd::<Client>(self, "seconds", Some(Type::SECONDS), None)
                }
            

                /// Convert a time object to a string in ISO 8601 format
///
/// 
/// __Example:__ Return the current ISO 8601 time.
/// 
/// ```javascript
/// r.now().toISO8601().run(conn, callback)
/// // Result passed to callback
/// "2015-04-20T18:37:52.690+00:00"
/// ```
/// 

                pub fn to_iso8601(&self) -> Client {
                    util::make_cmd::<Client>(self, "to_iso8601", Some(Type::TO_ISO8601), None)
                }
            

                /// Convert a time object to its epoch time
///
/// 
/// __Example:__ Return the current time in seconds since the Unix Epoch with millisecond-precision.
/// 
/// ```javascript
/// r.now().toEpochTime()
/// ```
/// 
/// 

                pub fn to_epoch_time(&self) -> Client {
                    util::make_cmd::<Client>(self, "to_epoch_time", Some(Type::TO_EPOCH_TIME), None)
                }
            

                /// Encapsulate binary data within a query
///
/// 
/// The type of data `binary` accepts depends on the client language. In JavaScript, it expects a [Node.js](http://nodejs.org) `Buffer`. Using a `Buffer` object within a query implies the use of `binary` and the ReQL driver will automatically perform the coercion.
/// 
/// Binary objects returned to the client in JavaScript will also be Node.js `Buffer` objects. This can be changed with the `binaryFormat` option provided to [run](/api/javascript/run) to return "raw" objects.
/// 
/// Only a limited subset of ReQL commands may be chained after `binary`:
/// 
/// * [coerceTo](/api/javascript/coerce_to/) can coerce `binary` objects to `string` types
/// * [count](/api/javascript/count/) will return the number of bytes in the object
/// * [slice](/api/javascript/slice/) will treat bytes like array indexes (i.e., `slice(10,20)` will return bytes 10&ndash;19)
/// * [typeOf](/api/javascript/type_of) returns `PTYPE<BINARY>`
/// * [info](/api/javascript/info) will return information on a binary object.
/// 
/// __Example:__ Save an avatar image to a existing user record.
/// 
/// ```javascript
/// var fs = require('fs');
/// fs.readFile('./defaultAvatar.png', function (err, avatarImage) {
///     if (err) {
///         // Handle error
///     }
///     else {
///         r.table('users').get(100).update({
///             avatar: avatarImage
///         })
///     }
/// });
/// ```
/// 
/// __Example:__ Get the size of an existing avatar image.
/// 
/// ```javascript
/// r.table('users').get(100)('avatar').count().run(conn, callback);
/// // result returned to callback
/// 14156
/// ```
/// 
/// Read more details about RethinkDB's binary object support: [Storing binary objects](/docs/storing-binary/).

                pub fn binary<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "binary", Some(Type::BINARY), Some(args))
                }
            

                /// Call an anonymous function using return values from other ReQL commands or queries as arguments
///
/// 
/// The last argument to `do` (or, in some forms, the only argument) is an expression or an anonymous function which receives values from either the previous arguments or from prefixed commands chained before `do`. The `do` command is essentially a single-element [map](/api/javascript/map/), letting you map a function over just one document. This allows you to bind a query result to a local variable within the scope of `do`, letting you compute the result just once and reuse it in a complex expression or in a series of ReQL commands.
/// 
/// Arguments passed to the `do` function must be basic data types, and cannot be streams or selections. (Read about [ReQL data types](/docs/data-types/).) While the arguments will all be evaluated before the function is executed, they may be evaluated in any order, so their values should not be dependent on one another. The type of `do`'s result is the type of the value returned from the function or last expression.
/// 
/// __Example:__ Compute a golfer's net score for a game.
/// 
/// ```javascript
/// r.table('players').get('f19b5f16-ef14-468f-bd48-e194761df255').do(
///     function (player) {
///         return player('gross_score').sub(player('course_handicap'));
///     }
/// ).run(conn, callback);
/// ```
/// 
/// __Example:__ Return the best scoring player in a two-player golf match.
/// 
/// ```javascript
/// r.do(r.table('players').get(id1), r.table('players').get(id2),
///     function (player1, player2) {
///         return r.branch(player1('gross_score').lt(player2('gross_score')),
///             player1, player2);
///     }
/// ).run(conn, callback);
/// ```
/// 
/// Note that `branch`, the ReQL conditional command, must be used instead of `if`. See the `branch` [documentation](/api/javascript/branch) for more.
/// 
/// __Example:__ Take different actions based on the result of a ReQL [insert](/api/javascript/insert) command.
/// 
/// ```javascript
/// var newData = {
///     id: 100,
///     name: 'Agatha',
///     gross_score: 57,
///     course_handicap: 4
/// };
/// r.table('players').insert(newData).do(
///     function (doc) {
///         return r.branch(doc('inserted').ne(0),
///             r.table('log').insert({time: r.now(), response: doc, result: 'ok'}),
///             r.table('log').insert({time: r.now(), response: doc, result: 'error'}))
///     }
/// ).run(conn, callback);
/// ```

                pub fn do_<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "do_", Some(Type::FUNCALL), Some(args))
                }
            

                /// Perform a branching conditional equivalent to `if-then-else`
///
/// 
/// The `branch` command takes 2n+1 arguments: pairs of conditional expressions and commands to be executed if the conditionals return any value but `false` or `null` (i.e., "truthy" values), with a final "else" command to be evaluated if all of the conditionals are `false` or `null`.
/// 
/// <!-- break -->
/// 
/// You may call `branch` infix style on the first test. (See the second example for an illustration.)
/// 
/// ```javascript
/// r.branch(test1, val1, test2, val2, elseval)
/// ```
/// 
/// is the equivalent of the JavaScript statement
/// 
/// ```javascript
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
/// ```javascript
/// var x = 10;
/// r.branch(r.expr(x).gt(5), 'big', 'small').run(conn, callback);
/// // Result passed to callback
/// "big"
/// ```
/// 
/// __Example:__ As above, infix-style.
/// 
/// ```javascript
/// var x = 10;
/// r.expr(x).gt(5).branch('big', 'small').run(conn, callback);
/// // Result passed to callback
/// "big"
/// ```
/// 
/// __Example:__ Categorize heroes by victory counts.
/// 
/// ```javascript
/// r.table('marvel').map(
///     r.branch(
///         r.row('victories').gt(100),
///         r.row('name').add(' is a superhero'),
///         r.row('victories').gt(10),
///         r.row('name').add(' is a hero'),
///         r.row('name').add(' is very nice')
///     )
/// ).run(conn, callback);
/// ```
/// 
/// If the documents in the table `marvel` are:
/// 
/// ```javascript
/// [
///     { name: "Iron Man", victories: 214 },
///     { name: "Jubilee", victories: 49 },
///     { name: "Slava", victories: 5 }
/// ]
/// ```
/// 
/// The results will be:
/// 
/// ```javascript
/// [
///     "Iron Man is a superhero",
///     "Jubilee is a hero",
///     "Slava is very nice"
/// ]
/// ```

                pub fn branch<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "branch", Some(Type::BRANCH), Some(args))
                }
            

                /// Loop over a sequence, evaluating the given write query for each element
///
/// 
/// __Example:__ Now that our heroes have defeated their villains, we can safely remove them from the villain table.
/// 
/// ```javascript
/// r.table('marvel').forEach(function(hero) {
///     return r.table('villains').get(hero('villainDefeated')).delete()
/// }).run(conn, callback)
/// ```

                pub fn for_each<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "for_each", Some(Type::FOR_EACH), Some(args))
                }
            

                /// Generate a stream of sequential integers in a specified range
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
/// ```javascript
/// > r.range(4).run(conn, callback)
/// // result returned to callback
/// [0, 1, 2, 3]
/// ```
/// 
/// <!-- stop -->
/// 
/// You can also use the [limit](/api/javascript/limit) command with the no-argument variant to achieve the same result in this case:
/// 
/// ```javascript
/// > r.range().limit(4).run(conn, callback)
/// // result returned to callback
/// [0, 1, 2, 3]
/// ```
/// 
/// __Example:__ Return a range from -5 through 5.
/// 
/// ```javascript
/// > r.range(-5, 6).run(conn, callback)
/// // result returned to callback
/// [-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5]
/// ```

                pub fn range(&self) -> Client {
                    util::make_cmd::<Client>(self, "range", Some(Type::RANGE), None)
                }
            

                /// Throw a runtime error
///
/// If called with no arguments inside the second argument to `default`, re-throw the current error.
///
/// 
/// __Example:__ Iron Man can't possibly have lost a battle:
/// 
/// ```javascript
/// r.table('marvel').get('IronMan').do(function(ironman) {
///     return r.branch(ironman('victories').lt(ironman('battles')),
///         r.error('impossible code path'),
///         ironman)
/// }).run(conn, callback)
/// ```
/// 
/// 

                pub fn error<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "error", Some(Type::ERROR), Some(args))
                }
            

                /// Provide a default value in case of non-existence errors
///
/// The `default` command evaluates its first argument (the value it's chained to). If that argument returns `null` or a non-existence error is thrown in evaluation, then `default` returns its second argument. The second argument is usually a default value, but it can be a function that returns a value.
///
/// 
/// __Example:__ Retrieve the titles and authors of the table `posts`.
/// In the case where the author field is missing or `null`, we want to retrieve the string
/// `Anonymous`.
/// 
/// ```javascript
/// r.table("posts").map(function (post) {
///     return {
///         title: post("title"),
///         author: post("author").default("Anonymous")
///     }
/// }).run(conn, callback);
/// ```
/// 
/// <!-- stop -->
/// 
/// We can rewrite the previous query with `r.branch` too.
/// 
/// ```javascript
/// r.table("posts").map(function (post) {
///     return r.branch(
///         post.hasFields("author"),
///         {
///             title: post("title"),
///             author: post("author")
///         },
///         {
///             title: post("title"),
///             author: "Anonymous" 
///         }
///     )
/// }).run(conn, callback);
/// ```
/// 
/// __Example:__ The `default` command can also be used to filter documents. Retrieve all our users who are not grown-ups or whose age is unknown
/// (i.e., the field `age` is missing or equals `null`).
/// 
/// ```javascript
/// r.table("users").filter(function (user) {
///     return user("age").lt(18).default(true)
/// }).run(conn, callback);
/// ```
/// 
/// One more way to write the previous query is to set the age to be `-1` when the
/// field is missing.
/// 
/// ```javascript
/// r.table("users").filter(function (user) {
///     return user("age").default(-1).lt(18)
/// }).run(conn, callback);
/// ```
/// 
/// This can be accomplished with [hasFields](/api/javascript/has_fields/) rather than `default`.
/// 
/// ```javascript
/// r.table("users").filter(function (user) {
///     return user.hasFields("age").not().or(user("age").lt(18))
/// }).run(conn, callback);
/// ```
/// 
/// The body of every [filter](/api/javascript/filter/) is wrapped in an implicit `.default(false)`. You can overwrite the value `false` with the `default` option.
/// 
/// ```javascript
/// r.table("users").filter(function (user) {
///     return user("age").lt(18)
/// }, {default: true} ).run(conn, callback);
/// ```
/// 
/// __Example:__ The function form of `default` receives the error message as its argument.
/// 
/// ```javascript
/// r.table("posts").map(function (post) {
///     return {
///         title: post("title"),
///         author: post("author").default(function (err) {
///             return err;
///         })
///     }
/// }).run(conn, callback);
/// ```
/// 
/// This particular example simply returns the error message, so it isn't very useful. But it would be possible to change the default value based on the specific error message thrown.

                pub fn default<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "default", Some(Type::DEFAULT), Some(args))
                }
            

                /// Construct a ReQL JSON object from a native object
///
/// 
/// If the native object is a Node.js `Buffer`, then `expr` will return a binary object. See [binary](/api/javascript/binary) for more information.
/// 
/// __Example:__ Objects wrapped with `expr` can then be manipulated by ReQL API functions.
/// 
/// ```javascript
/// r.expr({a:'b'}).merge({b:[1,2,3]}).run(conn, callback)
/// ```
/// 
/// 
/// __Example:__ In JavaScript, you can also do this with just r.
/// 
/// ```javascript
/// r({a: 'b'}).merge({b: [1,2,3]}).run(conn, callback)
/// ```
/// 

                pub fn expr<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "expr", None, Some(args))
                }
            

                /// Create a javascript expression
///
/// 
/// `timeout` is the number of seconds before `r.js` times out. The default value is 5 seconds.
/// 
/// {% infobox %}
/// Whenever possible, you should use native ReQL commands rather than `r.js` for better performance.
/// {% endinfobox %}
/// 
/// __Example:__ Concatenate two strings using JavaScript.
/// 
/// ```javascript
/// r.js("'str1' + 'str2'").run(conn, callback)
/// ```
/// 
/// __Example:__ Select all documents where the 'magazines' field is greater than 5 by running JavaScript on the server.
/// 
/// ```javascript
/// r.table('marvel').filter(
///     r.js('(function (row) { return row.magazines.length > 5; })')
/// ).run(conn, callback)
/// ```
/// 
/// 
/// __Example:__ You may also specify a timeout in seconds (defaults to 5).
/// 
/// ```javascript
/// r.js('while(true) {}', {timeout:1.3}).run(conn, callback)
/// ```
/// 

                pub fn js<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "js", Some(Type::JAVASCRIPT), Some(args))
                }
            

                /// Convert a value of one type into another
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
/// ```javascript
/// r.table('posts').map(function (post) {
///     return post.merge({ comments: r.table('comments').getAll(post('id'), {index: 'postId'}).coerceTo('array')});
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ Coerce an array of key-value pairs into an object.
/// 
/// 
/// ```javascript
/// r.expr([['name', 'Ironman'], ['victories', 2000]]).coerceTo('object').run(conn, callback)
/// ```
/// 
/// __Note:__ To coerce a list of key-value pairs like `['name', 'Ironman', 'victories', 2000]` to an object, use the [object](/api/javascript/object) command.
/// 
/// __Example:__ Coerce a number to a string.
/// 
/// ```javascript
/// r.expr(1).coerceTo('string').run(conn, callback)
/// ```

                pub fn coerce_to<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "coerce_to", Some(Type::COERCE_TO), Some(args))
                }
            

                /// Gets the type of a ReQL query's return value
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
/// ```javascript
/// r.expr("foo").typeOf().run(conn, callback);
/// // Result passed to callback
/// "STRING"
/// ```

                pub fn type_of(&self) -> Client {
                    util::make_cmd::<Client>(self, "type_of", Some(Type::TYPE_OF), None)
                }
            

                /// Get information about a ReQL value
///
/// 
/// __Example:__ Get information about a table such as primary key, or cache size.
/// 
/// ```javascript
/// r.table('marvel').info().run(conn, callback)
/// ```

                pub fn info(&self) -> Client {
                    util::make_cmd::<Client>(self, "info", Some(Type::INFO), None)
                }
            

                /// Parse a JSON string on the server
///
/// 
/// __Example:__ Send an array to the server.
/// 
/// ```javascript
/// r.json("[1,2,3]").run(conn, callback)
/// ```

                pub fn json<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "json", Some(Type::JSON), Some(args))
                }
            

                /// Convert a ReQL value or object to a JSON string
///
/// You may use either `toJsonString` or `toJSON`.
///
/// 
/// __Example:__ Get a ReQL document as a JSON string.
/// 
/// ```javascript
/// > r.table('hero').get(1).toJSON()
/// // result returned to callback
/// '{"id": 1, "name": "Batman", "city": "Gotham", "powers": ["martial arts", "cinematic entrances"]}'
/// ```

                pub fn to_json<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "to_json", Some(Type::TO_JSON_STRING), Some(args))
                }
            

                /// Retrieve data from the specified URL over HTTP
///
/// The return type depends on the `resultFormat` option, which checks the `Content-Type` of the response by default.
///
/// 
/// __Example:__ Perform an HTTP `GET` and store the result in a table.
/// 
/// ```javascript
/// r.table('posts').insert(r.http('http://httpbin.org/get')).run(conn, callback)
/// ```
/// 
/// <!-- stop -->
/// 
/// See [the tutorial](/docs/external-api-access/) on `r.http` for more examples on how to use this command.
/// 
/// # Options #
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
/// * `params`: object specifying URL parameters to append to the URL as encoded key/value pairs. `{ query: 'banana', limit: 2 }` will be appended as `?query=banana&limit=2`. Default: no parameters.
/// * `header`: Extra header lines to include. The value may be an array of strings or an object. Default: `Accept-Encoding: deflate;q=1, gzip;q=0.5` and `User-Agent: RethinkDB/<VERSION>`.
/// * `data`: Data to send to the server on a `POST`, `PUT`, `PATCH`, or `DELETE` request. For `POST` requests, data may be either an object (which will be written to the body as form-encoded key/value pairs) or a string; for all other requests, data will be serialized as JSON and placed in the request body, sent as `Content-Type: application/json`. Default: no data will be sent.
/// 
/// __Example:__ Perform multiple requests with different parameters.
/// 
/// ```javascript
/// r.expr([1, 2, 3]).map(function(i) {
///     return r.http('http://httpbin.org/get', { params: { user: i } });
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ Perform a `PUT` request for each item in a table.
/// 
/// ```javascript
/// r.table('data').map(function(row) {
///     return r.http('http://httpbin.org/put', { method: 'PUT', data: row });
/// }).run(conn, callback)
/// ```
/// 
/// __Example:__ Perform a `POST` request with accompanying data.
/// 
/// Using form-encoded data:
/// 
/// ```javascript
/// r.http('http://httpbin.org/post',
///        { method: 'POST', data: { player: 'Bob', game: 'tic tac toe' } })
/// .run(conn, callback)
/// ```
/// 
/// Using JSON data:
/// 
/// ```javascript
/// r.http('http://httpbin.org/post',
///        { method: 'POST',
///          data: r.expr(value).coerceTo('string'),
///          header: { 'Content-Type': 'application/json' } })
/// .run(conn, callback)
/// ```
/// 
/// ## Pagination
/// 
/// `r.http` supports depagination, which will request multiple pages in a row and aggregate the results into a stream.  The use of this feature is controlled by the optional arguments `page` and `pageLimit`.  Either none or both of these arguments must be provided.
/// 
/// * `page`: This option may specify either a built-in pagination strategy (see below), or a function to provide the next URL and/or `params` to request.
/// * `pageLimit`: An integer specifying the maximum number of requests to issue using the `page` functionality.  This is to prevent overuse of API quotas, and must be specified with `page`.
///     * `-1`: no limit
///     * `0`: no requests will be made, an empty stream will be returned
///     * `n`: `n` requests will be made
/// 
/// At the moment, the only built-in strategy is `'link-next'`, which is equivalent to `function(info) { return info('header')('link')('rel="next"').default(null); }`.
/// 
/// __Example:__ Perform a GitHub search and collect up to 3 pages of results.
/// 
/// ```javascript
/// r.http("https://api.github.com/search/code?q=addClass+user:mozilla",
///        { page: 'link-next', pageLimit: 3 }
/// ).run(conn, callback)
/// ```
/// 
/// As a function, `page` takes one parameter, an object of the format:
/// 
/// ```javascript
/// {
///     params: object // the URL parameters used in the last request
///     header: object // the HTTP headers of the last response as key/value pairs
///     body: value // the body of the last response in the format specified by `resultFormat`
/// }
/// ```
/// 
/// The `header` field will be a parsed version of the header with fields lowercased, like so:
/// 
/// ```javascript
/// {
///     'content-length': '1024',
///     'content-type': 'application/json',
///     'date': 'Thu, 1 Jan 1970 00:00:00 GMT',
///     'link': {
///         'rel="last"': 'http://example.com/?page=34',
///         'rel="next"': 'http://example.com/?page=2'
///     }
/// }
/// ```
/// 
/// The `page` function may return a string corresponding to the next URL to request, `null` indicating that there is no more to get, or an object of the format:
/// 
/// ```javascript
/// {
///     url: string // the next URL to request, or null for no more pages
///     params: object // new URL parameters to use, will be merged with the previous request's params
/// }
/// ```
/// 
/// __Example:__ Perform depagination with a custom `page` function.
/// 
/// ```javascript
/// r.http('example.com/pages',
///        { page: function(info) { return info('body')('meta')('next').default(null); },
///          pageLimit: 5 })
/// .run(conn, callback)
/// ```
/// 
/// # Learn more
/// 
/// See [the tutorial](/docs/external-api-access/) on `r.http` for more examples on how to use this command.

                pub fn http<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "http", Some(Type::HTTP), Some(args))
                }
            

                /// Return a UUID (universally unique identifier), a string that can be used as a unique ID
///
/// If a string is passed to `uuid` as an argument, the UUID will be deterministic, derived from the string's SHA-1 hash.
///
/// 
/// RethinkDB's UUIDs are standards-compliant. Without the optional argument, a version 4 random UUID will be generated; with that argument, a version 5 UUID will be generated, using a fixed namespace UUID of `91461c99-f89d-49d2-af96-d8e2e14e9b58`. For more information, read [Wikipedia's UUID article][uu].
/// 
/// [uu]: https://en.wikipedia.org/wiki/Universally_unique_identifier
/// 
/// __Example:__ Generate a UUID.
/// 
/// ```javascript
/// > r.uuid().run(conn, callback)
/// // result returned to callback
/// "27961a0e-f4e8-4eb3-bf95-c5203e1d87b9"
/// ```
/// 
/// __Example:__ Generate a UUID based on a string.
/// 
/// ```javascript
/// > r.uuid("slava@example.com").run(conn, callback)
/// // Result passed to callback
/// "90691cbc-b5ea-5826-ae98-951e30fc3b2d"
/// ```

                pub fn uuid(&self) -> Client {
                    util::make_cmd::<Client>(self, "uuid", Some(Type::UUID), None)
                }
            

                /// Construct a circular line or polygon
///
/// A circle in RethinkDB is a polygon or line *approximating* a circle of a given radius around a given center, consisting of a specified number of vertices (default 32).
///
/// 
/// The center may be specified either by two floating point numbers, the latitude (&minus;90 to 90) and longitude (&minus;180 to 180) of the point on a perfect sphere (see [Geospatial support](/docs/geo-support/) for more information on ReQL's coordinate system), or by a point object. The radius is a floating point number whose units are meters by default, although that may be changed with the `unit` argument.
/// 
/// Optional arguments available with `circle` are:
/// 
/// * `numVertices`: the number of vertices in the polygon or line. Defaults to 32.
/// * `geoSystem`: the reference ellipsoid to use for geographic coordinates. Possible values are `WGS84` (the default), a common standard for Earth's geometry, or `unit_sphere`, a perfect sphere of 1 meter radius.
/// * `unit`: Unit for the radius distance. Possible values are `m` (meter, the default), `km` (kilometer), `mi` (international mile), `nm` (nautical mile), `ft` (international foot).
/// * `fill`: if `true` (the default) the circle is filled, creating a polygon; if `false` the circle is unfilled (creating a line).
/// 
/// 
/// 
/// __Example:__ Define a circle.
/// 
/// ```javascript
/// r.table('geo').insert({
///     id: 300,
///     name: 'Hayes Valley',
///     neighborhood: r.circle([-122.423246,37.779388], 1000)
/// }).run(conn, callback);
/// ```

                pub fn circle<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "circle", Some(Type::CIRCLE), Some(args))
                }
            

                /// Compute the distance between a point and another geometry object
///
/// At least one of the geometry objects specified must be a point.
///
/// 
/// Optional arguments available with `distance` are:
/// 
/// * `geoSystem`: the reference ellipsoid to use for geographic coordinates. Possible values are `WGS84` (the default), a common standard for Earth's geometry, or `unit_sphere`, a perfect sphere of 1 meter radius.
/// * `unit`: Unit to return the distance in. Possible values are `m` (meter, the default), `km` (kilometer), `mi` (international mile), `nm` (nautical mile), `ft` (international foot).
/// 
/// If one of the objects is a polygon or a line, the point will be projected onto the line or polygon assuming a perfect sphere model before the distance is computed (using the model specified with `geoSystem`). As a consequence, if the polygon or line is extremely large compared to Earth's radius and the distance is being computed with the default WGS84 model, the results of `distance` should be considered approximate due to the deviation between the ellipsoid and spherical models.
/// 
/// 
/// __Example:__ Compute the distance between two points on the Earth in kilometers.
/// 
/// ```javascript
/// var point1 = r.point(-122.423246,37.779388);
/// var point2 = r.point(-117.220406,32.719464);
/// r.distance(point1, point2, {unit: 'km'}).run(conn, callback);
/// // result returned to callback 
/// 734.1252496021841
/// ```

                pub fn distance<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "distance", Some(Type::DISTANCE), Some(args))
                }
            

                /// Convert a Line object into a Polygon object
///
/// If the last point does not specify the same coordinates as the first point, `polygon` will close the polygon by connecting them.
///
/// 
/// Longitude (&minus;180 to 180) and latitude (&minus;90 to 90) of vertices are plotted on a perfect sphere. See [Geospatial support](/docs/geo-support/) for more information on ReQL's coordinate system.
/// 
/// If the last point does not specify the same coordinates as the first point, `polygon` will close the polygon by connecting them. You cannot directly construct a polygon with holes in it using `polygon`, but you can use [polygonSub](/api/javascript/polygon_sub) to use a second polygon within the interior of the first to define a hole.
/// 
/// 
/// __Example:__ Create a line object and then convert it to a polygon.
/// 
/// ```javascript
/// r.table('geo').insert({
///     id: 201,
///     rectangle: r.line(
///         [-122.423246,37.779388],
///         [-122.423246,37.329898],
///         [-121.886420,37.329898],
///         [-121.886420,37.779388]
///     )
/// }).run(conn, callback);
/// 
/// r.table('geo').get(201).update({
///     rectangle: r.row('rectangle').fill()
/// }, {nonAtomic: true}).run(conn, callback);
/// ```

                pub fn fill(&self) -> Client {
                    util::make_cmd::<Client>(self, "fill", Some(Type::FILL), None)
                }
            

                /// Convert a [GeoJSON](http://geojson
///
/// org) object to a ReQL geometry object.
///
/// 
/// RethinkDB only allows conversion of GeoJSON objects which have ReQL equivalents: Point, LineString, and Polygon. MultiPoint, MultiLineString, and MultiPolygon are not supported. (You could, however, store multiple points, lines and polygons in an array and use a geospatial multi index with them.)
/// 
/// Only longitude/latitude coordinates are supported. GeoJSON objects that use Cartesian coordinates, specify an altitude, or specify their own coordinate reference system will be rejected.
/// 
/// __Example:__ Convert a GeoJSON object to a ReQL geometry object.
/// 
/// ```javascript
/// var geoJson = {
///     'type': 'Point',
///     'coordinates': [ -122.423246, 37.779388 ]
/// };
/// r.table('geo').insert({
///     id: 'sfo',
///     name: 'San Francisco',
///     location: r.geojson(geoJson)
/// }).run(conn, callback);
/// ```

                pub fn geojson<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "geojson", Some(Type::GEOJSON), Some(args))
                }
            

                /// Convert a ReQL geometry object to a [GeoJSON](http://geojson
///
/// org) object.
///
/// 
/// __Example:__ Convert a ReQL geometry object to a GeoJSON object.
/// 
/// ```javascript
/// r.table('geo').get('sfo')('location').toGeojson.run(conn, callback);
/// // result passed to callback
/// {
///     'type': 'Point',
///     'coordinates': [ -122.423246, 37.779388 ]
/// }
/// ```

                pub fn to_geojson(&self) -> Client {
                    util::make_cmd::<Client>(self, "to_geojson", Some(Type::TO_GEOJSON), None)
                }
            

                /// Get all documents where the given geometry object intersects the geometry object of the requested geospatial index
///
/// 
/// The `index` argument is mandatory. This command returns the same results as `table.filter(r.row('index').intersects(geometry))`. The total number of results is limited to the array size limit which defaults to 100,000, but can be changed with the `arrayLimit` option to [run](/api/javascript/run).
/// 
/// __Example:__ Which of the locations in a list of parks intersect `circle1`?
/// 
/// ```javascript
/// var circle1 = r.circle([-117.220406,32.719464], 10, {unit: 'mi'});
/// r.table('parks').getIntersecting(circle1, {index: 'area'}).run(conn, callback);
/// ```

                pub fn get_intersecting<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "get_intersecting", Some(Type::GET_INTERSECTING), Some(args))
                }
            

                /// Return a list of documents closest to a specified point based on a geospatial index, sorted in order of increasing distance
///
/// 
/// The `index` argument is mandatory. Optional arguments are:
/// 
/// * `maxResults`: the maximum number of results to return (default 100).
/// * `unit`: Unit for the distance. Possible values are `m` (meter, the default), `km` (kilometer), `mi` (international mile), `nm` (nautical mile), `ft` (international foot).
/// * `maxDist`: the maximum distance from an object to the specified point (default 100 km).
/// * `geoSystem`: the reference ellipsoid to use for geographic coordinates. Possible values are `WGS84` (the default), a common standard for Earth's geometry, or `unit_sphere`, a perfect sphere of 1 meter radius.
/// 
/// The return value will be an array of two-item objects with the keys `dist` and `doc`, set to the distance between the specified point and the document (in the units specified with `unit`, defaulting to meters) and the document itself, respectively. The array will be sorted by the values of `dist`.
/// 
/// __Example:__ Return a list of the closest 25 enemy hideouts to the secret base.
/// 
/// ```javascript
/// var secretBase = r.point(-122.422876,37.777128);
/// r.table('hideouts').getNearest(secretBase,
///     {index: 'location', maxResults: 25}
/// ).run(conn, callback)
/// ```
/// 
/// <!-- stop -->
/// 
/// {% infobox %}
/// If you wish to find all points within a certain radius of another point, it's often faster to use [getIntersecting][gi] with [circle][c], as long as the approximation of a circle that `circle` generates is sufficient.
/// 
/// [gi]: /api/javascript/get_intersecting/
/// [c]:  /api/javascript/circle/
/// {% endinfobox %}

                pub fn get_nearest<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "get_nearest", Some(Type::GET_NEAREST), Some(args))
                }
            

                /// Tests whether a geometry object is completely contained within another
///
/// When applied to a sequence of geometry objects, `includes` acts as a [filter](/api/javascript/filter), returning a sequence of objects from the sequence that include the argument.
///
/// 
/// 
/// __Example:__ Is `point2` included within a 2000-meter circle around `point1`?
/// 
/// ```javascript
/// var point1 = r.point(-117.220406,32.719464);
/// var point2 = r.point(-117.206201,32.725186);
/// r.circle(point1, 2000).includes(point2).run(conn, callback);
/// // result returned to callback 
/// true
/// ```
/// 
/// __Example:__ Which of the locations in a list of parks include `circle1`?
/// 
/// ```javascript
/// var circle1 = r.circle([-117.220406,32.719464], 10, {unit: 'mi'});
/// r.table('parks')('area').includes(circle1).run(conn, callback);
/// ```
/// 
/// {% infobox %}
/// The `includes` command cannot take advantage of a geospatial [secondary index](/docs/secondary-indexes/javascript). If you're working with large data sets, consider using an index and [getIntersecting](/api/javascript/get_intersecting) before `includes` to narrow down the initial result set.
/// {% endinfobox %}
/// 
/// __Example:__ Rewrite the previous example with `getIntersecting`.
/// 
/// ```javascript
/// var circle1 = r.circle([-117.220406,32.719464], 10, {unit: 'mi'});
/// r.table('parks').getIntersecting(circle1, {index: 'area'})('area').
///     includes(circle1).run(conn, callback);
/// ```

                pub fn includes<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "includes", Some(Type::INCLUDES), Some(args))
                }
            

                /// Tests whether two geometry objects intersect with one another
///
/// When applied to a sequence of geometry objects, `intersects` acts as a [filter](/api/javascript/filter), returning a sequence of objects from the sequence that intersect with the argument.
///
/// 
/// __Example:__ Is `point2` within a 2000-meter circle around `point1`?
/// 
/// ```javascript
/// var point1 = r.point(-117.220406,32.719464);
/// var point2 = r.point(-117.206201,32.725186);
/// r.circle(point1, 2000).intersects(point2).run(conn, callback);
/// // result returned to callback 
/// true
/// ```
/// 
/// __Example:__ Which of the locations in a list of parks intersect `circle1`?
/// 
/// ```javascript
/// var circle1 = r.circle([-117.220406,32.719464], 10, {unit: 'mi'});
/// r.table('parks')('area').intersects(circle1).run(conn, callback);
/// ```
/// 
/// {% infobox %}
/// The `intersects` command cannot take advantage of a geospatial [secondary index](/docs/secondary-indexes/javascript). If you're working with large data sets, you should consider using an index and the [getIntersecting](/api/javascript/get_intersecting) command instead of `intersects`.
/// {% endinfobox %}

                pub fn intersects<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "intersects", Some(Type::INTERSECTS), Some(args))
                }
            

                /// Construct a geometry object of type Line
///
/// The line can be specified in one of two ways:
///
/// 
/// * Two or more two-item arrays, specifying latitude and longitude numbers of the line's vertices;
/// * Two or more [Point](/api/javascript/point) objects specifying the line's vertices.
/// 
/// <!-- break -->
/// 
/// Longitude (&minus;180 to 180) and latitude (&minus;90 to 90) of vertices are plotted on a perfect sphere. See [Geospatial support](/docs/geo-support/) for more information on ReQL's coordinate system.
/// 
/// __Example:__ Define a line.
/// 
/// ```javascript
/// r.table('geo').insert({
///     id: 101,
///     route: r.line([-122.423246,37.779388], [-121.886420,37.329898])
/// }).run(conn, callback);
/// ```
/// 
/// __Example:__ Define a line using an array of points.
/// 
/// You can use the [args](/api/javascript/args) command to pass an array of Point objects (or latitude-longitude pairs) to `line`.
/// 
/// ```javascript
/// var route = [
///     [-122.423246,37.779388],
///     [-121.886420,37.329898]
/// ];
/// r.table('geo').insert({
///     id: 102,
///     route: r.line(r.args(route))
/// }).run(conn, callback);
/// ```

                pub fn line<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "line", Some(Type::LINE), Some(args))
                }
            

                /// Construct a geometry object of type Point
///
/// The point is specified by two floating point numbers, the longitude (&minus;180 to 180) and latitude (&minus;90 to 90) of the point on a perfect sphere. See [Geospatial support](/docs/geo-support/) for more information on ReQL's coordinate system.
///
/// 
/// __Example:__ Define a point.
/// 
/// ```javascript
/// r.table('geo').insert({
///     id: 1,
///     name: 'San Francisco',
///     location: r.point(-122.423246,37.779388)
/// }).run(conn, callback);
/// ```

                pub fn point<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "point", Some(Type::POINT), Some(args))
                }
            

                /// Construct a geometry object of type Polygon
///
/// The Polygon can be specified in one of two ways:
///
/// 
/// * Three or more two-item arrays, specifying latitude and longitude numbers of the polygon's vertices;
/// * Three or more [Point](/api/javascript/point) objects specifying the polygon's vertices.
/// 
/// <!-- break -->
/// 
/// Longitude (&minus;180 to 180) and latitude (&minus;90 to 90) of vertices are plotted on a perfect sphere. See [Geospatial support](/docs/geo-support/) for more information on ReQL's coordinate system.
/// 
/// If the last point does not specify the same coordinates as the first point, `polygon` will close the polygon by connecting them. You cannot directly construct a polygon with holes in it using `polygon`, but you can use [polygonSub](/api/javascript/polygon_sub) to use a second polygon within the interior of the first to define a hole.
/// 
/// 
/// __Example:__ Define a polygon.
/// 
/// ```javascript
/// r.table('geo').insert({
///     id: 101,
///     rectangle: r.polygon(
///         [-122.423246,37.779388],
///         [-122.423246,37.329898],
///         [-121.886420,37.329898],
///         [-121.886420,37.779388]
///     )
/// }).run(conn, callback);
/// ```
/// 
/// __Example:__ Define a polygon using an array of vertices.
/// 
/// You can use the [args](/api/javascript/args) command to pass an array of Point objects (or latitude-longitude pairs) to `polygon`.
/// 
/// ```javascript
/// var vertices = [
///     [-122.423246,37.779388],
///     [-122.423246,37.329898],
///     [-121.886420,37.329898],
///     [-121.886420,37.779388]
/// ];
/// r.table('geo').insert({
///     id: 102,
///     rectangle: r.polygon(r.args(vertices))
/// }).run(conn, callback);
/// ```

                pub fn polygon<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "polygon", Some(Type::POLYGON), Some(args))
                }
            

                /// Use `polygon2` to "punch out" a hole in `polygon1`
///
/// `polygon2` must be completely contained within `polygon1` and must have no holes itself (it must not be the output of `polygonSub` itself).
///
/// 
/// 
/// __Example:__ Define a polygon with a hole punched in it.
/// 
/// ```javascript
/// var outerPolygon = r.polygon(
///     [-122.4,37.7],
///     [-122.4,37.3],
///     [-121.8,37.3],
///     [-121.8,37.7]
/// );
/// var innerPolygon = r.polygon(
///     [-122.3,37.4],
///     [-122.3,37.6],
///     [-122.0,37.6],
///     [-122.0,37.4]
/// );
/// outerPolygon.polygonSub(innerPolygon).run(conn, callback);
/// ```

                pub fn polygon_sub<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "polygon_sub", Some(Type::POLYGON_SUB), Some(args))
                }
            

                /// Grant or deny access permissions for a user account, globally or on a per-database or per-table basis
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
/// [http]: /api/javascript/http
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
/// }
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
/// ```javascript
/// r.db('users').grant('chatapp', {read: true, write: true}).run(conn, callback);
/// 
/// // Result passed to callback
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
/// ```javascript
/// r.db('users').table('admin').grant('chatapp', {write: false}).run(conn, callback);
/// ```
/// 
/// This will override the `write: true` permissions granted in the first example, but for this table only. Other tables in the `users` database will inherit from the database permissions.
/// 
/// __Example:__ Delete a table-level permission for the `chatapp` account.
/// 
/// ```javascript
/// r.db('users').table('admin').grant('chatapp', {write: null}).run(conn, callback);
/// ```
/// 
/// By specifying `null`, the table scope `write` permission is removed, and will again inherit from the next highest scope (database or global).
/// 
/// __Example:__ Grant `chatapp` the ability to use HTTP connections.
/// 
/// ```javascript
/// r.grant('chatapp', {connect: true}).run(conn, callback);
/// ```
/// 
/// This grant can only be given on a global level.
/// 
/// 
/// __Example:__ Grant a `monitor` account read-only access to all databases.
/// 
/// ```javascript
/// r.grant('monitor', {read: true}).run(conn, callback);
/// ```

                pub fn grant(&self) -> Client {
                    util::make_cmd::<Client>(self, "grant", Some(Type::GRANT), None)
                }
            

                /// Query (read and/or update) the configurations for individual tables or databases
///
/// 
/// The `config` command is a shorthand way to access the `table_config` or `db_config` [System tables](/docs/system-tables/#configuration-tables). It will return the single row from the system that corresponds to the database or table configuration, as if [get](/api/javascript/get) had been called on the system table with the UUID of the database or table in question.
/// 
/// __Example:__ Get the configuration for the `users` table.
/// 
/// ```javascript
/// > r.table('users').config().run(conn, callback);
/// ```
/// 
/// <!-- stop -->
/// 
/// Example return:
/// 
/// ```javascript
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
/// ```javascript
/// > r.table('users').config().update({write_acks: 'single'}).run(conn, callback);
/// ```

                pub fn config(&self) -> Client {
                    util::make_cmd::<Client>(self, "config", Some(Type::CONFIG), None)
                }
            

                /// Rebalances the shards of a table
///
/// When called on a database, all the tables in that database will be rebalanced.
///
/// 
/// The `rebalance` command operates by measuring the distribution of primary keys within a table and picking split points that will give each shard approximately the same number of documents. It won't change the number of shards within a table, or change any other configuration aspect for the table or the database.
/// 
/// A table will lose availability temporarily after `rebalance` is called; use the [wait](/api/javascript/wait) command to wait for the table to become available again, or [status](/api/javascript/status) to check if the table is available for writing.
/// 
/// RethinkDB automatically rebalances tables when the number of shards are increased, and as long as your documents have evenly distributed primary keys&mdash;such as the default UUIDs&mdash;it is rarely necessary to call `rebalance` manually. Cases where `rebalance` may need to be called include:
/// 
/// * Tables with unevenly distributed primary keys, such as incrementing integers
/// * Changing a table's primary key type
/// * Increasing the number of shards on an empty table, then using non-UUID primary keys in that table
/// 
/// The [web UI](/docs/administration-tools/) (and the [info](/api/javascript/info) command) can be used to tell you when a table's shards need to be rebalanced.
/// 
/// The return value of `rebalance` is an object with two fields:
/// 
/// * `rebalanced`: the number of tables rebalanced.
/// * `status_changes`: a list of new and old table status values. Each element of the list will be an object with two fields:
///     * `old_val`: The table's [status](/api/javascript/status) value before `rebalance` was executed. 
///     * `new_val`: The table's `status` value after `rebalance` was executed. (This value will almost always indicate the table is unavailable.)
/// 
/// See the [status](/api/javascript/status) command for an explanation of the objects returned in the `old_val` and `new_val` fields.
/// 
/// __Example:__ Rebalance a table.
/// 
/// ```javascript
/// > r.table('superheroes').rebalance().run(conn, callback);
/// ```
/// 
/// <!-- stop -->
/// 
/// Example return:
/// 
/// ```javascript
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
                    util::make_cmd::<Client>(self, "rebalance", Some(Type::REBALANCE), None)
                }
            

                /// Reconfigure a table's sharding and replication
///
/// 
/// * `shards`: the number of shards, an integer from 1-64. Required.
/// * `replicas`: either an integer or a mapping object. Required.
///     * If `replicas` is an integer, it specifies the number of replicas per shard. Specifying more replicas than there are servers will return an error.
///     * If `replicas` is an object, it specifies key-value pairs of server tags and the number of replicas to assign to those servers: `{tag1: 2, tag2: 4, tag3: 2, ...}`. For more information about server tags, read [Administration tools](/docs/administration-tools/).
/// * `primaryReplicaTag`: the primary server specified by its server tag. Required if `replicas` is an object; the tag must be in the object. This must *not* be specified if `replicas` is an integer.
/// * `dryRun`: if `true` the generated configuration will not be applied to the table, only returned.
/// * `nonvotingReplicaTags`: replicas with these server tags will be added to the `nonvoting_replicas` list of the resulting configuration. (See [failover](/docs/failover) for details about non-voting replicas.)
/// 
/// * `emergencyRepair`: Used for the Emergency Repair mode. See the separate section below.
/// 
/// The return value of `reconfigure` is an object with three fields:
/// 
/// * `reconfigured`: the number of tables reconfigured. This will be `0` if `dryRun` is `true`.
/// * `config_changes`: a list of new and old table configuration values. Each element of the list will be an object with two fields:
///     * `old_val`: The table's [config](/api/javascript/config) value before `reconfigure` was executed. 
///     * `new_val`: The table's `config` value after `reconfigure` was executed.
/// * `status_changes`: a list of new and old table status values. Each element of the list will be an object with two fields:
///     * `old_val`: The table's [status](/api/javascript/status) value before `reconfigure` was executed.
///     * `new_val`: The table's `status` value after `reconfigure` was executed.
/// 
/// For `config_changes` and `status_changes`, see the [config](/api/javascript/config) and [status](/api/javascript/status) commands for an explanation of the objects returned in the `old_val` and `new_val` fields.
/// 
/// A table will lose availability temporarily after `reconfigure` is called; use the [wait](/api/javascript/wait) command to wait for the table to become available again, or [status](/api/javascript/status) to check if the table is available for writing.
/// 
/// **Note:** Whenever you call `reconfigure`, the write durability will be set to `hard` and the write acknowledgments will be set to `majority`; these can be changed by using the `config` command on the table.
/// 
/// If `reconfigure` is called on a database, all the tables in the database will have their configurations affected. The return value will be an array of the objects described above, one per table.
/// 
/// Read [Sharding and replication](/docs/sharding-and-replication/) for a complete discussion of the subject, including advanced topics.
/// 
/// __Example:__ Reconfigure a table.
/// 
/// ```javascript
/// > r.table('superheroes').reconfigure({shards: 2, replicas: 1}).run(conn, callback);
/// ```
/// 
/// <!-- stop -->
/// 
/// Example return:
/// 
/// ```javascript
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
/// ```javascript
/// > r.table('superheroes').reconfigure({shards: 2, replicas: {wooster: 1, wayne: 1}, primaryReplicaTag: 'wooster'}).run(conn, callback);
/// // Result passed to callback
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
/// The `emergencyRepair` argument is effectively a different command; when it is specified, no other arguments to `reconfigure` are allowed except for `dryRun`. When it's executed, each shard of the table is examined and classified into one of three categories:
/// 
/// * **Healthy:** more than half of the shard's voting replicas are still available.
/// * **Repairable:** the shard is not healthy, but has at least one replica, whether voting or non-voting, available.
/// * **Beyond repair:** the shard has no replicas available.
/// 
/// For each repairable shard, `emergencyRepair` will convert all unavailable voting replicas into non-voting replicas. If all the voting replicas were removed, an arbitrarily-chosen available non-voting replica will be converted into a voting replica. After this operation, all of the shard's available replicas will be voting replicas.
/// 
/// Specify `emergencyRepair` with one of two string options:
/// 
/// * `unsafe_rollback`: shards that are beyond repair will be left alone.
/// * `unsafe_rollback_or_erase`: a shard that is beyond repair will be destroyed and recreated on an available server that holds another shard for that table.
/// 
/// The return value of `reconfigure` in emergency repair mode is the same as before. Examine the `config_changes` field to see the old and new configuration settings for the table. As in the normal mode, if you specify `emergencyRepair` with `dryRun: true`, the table will not actually be reconfigured.
/// 
/// __Note:__ `emergencyRepair` may only be used on individual tables, not on databases. It cannot be used after the `db` command.
/// 
/// {% infobox alert %}
/// **The emergency repair mode is extremely dangerous.** It bypasses normal safeguards that prevent data loss and invalidates the [consistency guarantees](/docs/consistency/) that RethinkDB normally provides, and can easily lose data in either mode&mdash;in `unsafe_rollback_or_erase` mode it could lose *all* of a shard's data.
/// {% endinfobox %}
/// 
/// __Example:__ Perform an emergency repair on a table.
/// 
/// ```javascript
/// r.table('superheroes').reconfigure(
///   {emergencyRepair: "unsafe_rollback"}
/// ).run(conn, callback);
/// ```

                pub fn reconfigure<T: IntoArg>(&self, args: T) -> Client {
                    util::make_cmd(self, "reconfigure", Some(Type::RECONFIGURE), Some(args))
                }
            

                /// Return the status of a table
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
/// ```javascript
/// > r.table('superheroes').status().run(conn, callback);
/// ```
/// 
/// <!-- stop -->
/// 
/// Example return:
/// 
/// ```javascript
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
                    util::make_cmd::<Client>(self, "status", Some(Type::STATUS), None)
                }
            

                /// Wait for a table or all the tables in a database to be ready
///
/// A table may be temporarily unavailable after creation, rebalancing or reconfiguring. The `wait` command blocks until the given table (or database) is fully up to date.
///
/// 
/// The `wait` command takes two optional arguments:
/// 
/// * `waitFor`: a string indicating a table [status](/api/javascript/status) to wait on before returning, one of `ready_for_outdated_reads`, `ready_for_reads`, `ready_for_writes`, or `all_replicas_ready`. The default is `all_replicas_ready`.
/// * `timeout`: a number indicating maximum time, in seconds, to wait for the table to be ready. If this value is exceeded, a `ReqlRuntimeError` will be thrown. A value of`0` means no timeout. The default is `0` (no timeout).
/// 
/// The return value is an object consisting of a single field, `ready`. The value is an integer indicating the number of tables waited for. It will always be `1` when `wait` is called on a table, and the total number of tables when called on a database.
/// 
/// {% infobox %}
/// Versions of RethinkDB prior to 2.3 allowed `wait` to be called without a table or database specified. This is no longer valid; `wait` requires explicit selection of a database or table.
/// {% endinfobox %}
/// 
/// __Example:__ Wait on a table to be ready.
/// 
/// ```javascript
/// > r.table('superheroes').wait().run(conn, callback);
/// // Result passed to callback
/// { "ready": 1 }
/// ```

                pub fn wait(&self) -> Client {
                    util::make_cmd::<Client>(self, "wait", Some(Type::WAIT), None)
                }
            
            }
        