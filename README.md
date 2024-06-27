# SimpleWS: A WebSocket Counter Example

Made with love by Claude 3.5!

SimpleWS is a simple WebSocket-based counter application built for the Kinode platform. It demonstrates how to create a basic WebSocket server that maintains a counter, allowing clients to increment, decrement, and retrieve the counter value.

## Features

- WebSocket server implementation
- Counter functionality (increment, decrement, get value)
- Real-time updates to connected clients

## Building the Project
First, start a fake Kinode with
```
kit f
```

To build and install the project, run the following command in your terminal:
```
kit bs
```

Then navigate in your browser to [http://localhost:8080/simplews:simplews:basilesex.os](http://localhost:8080/simplews:simplews:basilesex.os)

Try refreshing the page: you'll notice that the counter value correctly stays the same, because it's reading from your Kinode.