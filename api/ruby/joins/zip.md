---
layout: api-command 
language: Ruby
permalink: api/ruby/zip/
command: zip 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/joins/zip.md
related_commands:
    eq_join: eq_join/
    inner_join: inner_join/
    outer_join: outer_join/
---

# Command syntax #

{% apibody %}
stream.zip() &rarr; stream
array.zip() &rarr; array
{% endapibody %}

# Description #

Used to 'zip' up the result of a join by merging the 'right' fields into 'left' fields of each member of the sequence.

__Example:__ 'zips up' the sequence by merging the left and right fields produced by a join.

```
r.table('marvel').eq_join(:main_dc_collaborator, r.table('dc')).zip.run(conn)
```


