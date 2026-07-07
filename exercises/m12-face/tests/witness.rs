//! Witness suite for m12-face — written as integration tests, the way a real
//! component gets exercised: build faces and make them talk end to end.

use m12_face::{ChannelTransport, FaceError, LinkFrame, LinkService, Transport};

// ---------------------------------------------------------- transport layer

#[test]
fn transport_moves_raw_frames_and_knows_its_id() {
    let (t_a, t_b) = ChannelTransport::pair(7, 8);
    assert_eq!(t_a.id(), 7);
    assert_eq!(t_b.id(), 8);

    t_a.send_frame(vec![1, 2, 3]).unwrap();
    assert_eq!(t_b.recv_frame(), Some(vec![1, 2, 3]));
    assert_eq!(t_b.recv_frame(), None); // nothing more pending
}

#[test]
fn send_to_a_dropped_peer_is_closed() {
    let (t_a, t_b) = ChannelTransport::pair(1, 2);
    drop(t_b); // the peer hangs up
    assert_eq!(t_a.send_frame(vec![9]), Err(FaceError::Closed));
}

// --------------------------------------------------------- link-service layer

#[test]
fn two_faces_exchange_a_tagged_packet() {
    let (t_a, t_b) = ChannelTransport::pair(1, 2);
    let face_a = LinkService::new(t_a);
    let face_b = LinkService::new(t_b);

    face_a.send_packet(b"hello from A").unwrap();
    let frame = face_b.recv_packet().expect("B should receive A's packet");
    assert_eq!(
        frame,
        LinkFrame {
            payload: b"hello from A".to_vec(),
            source: 1, // the LinkService tagged the packet with A's face id
        }
    );
    assert!(face_b.recv_packet().is_none());
}

#[test]
fn faces_talk_both_ways_with_correct_source_tags() {
    let (t_a, t_b) = ChannelTransport::pair(10, 20);
    let face_a = LinkService::new(t_a);
    let face_b = LinkService::new(t_b);

    face_a.send_packet(b"ping").unwrap();
    face_b.send_packet(b"pong").unwrap();

    let at_b = face_b.recv_packet().unwrap();
    assert_eq!(at_b.payload, b"ping");
    assert_eq!(at_b.source, 10);

    let at_a = face_a.recv_packet().unwrap();
    assert_eq!(at_a.payload, b"pong");
    assert_eq!(at_a.source, 20);
}

#[test]
fn frames_arrive_in_order() {
    let (t_a, t_b) = ChannelTransport::pair(1, 2);
    let face_a = LinkService::new(t_a);
    let face_b = LinkService::new(t_b);

    for i in 0..5u8 {
        face_a.send_packet(&[i]).unwrap();
    }
    for i in 0..5u8 {
        let frame = face_b.recv_packet().expect("packet present");
        assert_eq!(frame.payload, vec![i], "packet {i} out of order");
        assert_eq!(frame.source, 1);
    }
    assert!(face_b.recv_packet().is_none());
}

#[test]
fn empty_payload_still_round_trips_with_its_tag() {
    let (t_a, t_b) = ChannelTransport::pair(42, 43);
    let face_a = LinkService::new(t_a);
    let face_b = LinkService::new(t_b);

    face_a.send_packet(b"").unwrap();
    let frame = face_b.recv_packet().unwrap();
    assert_eq!(frame.payload, Vec::<u8>::new());
    assert_eq!(frame.source, 42);
}
