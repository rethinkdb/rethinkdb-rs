---
layout: documentation
title: SQL to ReQL cheat sheet
docs_active: sql-to-reql
permalink: docs/sql-to-reql/javascript/
alias: docs/sql-to-reql/
switcher: true
language: JavaScript
---

<img alt="Data Modeling Illustration" class="api_command_illustration"
    src="/assets/images/docs/api_illustrations/SQL-to-ReQL-cheat-sheet.png" />

&nbsp;

# Terminology #

SQL and RethinkDB share very similar terminology. Below is a table
of terms and concepts in the two systems.

<table class="table-top-aligned">
    <thead>
        <tr>
            <th>SQL</th>
            <th>RethinkDB</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>database</td>
            <td>database</td>
        </tr>
        <tr>
            <td>table</td>
            <td>table</td>
        </tr>
        <tr>
            <td>row</td>
            <td>document</td>
        </tr>
        <tr>
            <td>column</td>
            <td>field</td>
        </tr>
        <tr>
            <td>table joins</td>
            <td>table joins</td>
        </tr>
        <tr>
            <td>primary key</td>
            <td>primary key (by default <code>id</code>)</td>
        </tr>
        <tr>
            <td>index</td>
            <td>index</td>
        </tr>
    </tbody>
</table>

# INSERT #

This is a list of queries for inserting data into a database.

<table class="table-top-aligned">
    <thead><tr><th>SQL</th><th>ReQL</th></tr></thead>
    <tbody>
        <tr>
            <td>
{% highlight sql %}
INSERT INTO users(user_id,
                  age,
                  name)
VALUES ("f62255a8259f",
        30,
        Peter)
{% endhighlight %}
            </td>
            <td>
{% highlight javascript %}
r.table("users").insert({
   userId: "f62255a8259f",
   age: 30,
   name: "Peter"
})
{% endhighlight %}
            </td>
        </tr>
    </tbody>
</table>


# SELECT #

This is a list of queries for selecting data out of a database.

<table class="table-top-aligned">
    <thead><tr><th>SQL</th><th>ReQL</th></tr></thead>
    <tbody>
        <tr><td>

{% highlight sql %}
SELECT * FROM users
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("users")
{% endhighlight %}

        </td></tr>
        <tr><td>

{% highlight sql %}
SELECT user_id, name FROM users
{% endhighlight %}
        </td><td>
{% highlight javascript %}
r.table("users")
 .pluck("userId", "name")
{% endhighlight %}

        </td></tr>
        <tr><td>

{% highlight sql %}
SELECT * FROM users
WHERE name = "Peter"
{% endhighlight %}
        </td><td>
{% highlight javascript %}
r.table("users").filter({
    name: "Peter"
})
{% endhighlight %}

<p>If you have a secondary index built on the field <code>name</code>, you can run a
more efficient query:</p>
{% highlight javascript %}
r.table("users")
    .getAll("Peter", {index: "name"})
{% endhighlight %}

        </td></tr>
        <tr><td>

{% highlight sql %}
SELECT * FROM users
WHERE name = "Peter"
AND age = 30
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("users").filter({
    name: "Peter",
    age: 30
})
{% endhighlight %}

        </td></tr>


        <tr><td>

{% highlight sql %}
SELECT * FROM users
WHERE name LIKE "P%"
{% endhighlight %}

        </td><td>
{% highlight javascript %}
r.table("users").filter(
    r.row("name").match("^P")
)
{% endhighlight %}

        </td></tr>
        <tr><td>

{% highlight sql %}
SELECT * FROM users
ORDER BY name ASC
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("users").orderBy("name")
{% endhighlight %}

        </td></tr>

        <tr><td>

{% highlight sql %}
SELECT * FROM users
ORDER BY name DESC
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("users").orderBy(
    r.desc("name")
)
{% endhighlight %}

        </td></tr>

        <tr><td>

