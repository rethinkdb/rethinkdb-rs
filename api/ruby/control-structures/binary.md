---
layout: api-command
language: Ruby
permalink: api/ruby/binary/
command: binary
---

# Command syntax #

{% apibody %}
r.binary(data) &rarr; binary
{% endapibody %}

# Description #

Encapsulate binary data within a query.

The type of data `binary` accepts depends on the client language. In Ruby, it expects a `String` object.

Binary objects returned to the client in JavaScript will also be of the `String` type. This can be changed with the `binary_format` option provided to [run](/api/ruby/run) to return "raw" objects.

Only a limited subset of ReQL commands may be chained after `binary`:

* [coerce_to](/api/ruby/coerce_to/) can coerce `binary` objects to `string` types
* [count](/api/ruby/count/) will return the number of bytes in the object
* [slice](/api/ruby/slice/) will treat bytes like array indexes (i.e., `slice(10,20)` will return bytes 10&ndash;19)
* [type_of](/api/ruby/type_of) returns `PTYPE<BINARY>`
* [info](/api/ruby/info) will return information on a binary object.

__Example:__ Save an avatar image to a existing user record.

```rb
f = File.open('./default_avatar.png', 'rb')
avatar_image = f.read()
f.close()
r.table('users').get(100).update({:avatar => r.binary(avatar_image)}).run(conn)
```

__Example:__ Get the size of an existing avatar image.

```py
r.table('users').get(100)['avatar'].count().run(conn)

14156
```
