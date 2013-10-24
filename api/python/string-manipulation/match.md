---
layout: api-command 
language: Python
permalink: api/python/match/
command: match
github_doc: https://github.com/rethinkdb/docs/edit/master/2-query-language/api/python/string-manipulation/match.md
---

# Command syntax #

{% apibody %}
string.match(regexp) &rarr; array
{% endapibody %}

# Description #

Match against a regular expression. Returns a match object containing the matched string,
that string's start/end position, and the capture groups. Accepts RE2 syntax
([https://code.google.com/p/re2/wiki/Syntax](https://code.google.com/p/re2/wiki/Syntax)).
You can enable case-insensitive matching by prefixing the regular expression with
`(?i)`. (See linked RE2 documentation for more flags.)

__Example:__ Get all users whose name starts with A.

```py
r.table('users').filter(lambda row:row['name'].match("^A")).run(conn)
```


__Example:__ Parse out a name (returns "mlucy").

```py
r.expr('id:0,name:mlucy,foo:bar').match('name:(\w+)')['groups'][0]['str'].run(conn)
```


__Example:__ Fail to parse out a name (returns null).

```py
r.expr('id:0,foo:bar').match('name:(\w+)')['groups'][0]['str'].run(conn)
```
