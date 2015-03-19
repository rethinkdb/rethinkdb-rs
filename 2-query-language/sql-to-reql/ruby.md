---
layout: documentation
title: SQL to ReQL cheat sheet
active: docs
docs_active: sql-to-reql
permalink: docs/sql-to-reql/ruby/
switcher: true
language: Ruby
---

<img alt="Data Modeling Illustration" class="api_command_illustration"
    src="/assets/images/docs/api_illustrations/SQL-to-ReQL-cheat-sheet.png" />

&nbsp;

# Terminology #

SQL and RethinkDB share very similar terminology. Below is a table
of terms and concepts in the two systems.

<table class="table-2-columns">
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

<table class="table-2-columns">
    <thead><tr><th>SQL</th><th>ReQL</th></tr></thead>
    <tbody>
        <tr>
            <td>
<pre>
INSERT INTO users(user_id,
                  age,
                  name)
VALUES ("f62255a8259f",
        30,
        Peter)
</pre>
            </td>
            <td>
<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/insert/">insert</a>({
   :user_id => "f62255a8259f",
   :age => 30,
   :name => "Peter"
})
</pre>
            </td>
        </tr>
    </tbody>
</table>

# SELECT #

This is a list of queries for selecting data out of a database.

<table class="table-2-columns">
    <thead><tr><th>SQL</th><th>ReQL</th></tr></thead>
    <tbody>
        <tr><td>

<pre>
SELECT * FROM users
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users")
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT user_id, name FROM users
</pre>
        </td><td>
<pre>
r.<a href="/api/ruby/table/">table</a>("users")
 .<a href="/api/ruby/pluck/">pluck</a>("user_id", "name")
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT * FROM users
WHERE name = "Peter"
</pre>
        </td><td>
<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/filter/">filter</a>({
    :name => "Peter"
})
</pre>

<p>An alternative is to use a block:</p>
<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/filter/">filter</a>{ |doc|
    doc["name"] <a href="/api/ruby/eq/">==</a> "Peter"
}
</pre>

<p>If you have a secondary index built on the field <code>name</code>, you can run a
more efficient query:
<pre>
r.<a href="/api/ruby/table/">table</a>("users")
    .<a href="/api/ruby/get_all/">get_all</a>("Peter", :index => "name")
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT user_id, name FROM users
WHERE name = "Peter"
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/filter/">filter</a>({
    :name => "Peter"
}).<a href="/api/ruby/pluck/">pluck</a>("user_id", "name")
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT * FROM users
WHERE name = "Peter"
AND age = 30
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/filter/">filter</a>({
    :name => "Peter",
    :age => 30
})
</pre>

        </td></tr>


        <tr><td>

<pre>
SELECT * FROM users
WHERE age &gt; 30
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/filter/">filter</a>{ |row|
    row['age'] <a href="/api/ruby/gt/">&gt;</a>(30)
}
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT * FROM users
WHERE name LIKE "P%"
</pre>

        </td><td>
<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/filter/">filter</a>{ |row|
    row['name'].<a href="/api/ruby/match/">match</a>("^P")
}
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT * FROM users
WHERE name LIKE "%er"
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/filter/">filter</a>{ |row|
    row['name'].<a href="/api/ruby/match/">match</a>("er$")}
)
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT * FROM users
ORDER BY name ASC
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/order_by/">order_by</a>("name")
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT * FROM users
ORDER BY name DESC
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/order_by/">order_by</a>(
    r.desc("name")
)
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT user_id FROM users
WHERE name = "Peter"
ORDER BY name DESC
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/filter/">filter</a>({
    :name => "Peter"
}).<a href="/api/ruby/order_by/">order_by</a>(
    r.desc("name")
).<a href="/api/ruby/pluck/">pluck</a>("user_id")
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT * FROM users LIMIT 5
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/limit/">limit</a>(5)
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT * FROM users LIMIT 5 SKIP 10
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/skip/">skip</a>(10).<a href="/api/ruby/limit/">limit</a>(5)
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT * FROM users
WHERE name IN ('Peter', 'John')
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/filter/">filter</a>{ |doc|
    r.<a href="/api/ruby/expr/"</a>expr</a>(["Peter", "John"])
        .<a href="/api/ruby/contains">contains</a>(doc["name"])
}
</pre>

