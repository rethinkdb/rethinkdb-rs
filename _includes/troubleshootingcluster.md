RethinkDB uses three ports to operate &mdash; the HTTP web UI port,
the client drivers port, and the intracluster traffic port. You can
connect the browser to the web UI port to administer the cluster right
from your browser, and connect the client drivers to the client driver
port to run queries from your application. If you're running a
cluster, different RethinkDB nodes communicate with each other via the
intracluster traffic port.

The message `received invalid clustering header` means there is a port
mismatch, and something is connecting to the wrong port. For example,
it's common to get this message if you accidently point the browser or
connect the client drivers to the intracluster traffic port.