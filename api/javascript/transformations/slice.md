---
layout: api-command
language: JavaScript
permalink: api/javascript/slice/
command: slice
io:
    -   - sequence
        - stream
    -   - array
        - array
related_commands:
    skip: skip/
    limit: limit/
    nth: nth/
---

# Command syntax #

{% apibody %}
sequence.slice(startIndex[, endIndex, {left_bound:'closed', right_bound:'open'}]) &rarr; stream
array.slice(startIndex[, endIndex, {left_bound:'closed', right_bound:'open'}]) &rarr; array
{% endapibody %}

# Description #

Return the elements within a sequence within the specified range.

`slice` returns the range between `startIndex` and `endIndex`. If only `startIndex` is specified, `slice` returns the range from that index to the end of the sequence. Specify `left_bound` or `right_bound` as `open` or `closed` to indicate whether to include that endpoint of the range by default: `closed` returns that endpoint, while `open` does not. By default, `left_bound` is closed and `right_bound` is open, so the range `(10,13)` will return the tenth, eleventh and twelfth elements in the sequence.

**Example:** Return players 11-20 (index positions 10 through 19) in the amateur class.

```js
r.table('players').filter({class: 'amateur'}).slice(10, 20)
```

**Example:** Return all but the top three players who have a red flag.

```js
r.table('players').filter({flag: 'red'}).slice(3)
```

**Example:** Return holders of tickets `X` through `Y`, assuming tickets are numbered sequentially. We want to include ticket `Y`.

```js
r.table('users').orderBy('ticket').slice(x, y, {right_bound: 'closed'})
```

__Example:__ For this fight, we need heroes with a good mix of strength and agility.

```js
r.table('marvel').orderBy('strength').slice(5, 10).run(conn, callback)
```
