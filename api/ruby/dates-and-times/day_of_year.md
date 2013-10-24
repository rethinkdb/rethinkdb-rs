---
layout: api-command 
language: Ruby
permalink: api/ruby/day_of_year/
command: day_of_year 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/dates-and-times/day_of_year.md
related_commands:
    now: now/
    time: time/
---

# Command syntax #

{% apibody %}
time.day_of_year() &rarr; number
{% endapibody %}

# Description #

Return the day of the year of a time object as a number between 1 and 366 (following ISO 8601 standard).

__Example:__ Retrieve all the users who were born the first day of a year.

```rb
r.table("users").filter{ |user|
    user["birthdate"].day_of_year().eq(1)
}
```


