---
layout: api-command 
language: JavaScript
permalink: api/javascript/error/
command: error 
github_doc: https://github.com/rethinkdb/docs/blob/docs/2-query-language/api/javascript/control-structures/error.md
io:
    -   - r
        - error
---

{% apibody %}
r.error(message) → error
{% endapibody %}

Throw a runtime error. If called with no arguments inside the second argument to `default`, re-throw the current error.

__Example:__ Iron Man can't possibly have lost a battle:

```js
r.table('marvel').get('IronMan').do(function(ironman) {
    return r.branch(ironman('victories').lt(ironman('battles')),
        r.error('impossible code path'),
        ironman)
}).run(conn, callback)
```


