---
layout: api-command 
language: Ruby
permalink: api/ruby/day_of_week/
command: day_of_week 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/dates-and-times/day_of_week.md
related_commands:
    
---

# Command syntax #

{% apibody %}
time.day_of_week() &rarr; number
{% endapibody %}

# Description #

Return the day of week of a time object as a number between 1 and 7 (following ISO 8601
standard). For your convenience, the terms r.monday, r.tuesday etc. are defined and map
to the appropriate integer.

__Example:__ Return today's day of week.

```rb
r.now().day_of_week().run(conn)
```


__Example:__ Retrieve all the users who were born on a Tuesday.

```
r.table("users").filter{ |user|
    user["birthdate"].day_of_week().eq(r.tuesday)
}
```

