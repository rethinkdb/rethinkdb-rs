---
layout: documentation
title: Writing RethinkDB drivers
permalink: docs/writing-drivers/
alias: docs/driver-spec/
active: docs
docs_active: writing-drivers
---


RethinkDB client drivers are responsible for serializing queries, sending them to the server using the ReQL wire protocol, and receiving responses from the server and returning them to the calling application. This process takes the following steps:

* Open a connection
* Perform a handshake
* Serialize the query
* Send the message
* Receive responses

{% infobox %}
For updates on protocol and behavior changes in new versions of RethinkDB and general assistance with writing drivers, join the [RethinkDB-Dev Google Group](https://groups.google.com/forum/?fromgroups#!forum/rethinkdb-dev).
{% endinfobox %}

# Initial steps #

ReQL types and commands are defined in the [ql2.proto][ql2] file.

[ql2]: https://github.com/rethinkdb/rethinkdb/blob/next/src/rdb_protocol/ql2.proto

For a JavaScript version of the file, run `make js-driver` in the `rethinkdb` repo, and retrieve the JSON version of the file in `build/packages/js/proto-def.js`. Alternatively you may grab the equivalent file from [rethinkdbdash][dash].

[dash]: https://github.com/neumino/rethinkdbdash/blob/master/lib/protodef.js

The `ql2.proto` file is well-commented, showing arguments and output for each command.

# Open a connection #

Open a TCP connection to the server on the driver port. The default port is `28015`.

# Perform a handshake #

1. Send the protocol version, as a 32-bit little-endian integer (4 bytes). _Note:_ All instructions below assume a protocol of `V0_3` or higher. The current protocol as of RethinkDB 2.0 is `V0_4`.
2. Send the length of the authorization key, as a 32-bit little-endian integer (4 bytes). Send `0` if there is no authorization key.
3. Send the authorization key as an ASCII string. _If there is no authorization key, skip this step._
4. Send the protocol type, as a 32-bit little-endian integer (4 bytes). Protocol types are defined in the `Protocol` enum in `ql2.proto`. New drivers should use JSON, `0x7e6970c7`.

The server will respond with a `null`-terminated ASCII string describing the result of the handshake. If the string is `"SUCCESS"`, the client may proceed to stage 2 and begin sending queries. Any other string indicates an error. The server will close the connection, and the driver should report this error to the user.

## Example 1: No auth key ##

| Step | Direction | Element | Bytes |  
| --- | --- | --- | --- |  
| 1 | SEND | `V0_4` | `20 2d 0c 40` |  
| 2 | SEND | key size | `00 00 00 00` |  
| 3 | SEND | auth key | |   
| 4 | SEND | JSON | `c7 70 69 7e` |  
| 5 | RECV | success | `53 55 43 43 45 53 53` |  

## Example 2: Auth key ##

| Step | Direction | Element | Bytes |  
| --- | --- | --- | --- |  
| 1 | SEND | `V0_4` | `20 2d 0c 40` |  
| 2 | SEND | key size | `07 00 00 00` |  
| 3 | SEND | auth key | `68 75 6e 74 65 72 32` |  
| 4 | SEND | JSON | `c7 70 69 7e` |  
| 5 | RECV | success | `53 55 43 43 45 53 53` |  

# Serializing queries #

Your driver should assign each query a unique 8-byte token per connection. (The official RethinkDB drivers implement this as an unsigned 8-byte little-endian counter per connection.) The server will send responses to queries using this token as an identifier so the response can be matched to its query. The token can also be used to request more data for the query if all the results were not returned in the first response.

## A simple example ##

The next section will explain how to build complex queries. For now, we will just send the string `"foo"` (`r.expr("foo")`) to the server.

Sending a query to the server takes the following steps:

* Serialize the query as UTF8-encoded JSON
* Send the following data to the server:
    * The 8-byte unique query token
    * The size of the JSON-serialized, UTF8-encoded query, as a 4-byte little-endian integer
    * The wrapped query message (QueryType, serialized query and options)

The wrapped query message sent to the server is an array of three elements:

    [ QueryType, query, options ]

The next section will go into more detail, but in our example the `QueryType` is `1` (or `START`, as we'll see later), the `query` is simply the string `"foo"` and there are no options.


    [ 1, "foo", {} ]

So, the data we send to the server is as follows:

| Step | Element |  Transmitted bytes |  
| --- | --- |  --- |  
| 1 | query token |  `00 00 00 00 00 00 00 01` |  
| 2 | length |  `0c 00 00 00` |  
| 3 | query |  `[1,"foo",{}]` |  

Once the query is sent, you can read the response object back from the server. The response object takes the following form:

* The 8-byte unique query token
* The length of the response, as a 4-byte little-endian integer
* The JSON-encoded response

| Step | Element | Bytes on wire |  
| --- | --- | --- |  
| 1 | query token | `00 00 00 00 00 00 00 01` |  
| 2 | length | `13 00 00 00` |  
| 3 | response | `{"t":1,"r":["foo"]}` |  

When you parse the response string as JSON, you get the object:

```js
{
    t: 1,         // protodef.Response.ResponseType.SUCCESS_ATOM
    r: ["foo"]    // the response is the string 'foo"
}
```

Where `t:1` means that the response is a value, and `r: ["foo"]` the string `"foo"`.

## Queries in detail ##

ReQL is a [domain specific language][dsl] expressed in the host language. The three official drivers follow a very similar syntax; you should stick to that model as closely as your chosen language allows. Typically you are free to use either a prefix or infix notation, or mix the two.

[dsl]: http://en.wikipedia.org/wiki/Domain-specific_language

Internally, queries are represented as trees. A query of:

```js
r.db("blog").table("users").filter({name: "Michel"})
```

is represented by this tree:

<img alt="Query tree illustration" src="/assets/images/docs/query_tree.png" />

### ReQL commands ###

ReQL commands are represented as a list of two or three elements.

    [<command>, [<arguments>], {<options>}]

* `<command>` is the integer representing the command, from `ql2.proto`
* `<arguments>` is a list of all arguments. Each argument is itself a query (a command list, or data).
* `<options>` are the command's optional arguments. This element may be left out if the command has no optional arguments given.

Thus, this is how our previous query is represented:

    r.db("blog").table("users").filter({name: "Michel"});
    
    FILTER = 39     // from ql2.proto
    TABLE = 15
    DB = 14
    
    r.db("blog") =>
        [14, ["blog"]]
    
    r.db("blog").table("users") =>
        [15, [[14, ["blog"]], "users"]]
    
    r.db("blog").table("users").filter({name: "Michel"}) =>
        [39, [[15, [[14, ["blog"]], "users"]], {"name": "Michel"}]]

### Implementation considerations ###

If you want to use a prefix notation, you just need to implement all the commands on a module. If you want to use an infix notation, you should implement all the functions on a class "term" and some prefix commands on the module.

You can only check arity of the methods to a certain extent. If an `ARGS` term is one of the argument, only the server can effectively verify that enough arguments are provided (or not too many). The arity errors reported by the server suppose a prefix notation. Things may change if the solution in [#2463][2463] is implemented.

[2463]: https://github.com/rethinkdb/rethinkdb/issues/2463#issuecomment-44584491

### ReQL data ###

A *datum* (the singular of data) is any value that can be represented in JSON: booleans, numbers, strings, objects, arrays and `null`. They are sent to the server in JSON form.

Arrays, however, are a special case: since ReQL commands (as described above) are sent as arrays, you must send data arrays as arguments to the `MAKE_ARRAY` command. So the array

    [10, 20, 30]

Would be sent to the server as

    // MAKE_ARRAY = 2 (from ql2.proto)
    
    [2, [10, 20, 30]]

### ReQL pseudo types ###

Some native ReQL data types have no direct JSON representations. These are implemented as *pseudo types,* JSON objects with the special key `$reql_type$`. The three official ReQL drivers convert date and binary types to pseudo types.

**Date pseudo type**

```js
{
    $reql_type: "TIME",
    epoch_time: <timestamp>,
    timezone: <string>
}
```

The `epoch_time` field is a Unix timestamp, the number of seconds since January 1st, 1970, with millisecond precision. The `timezone` field is a string in the format `[+-]HH:MM`, indicating the offset from UTC. UTC is `+00:00`; PST is `-08:00`; and so on.

**Binary pseudo type**

```js
{
    $reql_type$: "BINARY",
    data: <string>
}
```

The `data` field is a Base64-encoded string of the binary object.

### Anonymous functions ###

A [good article][lf] by [Bill Rowan][wmrowan] explains anonymous functions (or lambda functions) in the drivers. The article covers why anonymous functions are useful and how they work. Here, we'll just focus on how to serialize anonymous functions.

[lf]: http://www.rethinkdb.com/blog/lambda-functions/ "All about lambda functions in RethinkDB queries"
[wmrowan]: https://github.com/wmrowan

When the driver finds an anonymous function, it returns a query object like this one:

```js
// FUNC = 69, MAKE_ARRAY = 2 (from ql2.proto)

[69, [[2, [p1, p2, ...]], function body]]
```

The parameters are represented as values `<p1>`, `<p2>`, etc.; the values are arbitrary, but must be unique per query to avoid collisions. Within the function body the values are referred to with the query term `VAR`, defined as `10` in `ql2.proto`. So the value of parameter `1` is retrieved with `[10, [1]]`.

Take the function:

```
function(x, y, z) {
    return r.add(x, y, z)
}
```

The function would be serialized as:

    [FUNC, 
     [[MAKE_ARRAY, [1, 2, 3]],
       [ADD,
        [[VAR, [1]],
         [VAR, [2]],
         [VAR, [3]]]]]]

    // FUNC = 69, MAKE_ARRAY = 2, ADD = 24, VAR = 10 (from ql2.proto)

    [69, [[2, [1, 2, 3]], [24, [[10, [1]], [10, [2]], [10, [3]]]]]]

### Implementation details ###

Serializing functions depends heavily on your driver's language. The JavaScript driver does it this way:

* Look at how many arguments the function takes (`num_args`)
* Create that many `VAR` terms
* Call the function with those terms
* Serialize the result as the function body

If your driver uses infix notation, you must make sure that the `VAR` term implements all the ReQL methods.

### Serializing IMPLICIT_VAR (r.row) ###

The `IMPLICIT_VAR` term is equivalent to the [row](/api/python/row) command in the official JavaScript and Python drivers. It's useful for languages where anonymous functions are too verbose.

If you support `IMPLICIT_VAR` in your driver, then every time you parse the argument of a function you should check if the method can take a function. If it can, you should look for an `IMPLICIT_VAR` term (i.e., `row`). If you find one, wrap the argument in a function that takes one parameter:

    [69, [[2, [1]], argument]]

If you do not find one, treat the argument normally.

In the case of nested functions, the `IMPLICIT_VAR` term is ambiguous, and should not be used. Your driver should either throw an error or let the server return an error.

### Serializing BINARY ###

Binary objects created with `r.binary` can be serialized in two different ways.

If the argument is a ReQL term (not including a datum), serialize it using the standard term:

    [BINARY, argument]

If the language's native binary format is used, use the pseudotype serialization described above.

```js
{
    $reql_type$: "BINARY",
    data: <base64 string>
}
```

### Serializing FUNCALL (r.do) ###

The `r.do()` command is serialized with the `FUNCALL` term.

    [FUNCALL, [function], arguments]

Take the `do` command:

```js
r.do(10, 20, function (x, y) {
  return r.add(x, y);
})
```

This would be serialized as:

    [FUNCALL,
      [FUNC,
        [[MAKE_ARRAY, [1, 2]],
          [ADD,
            [[VAR, [1]],
             [VAR, [2]]]]]],
      10,
      20]
    
    // FUNCALL = 64, FUNC = 69, MAKE_ARRAY = 2, ADD = 24, VAR = 10
    
    [64, [69, [[2, [1, 2]], [24, [[10, [1]], [10, [2]]]]]], 10, 20]

Note that while `r.do()` takes the function as its *last* argument, `FUNCALL` serializes the function as its *first* argument.

# Send the message #

Because you can keep chaining commands (or calling them in prefix notation), you need a command to signify the end of the chain and send the query to the server. This command is `run` in the official drivers.

## Wrapping queries ##

Once the [run](/api/python/run) command is processed, the serialized query needs to be wrapped in the message sent to the server. The complete message takes the form:

    [ QueryType, query, options ]

The query types are defined in `ql2.proto`. When a query is first sent to the server, it will be sent with a `QueryType` of `START` (`1`). The options (sometimes referred to as "global optargs") are options passed to the `run` command itself; see the [run documentation](/api/python/run) for a complete list. (Commands sent to the server are snake_case, not camelCase.)

The full list of `QueryType` values is as follows:

* `1` `START`: Start a new query.
* `2` `CONTINUE`: Continue a query that returned `SUCCESS_PARTIAL` (see [Receive responses](#receive-responses)).
* `3` `STOP`: Stop a query that is still executing.
* `4` `NOREPLY_WAIT`: Wait for noreply operations to finish. The server will return a `WAIT_COMPLETE` response.
* `5` `SERVER_INFO`: Ask for server information. The server will return a `SERVER_INFO` response.

`CONTINUE` and `STOP` should be sent on the same connection with the same token generated for that query's `START` message.

## Sending queries ##

To recap, sending a query to the server takes the following steps:

* Serialize the query as UTF8-encoded JSON
* Send the following data to the server:
    * The 8-byte unique query token
    * The size of the JSON-serialized, UTF8-encoded wrapped query, as a 4-byte little-endian integer
    * The wrapped query message (QueryType, serialized query and options)

The token is a unique integer per connection. Keeping a counter per connection is a simple way to implement it.

So, our initial example query of:

```js
r.db("blog").table("users").filter({name: "Michel"})
```

is sent as follows on the wire:

| Step | Semantic command |  Transmitted |  
|  ------ | ------ |  ------ |  
| 1 | query token |  `00 00 00 00 00 00 00 01` |  
| 2 | length |  `3C 00 00 00` |  
| 3 | query | `[1,[39,[[15,[[14,["blog"]],"users"]],{"name":"Michel"}]],{}]` |  

## Wrapping the DB query option ##

If the `db` option is passed to the `run` command, its value must be a `DB` term. The query:

```js
r.table("users").run({db: "blog"});
```

should be sent as as if the argument to `db` was `r.db("blog")`:

    [1,[15,["users"]],{"db":[14,["blog"]]}]

# Receive responses #

Responses from the server take the following form:

* The 8-byte unique query token the response corresponds to
* The size of the JSON-encoded response, as a 4-byte little-endian integer
* The JSON-encoded `Response` object

The `Response` object will have the following fields:

* `t`: the `ResponseType`, as defined in `ql2.proto`
* `r`: data from the result, as a JSON array
* `b`: a backtrace if `t` is an error type; this field will not be present otherwise
* `p`: a profile if the global optarg `profile: true` was specified; this field will not be present otherwise
* `n`: an optional array of `ResponseNote` values, as defined in `ql2.proto`

## Response types ##

These will be numeric values, corresponding to the types in `ql2.proto`.

* `1` `SUCCESS_ATOM`: The whole query has been returned and the result is in the first (and only) element of `r`.
* `2` `SUCCESS_SEQUENCE`: Either the whole query has been returned in `r`, or the last section of a multi-response query has been returned.
* `3` `SUCCESS_PARTIAL`: The query has returned a stream, which may or may not be complete. To retrieve more results for the query, send a `CONTINUE` message (see below).
* `4` `WAIT_COMPLETE`: This `ResponseType` indicates all queries run in `noreply` mode have finished executing. `r` will be empty. 
* `5` `SERVER_INFO`: The response to a `SERVER_INFO` request. The data will be in the first (and only) element of `r`.
* `16` `CLIENT_ERROR`: The server failed to run the query due to a bad client request. The error message will be in the first element of `r`.
* `17` `COMPILE_ERROR`: The server failed to run the query due to an ReQL compilation error. The error message will be in the first element of `r`.
* `18` `RUNTIME_ERROR`: The query compiled correctly, but failed at runtime. The error message will be in the first element of `r`.

## Response notes ##

The `n` field, if present, will be an array of one or more `ResponseNote` values, giving further information about the kind of the stream being returned. These will be numeric values, corresponding to the notes in `ql2.proto`.

All of the response notes involve changefeeds; read [Changefeeds in RethinkDB](/docs/changefeeds/) for more detailed information.

* `1` `SEQUENCE_FEED`: The stream is a changefeed.
* `2` `ATOM_FEED`: The stream is a *point* changefeed, i.e., returning changes from a single document.
* `3` `ORDER_BY_LIMIT_FEED`: The stream is a changefeed generated with an `order_by().limit()` query.
* `4` `UNIONED_FEED`: The stream is a union of multiple changefeed types that cannot be collapsed to a single type, e.g., `r.table('test').changes().union(r.table('test').get(0).changes())`.
* `5` `INCLUDES_STATES`: The stream is a changefeed that includes states notes, e.g., `{state: 'initializing'}.

## Multipart responses ##

Streams and feeds are lazily-computed sequences, and return a `ResponseType` of `SUCCESS_PARTIAL` (`3`), with currently available data in the `r` array. When the driver receives a feed or stream, it should return a cursor (or an object with a cursor-like interface). *N.B.:* `SUCCESS_SEQUENCE` and `SUCCESS_PARTIAL` responses should be both be represented as cursors. Depending on the size of the query results and the time it takes to return them, you may receive either one `SUCCESS_SEQUENCE` result, or one or more `SUCCESS_PARTIAL` results followed by a final `SUCCESS_SEQUENCE` result.

To retrieve more data for the cursor, the driver should send a query with a `QueryType` of `CONTINUE` _on the same connection with the same token._ As with other queries, this must be sent with the query token, the size of the query, and the query itself, simply `[2]`.

| Step | Element | Transmitted bytes |  
|  ------ | ------ | ------ |  
| 1 | token | `00 00 00 00 00 00 00 01` |  
| 2 | length | `03 00 00 00` |  
| 3 | query | `[2]` |  

You will receive another response of either type `SUCCESS_PARTIAL`, indicating there is still more data available, or `SUCCESS_SEQUENCE` if you have reached the end of the stream. (This will never be returned for a feed.) Note that these `ResponseType`s can be returned without data (an empty array as the `r` value). A driver can send `CONTINUE` to fetch the next batch of a sequence as soon as the response is received.

To close a cursor and stop receiving data from the stream or feed, send a query with a `QueryType` of `STOP` on the same connection with the same token.

# Notes on connections #

Starting with RethinkDB 2.0 (`V0_4`), the server will process multiple queries in parallel rather than sequentially, and there is no guarantee that a read following a write on the same connection will "see" the results of the write as long as it's successful. (Previous versions of the server would process multiple queries on the same connection sequentially.)

You should not release a connection in the pool as soon as you receive a response. Only release the connection when you receive a response of a type other than `SUCCESS_PARTIAL`.

# Get help #

You can ask questions and get notes on changes introduced in new versions of RethinkDB on the [RethinkDB-Dev Google Group][rgg]. You can also visit the [RethinkDB IRC Channel][irc], where core developers and other driver developers frequently hang out. Also, you can ask questions on [Stack Overflow][so] using the tag "[rethinkdb][sotag]."

[rgg]: https://groups.google.com/forum/?fromgroups#!forum/rethinkdb-dev
[irc]: irc://irc.freenode.org/rethinkdb
[so]: http://stackoverflow.com/
[sotag]: http://stackoverflow.com/questions/tagged/rethinkdb
