---
layout: document
title: RethinkDB client driver protocol
active: docs
permalink: docs/driver-spec/
---
{{ 'RethinkDB client driver protocol' | page_title }}

# Stage 0: Connect

Open a TCP connection to the server on the driver port.  The default
driver port is `28015`.

# Stage 1: Driver Handshake

At the start of every connection, the driver needs to send some
information to the server and get back a response indicating whether
it's OK to go ahead and send queries.

1. Send the protocol version to the server as a 4-byte little-endian-encoded integer.
  - Versions can be found in the `Version` enum in
    https://github.com/rethinkdb/rethinkdb/blob/v{{site.version.major}}.x/src/rdb_protocol/ql2.proto
    .
  - The first version to support the JSON protocol is `V0_3`.  All
    instructions below assume a protocol of `V0_3` or higher.
2. Send the length of the auth key to the server as a 4-byte
  little-endian-encoded integer.
  - Send `0` if there is no authorization key.
3. Send the auth key as an ASCII string.
  - If there is no auth key, skip this step, as the length of the
    string is 0.
4. Send the protocol type as a 4-byte little-endian-encoded integer.
  - Protocol types can be found in the `Protocol` enum in
    https://github.com/rethinkdb/rethinkdb/blob/v{{site.version.major}}.x/src/rdb_protocol/ql2.proto
    .
  - This guide assumes you send the `JSON` protocol type, but if
    you're updating a driver from `V0_2` you can specify `PROTOBUF` to
    upgrade your driver to `V0_3` without any additional work.  The
    `PROTOBUF` protocol is considered deprecated, so at some point you
    should upgrade to the `JSON` protocol.  (It should also be
    faster.)

The server will respond with a NULL-terminated ASCII string describing
the result of the handshake.  If the string is "SUCCESS", the client
may proceed to stage 2 and begin sending queries.  Any other string
indicates an error.  The server will close the connection, and the
driver should report this error to the user.

## Example 1: No auth key.

| Step | Direction | Semantic Command | Value | Bytes on Wire |
| --- | --- | --- | --- | --- |
| 1 | SEND | V0_3 | 0x5f75e83e | `3e e8 75 5f` |
| 2 | SEND | 0 length auth key | 0 | `00 00 00 00` |
| 3 | SEND | no auth key | | |
| 4 | SEND | JSON | 0x7e6970c7 | `c7 70 69 7e` |
| 5 | RECV | success | "SUCCESS" | `53 55 43 43 45 53 53` |

## Example 2: Auth key.

| Step | Direction | Semantic Command | Value | Bytes on Wire |
| --- | --- | --- | --- | --- |
| 1 | SEND | V0_3 | 0x5f75e83e | `3e e8 75 5f` |
| 2 | SEND | 7 character auth key | 7 | `07 00 00 00` |
| 3 | SEND | auth key | "hunter2" | `68 75 6e 74 65 72 32` |
| 4 | SEND | JSON | 0x7e6970c7 | `c7 70 69 7e` |
| 5 | RECV | success | "SUCCESS" | `53 55 43 43 45 53 53` |

# Stage 2: Query

Each query you send to the server has the following form:

1. A unique 8-byte token.  (This is often an 8-byte
  little-endian-encoded counter.)
  - The server will respond to your query with this same token.
2. The length of the JSON-encoded query as a 4-byte little-endian-encoded integer.
3. The JSON encoding of the query itself.

The server will reply with the JSON encoding of a response.  More
details on the query and response are below.

## The Query

A `Query` is a 1 or 3 element array:

```
[QueryType]
# or
[QueryType, Term, Optargs]
```

The available `QueryType`s are documented in
https://github.com/rethinkdb/rethinkdb/blob/v{{site.version.major}}.x/src/rdb_protocol/ql2.proto
.  Only a `START` query needs a `Term` and
`Optargs`.  `Term` is the body of the query, described below, and
`Optargs` is an object representing the global optional arguments for
the query.  The valid keys for this object are:

* `db` -- the default database to use.
* `use_outdated` -- true if the query can do outdated reads for speed.
* `noreply` -- true if the server doesn't need to respond to the
  query.
* `durability` -- either `hard` or `soft` depending on the durability
  you want for writes.
* `profile` -- true if profiling should be enabled.

### Example 1: Send a STOP query for token 5

If we send `r.table('test')` to the server with token 5, we get back a
stream.  If we read a few rows from that stream and then want to close
the stream on the server, we need to send a `STOP` query for that same
token.  `Query::QueryType::STOP` is `3` in
https://github.com/rethinkdb/rethinkdb/blob/v{{site.version.major}}.x/src/rdb_protocol/ql2.proto
, so we want to send `[3]` for token 5.

