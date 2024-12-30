# HTTP server module
To start with, we'll be writing a basic HTTP server for HTTP 1.3. Later we'll add on TLS, maybe later support HTTP2/3 (I'm leaning towards 3). Intent is that we'll structure our code in a way that HTTP handling is separate from the transport layer, though perhaps we'll write a basic handler example that doesn't do that.

The motivation behind making sure that HTTP handling is separate from the transport handling is two-fold:
* testing --> If we don't need to create a tcp/udp socket, that greatly helps us with testing.


## Basic info

## TODO
- [ ] Header parser (parse out URI and request type)
- [ ] endpoint registration
- [ ] request body handling (POST, PUT, DELETE)
- [ ] response handling
- [ ] test suite
- [ ] TLS 1.3 for our sockets.

### References
* https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers
