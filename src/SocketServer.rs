struct WebSocketServer;

impl Handler for WebSocketServer {
    // Traits can have useful default implementations, so in fact the handler
    // interface requires us to provide only two things: concrete types for
    // timeouts and messages.
    // We're not ready to cover these fancy details, and we wouldn't get to them
    // anytime soon, so let's get along with the defaults from the mio examples:
    type Timeout = usize;
    type Message = ();
}