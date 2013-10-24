---
layout: api-command 
language: Ruby
permalink: api/ruby/to_iso8601/
command: to_iso8601 
related_commands:
    now: now/
    time: time/
---

# Command syntax #

{% apibody %}
time.to_iso8601() &rarr; number
{% endapibody %}

# Description #

Convert a time object to its iso 8601 format.

__Example:__ Return the current time in an ISO8601 format.

```rb
r.now().to_iso8601()
```


