# Simple Message Queue

## Functionalities

- client will connect to server
- the client is free to push (enqueue) or pull (dequeue) messages to the queue after it is
  connected
- when the client pulls (dequeue), they will get `EMPTY_QUEUE` response if the queue is empty and
  `SUCCESS` response if the queue is not empty.
- client can push (enqueue) anytime they want, the messages are kept in the server until the
  server is stopped

## Request and Response

Each request and response have a 1 byte header and a body. The body can have a variable size. The
header denotes the type of action done to the server.

There are 2 types of action that can be done when doing request to the server:

- push
- pull

### Push

#### Request

- Header is `0`.
- The body is the message (see [Message Format](#message-format)).

#### Response

- Header is 1 byte. If message is successfully pushed, header is `0`, else header is `1`.
- Body is empty.

### Pull

#### Request

- Header is `1`.
- The body is empty.

#### Response

- The header can be empty.
- The body is the message saved in the message queue (see [Message Format](#message-format)).

## Message Format

Metadata (when pull):

```
code: EMPTY_QUEUE, SUCCESS
type: I8, I16, I32, I64, U8, U16, U32, U64 , F32, F64, Str
count: unsigned long
```

3 kinds metadata are used to describe the data. `code` describes the kind of response and `type`
describes the data type. `count` describes how many data is in it. If `count > 1` then the data is
treated as an array. For strings, each element will be seperated with a null-terminator character
(\0) and a null-terminator character is added at the end of the data bytes.

The metadata is a 5 bytes data to explain the sent data. The first byte consists of the `code` and
`type` metadata. `code` takes first 4 bits and `type` takes the 2nd 4 bits. `count` is 4 bytes
(which is an unsigned 4 bytes integer) which means the whole metadata takes 5 bytes. This means
when sending an array, the maximum element limit is 4,294,967,295 elements.

With the metadata, the response will be 5 megabytes with some additional bytes if the data exists.

Code and type to bits mapping

| bits | code and type      |
|------|--------------------|
| 0000 | SUCCESS or U8      |
| 0001 | U16                |
| 0010 | EMPTY_QUEUE or U32 |
| 0011 | U64                |
| 0100 | I8                 |
| 0101 | I16                |
| 0110 | I32                |
| 0111 | I64                |
| 1000 | F32                |
| 1001 | F64                |
| 1010 | Str                |
