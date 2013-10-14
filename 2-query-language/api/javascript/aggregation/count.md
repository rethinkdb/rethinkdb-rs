---
layout: api-command 
permalink: api/javascript/count/
command: count
---

{% apibody %}
sequence.count([filter]) → number
{% endapibody %}

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

