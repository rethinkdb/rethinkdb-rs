---
layout: documentation
title: Table joins in RethinkDB
docs_active: table-joins
permalink: docs/table-joins/
---

{% infobox %}
__Wondering how to model your data?__ Read about [data modeling in RethinkDB](/docs/data-modeling).
{% endinfobox %}

Like many traditional database systems, RethinkDB supports `JOIN`
commands to combine data from multiple tables. In RethinkDB joins are
automatically distributed&mdash;a join command is automatically sent
to the appropriate nodes across the cluster, the relevant data is
combined, and the final result is presented to the user.

Let's see how we can use joins in RethinkDB to query data based on
__one to many__, and __many to many__ relations.

{% toctag %}

<img src="/assets/images/docs/api_illustrations/table-joins.png"
     alt="Table Join Illustration"
     class="api_command_illustration" />

# One to many relations #

## Using primary keys ##

Let's suppose we've created two tables: `employees` and
`companies`. We'll use these tables to model the notion of people
working for organizations (each organization has multiple people
working for it, but any given person works at a single
organization). Here's an example document in the `employees` table:

```json
{
    "id": "543ad9c8-1744-4001-bb5e-450b2565d02c",
    "name": "Jean-Luc Picard",
    "company_id": "064058b6-cea9-4117-b92d-c911027a725a",
    "rank": "captain"
}
```

And here's an example document in the `companies` table:

```json
{
    "id": "064058b6-cea9-4117-b92d-c911027a725a",
    "company": "Starfleet",
    "type": "paramilitary"
}
```

We can join the two tables as follows:

```python
r.table("employees").eq_join("company_id", r.table("companies")).run()
```

This query joins the `company_id` of the employee table with the
primary key of the company table. It returns a sequence of documents
where each document contains two fields&mdash;the employee
information and the company information:

```json
{
    "left": {
        "id": "543ad9c8-1744-4001-bb5e-450b2565d02c",
        "name": "Jean-Luc Picard",
        "company_id": "064058b6-cea9-4117-b92d-c911027a725a",
        "rank": "captain"
    },
    "right": {
        "id": "064058b6-cea9-4117-b92d-c911027a725a",
        "company": "Starfleet",
        "type": "paramilitary"
    }
}
```

* The field `left` contains the information from the left table in the
  query (in this case, the employee)
* The field `right` contains the information from the right table in
  the query (in this case, the company)

We can chain the `zip` command at the end of the query to merge the
two fields into a single document. For example, the following query:

```python
r.table("employees").eq_join("company_id", r.table("companies")).zip().run()
```

Returns the following result:

```json
{
    "id": "064058b6-cea9-4117-b92d-c911027a725a",
    "name": "Jean-Luc Picard",
    "company_id": "064058b6-cea9-4117-b92d-c911027a725a",
    "rank": "captain",
    "company": "Starfleet",
    "type": "paramilitary"
}
```

## Using subqueries ##

A common data access task is retrieving one document with associated "child" documents. (This would often be in a one-to-many relationship as shown here, but could be a many-to-many or one-to-one relationship.) In our example data set, we might want to retrieve information about a company and all its employees. We can do this in one ReQL command using `merge` and a subquery in its lambda function.

```py
id = "064058b6-cea9-4117-b92d-c911027a725a"
r.table("companies").get(id).merge(lambda company:
    { 'employees': r.table('employees').get_all(company['id'],
                           index='company_id').coerce_to('array') }
).run()
```

This will return a result similar to:

```json
{
    "id": "064058b6-cea9-4117-b92d-c911027a725a",
    "company": "Starfleet",
    "type": "paramilitary",
    "employees": [
        {
            "id": "543ad9c8-1744-4001-bb5e-450b2565d02c",
            "name": "Jean-Luc Picard",
            "company_id": "064058b6-cea9-4117-b92d-c911027a725a",
            "rank": "captain"
        },
        ...
    ]
}
```