<p>If you have a secondary index built on the field <code>name</code>, you can run a
more efficient query:
<pre>
r.<a href="/api/ruby/table/">table</a>("users")
    .<a href="/api/ruby/get_all/">get_all</a>("Peter", "John",
        :index => "name")
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT * FROM users
WHERE name NOT IN ('Peter', 'John')
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/filter/">filter</a>{ |doc|
    r.<a href="/api/ruby/expr/"</a>expr</a>(["Peter", "John"])
        .<a href="/api/ruby/contains/">contains</a>(doc["name"])
        .not()
}
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT COUNT(*) FROM users
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/count/">count</a>()
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT COUNT(name) FROM users
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/filter/">filter</a>{ |doc|
    doc.<a href="/api/ruby/has_fields/">has_fields</a>("name")
}.<a href="/api/ruby/count/">count</a>()
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT COUNT(name) FROM users
WHERE age &gt; 18
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/filter/">filter</a>{ |doc|
    (doc.<a href="/api/ruby/has_fields/">has_fields</a>("name") & doc["age"] <a href="/api/ruby/gt/">></a> 18)
}.<a href="/api/ruby/count/">count</a>()
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT AVG("age")
    FROM users
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users")
 .<a href="/api/ruby/avg/">avg</a>("age")
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT MAX("age")
    FROM users
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users")["age"]
 .<a href="/api/ruby/max/">max</a>()
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT MIN("age")
    FROM users
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users")["age"]
 .<a href="/api/ruby/min/">min</a>()
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT SUM("num_posts")
    FROM users
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users")
 .<a href="/api/ruby/sum/">sum</a>("num_posts")
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT DISTINCT(name) FROM users
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/pluck/">pluck</a>("name").<a href="/api/ruby/distinct/">distinct</a>()
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT *
    FROM users
    WHERE age BETWEEN 18 AND 65;
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/filter/">filter</a>{ |doc|
    (doc["age"] <a href="/api/ruby/ge/">>=</a> 18) & (doc["age"] <a href="/api/ruby/le/"><=</a> 65)
}
</pre>

If you have a secondary index built on the field <code>age</code>, you can run a
more efficient query:
<pre>
r.<a href="/api/ruby/table/">table</a>("users")
    .<a href="/api/ruby/between/">between</a>(18, 65, :index => "age")
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT name, 'is_adult' = CASE
    WHEN age>18 THEN 'yes'
    ELSE 'no'
    END
FROM users
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/map/">map</a>{ |user|
    {
        :name => user["name"],
        :is_adult => r.<a href="/api/ruby/branch/">branch</a>(
            user["age"] <a href="/api/ruby/gt/">&gt;</a> 18
            "yes",
            "no"
        )
    }
}
</pre>

        </td></tr>


        <tr><td>

<pre>
SELECT *
  FROM posts
  WHERE EXISTS
    (SELECT * FROM users
     WHERE posts.author_id
         = users.id)
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("posts")
  .<a href="/api/ruby/filter/">filter</a>{ |post|
    r.<a href="/api/ruby/table/">table</a>("users")
      .<a href="/api/ruby/filter/">filter</a>{ |user|
        user.id <a href="/api/ruby/eq/">==</a> post.author_id
      }.<a href="/api/ruby/count/">count</a>() <a href="/api/ruby/gt/">&gt;</a> 0
    }
</pre>

        </td></tr>





    </tbody>
</table>

# UPDATE #

This is a list of commands for updating data in the database.

<table class="table-2-columns">
    <thead><tr><th>SQL</th><th>ReQL</th></tr></thead>
    <tbody>

        <tr><td>

<pre>
UPDATE users
    SET age = 18
    WHERE age &lt; 18
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/filter/">filter</a>{ |doc|
    doc["age"] < 18
}.<a href="/api/ruby/update/">update</a>({
    :age => 18
})
</pre>

        </td></tr>

        <tr><td>

<pre>
UPDATE users
    SET age = age+1
</pre>

        </td><td>


<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/update/">update</a>{ |doc|
    { :age => doc["age"]+1 }
}
</pre>

        </td></tr>

    </tbody>
</table>

# DELETE #

This is a list of queries for deleting data from the database.

<table class="table-2-columns">
    <thead><tr><th>SQL</th><th>ReQL</th></tr></thead>
    <tbody>

        <tr><td>
