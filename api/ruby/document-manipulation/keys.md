---
layout: api-command 
language: Ruby
permalink: api/ruby/keys/
command: keys 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/document-manipulation/keys.md
related_commands:
    insert_at: insert_at/
    delete_at: delete_at/
    splice_at: splice_at/
---

# Command syntax #

{% apibody %}
singleSelection.keys() &rarr; array
object.keys() &rarr; array
{% endapibody %}

# Description #

Return an array containing all of the object's keys.

__Example:__ Get all the keys of a row.

```rb
r.table('marvel').get('ironman').keys.run(conn)
```


