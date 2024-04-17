### April 17, 2024
- Upgraded dependencies.

### August 27, 2022
- The socket stream moves to `on_socket_accept` and splits in reader and writer and these are used locally from there to process the incoming message.

### August 26, 2022
- Adds tokio to deal with the async nature of the service.

### July 24, 2022
- Adds support for command line arguments to set which address to bind and port to listen on.
- Addds CHANGES.md

### June 29, 2022
- Using XXHash algorithm for performance optimization https://github.com/Cyan4973/xxHash/wiki/Performance-comparison

### June 26, 2022
- Adds UnixSocket support.
- Adds conveniences to produce benchmarks.
