---
layout: api-command 
language: JavaScript
permalink: api/javascript/day_of_year/
command: dayOfYear 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/dates-and-times/dayOfYear.md
io:
    -   - time
        - number
related_commands:
    now: now/
    time: time/
---

{% apibody %}
time.day_of_year() &rarr; number
{% endapibody %}

Return the day of the year of a time object as a number between 1 and 366 (following ISO 8601 standard).

__Example:__ Retrieve all the users who were born the first day of a year.

```js
r.table("users").filter(
    r.row("birthdate").dayOfYear().eq(1)
)
```


