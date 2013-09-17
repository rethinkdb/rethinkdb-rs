---
layout: documentation
title: Troubleshooting common RethinkDB problems
active: docs
docs_active: troubleshooting 
permalink: docs/troubleshooting/
js: faq_index
---
<div id="faqcontents"></div>
{% faqsection Table of contents | %}
---
## My insert queries are slow. How can I speed them up? ##

{% infobox info %}
__Have you installed optimized `protobuf` libraries?__ [See this
document](/docs/driver-performance/) to learn how to use optimized
`protobuf` libraries with the client drivers.
{% endinfobox %}

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

  {% infobox info %}
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

## What does 'received invalid clustering header' mean? ##

{% include troubleshootingcluster.md %} 

## Does the web UI support my browser? ##

The following browsers are supported and known to work with the web
UI:

- Chrome 9 or higher
- Firefox 15 or higher
- Safari 6.02 or higher
- Opera 1.62 or higher

{% infobox info %}
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

{% endfaqsection %}
