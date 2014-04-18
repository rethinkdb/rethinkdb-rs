---
layout: api-command
language: Ruby
permalink: api/ruby/to_epoch_time/
command: to_epoch_time
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

__Example:__ Return the current time in seconds since the Unix Epoch with millisecond-precision.

```rb
r.now().to_epoch_time()
```
