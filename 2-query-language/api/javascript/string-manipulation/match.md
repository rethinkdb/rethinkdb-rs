---
layout: api-command 
language: JavaScript
permalink: api/javascript/match/
command: match
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/javascript/string-manipulation/match.md
---

{% apibody %}
string.match(regexp) → array
{% endapibody %}

Match against a regular expression. Returns a match object containing the matched string,
that string's start/end position, and the capture groups. Accepts RE2 syntax
([https://code.google.com/p/re2/wiki/Syntax](https://code.google.com/p/re2/wiki/Syntax)).
You can enable case-insensitive matching by prefixing the regular expression with
`(?i)`. (See linked RE2 documentation for more flags.)

__Example:__ Get all users whose name starts with A.

```js
r.table('users').filter(function(row){return row('name').match("^A")}).run(conn, callback)
```

__Example:__ Parse out a name (returns "mlucy").

```js
r.expr('id:0,name:mlucy,foo:bar').match('name:(\w+)')('groups').nth(0)('str').run(conn, callback)
```


__Example:__ Fail to parse out a name (returns null).

```js
r.expr('id:0,foo:bar').match('name:(\w+)')('groups').nth(0)('str').run(conn, callback)
```

