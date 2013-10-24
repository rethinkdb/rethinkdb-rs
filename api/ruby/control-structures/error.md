---
layout: api-command 
language: Ruby
permalink: api/ruby/error/
command: error 
github_doc: https://github.com/rethinkdb/docs/blob/docs/2-query-language/api/ruby/control-structures/error.md
---

# Command syntax #

{% apibody %}
r.error(message) &rarr; error
{% endapibody %}

# Description #

Throw a runtime error. If called with no arguments inside the second argument to `default`, re-throw the current error.

__Example:__ Iron Man can't possibly have lost a battle:

```rb
r.table('marvel').get('IronMan').do { |ironman|
    r.branch(ironman[:victories] < ironman[:battles],
    r.error('impossible code path'),
    ironman)
}.run(conn)
```
