---
layout: api-command 
language: JavaScript
permalink: api/javascript/to_epoch_time/
command: toEpochTime
io:
    -   - time
        - number
related_commands:
    now: now/
    time: time/
---

# Command syntax #

{% apibody %}
time.to_epoch_time() &rarr; number
{% endapibody %}

# Description #

Convert a time object to its epoch time.

__Example:__ Return the current time in an ISO8601 format.

```js
r.now().toEpochTime()
```


