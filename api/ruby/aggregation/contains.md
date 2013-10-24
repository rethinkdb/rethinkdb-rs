---
layout: api-command 
language: Ruby
permalink: api/ruby/contains/
command: contains 
github_doc: https://github.com/rethinkdb/docs/blob/master/2-query-language/api/ruby/aggregation/contains.md
related_commands:
    map: map/
    reduce: reduce/
---

# Command syntax #

{% apibody %}
sequence.contains(value1[, value2...]) &rarr; bool
{% endapibody %}

# Description #

Returns whether or not a sequence contains all the specified values, or if functions are
provided instead, returns whether or not a sequence contains values matching all the
specified functions.

__Example:__ Has Iron Man ever fought Superman?

```rb
r.table('marvel').get('ironman')[:opponents].contains('superman').run(conn)
```


__Example:__ Has Iron Man ever defeated Superman in battle?

```rb
r.table('marvel').get('ironman')[:battles].contains{|battle|
    battle[:winner].eq('ironman') & battle[:loser].eq('superman')
}.run(conn)
```

