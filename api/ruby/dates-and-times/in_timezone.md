---
layout: api-command 
language: Ruby
permalink: api/ruby/in_timezone/
command: in_timezone 
related_commands:
    timezone: timezone/
    now: now/
    time: time/
---

# Command syntax #

{% apibody %}
time.in_timezone(timezone) &rarr; time
{% endapibody %}

# Description #

Return a new time object with a different timezone. While the time stays the same, the results returned by methods such as hours() will change since they take the timezone into account. The timezone argument has to be of the ISO 8601 format.

__Example:__ Hour of the day in San Francisco (UTC/GMT -8, without daylight saving time).

```rb
r.now().in_timezone('-08:00').hours().run(conn)
```


