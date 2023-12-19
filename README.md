# memoizer
Minimalist thread-safe key-value store shared over TCP sockets.


## Project build

Development version:

    cargo build

Release version:

    cargo build --release

## Running the service (release build)

    ./target/release/memoizer -b localhost -p 9091
