---
layout: api-command
language: Python
permalink: api/python/to_epoch_time/
command: to_epoch_time
related_commands:
    now: now/
    time: time/
    to_iso8601: to_iso8601/
---

# Command syntax #

{% apibody %}
time.to_epoch_time() &rarr; number
{% endapibody %}

# Description #

Convert a time object to its epoch time.

__Example:__ Return the current time in seconds since the Unix Epoch with millisecond-precision.

```py
r.now().to_epoch_time()
```


