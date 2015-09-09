---
layout: documentation
title: Data modeling in RethinkDB
docs_active: data-modeling
permalink: docs/data-modeling/
---

There are two ways to model relationships between documents in
RethinkDB:

- By using __embedded arrays__.
- By linking documents stored in __multiple tables__ (similar to
  traditional relational database systems).

Let's explore the advantages and disadvantages of each approach. We'll use
a simple blog database that stores information about authors and their
posts to demonstrate them.

{% toctag %}

<img alt="Data Modeling Illustration" class="api_command_illustration"
    src="/assets/images/docs/api_illustrations/data-modeling.png" />

# Using embedded arrays #

We can model the relationship between authors and posts by using
embedded arrays as follows. Consider this example document in the
table `authors`:

```json
{
  "id": "7644aaf2-9928-4231-aa68-4e65e31bf219",
  "name": "William Adama", "tv_show": "Battlestar Galactica",
  "posts": [
    {"title": "Decommissioning speech", "content": "The Cylon War is long over..."},
    {"title": "We are at war", "content": "Moments ago, this ship received..."},
    {"title": "The new Earth", "content": "The discoveries of the past few days..."}
  ]
}
```

The `authors` table contains a document for each author. Each document
contains information about the relevant author and a field `posts` with
an array of posts for that author. In this case the query to retrieve
all authors with their posts is simple:

```python
# Retrieve all authors with their posts
r.db("blog").table("authors").run()

# Retrieve a single author with her posts
r.db("blog").table("authors").get(AUTHOR_ID).run()
```

{% infobox %}

__Advantages of using embedded arrays:__

- Queries for accessing authors and posts tend to be simpler.
- The data is often colocated on disk. If
  you have a dataset that doesn't fit into RAM, data is loaded
  from disk faster.
- Any update to the authors document atomically
  updates both the author data and the posts data.

__Disadvantages of using embedded arrays:__

- Any operation on a document in the `author` table requires loading all posts into memory. As well, any update to the document requires rewriting the entire document to disk. At the moment, RethinkDB does not support modifying only the single value on disk. 
- Because of the previous limitation, it's best to keep the size of
  the `posts` array to no more than a few hundred documents.

{% endinfobox %}

# Linking documents in multiple tables #

You can use a relational data modeling technique and create two tables to store your data. A typical document in the `authors` table would look like this:

```json
{
  "id": "7644aaf2-9928-4231-aa68-4e65e31bf219",
  "name": "William Adama",
  "tv_show": "Battlestar Galactica"
}
```

A typical document in the `posts` table would look like this:

```json
{
  "id": "064058b6-cea9-4117-b92d-c911027a725a",
  "author_id": "7644aaf2-9928-4231-aa68-4e65e31bf219",
  "title": "Decommissioning speech",
  "content": "The Cylon War is long over..."
}
```

Every post contains an `author_id` field that links each post to its author. We can retrieve all posts for a given author as follows:

```python
# If we have a secondary index on `author_id` in the table `posts`
r.db("blog").table("posts").
  get_all("7644aaf2-9928-4231-aa68-4e65e31bf219", index="author_id").
  run()

# If we didn't build a secondary index on `author_id`
r.db("blog").table("posts").
  filter({"author_id": "7644aaf2-9928-4231-aa68-4e65e31bf219"}).
  run()
```

In a relational database, we'd use a `JOIN` here; in RethinkDB, we use the `eq_join` command. To get all posts along with the author information for William Adama:

```python
# In order for this query to work, we need to have a secondary index
# on the `author_id` field of the table `posts`.
r.db("blog").table("authors").getAll("7644aaf2-9928-4231-aa68-4e65e31bf219").eq_join(
    'id',
    r.db("blog").table("posts"),
    index='author_id'
).zip().run()
```

Note that the values for `author_id` correspond to the `id` field of
the author, which allows us to link the documents.

{% infobox %}

__Advantages of using multiple tables:__

- Operations on authors and posts don't require loading the data for
  every post for a given author into memory.
- There is no limitation on the number of posts, so this approach is
  more suitable for large amounts of data.

__Disadvantages of using multiple tables:__

- The queries linking the data between the authors and their posts
  tend to be more complicated.
- With this approach you cannot atomically update both the author data
  and and the posts data.

{% endinfobox %}

# Read more #

There's a separate article, [Table joins in RethinkDB](/docs/table-joins/), with much more information about the multiple-table approach, including how to do the ReQL equivalents of inner, outer and cross joins. If you aren't sure which schema to use, ask us on [Stack Overflow](http://stackoverflow.com/questions/ask) or join the `#rethinkdb` IRC channel on [Freenode](http://www.freenode.org/).
