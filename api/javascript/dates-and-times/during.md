---
layout: api-command 
language: JavaScript
permalink: api/javascript/during/
command: during
io:
    -   - time
        - bool
related_commands:
    now: now/
    time: time/
    inTimezone: in_timezone/
---

# Command syntax #

{% apibody %}
time.during(startTime, endTime[, options]) &rarr; bool
{% endapibody %}

# Description #

Return if a time is between two other times (by default, inclusive for the start,
exclusive for the end).

__Example:__ Retrieve all the posts that were posted between December 1st, 2013
(inclusive) and December 10th, 2013 (exclusive).

```js
r.table("posts").filter(
    r.row('date').during(r.time(2013, 12, 1, "Z"), r.time(2013, 12, 10, "Z"))
).run(conn, callback)
```


__Example:__ Retrieve all the posts that were posted between December 1st, 2013
(exclusive) and December 10th, 2013 (inclusive).

```js
r.table("posts").filter(
  r.row('date').during(r.time(2013, 12, 1, "Z"), r.time(2013, 12, 10, "Z"), {leftBound: "open", rightBound: "closed"})
).run(conn, callback)
```

