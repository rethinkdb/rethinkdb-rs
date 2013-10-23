---
layout: api-command 
language: JavaScript
permalink: api/javascript/default/
command: default 
github_doc: https://github.com/rethinkdb/docs/blob/docs/2-query-language/api/javascript/control-structures/default.md
io:
    -   - value
        - any
    -   - sequence
        - any
---

{% apibody %}
value.default(default_value) &rarr; any
sequence.default(default_value) &rarr; any
{% endapibody %}

Handle non-existence errors. Tries to evaluate and return its first argument. If an
error related to the absence of a value is thrown in the process, or if its first
argument returns null, returns its second argument. (Alternatively, the second argument
may be a function which will be called with either the text of the non-existence error
or null.)

__Example:__ Stark Industries made the mistake of trusting an intern with data entry,
and now a bunch of fields are missing from some of their documents. Iron Man takes a
break from fighting Mandarin to write some safe analytics queries.

```js
r.table('projects').map(function(p) {
    return p('staff').default(0).add(p('management').default(0))
}).run(conn, callback)
```


