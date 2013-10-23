---
layout: api-command 
language: Ruby
permalink: api/ruby/filter/
command: filter 
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/ruby/selecting-data/filter.md
related_commands:
    get: get/
    get_all: get_all/
    between: between/
---


# Command syntax #

{% apibody %}
sequence.filter(predicate) &rarr; selection
stream.filter(predicate) &rarr; stream
array.filter(predicate) &rarr; array
{% endapibody %}

# Description #

Get all the documents for which the given predicate is true.

filter can be called on a sequence, selection, or a field containing an array of
elements. The return type is the same as the type on which the function was called on.
The body of every filter is wrapped in an implicit `.default(false)`, and the default
value can be changed by passing the optional argument `default`. Setting this optional
argument to `r.error()` will cause any non-existence errors to abort the filter.

__Example:__ Get all active users aged 30.

```rb
r.table('users').filter('active' => true, 'profile' => {'age' => 30}).run(conn)
```


__Example:__ Filter supports the r.literal syntax if you want to get an exact match.

```rb
r.table('users').filter('active' => true, 'profile' => r.literal({'age' => 30})).run(conn)
```


__Example:__ Select all documents where the 'magazines' field is greater than 5.

```rb
r.table('users').filter{|user| user[:magazines] > 5}.run(conn)
```


__Example:__ Select all documents where the 'abilities' embedded document has an
attribute called 'super-strength'.

```rb
r.table('marvel').filter {|hero|
    hero[:abilities].has_fields('super-strength')
}.run(conn)
```


__Example:__ Select all documents where the field 'powers' containing an array has an
element equal to 10.

```rb
r.table('marvel').filter { |hero|
    hero[:powers].filter { |power_rank| power_rank.eq(10) }.count() > 0
}.run(conn)
```