{% highlight sql %}
SELECT user_id FROM users
WHERE name = "Peter"
ORDER BY name DESC
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("users").filter({
    name: "Peter"
}).orderBy(
    r.desc("name")
).pluck("userId")
{% endhighlight %}

        </td></tr>

        <tr><td>

{% highlight sql %}
SELECT * FROM users LIMIT 5 SKIP 10
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("users").skip(10).limit(5)
{% endhighlight %}

        </td></tr>

        <tr><td>

{% highlight sql %}
SELECT * FROM users
WHERE name IN ('Peter', 'John')
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("users").filter(
  function (doc) {
    return r.expr(["Peter","John"])
            .contains(doc("name"));
  }
)
{% endhighlight %}

<p>If you have a secondary index built on the field <code>name</code>, you can run a
more efficient query:</p>
{% highlight javascript %}
r.table("users")
    .getAll("Peter", "John",
        {index: "name"})
{% endhighlight %}

        </td></tr>

        <tr><td>

{% highlight sql %}
SELECT * FROM users
WHERE name NOT IN ('Peter', 'John')
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("users").filter(
  function (doc) {
    return r.expr(["Peter","John"])
            .contains(doc("name"))
            .not();
  }
)
{% endhighlight %}

        </td></tr>
        <tr><td>

{% highlight sql %}
SELECT COUNT(*) FROM users
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("users").count()
{% endhighlight %}

        </td></tr>

        <tr><td>

{% highlight sql %}
SELECT COUNT(name) FROM users
WHERE age > 18
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("users").filter(
    r.row.hasFields("name")
    .and(r.row("age").gt(18))
).count()
{% endhighlight %}

        </td></tr>
        <tr><td>

{% highlight sql %}
SELECT AVG("age")
    FROM users
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("users").avg("age")
{% endhighlight %}

        </td></tr>

        <tr><td>

{% highlight sql %}
SELECT MAX("age")
    FROM users
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("users")("age").max()
{% endhighlight %}

        </td></tr>

        <tr><td>

{% highlight sql %}
SELECT DISTINCT(name) FROM users
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("users").pluck("name")
 .distinct()
{% endhighlight %}

        </td></tr>
        <tr><td>

{% highlight sql %}
SELECT *
    FROM users
    WHERE age BETWEEN 18 AND 65;
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("users").filter(
    r.row("age").ge(18)
     .and(r.row("age").le(65))
)
{% endhighlight %}

If you have a secondary index built on the field <code>age</code>, you can run a
more efficient query:
{% highlight javascript %}
r.table("users")
 .between(18, 65, {index: "age"})
{% endhighlight %}

        </td></tr>

        <tr><td>

{% highlight sql %}
SELECT name, 'is_adult' = CASE
    WHEN age>18 THEN 'yes'
    ELSE 'no'
    END
FROM users
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("users").map({
    name: r.row("name"),
    is_adult: r.branch(
        r.row("age").gt(18),
        "yes",
        "no"
    )
})
{% endhighlight %}


        </td></tr>


        <tr><td>

{% highlight sql %}
SELECT *
  FROM posts
  WHERE EXISTS
    (SELECT * FROM users
     WHERE posts.author_id
         = users.id)
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("posts")
  .filter(function (post) {
    return r.table("users")
      .filter(function (user) {
        return user("id").eq(post("authorId"))
      }).count().gt(0)
    })
{% endhighlight %}

        </td></tr>





    </tbody>
</table>

# UPDATE #

This is a list of commands for updating data in the database.

<table class="table-top-aligned">
    <thead><tr><th>SQL</th><th>ReQL</th></tr></thead>
    <tbody>

        <tr><td>

{% highlight sql %}
UPDATE users
    SET age = 18
    WHERE age < 18
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("users").filter(
    r.row("age").lt(18)
).update({age: 18})
{% endhighlight %}


        </td></tr>

        <tr><td>

{% highlight sql %}
UPDATE users
    SET age = age+1
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("users").update(
    {age: r.row("age").add(1)}
)
{% endhighlight %}

        </td></tr>

    </tbody>
