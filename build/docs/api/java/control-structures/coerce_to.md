---
layout: api-command
language: Java
permalink: api/java/coerce_to/
command: coerceTo
related_commands:
    object: object/
---

# Command syntax #

{% apibody %}
sequence.coerceTo("array") &rarr; array
value.coerceTo("string") &rarr; string
string.coerceTo("number") &rarr; number
array.coerceTo("object") &rarr; object
sequence.coerceTo("object") &rarr; object
object.coerceTo("array") &rarr; array
binary.coerceTo("string") &rarr; string
string.coerceTo("binary") &rarr; binary
{% endapibody %}

# Description #

Convert a value of one type into another.

* a sequence, selection or object can be coerced to an array
* a sequence, selection or an array of key-value pairs can be coerced to an object
* a string can be coerced to a number
* any datum (single value) can be coerced to to a string
* a binary object can be coerced to a string and vice-versa

__Example:__ Coerce a stream to an array to store its output in a field. (A stream cannot be stored in a field directly.)

```java
r.table("posts").map(post -> post.merge(
    r.hashMap("comments",
              r.table("comments").getAll(post.g("id")).optArg("index", "post_id")
              .coerceTo("array"))
)).run(conn);
```

__Example:__ Coerce an array of key-value pairs into an object.


```java
r.expr(r.array(r.array("name", "Ironman"), r.array("victories", 2000)))
 .coerceTo("object").run(conn);
```

__Note:__ To coerce a list of key-value pairs like `["name", "Ironman", "victories", 2000]` to an object, use the [object](/api/java/object) command.

__Example:__ Coerce a number to a string.

```java
r.expr(1).coerceTo("string").run(conn);
```
