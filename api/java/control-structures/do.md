---
layout: api-command
language: Java
permalink: api/javascript/do/
command: do
related_commands:
    map: map/
io:
    -   - any
        - any
---

# Command syntax #

{% apibody %}
any.do(function) &rarr; any
r.do([args]*, function) &rarr; any
any.do(expr) &rarr; any
r.do([args]*, expr) &rarr; any
{% endapibody %}

# Description #

Call an anonymous function using return values from other ReQL commands or queries as arguments.

The last argument to `do` (or, in some forms, the only argument) is an expression or an anonymous function which receives values from either the previous arguments or from prefixed commands chained before `do`. The `do` command is essentially a single-element [map](/api/javascript/map/), letting you map a function over just one document. This allows you to bind a query result to a local variable within the scope of `do`, letting you compute the result just once and reuse it in a complex expression or in a series of ReQL commands.

Arguments passed to the `do` function must be basic data types, and cannot be streams or selections. (Read about [ReQL data types](/docs/data-types/).) While the arguments will all be evaluated before the function is executed, they may be evaluated in any order, so their values should not be dependent on one another. The type of `do`'s result is the type of the value returned from the function or last expression.

__Example:__ Compute a golfer's net score for a game.

```js
r.table('players').get('f19b5f16-ef14-468f-bd48-e194761df255').do(
    function (player) {
        return player('gross_score').sub(player('course_handicap'));
    }
).run(conn);
```

__Example:__ Return the best scoring player in a two-player golf match.

```js
r.do(r.table('players').get(id1), r.table('players').get(id2),
    function (player1, player2) {
        return r.branch(player1('gross_score').lt(player2('gross_score')),
            player1, player2);
    }
).run(conn);
```

Note that `branch`, the ReQL conditional command, must be used instead of `if`. See the `branch` [documentation](/api/javascript/branch) for more.

__Example:__ Take different actions based on the result of a ReQL [insert](/api/javascript/insert) command.

```js
var newData = {
    id: 100,
    name: 'Agatha',
    gross_score: 57,
    course_handicap: 4
};
r.table('players').insert(newData).do(
    function (doc) {
        return r.branch(doc('inserted').ne(0),
            r.table('log').insert({time: r.now(), response: doc, result: 'ok'}),
            r.table('log').insert({time: r.now(), response: doc, result: 'error'}))
    }
).run(conn);
```