| Step | Direction | Semantic Command | Value | Bytes on Wire |
| --- | --- | --- | --- | --- |
| 1 | SEND | token | 5 | `05 00 00 00 00 00 00 00` |
| 2 | SEND | query length | 3 | `03 00 00 00` |
| 3 | SEND | query | "[3]" | `5b 33 5d` |

## The Term

A `Term` is just a JSON expression, with the exception that arrays
represent function calls rather than literal arrays.  (There is a
function `MAKE_ARRAY` that can be used to produce an actual array.)
The function call is a 1-3 element array of the following form:

```
[TermType, Args, Optargs]
```

The available `QueryType`s and their legal optargs are documented in
https://github.com/rethinkdb/rethinkdb/blob/v{{site.version.major}}.x/src/rdb_protocol/ql2.proto
.  The `Args` should be an array of arguments, themselves terms, and
the `Optargs` should be an object mapping from optarg names to terms.

### Example 1: r.table('test').insert({})

```
# [INSERT, [[TABLE, ['test']], {}]]
[56, [[15, ['test']], {}]]
```

### Example 2: r.table('test').insert([{}, {}], durability:'soft')

```
# [INSERT, [[TABLE, ['test']], [MAKE_ARRAY, [{}, {}]]], {durability: 'soft'}]
[56, [[15, ['test']]], [2, [{}, {}]], {durability: 'soft'}]
```

### Example 3: r.expr(1)

```
# Note the absence of `TermType::DATUM` -- we just write number directly
1
```

# Stage 3: The Response

The server's response to your query will take the following form:

1. The 8-byte token of the query the response corresponds to.
2. The length of the JSON-encoded response as a 4-byte little-endian-encoded
integer.
3. The JSON encoding of the `Response` itself.

A `Response` is a JSON object with the following fields:

* `t` -- the `ResponseType` from
  https://github.com/rethinkdb/rethinkdb/blob/v{{site.version.major}}.x/src/rdb_protocol/ql2.proto
  .
* `r` -- An array of JSON expressions representing the query's result.
  If `t` is `SUCCESS_ATOM` or an error type, this will be an array of
  one element.  (In the case of an error type, that one element will
  be the error message.)
* `b` -- A backtrace in the case where `t` is an error type.  This is
  an array of frames, which are either strings or integers.  A string
  indicates that the error occurred in the optarg named by the string,
  and an integer indicates that the error occurred in the argument at
  that index.  For example, the query `r.random(1, 2, float:
  r.add(r.add(1, "a")))` would return a type error with the backtrace
  `["float", 0]` to indicate that `r.add(1, "a")` should be underlined.
* `p` -- A profile in the case where the global optarg `profile:true` was specified.

## Example 1: A response to `r.table('test').count()`

Assuming `r.table('test').count()` returns `7`:

| Step | Direction | Semantic Command | Value | Bytes on Wire |
| --- | --- | --- | --- | --- |
| 1 | RECV | token | 5 | `05 00 00 00 00 00 00 00` |
| 2 | RECV | response length | 15 | `0F 00 00 00` |
| 3 | RECV | response | '{"t":1,"r":[7]}' | `7b 22 74 22 3a 31 2c 22 72 22 3a 5b 37 5d 7d` |

# A complete example

Let's run `r.table('test').count()`.  The `Term` is
`[COUNT, [[TABLE, ['test']]]] = [43, [[15, ['test']]]]`, and the query
will return `7`.

| Step | Direction | Semantic Command | Value | Bytes on Wire |
| --- | --- | --- | --- | --- |
| Handshake 1 | SEND | V0_3 | 0x5f75e83e | `3e e8 75 5f` |
| Handshake 2 | SEND | 0 length auth key | 0 | `00 00 00 00` |
| Handshake 3 | SEND | no auth key | | |
| Handshake 4 | SEND | JSON | 0x7e6970c7 | `c7 70 69 7e` |
| Handshake 5 | RECV | success | "SUCCESS" | `53 55 43 43 45 53 53` |
| Query 1 | SEND | token | 5 | `05 00 00 00 00 00 00 00` |
| Query 2 | SEND | query length | 20 | `1B 00 00 00` |
| Query 3 | SEND | query | '[1,[43,[[15,["test"]]]],{}]' | `5b 31 2c 5b 34 33 2c 5b 5b 31 35 2c 5b 22 74 65 73 74 22 5d 5d 5d 5d 2c 7b 7d 5d` |
| Response 1 | RECV | token | 5 | `05 00 00 00 00 00 00 00` |
| Response 2 | RECV | response length | 15 | `0F 00 00 00` |
| Response 3 | RECV | response | '{"t":1,"r":[7]}' | `7b 22 74 22 3a 31 2c 22 72 22 3a 5b 37 5d 7d` |