<pre>
DELETE FROM users
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/delete/">delete</a>()
</pre>

        </td></tr>

        <tr><td>
<pre>
DELETE FROM users
WHERE age &lt; 18
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/filter/">filter</a>{ |doc|
    doc["age"] < 18
}.<a href="/api/ruby/delete/">delete</a>()
</pre>

        </td></tr>

    </tbody>
</table>



# JOINS #

This is a list of queries for performing joins between multiple
tables.

<table class="table-2-columns">
    <thead><tr><th>SQL</th><th>ReQL</th></tr></thead>
    <tbody>

        <tr><td>
<pre>
SELECT *
FROM posts
JOIN users
ON posts.user_id = users.id
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("posts").<a href="/api/ruby/inner_join/">inner_join</a>(
    r.<a href="/api/ruby/table/">table</a>("users")
) { |post, user|
    post["user_id"] <a href="/api/ruby/eq/">==</a> user["id"]
}.zip()
</pre>

<p><em>Note:</em> <code>zip()</code> will merge the user in the post, overwriting fields in case of conflict.</p>

<p>If you have an index (primary key or secondary index) built on the field of the right table, you can perform a more efficient join with <a href="/api/ruby/eq_join/">eq_join</a>.</p>

<pre>
r.<a href="/api/ruby/table/">table</a>("posts").<a href="/api/ruby/eq_join/">eq_join</a>("id",
    r.<a href="/api/ruby/table/">table</a>("users"),
    :index => "id"
).<a href="/api/ruby/zip/">zip</a>()
</pre>

        </td></tr>

        <tr><td>
<pre>
SELECT posts.id AS post_id,
       user.name,
       users.id AS user_id
    FROM posts
    JOIN users
        ON posts.user_id = users.id
</pre>

<pre>
SELECT posts.id AS post_id,
       user.name,
       users.id AS user_id
    FROM posts
    INNER JOIN users
        ON posts.user_id = users.id
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("posts").<a href="/api/ruby/inner_join/">inner_join</a>(
  r.<a href="/api/ruby/table/">table</a>("users")
) { |post, user|
    post["user_id"] <a href="/api/ruby/eq/">==</a> user["id"]
}.<a href="/api/ruby/map/">map</a> { |post, user|
  :post_id => post["id"],
  :user_id => user["id"],
  :name => user["name"]
}
</pre>



        </td></tr>

        <tr><td>
<pre>
SELECT *
    FROM posts
    RIGHT JOIN users
        ON posts.user_id = users.id
</pre>

<pre>
SELECT *
    FROM posts
    RIGHT OUTER JOIN users
        ON posts.user_id = users.id
</pre>


        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("posts").<a href="/api/ruby/outer_join/">outer_join</a>(
    r.<a href="/api/ruby/table/">table</a>("users")
) { |post, user|
        post["user_id"] <a href="/api/ruby/eq/">==</a> user["id"]
}.<a href="/api/ruby/zip/">zip</a>()
</pre>

<p><em>Note</em>: You can perform more efficient <code>OUTER JOIN</code> operations with the <a href="/api/ruby/concat_map/">concat_map</a> command.</p>

<pre>
r.<a href="/api/ruby/table/">table</a>("posts").<a href="/api/ruby/concat_map/">concat_map</a>{ |post|
  r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/get_all/">get_all</a>(
    post["id"], :index => "id"
  ).<a href="/api/ruby/do/">do</a>{ |results| r.branch(
    results.<a href="/api/ruby/count/">count</a>() <a href="/api/ruby/eq/">==</a> 0,
    [{:left => post}],
    results.<a href="/api/ruby/map/">map</a> { |user|
      {:left => post, :right => user}
    }
  )}
}.zip()
</pre>

        </td></tr>

        <tr><td>
<pre>
SELECT *
    FROM posts
    LEFT JOIN users
        ON posts.user_id = users.id
</pre>
<pre>
SELECT *
    FROM posts
    LEFT OUTER JOIN users
        ON posts.user_id = users.id
</pre>


        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("posts").<a href="/api/ruby/outer_join/">outer_join</a>(
    r.<a href="/api/ruby/table/">table</a>("users")
) { |user, post|
        post["user_id"] <a href="/api/ruby/eq/">==</a> user["id"]
}.<a href="/api/ruby/zip/">zip</a>()
</pre>

