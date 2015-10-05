---
layout: api-command
language: Java
permalink: api/java/error/
command: error
io:
    -   - r
        - error
---

# Command syntax #

{% apibody %}
r.error(message) &rarr; error
{% endapibody %}

# Description #

Throw a runtime error. If called with no arguments inside the second argument to `default`, re-throw the current error.

__Example:__ Iron Man can't possibly have lost a battle:

```java
r.table('marvel').get('IronMan').do(function(ironman) {
    return r.branch(ironman('victories').lt(ironman('battles')),
        r.error('impossible code path'),
        ironman)
}).run(conn)
```


