---
layout: api-command 
language: Ruby
permalink: api/ruby/union/
command: union 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/transformations/union.md
---

# Command syntax #

{% apibody %}
sequence.union(sequence) &rarr; array
{% endapibody %}

# Description #

Concatenate two sequences.

__Example:__ Construct a stream of all heroes.

```rb
r.table('marvel').union(r.table('dc')).run(conn)
```


