---
layout: api-command 
language: Ruby
permalink: api/ruby/timezone/
command: timezone 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/dates-and-times/timezone.md
related_commands:
    in_timezone: in_timezone/
    now: now/
    time: time/
---

# Command syntax #

{% apibody %}
time.timezone() &rarr; string
{% endapibody %}

# Description #

Return the timezone of the time object.

__Example:__ Return all the users in the "-07:00" timezone.

```rb
r.table("users").filter{ |user|
    user["subscriptionDate"].timezone().eq("07:00")
}
```
