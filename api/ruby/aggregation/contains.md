---
layout: api-command
language: Ruby
permalink: api/ruby/contains/
command: contains
related_commands:
    map: map/
    concat_map: concat_map/
    group: group/
---

# Command syntax #

{% apibody %}
sequence.contains(value1[, value2...]) &rarr; bool
sequence.contains(predicate1[, predicate2...]) &rarr; bool
{% endapibody %}

# Description #

When called with values, returns `true` if a sequence contains all the
specified values.  When called with predicate functions, returns `true`
if for each predicate there exists at least one element of the stream
where that predicate returns `true`.


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

