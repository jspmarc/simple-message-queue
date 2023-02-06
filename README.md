# Simple Message Queue

## Functionalities

- client will connect to server
- the client is free to push (enqueue) or pull (dequeue) messages to the queue after it is
  connected
- when the client pulls (dequeue), they will get `EMPTY_QUEUE` response if the queue is empty and
  `SUCCESS` response if the queue is not empty. Clients could also get `NULL_DATA` response if
  the data is empty.
- client can push (enqueue) anytime they want, the messages are kept in the server until the
  server is stopped

## Pull Response

Metadata (when pull):

```
code: EMPTY_QUEUE, SUCCESS, NULL_DATA
type: (u_)i8, (u_)i16, (u_)i32, (u_)i64, f32, f64, str
count: unsigned long
```

3 kinds metadata are used to describe the data. `code` describes the kind of response and `type`
describes the data type. `count` describes how many data is in it. If `count > 1` then the data is
treated as an array.

The metadata is a 5 bytes data to explain the sent data. The first byte consists of the `code` and
`type` metadata. `code` takes first 4 bits and `type` takes the 2nd 4 bits. `count` is 4 bytes
(which is an unsigned 4 bytes integer) which means the whole metadata takes 5 bytes. This means
when sending an array, the maximum element limit is 4,294,967,295 elements.

With the metadata, the response will be 5 megabytes with some additional bytes if the data exists.

Code and type to bits mapping

| bits | code and type        |
|------|----------------------|
| 0000 | SUCCESS or u_i8      |
| 0001 | NULL_DATA or u_i16   |
| 0010 | EMPTY_QUEUE or u_i32 |
| 0011 | u_i64                |
| 0100 | i8                   |
| 0101 | i16                  |
| 0110 | i32                  |
| 0111 | i64                  |
| 1000 | f32                  |
| 1001 | f64                  |
| 1010 | str                  |

## Push

### Request

The form for push request is similar to the response for pull.

### Response

The response would only be 1 byte consisting of `code` metadata.
