---
layout: api-command
language: Ruby
permalink: api/ruby/get_all/
command: get_all
related_commands:
    get: get/
    between: between/
    filter: filter/
---

# Command syntax #

{% apibody %}
table.get_all(key[, key2...], [, :index => 'id']) &rarr; selection
{% endapibody %}

<img src="/assets/images/docs/api_illustrations/get-all.png" class="api_command_illustration" />

# Description #

Get all documents where the given value matches the value of the requested index.

__Example:__ Secondary index keys are not guaranteed to be unique so we cannot query via [get](/api/ruby/get/) when using a secondary index.

```rb
r.table('marvel').get_all('man_of_steel', :index => 'code_name').run(conn)
```

__Example:__ Without an index argument, we default to the primary index. While `get` will either return the document or `nil` when no document with such a primary key value exists, this will return either a one or zero length stream.

```rb
r.table('dc').get_all('superman').run(conn)
```

__Example:__ You can get multiple documents in a single call to `get_all`.

```rb
r.table('dc').get_all('superman', 'ant man').run(conn)
```

__Example:__ You can use [args](/api/ruby/args/) with `get_all` to retrieve multiple documents whose keys are in a list. This uses `get_all` to get a list of female superheroes, coerces that to an array, and then gets a list of villains who have those superheroes as enemies.

```rb
r.table('heroes').get_all('f', :index => 'gender')['id'].coerce_to('array').do {
    |heroines| r.table('villains').get_all(r.args(heroines))
}.run(conn)
```
