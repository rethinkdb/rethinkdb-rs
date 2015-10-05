---
layout: api-command
language: Java
permalink: api/java/binary/
command: binary
io:
    -   - r
        - binary
---

# Command syntax #

{% apibody %}
r.binary(data) &rarr; binary
{% endapibody %}

# Description #

Encapsulate binary data within a query.

The type of data `binary` accepts depends on the client language. In JavaScript, it expects a [Node.js](http://nodejs.org) `Buffer`. Using a `Buffer` object within a query implies the use of `binary` and the ReQL driver will automatically perform the coercion.

Binary objects returned to the client in JavaScript will also be Node.js `Buffer` objects. This can be changed with the `binaryFormat` option provided to [run](/api/java/run) to return "raw" objects.

Only a limited subset of ReQL commands may be chained after `binary`:

* [coerceTo](/api/java/coerce_to/) can coerce `binary` objects to `string` types
* [count](/api/java/count/) will return the number of bytes in the object
* [slice](/api/java/slice/) will treat bytes like array indexes (i.e., `slice(10,20)` will return bytes 10&ndash;19)
* [typeOf](/api/java/type_of) returns `PTYPE<BINARY>`
* [info](/api/java/info) will return information on a binary object.

__Example:__ Save an avatar image to a existing user record.

```java
var fs = require('fs');
fs.readFile('./defaultAvatar.png', function (err, avatarImage) {
    if (err) {
        // Handle error
    }
    else {
        r.table('users').get(100).update({
            avatar: avatarImage
        })
    }
});
```

__Example:__ Get the size of an existing avatar image.

```java
r.table('users').get(100)('avatar').count().run(conn);
// result returned to callback
14156
```

Read more details about RethinkDB's binary object support: [Storing binary objects](/docs/storing-binary/).
