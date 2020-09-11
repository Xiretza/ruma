//! [GET /_matrix/client/r0/pushers](https://matrix.org/docs/spec/client_server/r0.6.0#get-matrix-client-r0-pushers)

use ruma_api::ruma_api;

use super::Pusher;

ruma_api! {
    metadata: {
        description: "Gets all currently active pushers for the authenticated user.",
        method: GET,
        name: "get_pushers",
        path: "/_matrix/client/r0/pushers",
        rate_limited: false,
        requires_authentication: true,
    }

    #[derive(Default)]
    #[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
    request: {}

    #[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
    response: {
        /// An array containing the current pushers for the user.
        pub pushers: Vec<Pusher>
    }

    error: crate::Error
}

impl Request {
    /// Creates an empty `Request`.
    pub fn new() -> Self {
        Self
    }
}

impl Response {
    /// Creates a new `Response` with the given pushers.
    pub fn new(pushers: Vec<Pusher>) -> Self {
        Self { pushers }
    }
}
