---
layout: api-command 
language: JavaScript
permalink: api/javascript/slice/
command: slice 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/transformations/slice.md
io:
    -   - sequence
        - stream
    -   - array
        - array
related_commands:
    skip: skip/
    limit: limit/
    nth: nth/
---

# Command syntax #

{% apibody %}
sequence.slice(startIndex[, endIndex]) &rarr; stream
array.slice(startIndex[, endIndex]) &rarr; array
{% endapibody %}

# Description #

Trim the sequence to within the bounds provided.

__Example:__ For this fight, we need heroes with a good mix of strength and agility.

```js
r.table('marvel').orderBy('strength').slice(5, 10).run(conn, callback)
```