</table>

# DELETE #

This is a list of queries for deleting data from the database.

<table class="table-top-aligned">
    <thead><tr><th>SQL</th><th>ReQL</th></tr></thead>
    <tbody>

        <tr><td>
{% highlight sql %}
DELETE FROM users
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("users").delete()
{% endhighlight %}

        </td></tr>

        <tr><td>
{% highlight sql %}
DELETE FROM users
WHERE age < 18
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("users")
    .filter(r.row("age").lt(18))
    .delete()
{% endhighlight %}

        </td></tr>

    </tbody>
</table>



# JOINS #

This is a list of queries for performing joins between multiple
tables.

<table class="table-top-aligned">
    <thead><tr><th>SQL</th><th>ReQL</th></tr></thead>
    <tbody>

        <tr><td>
{% highlight sql %}
SELECT *
FROM posts
JOIN users
ON posts.user_id = users.id
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("posts").innerJoin(
  r.table("users"),
  function (post, user) {
    return post("userId").eq(user("id"));
}).zip()
{% endhighlight %}

<p><em>Note:</em> <code>zip()</code> will merge the user in the post, overwriting fields in case of conflict.</p>

<p>If you have an index (primary key or secondary index) built on the field of the right table, you can perform a more efficient join with <a href="/api/javascript/eq_join/">eqJoin</a>.</p>

{% highlight javascript %}
r.table("posts").eqJoin(
    "id",
    r.table("users"),
    {index: "id"}
).zip()
{% endhighlight %}

        </td></tr>

        <tr><td>
{% highlight sql %}
SELECT posts.id AS post_id,
       user.name,
       users.id AS user_id
    FROM posts
    JOIN users
        ON posts.user_id = users.id

SELECT posts.id AS post_id,
       user.name,
       users.id AS user_id
    FROM posts
    INNER JOIN users
        ON posts.user_id = users.id
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("posts").innerJoin(
  r.table("users"),
  function (post, user) {
    return post("userId").eq(user("id"));
}).map({
  postId: r.row("left")("id"),
  userId: r.row("right")("id"),
  name: r.row("right")("name")
})
{% endhighlight %}



        </td></tr>

        <tr><td>
{% highlight sql %}
SELECT *
    FROM posts
    RIGHT JOIN users
        ON posts.user_id = users.id

SELECT *
    FROM posts
    RIGHT OUTER JOIN users
        ON posts.user_id = users.id
{% endhighlight %}


        </td><td>

{% highlight javascript %}
r.table("posts").outerJoin(
  r.table("users"),
  function (post, user) {
    return post("userId").eq(user("id"));
}).zip()
{% endhighlight %}

<p><em>Note</em>: You can perform more efficient <code>OUTER JOIN</code> operations with the <a href="/api/javascript/concat_map/">concatMap</a> command.</p>

{% highlight javascript %}
r.table("posts").concatMap(
  function (post) {
    return r.table("users")
    .getAll(post("id"), {index: id})
    .do(
      function (result) {
        return r.branch(
          result.count().eq(0),
          [{left: post}],
          result.map(function (user) {
            return {
              left: post, right: user
            };
          })
        );
      }
    );
  }
).zip();
{% endhighlight %}

        </td></tr>

        <tr><td>
{% highlight sql %}
SELECT *
    FROM posts
    LEFT JOIN users
        ON posts.user_id = users.id
{% endhighlight %}
{% highlight sql %}
SELECT *
    FROM posts
    LEFT OUTER JOIN users
        ON posts.user_id = users.id
{% endhighlight %}


        </td><td>

{% highlight javascript %}
r.table("users").outerJoin(
  r.table("posts"),
  function (user, post) {
    return post("userId").eq(user("id"));
  }
).zip()
{% endhighlight %}

