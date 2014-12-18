---
layout: api-command
language: Ruby
permalink: api/ruby/to_array/
command: to_a
related_commands:
    each: each/
    close (cursor): close-cursor/
---

# Command syntax #

{% apibody %}
cursor.to_a()
{% endapibody %}

# Description #

Retrieve all results as an array.

RethinkDB sequences can be iterated through via the Ruby [Enumerable][en] interface; to coerce a cursor into an array, use the Ruby `to_a()` command.

[en]: http://www.ruby-doc.org/core/Enumerable.html

__Example:__ For small result sets it may be more convenient to process them at once as an array.

```rb
cursor = r.table('users').run()
users = cursor.to_a()
process_results(users)
```

The equivalent query with an `each` block would be:

```rb
cursor = r.table('users').run()
cursor.each { |doc|
    process_results(doc)
}
```

__Note:__ Because a feed is a cursor that never terminates, using `to_a` with a feed will never return. Use [each](../each/) instead. See the [changes](/api/ruby/changes) command for more information on feeds.
