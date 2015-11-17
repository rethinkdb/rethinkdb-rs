---
layout: api-command
language: Java
permalink: api/java/do/
command: do_
related_commands:
    map: map/
io:
    -   - any
        - any
---

# Command syntax #

{% apibody %}
any.do_(function) &rarr; any
r.do_([args]*, function) &rarr; any
any.do_(expr) &rarr; any
r.do_([args]*, expr) &rarr; any
{% endapibody %}

# Description #

Call an anonymous function using return values from other ReQL commands or queries as arguments.

The last argument to `do_` (or, in some forms, the only argument) is an expression or an anonymous function which receives values from either the previous arguments or from prefixed commands chained before `do_`. The `do_` command is essentially a single-element [map](/api/java/map/), letting you map a function over just one document. This allows you to bind a query result to a local variable within the scope of `do_`, letting you compute the result just once and reuse it in a complex expression or in a series of ReQL commands.

Arguments passed to the `do_` function must be basic data types, and cannot be streams or selections. (Read about [ReQL data types](/docs/data-types/).) While the arguments will all be evaluated before the function is executed, they may be evaluated in any order, so their values should not be dependent on one another. The type of `do_`'s result is the type of the value returned from the function or last expression.

__Example:__ Compute a golfer's net score for a game.

```java
r.table("players").get("86be93eb-a112-48f5-a829-15b2cb49de1d").do_(
    player -> player.g("gross_score").sub(player.g("course_handicap"))
).run(conn);
```

__Example:__ Return the best scoring player in a two-player golf match.

```java
r.do_(r.table("players").get(id1), r.table("players").get(id2),
    (player1, player2) -> r.branch(
        player1.g("gross_score").lt(player2.g("gross_score")),
        player1,
        player2
    )
).run(conn);

```

Note that `branch`, the ReQL conditional command, must be used instead of `if`. See the `branch` [documentation](/api/java/branch) for more.

__Example:__ Take different actions based on the result of a ReQL [insert](/api/java/insert) command.

```java
import com.rethinkdb.model.MapObject;

MapObject newData = r.expr(
    r.hashMap("id", 100)
        .with("name", "Agatha")
        .with("gross_score", 57)
        .with("course_handicap", 4)
).run(conn);

r.table("players").insert(newData).do_(doc ->
    r.branch(doc.g("inserted").ne(0),
        r.table("log").insert(
            r.hashMap("time", r.now())
               .with("response", doc)
               .with("result", "ok")),
        r.table("log").insert(
            r.hashMap("time", r.now())
               .with("response", doc)
               .with("result", "error"))
    )
).run(conn);
```
