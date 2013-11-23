---
layout: api-command
language: Ruby
permalink: api/ruby/count/
command: count
related_commands:
    map: map/
    reduce: reduce/
    grouped_map_reduce: grouped_map_reduce/
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

```rb
(r.table('marvel').count() + r.table('dc').count()).run(conn)
```


__Example:__ Just how many super heroes have invisibility?

```rb
r.table('marvel').concat_map{ |row| row[:superpowers] }.count('invisibility').run(conn)
```


__Example:__ Just how many super heroes have defeated the Sphinx?

```rb
r.table('marvel').count{ |row| row['monstersKilled'].contains('Sphinx') }.run(conn)
```

