dokku-daemon-rs[![Continuous integration](https://github.com/palfrey/dokku-daemon-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/palfrey/dokku-daemon-rs/actions/workflows/ci.yml)
==

A daemon wrapper around [Dokku](https://dokku.com/). Basically, this is https://github.com/dokku/dokku-daemon but rewritten in Rust to solve the JSON issues there. It's primary target is supporting https://github.com/palfrey/wharf and enabling Wharf to do it's thing without needing SSH keys.

Installing
==

As a user with access to `sudo`:
```
git clone https://github.com/palfrey/dokku-daemon-rs
cd dokku-daemon-rs
make install
```

Specifications
--------------

* Daemon listens on a UNIX domain socket (by default created at `/var/run/dokku-daemon/dokku-daemon.sock`, but set `DOKKU_DAEMON_SOCKET_PATH` environment variable for a different path)
* Commands issued to the daemon take the same form as those used with dokku on the command-line
* Responses are sent as line-delimited JSON
* No authentication layer (local/container connections only)
* Multiple client connections are supported but only one command will be processed at a given time
