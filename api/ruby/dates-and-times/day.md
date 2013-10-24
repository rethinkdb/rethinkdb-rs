---
layout: api-command 
language: Ruby
permalink: api/ruby/day/
command: day 
related_commands:
    now: now/
    time: time/
---

# Command syntax #

{% apibody %}
time.day() &rarr; number
{% endapibody %}

# Description #

Return the day of a time object as a number between 1 and 31.

__Example:__ Return the users born on the 24th of any month.

```rb
r.table("users").filter{ |user|
    user["birthdate"].day().eq(24)
}
```


