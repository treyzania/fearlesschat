# Fearlesschat Protocol Specification

Fearlesschat's protocol for interacting with servers is very simple.  It was
modelled after the protocol for memcache and redis.

## Connections

The default port is not yet defined.  The DHT should be able to figure it out.

### Streaming IO

Open a TCP connection.  Now you're the client and the other end of the
connection is the server.  Use backslash (`\`) as an escape character.  To
separate messages, send a newline.

There's two types of messages.  Commands and control signals.  Control signals
always start with a capital letter, commands are all lowercase.

Examples:

* `pubmsg` : command

* `subchan` : command (this one only makes sense if it's a simple client)

* `dhthello` : command (this one only makes sense if it's a full node)

* `MyCtrlSignal` : control signal

### One-offs

Just send a UDP datagram of the command you're issuing, ending in a newline.
You should only do this if you send the command out to multiple servers as you
don't have any guarantee that the command will be delivered.  The network knows
how to handle commands being sent out awkwardly.  You can't send control
signals over UDP because that doesn't make sense.  You also can't send dhthello
commands over UDP because it's obviously not a good idea to do that.

Commands as always start with a lowercase letter.

## Commands

Below is a summary of the commands that clients and servers should know.  For
more details, see below.

| Name        | Argument(s)                              | Description                                                              |
|-------------|------------------------------------------|--------------------------------------------------------------------------|
| `pubmsg`    | blob                                     | Publishes a signed message to the network.                               |
| `msg`       | name@channel msg...                      | Instructs the server to construct and public a simple, unsigned message. |
| `subchan`   | on,off channelname                       | Subscribes a light client to a channel.                                  |
| `dhthello`  | jsonblob                                 | Announces the node to the DHT, etc.                                      |
| `dhtid`     | signature name publickey misc...         | Publishes the identity record on the DHT.                                |
| `dhtsync`   | speed                                    | Requests a synchronization with the DHT, in records per second.          |

## Control Signals

*Nothing much here yet because we don't really need this right now.  The
protocol is largely symmetric as-is.*

| Name    | Parameters             | Description                                 |
|---------|------------------------|---------------------------------------------|
| `LcMsg` | name@channel msg       | A message was sent on a subscribed channel. |

## Chat Message Format

The chat message format is quite similar to the format of HTTP requests.  The
general format is as follows:

```
<sigtype> [<signature>]
Sender: <sendername>
[Identity-Hash: <dht-stored signature id>]
Channel: <channel name>
Timestamp: <iso8601>|<unix>|<day of the week in spanish>

<message body>
```

The sigtype describes what kind of signature was used to sign the message.  The
signature is usually an encryption scheme, followed by a hash algorithm.  In
order to sign a message, hash the contents of the rest of the message, then sign
it with your private key.  You can also put *just* a hash algorithm, in which
case the "signature" functions as a checksum for messages stored on disk or
when being sent over volatile channels.  A list of encryption schemes and hash
algorithms that are expected to be understood is below.

If a proper signature is used, then you can provide the key for the record of
your identity in the DHT as necessary.

The timestamp can be given in a variety of formats, ISO 8601 is preferred, but
UNIX timestamps are also acceptable.  Using the day of the week in Spanish is
also supported for legacy reasons.

The message body can be up to 16k.

Here's a (bogus) example that gets the idea across.

```
RSA-2048,SHA-256 kasgdhfakjsdlkfjaydsjzxcmvhaskjdfyaosidjkfhmdlfajdfkashdfjaf
Sender: notch
Identity-Hash: asdfhkashdfkhasdjkfasgdhfkjasfkjadh
Channel: minecraft
Timestamp: 2017-07-21T02:22:47+00:00

Hello, this is the message.  I'm a billionare.
```

### Encryption Schemes and Hash Algorithms

Encryption:

* RSA-1024

* RSA-2048

* RSA-4096

* plain (Aka, no signing.  Not suggested.)

Hash Algorithms:

* SHA-256

* SHA-512

* MD6

* `String:hashCode` (Java)

* CRC-64
