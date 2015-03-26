---
layout: documentation
title: Troubleshooting common RethinkDB problems
docs_active: troubleshooting 
permalink: docs/troubleshooting/
js: faq_index
---

## How can I get a dump of the RethinkDB system tables?

This can be useful for diagnostic purposes, as well as for filing bug reports. The easiest way to do this is with ReQL administration commands. Any individual table can be examined with `r.db('rethinkdb').table(<tablename>)`.

The following command will output the contents of *all* the configuration/status tables as well as the most recent 50 lines of the `logs` table:

```js
r.expr(["current_issues", "jobs", "stats", "server_config", "server_status",
"table_config", "table_status", "db_config", "cluster_config"]).map(
    [r.row, r.db('rethinkdb').table(r.row).coerceTo('array')]
).coerceTo('object').merge(
    {logs: r.db('rethinkdb').table('logs').limit(50).coerceTo('array')}
)
```

(That command is suitable for running in the Data Explorer, but can be easily adapted into other languages.)

## I get a "RqlRuntimeError: Array over size limit 100000" when trying to order a table

Ordering without an index requires the server to load the whole sequence in an array, which is limited by default to 100,000 documents. You can use the `arrayLimit` option to [run](/api/javascript/run/) to temporarily raise this limit. However, a more efficient option is to use an index. See the documentation for [orderBy](/api/javascript/order_by/) for more information.

## My insert queries are slow. How can I speed them up? ##

RethinkDB uses a safe default configuration for write
acknowledgement. Each write is committed to disk before the server
acknowledges it to the client. If you're running a single thread that
inserts documents into RethinkDB in a loop, each insert must wait for
the server acknowledgement before proceeding to the next one. This can
significantly slow down the overall throughput.

This behavior is similar to any other safe database system. Below is a
number of steps you can take to speed up insert performance in
RethinkDB. Most of these guidelines will also apply to other database
systems.

- __Increase concurrency.__ Instead of having a single thread
  inserting data in a loop, create multiple threads with multiple
  connections. This will allow parallelization of insert queries
  without spending most of the time waiting on disk acknowledgement.

- __Batch writes.__ Instead of doing single writes in a loop, group
  writes together. This can result in significant increases in
  throughput. Instead of doing multiple queries like this:

   ```python
   r.db("foo").table("bar").insert(document_1).run()
   r.db("foo").table("bar").insert(document_2).run()
   r.db("foo").table("bar").insert(document_3).run()
   ```
   Combine them into a single query:

   ```python
   r.db("foo").table("bar").insert([document_1, document_2, document_3]).run()
   ```

   RethinkDB operates at peak performance when the batch size is
   around two hundred documents.

- __Consider using soft durability mode.__ In soft durability mode
  RethinkDB will acknowledge the write immediately after receiving it,
  but before the write has been committed to disk. The server will use
  main memory to absorb the write, and will flush new data to disk in
  the background.

  This mode is __not as safe__ as the default hard durability mode. If
  you're writing using soft durability, a few seconds worth of data
  might be lost in case of power failure.

  {% infobox %}
  __Note:__ while some data may be lost in case of power failure in soft
  durability mode, the RethinkDB database will not get corrupted.
  {% endinfobox %}

  You can insert data in soft durability mode as follows:

  ```python
  r.db("foo").table("bar").insert(document).run(durability="soft")
  ```

- __Consider using `noreply` mode.__ In this mode, the client driver
  will not wait for the server acknowledgement of the query before
  moving on to the next query. This mode is even less safe than the
  soft durability mode, but can result in the highest performance
  improvement. You can run a command in a `noreply` mode as follows:

  ```python
  r.db("foo").table("bar").insert(document).run(noreply=True)
  ```

  You can also combine soft durability and `noreply` for the highest
  performance:

  ```python
  r.db("foo").table("bar").insert(document).run(durability="soft", noreply=True)
  ```

## How can I order the output of `group`? ##

Commands chained after `group` operate on each group separately.  If
you want to operate on all the groups at once (e.g. to order them),
you need to call [**ungroup**](/api/python/ungroup/) before doing so.

