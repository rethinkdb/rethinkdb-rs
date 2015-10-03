---
layout: api-command
language: Java
permalink: api/javascript/during/
command: during
related_commands:
    now: now/
    time: time/
    inTimezone: in_timezone/
---

# Command syntax #

{% apibody %}
time.during(startTime, endTime[, {leftBound: "closed", rightBound: "open"}]) &rarr; bool
{% endapibody %}

# Description #

Return whether a time is between two other times. By default, this is inclusive of the start time and exclusive of the end time. Set `leftBound` and `rightBound` to explicitly include (`closed`) or exclude (`open`) that endpoint of the range.

__Example:__ Retrieve all the posts that were posted between December 1st, 2013
(inclusive) and December 10th, 2013 (exclusive).

```js
r.table("posts").filter(
    r.row('date').during(r.time(2013, 12, 1, "Z"), r.time(2013, 12, 10, "Z"))
).run(conn)
```


__Example:__ Retrieve all the posts that were posted between December 1st, 2013
(exclusive) and December 10th, 2013 (inclusive).

```js
r.table("posts").filter(
  r.row('date').during(r.time(2013, 12, 1, "Z"), r.time(2013, 12, 10, "Z"), {leftBound: "open", rightBound: "closed"})
).run(conn)
```

