---
layout: api-command
language: Ruby
permalink: api/ruby/hours/
command: hours
related_commands:
    now: now/
    time: time/
    in_timezone: in_timezone/
---

# Command syntax #

{% apibody %}
time.hours() &rarr; number
{% endapibody %}

# Description #

Return the hour in a time object as a number between 0 and 23.

__Example:__ Return all the posts submitted after midnight and before 4am.

```rb
r.table("posts").filter{ |post|
    post["date"].hours() < 4
}
```
