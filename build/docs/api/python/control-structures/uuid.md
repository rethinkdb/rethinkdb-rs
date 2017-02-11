---
layout: api-command
language: Python
permalink: api/python/uuid/
command: uuid
---

# Command syntax #

{% apibody %}
r.uuid([string]) &rarr; string
{% endapibody %}

# Description #

Return a UUID (universally unique identifier), a string that can be used as a unique ID. If a string is passed to `uuid` as an argument, the UUID will be deterministic, derived from the string's SHA-1 hash.

RethinkDB's UUIDs are standards-compliant. Without the optional argument, a version 4 random UUID will be generated; with that argument, a version 5 UUID will be generated, using a fixed namespace UUID of `91461c99-f89d-49d2-af96-d8e2e14e9b58`. For more information, read [Wikipedia's UUID article][uu].

[uu]: https://en.wikipedia.org/wiki/Universally_unique_identifier

__Example:__ Generate a UUID.

```py
> r.uuid().run(conn)

"27961a0e-f4e8-4eb3-bf95-c5203e1d87b9"
```

__Example:__ Generate a UUID based on a string.

```py
> r.uuid("slava@example.com").run(conn)

"90691cbc-b5ea-5826-ae98-951e30fc3b2d"
```
