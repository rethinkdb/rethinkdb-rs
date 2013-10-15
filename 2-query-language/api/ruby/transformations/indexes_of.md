---
layout: api-command 
language: Ruby
permalink: api/ruby/indexes_of/
command: indexes_of
---

{% apibody %}
sequence.indexes_of(datum | predicate) → array
{% endapibody %}

Get the indexes of an element in a sequence. If the argument is a predicate, get the indexes of all elements matching it.

__Example:__ Find the position of the letter 'c'.

```rb
r.expr(['a','b','c']).indexes_of('c').run(conn)
```


__Example:__ Find the popularity ranking of invisible heroes.

```rb
r.table('marvel').union(r.table('dc')).order_by(:popularity).indexes_of{
    |row| row[:superpowers].contains('invisibility')
}.run(conn)
```