<pre>
r.<a href="/api/ruby/table/">table</a>("posts").<a href="/api/ruby/concat_map/">concat_map</a>{ |post|
  r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/get_all/">get_all</a>(
    post["id"], :index => "id"
  ).<a href="/api/ruby/do/">do</a>{ |results| r.branch(
    results.<a href="/api/ruby/count/">count</a>() <a href="/api/ruby/eq/">==</a> 0,
    [{:left => user}],
    results.<a href="/api/ruby/map/">map</a> { |post|
      {:left => user, :right => post}
    }
  )}
}.zip()
</pre>

        </td></tr>
    </tbody>
</table>


# AGGREGATIONS #

This is a list of queries for performing data aggregation.

<table class="table-2-columns">
    <thead><tr><th>SQL</th><th>ReQL</th></tr></thead>
    <tbody>

        <tr><td>

<pre>
SELECT category
    FROM posts
    GROUP BY category
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("posts").<a href="/api/ruby/map/">map</a>{ |doc|
    doc["category"]
}.<a href="/api/ruby/distinct/">distinct</a>()
</pre>

        </td></tr>
        <tr><td>

<pre>
SELECT category,
       SUM('num_comments')
    FROM posts
    GROUP BY category
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>('posts')
 .<a href="/api/ruby/group/">group</a>('category')
 .<a href="/api/ruby/sum/">sum</a>('num_comments')
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT category,
       status,
       SUM('num_comments')
    FROM posts
    GROUP BY category, status
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("posts")
 .<a href="/api/ruby/group/">group</a>('category', 'status')
 .<a href="/api/ruby/sum/">sum</a>('num_comments')
</pre>
        </td></tr>

        <tr><td>

<pre>
SELECT category,
       SUM(num_comments)
    FROM posts
    WHERE num_comments &gt; 7
    GROUP BY category

</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("posts").<a href="/api/ruby/filter/">filter</a>{ |doc|
    doc['num_comments'] > 7
}.<a href="/api/ruby/group/">group</a>('category')
 .<a href="/api/ruby/sum/">sum</a>('num_comments')
</pre>

        </td></tr>

        <tr><td>

<pre>
SELECT category,
       SUM(num_comments)
    FROM posts
    GROUP BY category
    HAVING num_comments &gt; 7

</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("posts")
 .<a href="/api/ruby/group/">group</a>('category')
 .<a href="/api/ruby/sum/">sum</a>('num_comments')
 .<a href="/api/ruby/filter/">filter</a>{ |val| val > 7 }
</pre>

        </td></tr>

    </tbody>
</table>

# TABLE and DATABASE manipulation #

This is a list of queries for creating and dropping tables and
databases.

<table class="table-2-columns">
    <thead><tr><th>SQL</th><th>ReQL</th></tr></thead>
    <tbody>

        <tr><td>

<pre>
CREATE DATABASE my_database;
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/db_create/">db_create</a>('my_database')
</pre>

        </td></tr>

        <tr><td>

<pre>
DROP DATABASE my_database;
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/db_drop/">db_drop</a>('my_database')
</pre>

        </td></tr>



        <tr><td>

<pre>
CREATE TABLE users
    (id INT IDENTITY(1,1) PRIMARY KEY,
    name VARCHAR(50),
    age INT);
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table_create/">table_create</a>('users',
    :primary_key => "id")
</pre>
<p><em>Note:</em> RethinkDB is a NoSQL database and does not enforce
schemas.</p>

<p><em>Note:</em> The default primary key is <code>id</code></p>



        </td></tr>


        <tr><td>

<pre>
TRUNCATE TABLE users;
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table/">table</a>("users").<a href="/api/ruby/delete/">delete</a>()
</pre>



        </td></tr>

        <tr><td>

<pre>
DROP TABLE users;
</pre>

        </td><td>

<pre>
r.<a href="/api/ruby/table_drop/">table_drop</a>("users")
</pre>

        </td></tr>

    </tbody>
</table>



# Read More #

Browse the following resources to learn more about ReQL:

- [Lambda functions in RethinkDB](/blog/lambda-functions/)
- [Introduction to map-reduce](/docs/map-reduce/)
- [Introduction to Joins](/docs/table-joins/)
- [API Reference](/api/)