{% highlight javascript %}
r.table("users").concatMap(
  function (user) {
    return r.table("posts").getAll(user("id"), {index: "id"}).do(
      function (results) {
        return r.branch(
          results.count().eq(0),
          [{left: user}],
          results.map(function (post) {
            return {left: user, right: post};
          })
        );
      }
    );
  }
).zip()
{% endhighlight %}

        </td></tr>
    </tbody>
</table>


# AGGREGATIONS #

This is a list of queries for performing data aggregation.

<table class="table-top-aligned">
    <thead><tr><th>SQL</th><th>ReQL</th></tr></thead>
    <tbody>

        <tr><td>

{% highlight sql %}
SELECT category
    FROM posts
    GROUP BY category
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("posts").map(
    r.row("category")
).distinct()
{% endhighlight %}

        </td></tr>
        <tr><td>

{% highlight sql %}
SELECT category,
       SUM('num_comments')
    FROM posts
    GROUP BY category
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table('posts')
 .group('category')
 .sum('num_comments')
{% endhighlight %}

        </td></tr>

        <tr><td>

{% highlight sql %}
SELECT category,
       status,
       SUM('num_comments')
    FROM posts
    GROUP BY category, status
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("posts")
 .group('category', 'status')
 .sum('num_comments')
{% endhighlight %}
        </td></tr>

        <tr><td>

{% highlight sql %}
SELECT category,
       SUM(num_comments)
    FROM posts
    WHERE num_comments > 7
    GROUP BY category

{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("posts")
 .filter(r.row('num_comments').gt(7))
 .group('category')
 .sum('num_comments')
{% endhighlight %}

        </td></tr>

        <tr><td>

{% highlight sql %}
SELECT category,
       SUM(num_comments)
    FROM posts
    GROUP BY category
    HAVING num_comments > 7

{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("posts")
 .group('category')
 .sum('num_comments')
 .ungroup()
 .filter(r.row("reduction").gt(7))
{% endhighlight %}

        </td></tr>
        
        <tr><td>

{% highlight sql %}
SELECT title,
        COUNT(title)
    FROM movies
    GROUP BY title
    HAVING COUNT(title) > 1
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("movies")
 .group("title")
 .count()
 .ungroup()
 .filter(r.row("reduction").gt(1))
{% endhighlight %}

        </td></tr>

    </tbody>
</table>

# TABLE and DATABASE manipulation #

This is a list of queries for creating and dropping tables and
databases.

<table class="table-top-aligned">
    <thead><tr><th>SQL</th><th>ReQL</th></tr></thead>
    <tbody>

        <tr><td>

{% highlight sql %}
CREATE DATABASE my_database;
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.dbCreate('my_database')
{% endhighlight %}

        </td></tr>

        <tr><td>

{% highlight sql %}
DROP DATABASE my_database;
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.dbDrop('my_database')
{% endhighlight %}

        </td></tr>



        <tr><td>

{% highlight sql %}
CREATE TABLE users
    (id INT IDENTITY(1,1) PRIMARY KEY,
    name VARCHAR(50),
    age INT);
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.tableCreate('users', {primaryKey: "id"})
{% endhighlight %}
<p><em>Note:</em> RethinkDB is a NoSQL database and does not enforce
schemas.</p>

<p><em>Note:</em> The default primary key is <code>id</code></p>



        </td></tr>


        <tr><td>

{% highlight sql %}
TRUNCATE TABLE users;
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.table("users").delete()
{% endhighlight %}



        </td></tr>

        <tr><td>

{% highlight sql %}
DROP TABLE users;
{% endhighlight %}

        </td><td>

{% highlight javascript %}
r.tableDrop("users")
{% endhighlight %}

        </td></tr>

    </tbody>
</table>



# Read More #

Browse the following resources to learn more about ReQL:

- [Lambda functions in RethinkDB](/blog/lambda-functions/)
- [Introduction to map-reduce](/docs/map-reduce/)
- [Introduction to Joins](/docs/table-joins/)
- [API Reference](/api/)