## What does 'received invalid clustering header' mean? ##

{% include docs/troubleshootingcluster.md %}

## Does the web UI support my browser? ##

The following browsers are supported and known to work with the web
UI:

- Chrome 9 or higher
- Firefox 15 or higher
- Safari 6.02 or higher
- Opera 1.62 or higher

{% infobox %}
The web UI requires `DataView` and `Uint8Array` JavaScript features to
be supported by your browser.
{% endinfobox %}

## Which versions of Node.js are supported? ##

The JavaScript driver currently works with Node.js versions 0.10.0 and
above. You can check your node version as follows:

```
node --version
```

You can upgrade your version of Node.js via `npm`:

```
sudo npm install -g n
```

If you're trying to run the RethinkDB JavaScript driver on an older
version of Node.js, you might get an error similar to this one:

```js
/home/user/rethinkdb.js:13727
return buffer.slice(offset, end);
             ^
TypeError: Object #<ArrayBuffer> has no method 'slice'
at bufferSlice (/home/user/rethinkdb.js:13727:17)
at Socket.TcpConnection.rawSocket.once.handshake_callback (/home/user/rethinkdb.js:13552:26)
```

## I get back a connection in my callback with the Node driver ##

Many people have been reporting that they get back a connection object when they
run a query, the object being:

```js
{
    _conn: {
        host: 'localhost',
        port: 28015,
        db: undefined,
        authKey: '',
        timeout: 20,
        outstandingCallbacks: {},
        nextToken: 2,
        open: true,
        buffer: <Buffer 04 00 00 00 08 02 10 01>,
        _events: {},
        rawSocket: { ... }
    },
    _token: 1,
    _chunks: [],
    _endFlag: true,
    _contFlag: true,
    _cont: null,
    _cbQueue: []
}
```

This object is not a connection, but a cursor. To retrieve the results, you can
call `next`, `each` or `toArray` on this object.

For example you can retrieve all the results and put them in an array with
`toArray`:

```js
r.table("test").run( conn, function(error, cursor) {
    cursor.toArray( function(error, results) {
        console.log(results) // results is an array of documents
    })
})
```
## RethinkDB is running out of memory ##

You may need to adjust RethinkDB's page cache size, using the `--cache-size` argument or configuration file option. Read "[Understanding RethinkDB memory requirements](/docs/memory-usage/)" for a more detailed explanation of how RethinkDB uses memory and how to tune its performance.

## I get incorrect results when I pass functions with if/for statements to ReQL ##

When you pass functions to ReQL, your language's driver serializes those functions into ReQL lambda functions that are run on the server, not in your client language. (See [All about lambda functions in RethinkDB queries](/blog/lambda-functions/) for more details.) A consequence of this is that native language constructs like `if` and `for` will not produce the expected result when their conditions involve ReQL commands. While they may not cause errors, they will be executed on the client side before the function is compiled for ReQL, and thus give an incorrect result. Instead, you must use equivalent ReQL control functions such as [branch](/api/javascript/branch/) and [forEach](/api/javascript/for_each/). Here's an example in Python from the [Introduction to ReQL](/docs/introduction-to-reql/) document:

```py
# WRONG: Get all users older than 30 using the `if` statement
r.table('users').filter(lambda user:
    True if user['age'] > 30 else False
).run(conn)

# RIGHT: Get all users older than 30 using the `r.branch` command
r.table('users').filter(lambda user:
    r.branch(user['age'] > 30, True, False)
).run(conn)
```

And an equivalent example in JavaScript:

```js
// WRONG: Get all users older than 30 using the ternary operator
r.table('users').filter(function(user) {
    return (r.row('age').gt(30) ? true : false);
}).run(conn, callback)

// RIGHT: Get all users older than 30 using the `r.branch` command
r.table('users').filter(function(user) {
    r.branch(user('age').gt(30), true, false)
}).run(conn, callback)
```

