---
layout: documentation
title: External API access
active: docs
docs_active: external-api-access
permalink: docs/external-api-access/
---

ReQL is the RethinkDB query language. It offers a very powerful and
convenient way to manipulate JSON documents. This document is a gentle
introduction to ReQL concepts. You don't have to read it to be
productive with RethinkDB, but it helps to understand some basics.

{% infobox info %}
<strong>Note:</strong> the following examples use the JavaScript
driver. See the [`r.http` API reference](/api/javascript/http/) for
documentation for other languages.
{% endinfobox %}

# Accessing external APIs #

You start using ReQL in your program similarly to how you'd use other
databases:

```python
import rethinkdb as r  # import the RethinkDB package
conn = r.connect()       # connect to the server on localhost and default port
```

# All ReQL queries are chainable #

In ReQL, you can chain commands at the end of other commands using the
`.` operator:

```python
# Get an iterable cursor to the `users` table (we've seen this above)
r.table('users').run(conn)
```

# Read More #

Browse the following resources to learn more about ReQL:

- [Lambda functions in RethinkDB](/blog/lambda-functions/)
- [Introduction to map/reduce](/docs/map-reduce/)
- [Introduction to Joins](/docs/table-joins/)
- [API Reference](/api/)
