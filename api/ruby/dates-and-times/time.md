---
layout: api-command 
language: Ruby
permalink: api/ruby/time/
command: time 
related_commands:
    now: now/
    epoch_time: epoch_time/
    iso8601: iso8601/
---

# Command syntax #

{% apibody %}
r.time(year, month, day[, hour, minute, second], timezone)
    &rarr; time
{% endapibody %}

# Description #

Create a time object for a specific time.

A few restrictions exist on the arguments:

- `year` is an integer between 1400 and 9,999.
- `month` is an integer between 1 and 12.
- `day` is an integer between 1 and 31.
- `hour` is an integer.
- `minutes` is an integer.
- `seconds` is a double. Its value will be rounded to three decimal places
(millisecond-precision).
- `timezone` can be `'Z'` (for UTC) or a string with the format `±[hh]:[mm]`.


__Example:__ Update the birthdate of the user "John" to November 3rd, 1986 UTC.

```rb
r.table("user").get("John").update(:birthdate => r.time(1986, 11, 3, 'Z')).run(conn)
```
