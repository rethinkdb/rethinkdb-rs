---
layout: api-command 
language: Ruby
permalink: api/ruby/seconds/
command: seconds 
related_commands:
    now: now/
    time: time/
    in_timezone: in_timezone/
---

# Command syntax #

{% apibody %}
time.seconds() &rarr; number
{% endapibody %}

# Description #

Return the seconds in a time object as a number between 0 and 59.999 (double precision).

__Example:__ Return the post submitted during the first 30 seconds of every minute.

```rb
r.table("posts").filter{ |post|
    post["date"].seconds() < 30
}
```
