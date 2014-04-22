---
layout: documentation
title: "Stability report"
short_title: Stability report
active: docs
docs_active: stability
permalink: stability/
alias: docs/stability/
---

RethinkDB has been publicly available for a little over a year and is still a young product under heavy development. There are hundreds of successful deployments, and we constantly get stability reports from users in production and development environments.

Since all complex software systems are quickly evolving, we use the following terminology to refer to potential issues:

- No `systemic` issues means there are no underlying architectural problems that would prevent us from quickly fixing a reported bug.
- No `required` missing features means we can suggest workarounds for potential problems.

{% infobox info %}
<strong>Found a problem?</strong> Help make RethinkDB better &mdash; <a href="/community">ask a question</a> or <a href="https://github.com/rethinkdb/rethinkdb/issues/new">submit a bug report</a>.
{% endinfobox %}

# Single node deployments #

Single node deployments have been extensively tested and all known systemic issues have been resolved. If you run into a problem, please <a href="https://github.com/rethinkdb/rethinkdb/issues/new">open a bug report</a>.

|                        | Single node stability report |
| :--------------------- | --------- |
| Data integrity               | There are no known data integrity issues. |
| Crash reports                | You may encounter crashes under certain workloads, but there are no systemic issues that we're aware of. |
| Query performance            | You may encounter slowdowns for certain queries, but there are no systemic issues that we're aware of. |
| Memory allocation            | There are no known memory leaks or systemic memory issues. |
| Performance under heavy load | There is no systemic performance degradation under heavy load. |
| Features                     | All of the required features are available. |
| Extent of testing            | Single node deployments have been extensively tested. |

# Deployments of small clusters #

Small cluster deployments are still undergoing heavy testing, but most known systemic issues have been resolved. If you run into a problem, please <a href="https://github.com/rethinkdb/rethinkdb/issues/new">open a bug report</a>.

|                        | Small cluster stability report (five machines or fewer) |
| :--------------------- | --------- |
| Data integrity               | There are no known data integrity issues. |
| Crash reports                | You may encounter crashes under certain workloads, but there are no systemic issues that we're aware of. |
| Query performance            | You may encounter slowdowns for certain queries, but there are no systemic issues that we're aware of. |
| Memory allocation            | There are no known memory leaks or systemic memory issues. |
| Performance under heavy load | There is no known systemic performance degradation under heavy load. |
| Features                     | All of the required features are available. |
| Extent of testing            | Small cluster deployments are still undergoing heavy testing. |

# Deployments of large clusters #

Large cluster deployments have not been sufficiently tested, and required features are still under heavy development. If you're interested in beta testing, please spin up a large RethinkDB cluster, and <a href="/community">send us your feedback</a>!

|                        | Large cluster stability report (more than five machines) |
| :--------------------- | --------- |
| Data integrity               | There are no known data integrity issues. |
| Crash reports                | You may encounter crashes under certain workloads, but there are no systemic issues that we're aware of. |
| Query performance            | You may encounter slowdowns for certain queries, but there are no systemic issues that we're aware of. |
| Memory allocation            | There are no known memory leaks or systemic memory issues. |
| Performance under heavy load | There may be systemic performance issues in large cluster deployments. See <a href="https://github.com/rethinkdb/rethinkdb/issues/1861">#1861</a> and <a href="https://github.com/rethinkdb/rethinkdb/issues/1944">#1944</a> for details. |
| Features                     | Required features are still under development. See <a href="https://github.com/rethinkdb/rethinkdb/issues/1911">#1911</a> for details. |
| Extent of testing            | Large cluster deployments have not been sufficiently tested. |

