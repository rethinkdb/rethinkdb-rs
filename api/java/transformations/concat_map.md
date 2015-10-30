---
layout: api-command
language: Java
permalink: api/java/concat_map/
command: concatMap
related_commands:
    map: map/
---

# Command syntax #

{% apibody %}
stream.concatMap(function) &rarr; stream
array.concatMap(function) &rarr; array
{% endapibody %}

# Description #

Concatenate one or more elements into a single sequence using a mapping function.

`concatMap` works in a similar fashion to [map](/api/java/map/), applying the given function to each element in a sequence, but it will always return a single sequence. If the mapping function returns a sequence, `map` would produce a sequence of sequences:

```java
r.expr(r.array(1, 2, 3)).map(x -> r.array(x, x.mul(2))).run(conn);
```

Result:

```json
[[1, 2], [2, 4], [3, 6]]
```

Whereas `concatMap` with the same mapping function would merge those sequences into one:

```java
r.expr(r.array(1, 2, 3)).concatMap(x -> r.array(x, x.mul(2))).run(conn);
```

Result:

```json
[1, 2, 2, 4, 3, 6]
```

The return value, array or stream, will be the same type as the input.

__Example:__ Construct a sequence of all monsters defeated by Marvel heroes. The field "defeatedMonsters" is an array of one or more monster names.

```java
r.table("marvel").concatMap(hero -> hero.g("defeatedMonsters")).run(conn);
```

__Example:__ Simulate an [eqJoin](/api/java/eq_join/) using `concatMap`. (This is how ReQL joins are implemented internally.)

```java
r.table("posts").concatMap(
    post -> r.table("comments").getAll(post.g("id")).optArg("index", "post_id")
             .map(comment -> r.hashMap("left", post).with("right", comment))
).run(conn);
```