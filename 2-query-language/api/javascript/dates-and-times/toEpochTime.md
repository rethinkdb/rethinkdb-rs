---
layout: api-command 
language: JavaScript
permalink: api/javascript/to_epoch_time/
command: toEpochTime
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/dates-and-times/toEpochTime.md
io:
    -   - time
        - number
related_commands:
    now: now/
    time: time/
---

{% apibody %}
time.to_epoch_time() &rarr; number
{% endapibody %}

Convert a time object to its epoch time.

__Example:__ Return the current time in an ISO8601 format.

```js
r.now().toEpochTime()
```


