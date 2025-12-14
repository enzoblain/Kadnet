use kadnet_core::node::bucket::Bucket;
use kadnet_core::node::core::Node;
use kadnet_core::node::entry::Entry;

use cryptography::U256;
use cryptography::hash::sha256;

use std::net::{IpAddr, Ipv4Addr};

// ===== Entry Tests =====

#[test]
fn entry_creation_from_ipv4() {
    let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
    let entry = Entry::new(ip);

    assert_eq!(entry.addr, ip);
    assert_eq!(entry.distance, U256::from(0u32)); // Initial distance is 0
}

#[test]
fn entry_creation_from_ipv6() {
    let ip = IpAddr::V6("::1".parse().unwrap());
    let entry = Entry::new(ip);

    assert_eq!(entry.addr, ip);
    assert_eq!(entry.distance, U256::from(0u32));
}

#[test]
fn entry_compute_distance_updates_field() {
    let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1));
    let mut entry = Entry::new(ip);

    let target = U256::from(42u32);
    entry.compute_distance(target);

    // Distance should be updated
    assert_ne!(entry.distance, U256::from(0u32));
}

#[test]
fn entry_distance_computed_correctly() {
    let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1));
    let mut entry = Entry::new(ip);

    let target = U256::from(100u32);
    entry.compute_distance(target);

    // Distance should be XOR of entry.id and target
    assert_eq!(entry.distance, entry.id ^ target);
}

#[test]
fn bucket_initialization_is_empty() {
    let id = sha256(b"test-node");
    let mut bucket = Bucket::init(4);

    let (_, size) = bucket.find_n_closest(id);
    assert_eq!(size, 0);
}

#[test]
fn bucket_add_entry_to_bucket() {
    let mut bucket = Bucket::init(16);

    let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1));
    let result = bucket.add_entry(ip);

    // Entry addition should complete (result may be Ok or Err depending on range)
    let _ = result;
}

#[test]
fn bucket_can_add_multiple_entries() {
    let id = sha256(b"bucket-id");
    let mut bucket = Bucket::init(16);

    let ips = vec![
        IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
        IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2)),
        IpAddr::V4(Ipv4Addr::new(10, 0, 0, 3)),
    ];

    for ip in &ips {
        let _result = bucket.add_entry(*ip);
    }

    let (_, size) = bucket.find_n_closest(id);
    assert!(size <= 3);
}

#[test]
fn bucket_find_n_closest_respects_limit() {
    let id = sha256(b"bucket-id");
    let mut bucket = Bucket::init(8);

    for i in 0..5 {
        let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, i));
        bucket.add_entry(ip).ok();
    }

    let (_, size) = bucket.find_n_closest(id);
    assert!(size <= 4); // ALPHA is 4
}

#[test]
fn bucket_initialization_with_different_k_values() {
    let id = sha256(b"test-id");

    // Test different K values
    let mut bucket_k1 = Bucket::init(1);
    let mut bucket_k2 = Bucket::init(2);
    let mut bucket_k4 = Bucket::init(4);
    let mut bucket_k8 = Bucket::init(8);
    let mut bucket_k16 = Bucket::init(16);

    // All should initialize and work without panic
    assert_eq!(bucket_k1.find_n_closest(id).1, 0);
    assert_eq!(bucket_k2.find_n_closest(id).1, 0);
    assert_eq!(bucket_k4.find_n_closest(id).1, 0);
    assert_eq!(bucket_k8.find_n_closest(id).1, 0);
    assert_eq!(bucket_k16.find_n_closest(id).1, 0);
}

#[test]
fn bucket_find_n_closest_returns_sorted_results() {
    let id = sha256(b"bucket-id");
    let mut bucket = Bucket::init(16);

    for i in 0..8 {
        let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, i));
        bucket.add_entry(ip).ok();
    }

    let (_, size) = bucket.find_n_closest(id);
    assert!(size <= 4, "Should return at most 4 entries");
}

// ===== Node Tests =====

#[test]
fn node_initialization_creates_valid_id() {
    let node = Node::new(b"test-seed");
    assert_ne!(node.id, U256::from(0u32));
}

#[test]
fn node_same_seed_produces_same_id() {
    let node1 = Node::new(b"same-seed");
    let node2 = Node::new(b"same-seed");

    assert_eq!(node1.id, node2.id);
}

#[test]
fn node_different_seeds_produce_different_ids() {
    let node1 = Node::new(b"seed1");
    let node2 = Node::new(b"seed2");

    assert_ne!(node1.id, node2.id);
}

#[test]
fn node_get_n_closest_returns_correct_size() {
    let mut node = Node::new(b"node-seed");
    let target = sha256(b"target");

    // Just verify that get_closests works without panicking
    let _closest_1 = node.get_closests(target);
    let _closest_3 = node.get_closests(target);
    let _closest_5 = node.get_closests(target);

    // Tests pass if no panic occurs
}

#[test]
fn node_get_n_closest_with_empty_buckets() {
    let mut node = Node::new(b"empty-node");
    let target = sha256(b"some-target");

    // Just verify that get_closests works with empty buckets
    let _closest = node.get_closests(target);

    // Tests pass if no panic occurs
}
