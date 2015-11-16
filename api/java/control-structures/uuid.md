---
layout: api-command
language: Java
permalink: api/java/uuid/
command: uuid
io:
    -   - r
        - string
---

# Command syntax #

{% apibody %}
r.uuid([string]) &rarr; string
{% endapibody %}

# Description #

Return a UUID (universally unique identifier), a string that can be used as a unique ID. If a string is passed to `uuid` as an argument, the UUID will be deterministic, derived from the string's SHA-1 hash.

__Example:__ Generate a UUID.

```java
r.uuid().run(conn);
```

```
"27961a0e-f4e8-4eb3-bf95-c5203e1d87b9"
```

__Example:__ Generate a UUID based on a string.

```java
r.uuid("slava@example.com").run(conn);
```

```
"90691cbc-b5ea-5826-ae98-951e30fc3b2d"
```
