---
layout: api-command 
language: JavaScript
permalink: api/javascript/contains/
command: contains
github_doc: https://github.com/rethinkdb/docs/blob/master/2-query-language/api/javascript/aggregation/contains.md
---

{% apibody %}
sequence.contains(value1[, value2...]) → bool
{% endapibody %}

Returns whether or not a sequence contains all the specified values, or if functions are
provided instead, returns whether or not a sequence contains values matching all the
specified functions.

__Example:__ Has Iron Man ever fought Superman?

```js
r.table('marvel').get('ironman')('opponents').contains('superman').run(conn, callback)
```

__Example:__ Has Iron Man ever defeated Superman in battle?

```js
r.table('marvel').get('ironman')('battles').contains(function (battle) {return battle('winner').eq('ironman').and(battle('loser').eq('superman'));})
```

