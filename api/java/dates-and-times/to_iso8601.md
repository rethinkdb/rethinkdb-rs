---
layout: api-command
language: Java
permalink: api/javascript/to_iso8601/
command: toISO8601
related_commands:
    now: now/
    time: time/
---

# Command syntax #

{% apibody %}
time.toISO8601() &rarr; string
{% endapibody %}

# Description #

Convert a time object to a string in ISO 8601 format.

__Example:__ Return the current ISO 8601 time.

```js
r.now().toISO8601().run(conn)
// Result passed to callback
"2015-04-20T18:37:52.690+00:00"
```

