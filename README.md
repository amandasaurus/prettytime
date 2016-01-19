Formats an integer number of seconds as a human readable string.

Examples
========

    $ prettytime 10
    10s
    $ prettytime 2034
    33m54s
    $ prettytime 142523m12.003s
    14wk0dy23hr23m12s


Motivation
==========

If you have a duration in seconds (etc) and the number is very long, it can be fiddly to figure out how long it actually is. If `time(1)` told you that a programme took `297m` to complete, how long is that? With `prettytime`, it'll tell you that is `4hr57m00s`.


Licence
=======

Currently licenced under the GNU Affero General Public Licence, v3, or at your option a later version.

If you'd like this under a different licence, please contact rory@technomancy.org
