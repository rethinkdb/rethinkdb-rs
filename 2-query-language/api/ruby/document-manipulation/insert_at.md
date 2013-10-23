---
layout: api-command 
language: Ruby
permalink: api/ruby/insert_at/
command: insert_at 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/document-manipulation/insert_at.md
related_commands:
    splice_at: splice_at/
    delete_at: delete_at/
    change_at: change_at/
---

# Command syntax #

{% apibody %}
array.insert_at(index, value) &rarr; array
{% endapibody %}

# Description #

Insert a value in to an array at a given index. Returns the modified array.

__Example:__ Hulk decides to join the avengers.

```rb
r.expr(["Iron Man", "Spider-Man"]).insert_at(1, "Hulk").run(conn)
```
