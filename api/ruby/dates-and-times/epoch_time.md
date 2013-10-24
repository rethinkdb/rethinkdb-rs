---
layout: api-command 
language: Ruby
permalink: api/ruby/epoch_time/
command: epoch_time
related_commands:
    now: now/
    time: time/
    iso8601: iso8601/
---

# Command syntax #

{% apibody %}
r.epoch_time(epoch_time) &rarr; time
{% endapibody %}

# Description #

Create a time object based on seconds since epoch.

__Example:__ Update the birthdate of the user "John" to November 3rd, 1986.

```rb
r.table("user").get("John").update(:birthdate => r.epoch_time(531360000)).run(conn)
```
