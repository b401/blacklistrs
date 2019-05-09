# Simple sqlite based blocklist

## Features
- Sqlite backend
- REST API
- Threadsafe communication

#### Why?
The whole intention for this application is to have access to a fast and
dynamically written blacklist for firewall appliances. 

With some help of a REST client it's possible to add new threats (domains) to
a blacklist and let your firewall pull the new entries.

#### Dependencies
- Rust nightly because of rocket.

#### Running
There are currently no arguments which you can provide so hardcoded values will
be used.

- creates a sqlitedb in /tmp

Get all entries
``GET /``

Add new entry
``PUT /www.malicioussite.com``

Delete entry
``DELETE /www.malicioussite.com``

#### TODO
- [ ] REGEX for entries 
- [ ] TLS
- [ ] Authentication
- [ ] CLI arguments

