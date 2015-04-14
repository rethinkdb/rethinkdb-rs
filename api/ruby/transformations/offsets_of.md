---
layout: api-command
language: Ruby
permalink: api/ruby/offsets_of/
command: offsets_of
alias: api/ruby/indexes_of/
---

# Command syntax #

{% apibody %}
sequence.offsets_of(datum | predicate) &rarr; array
{% endapibody %}

# Description #

Get the indexes of an element in a sequence. If the argument is a predicate, get the indexes of all elements matching it.

__Example:__ Find the position of the letter 'c'.

```rb
r.expr(['a','b','c']).offsets_of('c').run(conn)
```


__Example:__ Find the popularity ranking of invisible heroes.

```rb
r.table('marvel').union(r.table('dc')).order_by(:popularity).offsets_of{
    |row| row[:superpowers].contains('invisibility')
}.run(conn)
```

