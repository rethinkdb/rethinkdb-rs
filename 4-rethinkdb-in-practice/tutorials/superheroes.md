---
layout: documentation
title: "Tutorial: playing with a superhero dataset (RethinkDB and Python)"
active: docs
permalink: docs/tutorials/superheroes/
---
In this tutorial we'll introduce using RethinkDB in Python by playing
with a superhero dataset. We'll show how to insert and retrieve
documents, query the database for specific data, and update documents
with new information.

## Prerequisites ##

Before following this tutorial you must have [RethinkDB installed and
running](/docs/install). You'll also need to [install the
RethinkDB Python library](/docs/install-drivers/python).

## Connecting ##

Let's now confirm the setup is correct and we can connect to the RethinkDB
instance. We will assume that RethinkDB is running on the same server and on
the default port (you can change the parameters passed to `connect`): 

```python
> import rethinkdb as r
> # connect and make the connection available to subsequent commands 
> r.connect('localhost', 28015).repl()
> print r.db_list().run()

[u'test']
```

The result lists the default database available in RethinkDB.

{% infobox %}
__Tips__: The `r.connect(...).repl()` function is useful when using 
the RethinkDB Python driver from interactive shells making available
the connection for all subsequent `run` calls.
{% endinfobox %}

## Databases and tables ##

### Accessing databases and tables ###

A single RethinkDB instance can host multiple database, but most of the time
you'll work with a single database at a time. RethinkDB comes with a default
database so you can quickly experiment with it:

```python
> test = r.db('test')
```

### Creating a Database and a Table ###

For your application you'll probably want to use a new database and define new tables:

```python
// creating a new database
> r.db_create('python_tutorial').run()

// creating a new table
> r.db('python_tutorial').table_create('heroes').run()
```

You can see the new database and table through the browser-based administrative
UI: http://localhost:8080/#tables.

![Python Tutorial Heroes table](/assets/images/docs/python-tutorial/python-tutorial-table.png)

Because we will continue to use the `heroes` table, let's save it as a reference for the next operations:

```python
> heroes = r.db('python_tutorial').table('heroes')
```

## Inserting Documents ##

RethinkDB stores data in JSON, so passing `dict`s from Python requires no additional conversions:


```python
> heroes.insert({
    "hero": "Wolverine", 
    "name": "James 'Logan' Howlett", 
    "magazine_titles": ["Amazing Spider-Man vs. Wolverine", "Avengers",
        "X-MEN Unlimited", "Magneto War", "Prime"],
    "appearances_count": 98
}).run()

{u'errors': 0,
 u'generated_keys': [u'c6677d9f-1740-4499-bf17-92f10cab30cf'],
 u'inserted': 1}
```

{% infobox %}
__Tips:__ As you can notice in the result, RethinkDB generates a unique ID for
the documents that do not provide one.
{% endinfobox %}

### Inserting Multiple Documents ###

You can also insert multiple documents at a time by passing `insert` an array of dicts:

```python
> heroes.insert([
    {
        "hero": "Magneto", 
        "name": "Max Eisenhardt", 
        "aka": ["Magnus", "Erik Lehnsherr", "Lehnsherr"],  
        "magazine_titles": ["Alpha Flight", "Avengers", "Avengers West Coast"],
        "appearances_count": 42
    },
    {   
        "hero": "Professor Xavier", 
        "name": "Charles Francis Xavier", 
        "magazine_titles": ["Alpha Flight", "Avengers", "Bishop", "Defenders"],
        "appearances_count": 72
    },
    {
        "hero": "Storm", 
        "name": "Ororo Monroe", 
        "magazine_titles": ["Amazing Spider-Man vs. Wolverine", "Excalibur",
            "Fantastic Four", "Iron Fist"],
        "appearances_count": 72
    }
]).run()

{u'errors': 0,
 u'generated_keys': [u'd7d5e949-3f71-4e21-b5b7-42b6e7048ea3', 
                     u'747c057e-8810-4479-a6b2-3c28d8057b48',
                     u'372fa6fe-17ec-494b-a926-0d99ba8ced43'],
 u'inserted': 3}
```

## Retrieving all documents ##

Even if we only inserted 4 documents, you can double check that by running
`heroes.count().run()`. Let's take a quick look at them:

```python
> heroes.run()

<rethinkdb.net.Cursor object at 0x15d4710>
```

{% infobox %}

__Tips__: If the table contains a large number of documents a query will not return all of them at once, which would saturate the network and/or require a lot of memory on the client. Instead the query will return the results in batches and fetch more data as needed.

{% endinfobox %}

## Retrieving a single document ##

Let's now retrieve a document by its ID:

```python
> heroes.get('d7d5e949-3f71-4e21-b5b7-42b6e7048ea3').run()

{u'aka': [u'Magnus', u'Erik Lehnsherr', u'Lehnsherr'],
 u'appearances_count': 42,
 u'hero': u'Magneto',
 u'name': u'Max Eisenhardt',
 u'id': u'e4bbd5e0-de9c-15ac-672c-d00b9f23a1f5',
 u'magazine_titles': [...]}
```

## Querying ##

RethinkDB supports a wide range of filters, so let's try a couple of different
ones. Firstly let's retrieve Professor Xavier by his character name:

```python
> heroes.filter({'name': 'Charles Francis Xavier'}).run()

<rethinkdb.net.Cursor object at 0x15dc910>
```

Next thing we can do is to **order** the characters based on the number of
magazines they've appeared in:

```python
> heroes.order_by(r.desc('appearances_count')).pluck('hero',
        'appearances_count').run()

<rethinkdb.net.Cursor object at 0x15dc450>
```

{% infobox %}

__Tips__: Ordering result ascending or descending can be done using `asc` and
`desc` respectively. 
The server-side operation `pluck` allows fetching only the specified
attributes of the result documents.

{% endinfobox %}

As you see only 1 of the characters has appeared in more than 90 magazines. 
This is something we could also verify with the query:

```python
> heroes.filter(r.row['appearances_count'] >= 90).pluck('hero',
        'name', 'appearances_count').run()
```

For the last query example let's retrieve the characters that appeared in a
specific magazine. This demonstrates using `filter` on a nested list in a
document:

```python
> heroes.filter(
    r.row['magazine_titles'].filter(
        lambda mag: mag == 'Amazing Spider-Man vs. Wolverine'
    ).count() > 0
).pluck('hero').run()
```

## Updating multiple documents ##

We'll finish this tutorial by appending a new magazine to each of the
characters and also updating their number of appearances:

```python
heroes.update({
    'appearances_count': r.row['appearances_count'] + 1,
    'magazine_titles': r.row['magazine_titles'].append(
        'The Fantastic RethinkDB')
}).run()

{u'errors': 0, u'skipped': 0, u'updated': 4}
```

{% infobox %}

__Tips:__ RethinkDB supports atomic updates at the document level. You can read more about about the [RethinkDB atomicity model](/docs/architecture/#how-does-the-atomicity-model-work).

{% endinfobox %}

## What's next ##

1. [Learn more about RethinkDB queries](/docs/introduction-to-reql/)
2. [Learn how to scale an application running on RethinkDB](/docs/administration-tools)
3. [Learn how to experiment and tune queries using the Data Explorer](/docs/tutorials/elections)