Where `eq_join` produces a table-like result (the rough equivalent of SQL's `SELECT * FROM companies, employees WHERE companies.id = employees.company_id`), using a subquery produces a nested document, where the employee objects are returned in a list in the `employees` field.

## Using secondary indexes ##

Suppose that our data model for the employees stores a company name
instead of a company id:

```json
{
    "id": "543ad9c8-1744-4001-bb5e-450b2565d02c",
    "name": "Jean-Luc Picard",
    "company_name": "Starfleet",
    "rank": "captain"
}
```

We can create a secondary index on the `company` field of the
`companies` table, and perform our query by taking advantage of the
secondary index:

```python
r.table("companies").index_create("company").run()
```

The query would look like this:

```python
r.table("employees").eq_join("company_name",
                             r.table("companies"), index="company").run()
```

{% infobox %}
__Want to learn more about indexes?__: Read about [using secondary indexes in RethinkDB](/docs/secondary-indexes/).
{% endinfobox %}

{% infobox %}
__Note__: you can also join tables on arbitrary fields without
creating an index using the [inner_join](/api/python/inner_join/)
command. However, arbitrary inner joins are less efficient then
equijoins.
{% endinfobox %}

# Many to many relations #

You can also use RethinkDB to query many to many relations. Let's
suppose we have a collaborative blogging platform where authors
collaborate to create posts (multiple authors can work on any given
post, and publish multiple posts).

In order to model this data we'd create three tables&mdash;`authors`, `posts` and `authors_posts`, similarly to how we'd do it in a
relational system. Here is example data for the `authors` table:

```json
{
  "id": "7644aaf2-9928-4231-aa68-4e65e31bf219",
  "name": "William Adama",
  "tv_show": "Battlestar Galactica"
}
{
  "id": "064058b6-cea9-4117-b92d-c911027a725a",
  "name": "Laura Roslin",
  "tv_show": "Battlestar Galactica"
}
```

Here is example data for the `posts` table:

```json
{
    "id": "543ad9c8-1744-4001-bb5e-450b2565d02c",
    "title": "Decommissioning speech",
    "content": "The Cylon War is long over..."
}
```

And here is example data for the `authors_posts` table:

```json
{
    "author_id": "7644aaf2-9928-4231-aa68-4e65e31bf219",
    "post_id": "543ad9c8-1744-4001-bb5e-450b2565d02c"
}
{
    "author_id": "064058b6-cea9-4117-b92d-c911027a725a",
    "post_id": "543ad9c8-1744-4001-bb5e-450b2565d02c"
}
```

In a many to many relation, we can use multiple `eq_join` commands to join
the data from all three tables:

```python
r.table("authors_posts").eq_join("author_id", r.table("authors")).zip().
  eq_join("post_id", r.table("posts")).zip().run()
```

The result of this query is a stream of documents that includes every
post written by every author in our database:

```json
{
    "tv_show": "Battlestar Galactica",
    "title": "Decommissioning speech",
    "post_id": "543ad9c8-1744-4001-bb5e-450b2565d02c",
    "name": "William Adama",
    "id": "543ad9c8-1744-4001-bb5e-450b2565d02c",
    "content": "The Cylon War is long over...",
    "author_id": "7644aaf2-9928-4231-aa68-4e65e31bf219"
}
{
    "tv_show": "Battlestar Galactica",
    "title": "Decommissioning speech",
    "post_id": "543ad9c8-1744-4001-bb5e-450b2565d02c",
    "name": "Laura Roslin",
    "id": "543ad9c8-1744-4001-bb5e-450b2565d02c",
    "content": "The Cylon War is long over...",
    "author_id": "064058b6-cea9-4117-b92d-c911027a725a"
}
```

# Resolving field name conflicts #

If you use the `zip` command after `join`, the document from the right
table will be merged into the left one.

Consider the following query:

```py
r.table("employees").eq_join("company_id", r.table("companies"))
```

Suppose its output is as follows:

```py
{
    # Employee
    "left": {
        "id": "543ad9c8-1744-4001-bb5e-450b2565d02c",
        "name": "Jean-Luc Picard",
        "company_id": "064058b6-cea9-4117-b92d-c911027a725a",
        "rank": "captain"
    },
    # Company
    "right": {
        "id": "064058b6-cea9-4117-b92d-c911027a725a",
        "company": "Starfleet",
        "type": "paramilitary"
    }
}
```


The conflicting field is `id`. If you directly use the `zip` command,
the `id` field of the result will be the one from the company. There
are three ways to resolve potential field conflicts.

## Removing the conflicting fields ##

Suppose that you want to keep the `id` field of the employee, but not
the one of the company.  You can do it by removing the field
`right.id`, then calling the `zip` command.

```py
r.table("employees").eq_join("company_id", r.table("companies"))
    .without({"right": {"id": True}}) # Remove the field right.id
    .zip()
    .run()
```


## Renaming the fields ##

If you need to keep both fields, you can rename them with `map` and
`without` before using the `zip` command.

```py
r.table("employees").eq_join("company_id", r.table("companies"))
    # Copy the field right.id into right.c_id
    .map( r.row.merge({
        "right": {
            "c_id": r.row["right"]["id"]
        }
    }))
    # Remove the field right.id
    .without({"right": {"id": True}})
    .zip()
    .run()
```

## Manually merge the left and right fields ##

You can manually merge the `left` and `right` fields without using the
`zip` command. Suppose you want to keep the name of the employee and
the name of his company. You can do:

```py
r.table("employees").eq_join("company_id", r.table("companies"))
    .map({
        "name": r.row["left"]["name"],
        "company": r.row["right"]["company"]
    }).run()
```

# Read more #

To learn more, read about [data modeling in RethinkDB](/docs/data-modeling/). For detailed information, take
a look at the API documentation for the join commands:

- [eq_join](/api/python/eq_join/)
- [inner_join](/api/python/inner_join/)
- [outer_join](/api/python/outer_join/)
- [zip](/api/python/zip/)