(Note we must use `gt` instead of the native `>` operator in JavaScript, for the same reason. In Python the `>` operator is [overloaded](https://docs.python.org/2/reference/datamodel.html#special-method-names) to be translated to ReQL's `gt` command, a trick that is not possible in JavaScript.)

## How do I specify an external canonical IP address of a RethinkDB node? ##

When a RethinkDB node starts, it will broadcast its "canonical" IP address, the address other nodes should use to connect to it. By default, the canonical address is the server's primary IP address. However, if this address is an internal IP address that isn't reachable by other nodes (for example, the nodes are on different networks), the nodes will not be able to reach one another. You may receive an error message such as:

```
error: received inconsistent routing information (wrong address) from xxx.xxx.xxx.xxx (expected_address = peer_address{ips=[xxx.xxx.xxx.xxx], port=29015}, other_address = peer_address{ips=[xxx.xxx.xxx.xxx], port=29015}), closing connection
```

To solve this, specify the canonical address explicitly by using the `--canonical-address` argument.

```
rethinkdb --canonical-address <external IP>
```

This may also be specified in the [config file](http://rethinkdb.com/docs/cluster-on-startup/).

## My secondary index is outdated ##

You may receive a warning message about secondary indexes on startup being "outdated" when you upgrade RethinkDB versions.

```
warn: Namespace <x> contains these outdated indexes which should be recreated:
<index names>
```

(This may happen, for instance, between v1.13 and v1.14, when the internal format of secondary indexes changed.) Outdated indexes can still be used&mdash;they don't affect availability. However, you should rebuild your index before updating to the next version of RethinkDB.

You may rebuild indexes with the `rethinkdb` command line utility:

```
rethinkdb index-rebuild [-c HOST:PORT] [-r (DB|DB.TABLE)] [-n CONCURRENT_REBUILDS]
```

The `-c` and `-r` options are similar to other `rethinkdb` options, specifying the cluster host and port (defaulting to `localhost:28015`) and either a database or a table to rebuild. The `-n` option specifies the number of rebuilds that will be performed concurrently (defaulting to 1).

You may also rebuild indexes manually in ReQL:

* Use [index_status](/api/python/index_status/) to retrieve a binary representation of the existing secondary index (whether it is simple, compound, multi, or based on an expression)
* Create a new index using [index_create](/api/python/index_create/)
* Rename the new index to the old index's name with [index_rename](/api/python/index_rename).

A simple example in Python:

```py
old_index = r.table('posts').index_status('old_index').nth(0)['function'].run(conn)
r.table('posts').index_create('new_index', old_index).run(conn)
r.table('posts').index_wait('new_index').run(conn)
r.table('posts').index_rename('new_index', 'old_index', overwrite=True).run(conn)
```

(The same example can be found in `index_create` for both [Ruby](/api/ruby/index_create) and [JavaScript](/api/javascript/index_create).)

## How do I store a Ruby DateTime object in RethinkDB? ##

The short answer: you can't. Use `Time` objects instead.

The slightly longer answer: there's only one native `time` data type in RethinkDB. When a language supports more than one kind of date/time object, we think it's better to explicitly support one and only one of them in the client driver to avoid confusion. Otherwise, you might insert a `DateTime` object and get a `Time` object back.

You can use Ruby's `DateTime.to_time` and `Time.to_datetime` methods to easily convert between one and the other.

## Filters with `or` return incorrect/unexpected results ##

You might want to use `filter` to return documents that have one of two (or more) optional fields set, such as the following:

```js
r.table('posts').filter(
    r.row('category').eq('article').or(r.row('genre').eq('mystery'))
).run(conn, callback);
```

However, if any document in the `posts` table above lacks a `category` field, it won't be included in the result set even if it has a `genre` field whose value is `'mystery'`. The problem isn't the `or` command; it's that the invocation of `r.row('category')` on a document without that field returns an error, and the rest of the filter predicate isn't evaluated.

The solution is to add a `default` to the `row` command that always evaluates to something other than what you're testing for, so it will return `false` if the field doesn't exist:

```js
r.table('posts').filter(
    r.row('category').default('foo').eq('article').
    or(r.row('genre').default('foo').eq('mystery'))
).run(conn, callback);
```
