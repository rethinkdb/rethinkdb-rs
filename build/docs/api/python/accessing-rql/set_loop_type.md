---
layout: api-command
language: Python
permalink: api/python/set_loop_type/
command: set_loop_type
javascript: false
ruby: false
related_commands:
    connect: connect/
    run: run/
---

# Command syntax #

{% apibody %}
r.set_loop_type(string)
{% endapibody %}

# Description #

Set an asynchronous event loop model. There are two supported models:

* `"tornado"`: use the [Tornado web framework][tor]. Under this model, the [connect][con] and [run][] commands will return Tornado `Future` objects.
* `"twisted"`: use the [Twisted networking engine][twi]. Under this model, the [connect][con] and [run][] commands will return Twisted `Deferred` objects.
* `"gevent"`: use the [gevent networking library][gev]. (Unlike the other asynchronous models, this does not change [connect][con] and [run][] handling; inside the event loop, this is indistinguishable from the default Python driver.)
* `"asyncio"`: use Python 3's [asyncio package][asy]. Under this model, the [connect][con] and [run][] commands will return asyncio `Future` objects.

[tor]: http://www.tornadoweb.org/
[twi]: http://twistedmatrix.com/
[gev]: http://www.gevent.org
[asy]: https://docs.python.org/3/library/asyncio.html
[con]: /api/python/connect
[run]: /api/python/run

__Example:__ Read a table's data using Tornado.

```python
r.set_loop_type("tornado")
conn = r.connect(host='localhost', port=28015)

@gen.coroutine
def use_cursor(conn):
    # Print every row in the table.
    cursor = yield r.table('test').order_by(index="id").run(yield conn)
    while (yield cursor.fetch_next()):
        item = yield cursor.next()
        print(item)
```

For a longer discussion with both Tornado and Twisted examples, see the documentation article on [Asynchronous connections][ac].

[ac]: /docs/async-connections/#python-with-tornado-or-twisted
