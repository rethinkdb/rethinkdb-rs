---
layout: api-command
language: JavaScript
permalink: api/javascript/contains/
command: contains
io:
    -   - sequence
        - bool
related_commands:
    map: map/
    concat_map: concat_map/
    group: group/
---

# Command syntax #

{% apibody %}
sequence.contains([value | predicate_function, ...]) &rarr; bool
{% endapibody %}

# Description #

When called with values, returns `true` if a sequence contains all the
specified values.  When called with predicate functions, returns `true`
if for each predicate there exists at least one element of the stream
where that predicate returns `true`.

Values and predicates may be mixed freely in the argument list.

__Example:__ Has Iron Man ever fought Superman?

```js
r.table('marvel').get('ironman')('opponents').contains('superman').run(conn, callback)
```


__Example:__ Has Iron Man ever defeated Superman in battle?

```js
r.table('marvel').get('ironman')('battles').contains(function (battle) {
    return battle('winner').eq('ironman').and(battle('loser').eq('superman'));
}).run(conn, callback)
```

__Example:__ Use `contains` with a predicate function to simulate an `or`. Return the Marvel superheroes who live in Detroit, Chicago or Hoboken.

```js
r.table('marvel').filter(function(hero) {
    return r.expr(['Detroit', 'Chicago', 'Hoboken']).contains(hero('city'))
}).run(conn, callback)
```
