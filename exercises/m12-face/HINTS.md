# Hints — m12-face

Revealed one rung at a time by `./course hint m12-face`.

## Hint 1 — the Transport impl is three small methods

`id` returns `self.id`. `recv_frame` is `self.inbox.try_recv().ok()` — `try_recv`
takes the next frame without blocking and returns a `Result`; `.ok()` turns "nothing
waiting" (and "channel closed") into `None`. `send_frame` sends on the outbox and
maps failure to your error: `self.outbox.send(frame).map_err(|_| FaceError::Closed)`
— a channel `send` only fails when the receiving end has been dropped, which is
exactly "the peer is gone."

## Hint 2 — framing: the id is a 4-byte header

`send_packet` builds a frame that is `[our id as 4 big-endian bytes][payload]`:

```rust
let mut frame = self.transport.id().to_be_bytes().to_vec();
frame.extend_from_slice(payload);
self.transport.send_frame(frame)
```

`u32::to_be_bytes()` gives you the `[u8; 4]`; `.to_vec()` starts the frame with it,
and `extend_from_slice` appends the payload. The transport never inspects these bytes
— framing is the LinkService's job, and that separation is the whole point.

## Hint 3 — un-framing: split the header back off

`recv_packet` receives a frame and reverses it:

```rust
let frame = self.transport.recv_frame()?;      // None if nothing pending
let source = u32::from_be_bytes(frame[0..4].try_into().unwrap());
let payload = frame[4..].to_vec();
Some(LinkFrame { payload, source })
```

The `?` on the `Option` returns `None` when there's no frame. `frame[0..4]` is the
tag; `frame[4..]` is everything after — which is empty for an empty payload, and
that's fine.

## Hint 4 — keep the layers honest

If a test fails on the *source* being wrong, check that `send_packet` tags with
`self.transport.id()` (the sender's id), not something else — the receiver reads back
whoever *sent* it. If `recv_frame` never returns anything, remember `try_recv`, not
`recv` (which blocks). And resist putting the id header logic into `ChannelTransport`:
the transport moves opaque bytes; only the LinkService knows what they mean.
