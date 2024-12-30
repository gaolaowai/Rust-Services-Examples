# NTP Server (and client)
There are a few reasons to want to manage your own internal time server.

* Consistent timestamps across internal systems --> This is useful for a multitude of reason, including logging. If different systems are off in their times, piecing together failures after the fact becomes a bit more difficult.
* Security --> despite best efforts and intentions by people creating operating systems, time daemons/services have repeatedly been a vector for directly attacking into system kernals. Knowing that the source of time updates within your network are coming from a trusted source increases overall organizational security.
