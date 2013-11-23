---
layout: api-command
language: Ruby
permalink: api/ruby/now/
command: now
related_commands:
    time: time/
    epoch_time: epoch_time/
    iso8601: iso8601/
---

# Command syntax #

{% apibody %}
r.now() &rarr; time
{% endapibody %}

# Description #

Return a time object representing the current time in UTC. The command now() is computed once when the server receives the query, so multiple instances of r.now() will always return the same time inside a query.

__Example:__ Add a new user with the time at which he subscribed.

```rb
r.table("users").insert({
    :name => "John",
    :subscription_date => r.now()
}).run(conn)
```


