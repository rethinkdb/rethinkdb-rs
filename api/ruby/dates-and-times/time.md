---
layout: api-command
language: Ruby
permalink: api/ruby/time/
command: time
related_commands:
    now: now/
    epoch_time: epoch_time/
    iso8601: iso8601/
---

# Command syntax #

{% apibody %}
r.time(year, month, day[, hour, minute, second], timezone)
    &rarr; time
{% endapibody %}

# Description #

Create a time object for a specific time.

__Example:__ Update the birthdate of the user "John" to November 3rd, 1986 UTC.

```rb
r.table("user").get("John").update(:birthdate => r.time(1986, 11, 3, 'Z')).run(conn)
```
