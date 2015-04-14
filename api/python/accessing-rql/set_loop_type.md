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

Set an asynchronous event loop model. Currently, the only event loop model RethinkDB supports is `"tornado"`, for use with the [Tornado web framework](http://www.tornadoweb.org). After setting the event loop to `"tornado"`, the [connect](/api/python/connect) and [run](/api/python/run) commands will return Tornado `Future` objects.

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

For a longer discussion with Tornado examples, see the documentation article on [Asynchronous connections][ac].

[ac]: /docs/async-connections/
