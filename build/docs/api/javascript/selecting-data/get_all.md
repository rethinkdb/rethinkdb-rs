---
layout: api-command
language: JavaScript
permalink: api/javascript/get_all/
command: getAll
io:
    -   - table
        - selection
related_commands:
    get: get/
    between: between/
---

# Command syntax #

{% apibody %}
table.getAll([key, key2...], [, {index:'id'}]) &rarr; selection
{% endapibody %}

<img src="/assets/images/docs/api_illustrations/get-all.png" class="api_command_illustration" />

# Description #

Get all documents where the given value matches the value of the requested index.

__Example:__ Secondary index keys are not guaranteed to be unique so we cannot query via [get](/api/javascript/get/) when using a secondary index.

```js
r.table('marvel').getAll('man_of_steel', {index:'code_name'}).run(conn, callback)
```

__Example:__ Without an index argument, we default to the primary index. While `get` will either return the document or `null` when no document with such a primary key value exists, this will return either a one or zero length stream.

```js
r.table('dc').getAll('superman').run(conn, callback)
```

__Example:__ You can get multiple documents in a single call to `get_all`.

```js
r.table('dc').getAll('superman', 'ant man').run(conn, callback)
```

{% infobox %}
__Note:__ `getAll` does not perform any de-duplication. If you pass the same key more than once, the same document will be returned multiple times.
{% endinfobox %}

__Example:__ You can use [args](/api/javascript/args/) with `getAll` to retrieve multiple documents whose keys are in a list. This uses `getAll` to get a list of female superheroes, coerces that to an array, and then gets a list of villains who have those superheroes as enemies.

```js
r.do(
    r.table('heroes').getAll('f', {index: 'gender'})('id').coerceTo('array'),
    function(heroines) {
        return r.table('villains').getAll(r.args(heroines));
    }
).run(conn, callback)
```

Calling `getAll` with zero arguments&mdash;which could happen in this example if the `heroines` list had no elements&mdash;will return nothing, i.e., a zero length stream.

Secondary indexes can be used in extremely powerful ways with `getAll` and other commands; read the full article on [secondary indexes](/docs/secondary-indexes) for examples using boolean operations, `contains` and more.
