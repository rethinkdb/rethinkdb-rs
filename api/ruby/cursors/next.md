---
layout: api-command
language: Ruby
permalink: api/ruby/next/
command: next
related_commands:
    for (cursor): each/
    to_a: to_array/
    close (cursor): close-cursor/
---

# Command syntax #

{% apibody %}
cursor.next([true])
{% endapibody %}

# Description #

Get the next element in the cursor.

The optional unnamed argument specifies whether to wait for the next available element and how long to wait:

* `true`: Wait indefinitely (the default).
* `false`: Do not wait at all. If data is immediately available, it will be returned; if it is not available, a `Timeout::Error` will be raised. 
* number: Wait up to the specified number of seconds for data to be available before raising `Timeout::Error`.

The behavior of `next` will be identical with `false`, `nil` or the number `0`.

Calling `next` the first time on a cursor provides the first element of the cursor. If the data set is exhausted (e.g., you have retrieved all the documents in a table), a `StopIteration` error will be raised when `next` is called.

__Example:__ Retrieve the next element.

```rb
cursor = r.table('superheroes').run(conn)
doc = cursor.next()
```

__Example:__ Retrieve the next element on a [changefeed](/docs/changefeeds/ruby), waiting up to five seconds.

```rb
cursor = r.table('superheroes').changes().run(conn)
doc = cursor.next(5)
```

__Note:__ RethinkDB sequences can be iterated through via the Ruby [Enumerable][it] interface. The canonical way to retrieve all the results is to use an [each](../each/) command or [to_a()](../to_array/).

[it]: http://ruby-doc.org/core/Enumerable.html
