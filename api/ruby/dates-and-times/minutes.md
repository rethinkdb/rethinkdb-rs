---
layout: api-command
language: Ruby
permalink: api/ruby/minutes/
command: minutes
related_commands:
    now: now/
    time: time/
    in_timezone: in_timezone/
---

# Command syntax #

{% apibody %}
time.minutes() &rarr; number
{% endapibody %}

# Description #

Return the minute in a time object as a number between 0 and 59.

__Example:__ Return all the posts submitted during the first 10 minutes of every hour.

```rb
r.table("posts").filter{ |post|
    post["date"].minutes() < 10
}
```
