---
layout: api-command 
language: Ruby
permalink: api/ruby/day_of_week/
command: day_of_week 
---

{% apibody %}
time.day_of_week() → number
{% endapibody %}

Return the day of week of a time object as a number between 1 and 7 (following ISO 8601
standard). For your convenience, the terms r.monday, r.tuesday etc. are defined and map
to the appropriate integer.

__Example:__ Return today's day of week.

```rb
r.now().day_of_week().run(conn)
```
