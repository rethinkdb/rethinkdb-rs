---
layout: api-command 
language: JavaScript
permalink: api/javascript/count/
command: count
github_doc: https://github.com/rethinkdb/docs/blob/master/2-query-language/api/javascript/aggregation/count.md
io:
    -   - sequence
        - value
related_commands:
    map: map/
    reduce: reduce/
    groupedMapReduce: grouped_map_reduce/

---

# Command syntax #

{% apibody %}
sequence.count([filter]) &rarr; number
{% endapibody %}

# Description #

Count the number of elements in the sequence. With a single argument, count the number
of elements equal to it. If the argument is a function, it is equivalent to calling
filter before count.

__Example:__ Just how many super heroes are there?

```js
r.table('marvel').count().add(r.table('dc').count()).run(conn, callback)
```

__Example:__ Just how many super heroes have invisibility?

```js
r.table('marvel').concatMap(r.row('superpowers')).count('invisibility').run(conn, callback)
```

__Example:__ Just how many super heroes have defeated the Sphinx?

```js
r.table('marvel').count(r.row('monstersKilled').contains('Sphinx')).run(conn, callback)
```

