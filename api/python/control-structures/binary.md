---
layout: api-command
language: Python
permalink: api/python/binary/
command: binary
---

# Command syntax #

{% apibody %}
r.binary(data) &rarr; binary
{% endapibody %}

# Description #

Encapsulate binary data within a query.

The type of data `binary` accepts depends on the client language. In Python, it expects a parameter of `bytes` type. Using a `bytes` object within a query implies the use of `binary` and the ReQL driver will automatically perform the coercion (in Python 3 only).

Binary objects returned to the client in JavaScript will also be of the `bytes` type. This can be changed with the `binary_format` option provided to [run](/api/python/run) to return "raw" objects.

Only a limited subset of ReQL commands may be chained after `binary`:

* [coerce_to](/api/python/coerce_to/) can coerce `binary` objects to `string` types
* [count](/api/python/count/) will return the number of bytes in the object
* [slice](/api/python/slice/) will treat bytes like array indexes (i.e., `slice(10,20)` will return bytes 10&ndash;19)
* [type_of](/api/python/type_of) returns `PTYPE<BINARY>`
* [info](/api/python/info) will return information on a binary object.

__Example:__ Save an avatar image to a existing user record.

```py
f = open('./default_avatar.png', 'rb')
avatar_image = f.read()
f.close()
r.table('users').get(100).update({'avatar': r.binary(avatar_image)}).run(conn)
```

__Example:__ Get the size of an existing avatar image.

```py
r.table('users').get(100)['avatar'].count().run(conn)

14156
```
