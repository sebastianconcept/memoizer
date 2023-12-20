# memoizer
Minimalist thread-safe key-value store shared over TCP sockets.


## Project build

Development version:

    cargo build

Release version:

    cargo build --release

## Running the service (release build)

    ./target/release/memoizer -b localhost -p 9091

## Running the inner benchmark

    ./target/release/memoizer --bind 0.0.0.0 --port 9091 --bench 10000 --payload '{"id":123,"name":"Sample JSON","description":"This is a sample JSON object with approximately 1024 bytes of data. It''s used for demonstration purposes.","tags":["json","sample","data"],"details":{"created_at":"2023-04-01T12:00:00","updated_at":"2023-04-01T14:30:00","status":"active"},"values":[1,2,3,4,5,6,7,8,9,10],"settings":{"enabled":true,"threshold":50,"options":["option1","option2","option3"]},"comments":[{"user":"user1","text":"This is a comment."},{"user":"user2","text":"Another comment here."}]}'


On this hardware [1] it renders:

```
Starting the benchmarking...
Benchmarking warmed up and ready to go.
Measuring Rust HashMap 10000 inserts...
Measuring Rust HashMap 10000 inserts...
Measuring Rust HashMap 10000 inserts...
Measuring Rust HashMap 10000 inserts...
Measuring Rust HashMap 10000 inserts...
Measuring Rust HashMap 10000 inserts...
Measuring Rust HashMap 10000 inserts...
Measuring Rust HashMap 10000 inserts...
Measuring Rust HashMap 10000 inserts...
Measuring Rust HashMap 10000 inserts...
It took 14.473349ms to perform 10000 insertions
Measuring Rust HashMap 10000 reads...
Measuring Rust HashMap 10000 reads...
Measuring Rust HashMap 10000 reads...
Measuring Rust HashMap 10000 reads...
Measuring Rust HashMap 10000 reads...
Measuring Rust HashMap 10000 reads...
Measuring Rust HashMap 10000 reads...
Measuring Rust HashMap 10000 reads...
Measuring Rust HashMap 10000 reads...
Measuring Rust HashMap 10000 reads...
It took 10.014888ms to perform 10000 reads
```

## Comparing with Redis
From [Pharo](https://pharo.org/), if you have [RediStick](https://github.com/mumez/RediStick) and [ABBench](https://github.com/emdonahue/ABBench) installed in the image, you can benchmark this memoizer service and compare performance with Redis. Here is a snippet with the numbers provided with this hardware [1]:

```Smalltalk
RsRedisConnectionPool primaryUrl: 'sync://localhost:6379'.

"Create a Redis client."
redis := RsRedisProxy of: #client1.
 
"Payload to use as part of the value."
sample1 := '\{"id": "{1}","name":"Sample JSON","description":"This is a sample JSON object with approximately 1024 bytes of data. It''s used for demonstration purposes.","tags":["json","sample","data"],"details":\{"created_at":"2023-04-01T12:00:00","updated_at":"2023-04-01T14:30:00","status":"active"},"values":[1,2,3,4,5,6,7,8,9,10],"settings":\{"enabled":true,"threshold":50,"options":["option1","option2","option3"]\},"comments":[\{"user":"user1","text":"This is a comment."\},\{"user":"user2","text":"Another comment here."\}]\}'.

"Cook some data to later iterate and add it to the server."
keys := ((1 to: 10000) collect:[ :e | UUID new asString36 ]) shuffled.
values := (1 to: 10000) collect:[ :e | sample1 format: { UUID new asString36 } ].
source := Dictionary newFromKeys: keys andValues: values.

"Closure used to connect a memoizer client."
client := [ socket := Socket newTCP.
	hostAddress := NetNameResolver addressFromString: '127.0.0.1'.
	socket connectTo: hostAddress port: 9091.
	stream := SocketStream on: socket ].

"Closure used to reset the memoizer client."
reset := [ stream ifNotNil:[ stream close ] ].
	
"Get command using the given key."
get := [ :key | (stream nextPutAll: ('\{"s":"get","p": \{"k": "{1}","v": null \}\}' format: {key asString}); crlf; flush; nextLineLf) trim ].

"Set command using the given key/value."
set := [ :key :value | | content |
	content := ('\{"s":"set", "p": \{"k": "{1}","v": {2}\}\}' format: {key asString. value asString}) trim.
	stream nextPutAll: content; crlf; flush; nextLineLf ].

"Size command."
size := [ (stream nextPutAll: ('{"s":"size", "p": {}}'); crlf; flush; nextLineLf) trim ].

"Reset the client (useful if previously open)."
reset value.
	
"Connect to the memoizer server."
client value.

"Sanity check adding 1 value."
set value: 'answer' value: '42'.
record := source associations first.

"Sanity check adding sampled value."
set value: record key value: record value.

"Command to check memoizer's current size."
size value.

"Setting values in memoizer server"
Time millisecondsToRun: [ source keysAndValuesDo: [ :k :v | set value: k value: v ] ].     "710"

"Getting values from the memoizer server"
Time millisecondsToRun: [ keys collect: [ :k | get value: k ] ].     
"582"

"Setting values in a local Redis"
Time millisecondsToRun: [ source keysAndValuesDo: [ :k :v | redis at: k put: v ] ].   
"1364"

"Getting values from a local Redis"
Time millisecondsToRun: [ keys collect: [ :k | redis at: k ] ].   
"1330"

"Comparing repeated same write"
ABBench bench: [ ABBench 
	a: [ redis at: keys anyOne put: values anyOne ] 
	b: [ set value: keys anyOne value: values anyOne ] ].   
"B is 91.89% FASTER than A"

"Comparing repeated same read"
ABBench bench: [ ABBench 
	a: [ redis at: keys anyOne  ] 
	b: [ get value: keys anyOne ] ]. 
"B is 125.85% FASTER than A"
```

[1] An Intel based MacBook Pro, 2,5 GHz Quad-Core Intel Core i7