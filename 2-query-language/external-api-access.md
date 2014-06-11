---
layout: documentation
title: External API access
active: docs
docs_active: external-api-access
permalink: docs/external-api-access/
---

RethinkDB provides an [r.http](/api/javascript/http/) command for
accessing external APIs directly from the database. Since many APIs
accept and return JSON, RethinkDB is a convenient platform for
manipulating and analyzing API data, both interactively and in running
applications.

Let's see how you can use `r.http` and the GitHub API to perform a
common task &mdash; influencer analysis.

{% infobox info %}
<strong>Note:</strong> the following examples use the JavaScript
driver. See the [r.http](/api/javascript/http/) command reference for
documentation for other languages.
{% endinfobox %}

# Basic usage #

Let's try something very simple &mdash; accessing a website. Type the
following command in the Data Explorer and hit 'Run' (alternatively,
you can run it from a RethinkDB driver):

```javascript
r.http('www.google.com')
```

The `r.http` command will issue a request to `www.google.com` directly
from the database and return a string with the source of Google's
homepage.

# Accessing JSON APIs #

Now let's access a real JSON API. In this example we're going to use
GitHub &mdash; a collaborative development platform that hosts
thousands of open-source projects. Users on GitHub can indicate
interest by starring projects and following other users. GitHub calls
users who starred other projects "stargazers".

Let's find the most influential GitHub users who showed interest in
RethinkDB. First, let's grab the list of RethinkDB stargazers:

```javascript
r.http('https://api.github.com/repos/rethinkdb/rethinkdb/stargazers')
```

This query makes a call to GitHub and returns an array of JSON
documents. RethinkDB natively operates on JSON, and what's really
convenient about `r.http` is that its output is no different from any
other query. You can use ReQL commands to perform operations on the
output of `r.http`, just like on a standard RethinkDB table!

For example, let's count the number of results returned by the API:

```javascript
r.http('https://api.github.com/repos/rethinkdb/rethinkdb/stargazers').count()
```

Or, let's pluck out usernames and IDs, and sort the array in ascending
order by user IDs:

```javascript
r.http('https://api.github.com/repos/rethinkdb/rethinkdb/stargazers')
 .pluck('login', 'id').orderBy('id')
```

You can chain as many ReQL commands as necessary to perform data
manipulation tasks. Since ReQL is explicitly designed for JSON
querying and modification, it's also an excellent language for
operating on web services!

# Storing and enriching API data #

Since you'll be doing more manipulation on the data, you might want to
store the results of an API call in the database. Let's create a table
`stargazers` and insert the RethinkDB stargazers into this table:

```javascript
r.tableCreate('stargazers');
r.table('stargazers').insert(
  r.http('https://api.github.com/repos/rethinkdb/rethinkdb/stargazers'));
```

Now we'd like to sort RethinkDB stargazers by influence. When you
request stargazers from GitHub, the GitHub API doesn't include the
number of followers for each stargazer, but it does include a field
`url` specific to each stargazer. If you follow this URL, the GitHub
API will return additional information for the user, including the
number of their followers.

Let's update our stargazer data with this additional information:

```javascript
r.table('stargazers').update(r.http(r.row('url')))
```

The update command will go through every row and issue an API request
to the GitHub URL for the given user, grab the relevant data, and
update the user information with that data!

We can now sort the stargazers by the number of their followers!

```javascript
r.table('stargazers').orderBy(r.desc('followers'))
```

# Pagination #

The calls above only return a few dozen stargazers while RethinkDB has
thousands. Most APIs paginate large result sets and GitHub is no
exception. The `r.http` command has built-in support for pagination
via the `page` and `pageLimit` arguments. Let's get ten pages of
stargazers from GitHub instead of one:

```javascript
r.http('https://api.github.com/repos/rethinkdb/rethinkdb/stargazers',
       { page: 'link-next', pageLimit: 10 })
```

The `page` argument takes the type of pagination mechanism used by the
API. In this case GitHub uses the standard link header mechanism
`link-next`, but you can also specify custom pagination schemes for
unusual APIs. The `page-limit` argument specifies the number of
pages you'd like to get. See the [API
reference](/api/javascript/http/) for more details.

When you turn on pagination in `r.http`, instead of returning an array
of documents, RethinkDB returns a stream which you can access in the
driver via the usual cursor API. This is significant because
pagination happens lazily &mdash; RethinkDB will request new pages as
you iterate through the cursor to minimize the number of API
calls.

# Authentication #

Most APIs support some form of authentication and rate limiting. The
`r.http` command supports common forms of authentication (see the
[reference](/api/javascript/http/) for more details). For example,
here is how you can use GitHub tokens with basic auth:

```javascript
r.http('https://api.github.com/users/coffeemug', {
       auth: {
           user: GITHUB_TOKEN,
           pass: 'x-oauth-basic'
       }
})
```

# Read more #

Browse the following resources to learn more about ReQL and `r.http`:

- [r.http](/api/javascript/http) API reference
- [Introduction to ReQL](/docs/introduction-to-reql/)
- [Lambda functions in RethinkDB](/blog/lambda-functions/)
