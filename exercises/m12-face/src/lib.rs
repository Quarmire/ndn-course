//! m12-face — growing the system: your first real component.
//!
//! Read SPEC.md. Run the witness with `./course check m12-face`.
//!
//! A *face* is the forwarder's link to a peer. It's built from two layers (this is
//! how the real engine does it):
//!
//!   * a `Transport` — moves opaque byte frames over some medium, and
//!   * a `LinkService` — frames network-layer packets (tags them with link
//!     metadata) and hands the bytes to the transport.
//!
//! `Face = Transport + LinkService`. You'll implement the classic first transport —
//! an in-memory channel pair (the real tree ships this as `InProcFace`) — and the
//! LinkService framing on top. The real traits are async and trade `bytes::Bytes`;
//! this analog is synchronous and trades `Vec<u8>` so you see the shape without a
//! runtime. Stubs compile; tests are red until you fill them in.

use std::sync::mpsc::{Receiver, Sender};

/// A face identifier.
pub type FaceId = u32;

/// What can go wrong on a face. (The real `FaceError` also has `Io` and `Full`;
/// here a gone link is all we model.)
#[derive(Debug, PartialEq, Eq)]
pub enum FaceError {
    /// The link is gone — the peer hung up.
    Closed,
}

/// What a `LinkService` hands up on receive: the packet payload plus the link
/// metadata it extracted — here, which face the packet came from. (The real
/// `LinkServiceFrame` also carries congestion marks, addresses, and more.)
#[derive(Debug, PartialEq, Eq)]
pub struct LinkFrame {
    pub payload: Vec<u8>,
    pub source: FaceId,
}

/// The low-level byte pipe. It moves opaque frames and knows nothing about NDN
/// packets — only bytes.
pub trait Transport {
    /// This transport's face id.
    fn id(&self) -> FaceId;

    /// Send one frame to the peer. `Err(Closed)` if the link is gone.
    fn send_frame(&self, frame: Vec<u8>) -> Result<(), FaceError>;

    /// The next frame from the peer, or `None` if nothing is waiting (this
    /// analog is non-blocking; the real `recv_bytes` is an async await).
    fn recv_frame(&self) -> Option<Vec<u8>>;
}

/// An in-memory transport. A connected `pair` of them — what one sends, the other
/// receives — is the canonical first face to build (`InProcFace` in the real tree
/// is exactly this over tokio channels).
#[allow(dead_code)] // fields are read once you implement the Transport impl
pub struct ChannelTransport {
    id: FaceId,
    outbox: Sender<Vec<u8>>,
    inbox: Receiver<Vec<u8>>,
}

impl ChannelTransport {
    /// Two connected transports: `id_a`'s outbox is `id_b`'s inbox and vice versa.
    /// (provided)
    pub fn pair(id_a: FaceId, id_b: FaceId) -> (ChannelTransport, ChannelTransport) {
        let (tx1, rx1) = std::sync::mpsc::channel();
        let (tx2, rx2) = std::sync::mpsc::channel();
        (
            ChannelTransport {
                id: id_a,
                outbox: tx1,
                inbox: rx2,
            },
            ChannelTransport {
                id: id_b,
                outbox: tx2,
                inbox: rx1,
            },
        )
    }
}

impl Transport for ChannelTransport {
    fn id(&self) -> FaceId {
        todo!("return this transport's id")
    }

    fn send_frame(&self, frame: Vec<u8>) -> Result<(), FaceError> {
        let _ = frame;
        todo!("send on the outbox; a send error means the peer is gone → FaceError::Closed — see HINTS")
    }

    fn recv_frame(&self) -> Option<Vec<u8>> {
        todo!("take the next frame from the inbox without blocking — see HINTS")
    }
}

/// The `LinkService`: the layer above `Transport`. It frames an outgoing packet by
/// tagging it with the sending face's id, hands the bytes to the transport, and
/// un-frames on receive. It OWNS its transport — together they are a face. It's
/// generic over `T: Transport`, so the same framing works over any medium.
#[allow(dead_code)] // `transport` is read once you implement send/recv
pub struct LinkService<T: Transport> {
    transport: T,
}

impl<T: Transport> LinkService<T> {
    /// Build a face by wrapping a transport. (provided)
    pub fn new(transport: T) -> Self {
        Self { transport }
    }

    /// Frame `payload` — tag it with our face id — and send it over the transport.
    /// The 4-byte big-endian face id is the whole "link header" in this analog.
    pub fn send_packet(&self, payload: &[u8]) -> Result<(), FaceError> {
        let _ = payload;
        todo!("prepend our id() as 4 big-endian bytes, then transport.send_frame — see HINTS")
    }

    /// Receive one packet if any: un-frame it into its payload and source face id.
    pub fn recv_packet(&self) -> Option<LinkFrame> {
        todo!("recv a frame, split the 4-byte source header off the front — see HINTS")
    }
}
