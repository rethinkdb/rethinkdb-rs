---
layout: api-command 
language: JavaScript
permalink: api/javascript/keys/
command: keys
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/document-manipulation/keys.md
io:
    -   - singleSelection
        - array
    -   - object
        - array
---

# Command syntax #

{% apibody %}
singleSelection.keys() &rarr; array
object.keys() &rarr; array
{% endapibody %}

# Description #

Return an array containing all of the object's keys.

__Example:__ Get all the keys of a row.

```js
r.table('marvel').get('ironman').keys().run(conn, callback)
```


