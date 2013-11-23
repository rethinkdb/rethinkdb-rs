---
layout: api-command
language: JavaScript
permalink: api/javascript/to_iso8601/
command: to_iso8601
io:
    -   - time
        - string
related_commands:
    now: now/
    time: time/
---

# Command syntax #

{% apibody %}
time.toISO8601() &rarr; number
{% endapibody %}

# Description #

Convert a time object to its iso 8601 format.

__Example:__ Return the current time in an ISO8601 format.

```js
r.now().toISO8601()
```

