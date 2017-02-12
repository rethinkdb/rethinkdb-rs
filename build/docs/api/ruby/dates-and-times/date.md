---
layout: api-command
language: Ruby
permalink: api/ruby/date/
command: date
related_commands:
    now: now/
    time: time/
    in_timezone: in_timezone/
---

# Command syntax #

{% apibody %}
time.date() &rarr; time
{% endapibody %}

# Description #

Return a new time object only based on the day, month and year (ie. the same day at 00:00).

__Example:__ Retrieve all the users whose birthday is today.

```rb
r.table("users").filter{ |user|
    user["birthdate"].date() == r.now().date()
}.run(conn)
```

<!-- stop -->

Note that the [now][] command always returns UTC time, so the comparison may fail if `user["birthdate"]` isn't also in UTC. You can use the [in_timezone][itz] command to adjust for this:

```rb
r.table("users").filter{ |user|
    user["birthdate"].date() == r.now().in_timezone("-08:00").date()
}.run(conn)
```

[now]: /api/ruby/now/
[itz]: /api/ruby/in_timezone/
