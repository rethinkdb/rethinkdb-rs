---
layout: api-command 
language: Ruby
permalink: api/ruby/r/
command: r
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/accessing-rql/r.md
---

# Command syntax #

{% apibody %}
r &rarr; r
{% endapibody %}

# Description #

The toplevel RQL namespace.

__Example:__ Setup your top level namespace.

```rb
require 'rethinkdb'
include RethinkDB::Shortcuts
```

