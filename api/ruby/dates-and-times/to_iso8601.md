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
time.to_iso8601() &rarr; string
{% endapibody %}

# Description #

Convert a time object to a string in ISO 8601 format.

__Example:__ Return the current ISO 8601 time.

```rb
> r.now().to_iso8601().run(conn)

"2015-04-20T18:37:52.690+00:00"
```
