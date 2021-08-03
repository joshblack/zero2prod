# Notes

## Chapter 2

No notes available.

## Chapter 3

- We're going to implement the `POST /subscribe` enpoint
  - before that, we'll implement a `/health_check` endpoint
- Terminology
  - `HttpServer` handles transport level concerns (listen for requests, tls,
    etc)
  - `App` is where application logic lives, such as routing, middleware, request
    handlers, etc.
  - Endpoints are implemented using the `route` method. A `Route` combines a
    handler with a set of guards
      - Guards specify conditions that a request must satisfy in order to match
        the request (`Guard::check` trait is where this happens)
