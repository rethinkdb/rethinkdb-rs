---
layout: api-command
language: Ruby
permalink: api/ruby/each/
command: each
related_commands:
    to_a: to_array/
    close (cursor): close-cursor/
---

# Command syntax #

{% apibody %}
cursor.each { ... }
array.each { ... }
feed.each { ... }
{% endapibody %}

# Description #

Lazily iterate over a result set one element at a time.

RethinkDB sequences can be iterated through via the Ruby [Enumerable][en] interface; use standard Ruby commands like `each` blocks to access each item in the sequence.

[en]: http://www.ruby-doc.org/core/Enumerable.html


__Example:__ Let's process all the elements!

```rb
cursor = r.table('users').run(conn)
cursor.each { |doc|
    process_row(doc)
}
```

__Example:__ Stop the iteration prematurely and close the connection manually.

```rb
cursor = r.table('users').run(conn)
cursor.each do |doc|
    ok = process_row(doc)
    if not ok
        cursor.close()
        break
    end
end

```
