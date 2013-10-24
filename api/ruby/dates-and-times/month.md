---
layout: api-command 
language: Ruby
permalink: api/ruby/month/
command: month
related_commands:
    now: now/
    time: time/
---

# Command syntax #

{% apibody %}
time.month() &rarr; number
{% endapibody %}

# Description #

Return the month of a time object as a number between 1 and 12. For your convenience, the terms r.january, r.february etc. are defined and map to the appropriate integer.

__Example:__ Retrieve all the users who were born in November.

```rb
r.table("users").filter{ |user|
    user["birthdate"].month().eq(11)
}
```


__Example:__ Retrieve all the users who were born in November.

```
r.table("users").filter{ |user|
    user["birthdate"].month().eq(r.november)
}
```

