---
layout: api-command
language: Python
permalink: api/python/error/
command: error
---

# Command syntax #

{% apibody %}
r.error(message) &rarr; error
{% endapibody %}

# Description #

Throw a runtime error. If called with no arguments inside the second argument to `default`, re-throw the current error.

__Example:__ Iron Man can't possibly have lost a battle:

```py
r.table('marvel').get('IronMan').do(
    lambda ironman: r.branch(ironman['victories'] < ironman['battles'],
                             r.error('impossible code path'),
                             ironman)
).run(conn)
```


