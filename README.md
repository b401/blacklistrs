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

#### TODO
- [ ] REGEX for entries 
- [ ] TLS
- [ ] Authentication
- [ ] CLI arguments
