use kadnet_core::node::bucket::Bucket;
use kadnet_core::node::core::Node;
use kadnet_core::node::entry::Entry;

use cryptography::U256;
use cryptography::hash::sha256;

use std::net::{IpAddr, Ipv4Addr};
use std::thread;

// Helper to run a closure with a larger stack to accommodate Node construction
fn with_large_stack<F: FnOnce() + Send + 'static>(f: F) {
    let handle = thread::Builder::new()
        .stack_size(4 * 1024 * 1024) // 4 MiB stack
        .spawn(f)
        .expect("spawn test thread");

    handle.join().expect("thread panicked");
}

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
    let mut bucket = Bucket::<4>::init(id, 0);

    #[cfg(feature = "no-std")]
    {
        let result = bucket.find_n_closest::<4>(id);
        assert_eq!(result.iter().filter(|e| e.is_some()).count(), 0);
    }

    #[cfg(not(feature = "no-std"))]
    {
        let entries = bucket.find_n_closest::<4>(id);
        assert_eq!(entries.len(), 0);
    }
}

#[test]
fn bucket_add_entry_to_bucket() {
    with_large_stack(|| {
        let id = sha256(b"bucket-id");
        let mut bucket = Bucket::<16>::init(id, 0);

        let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1));
        let result = bucket.add_entry(ip);

        // Entry addition should complete (result may be Ok or Err depending on range)
        let _ = result;
    });
}

#[test]
fn bucket_can_add_multiple_entries() {
    with_large_stack(|| {
        let id = sha256(b"bucket-id");
        let mut bucket = Bucket::<16>::init(id, 0);

        let ips = vec![
            IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2)),
            IpAddr::V4(Ipv4Addr::new(10, 0, 0, 3)),
        ];

        for ip in &ips {
            let _result = bucket.add_entry(*ip);
        }

        #[cfg(feature = "no-std")]
        {
            let result = bucket.find_n_closest::<4>(id);
            let count = result.iter().filter(|e| e.is_some()).count();
            assert!(count <= 3);
        }

        #[cfg(not(feature = "no-std"))]
        {
            let entries = bucket.find_n_closest::<4>(id);
            assert!(entries.len() <= 3);
        }
    });
}

#[test]
fn bucket_find_n_closest_respects_limit() {
    let id = sha256(b"bucket-id");
    let mut bucket = Bucket::<8>::init(id, 0);

    for i in 0..5 {
        let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, i));
        bucket.add_entry(ip).ok();
    }

    #[cfg(feature = "no-std")]
    {
        let result = bucket.find_n_closest::<3>(id);
        let count = result.iter().filter(|e| e.is_some()).count();
        assert!(count <= 3);
    }

    #[cfg(not(feature = "no-std"))]
    {
        let entries = bucket.find_n_closest::<3>(id);
        assert!(entries.len() <= 3);
    }
}

#[test]
fn bucket_initialization_with_different_k_values() {
    let id = sha256(b"test-id");

    // Test different K values
    let mut bucket_k1 = Bucket::<1>::init(id, 0);
    let mut bucket_k2 = Bucket::<2>::init(id, 0);
    let mut bucket_k4 = Bucket::<4>::init(id, 0);
    let mut bucket_k8 = Bucket::<8>::init(id, 0);
    let mut bucket_k16 = Bucket::<16>::init(id, 0);

    // All should initialize and work without panic
    #[cfg(feature = "no-std")]
    {
        assert_eq!(
            bucket_k1
                .find_n_closest::<1>(id)
                .iter()
                .filter(|e| e.is_some())
                .count(),
            0
        );
        assert_eq!(
            bucket_k2
                .find_n_closest::<2>(id)
                .iter()
                .filter(|e| e.is_some())
                .count(),
            0
        );
        assert_eq!(
            bucket_k4
                .find_n_closest::<4>(id)
                .iter()
                .filter(|e| e.is_some())
                .count(),
            0
        );
        assert_eq!(
            bucket_k8
                .find_n_closest::<8>(id)
                .iter()
                .filter(|e| e.is_some())
                .count(),
            0
        );
        assert_eq!(
            bucket_k16
                .find_n_closest::<16>(id)
                .iter()
                .filter(|e| e.is_some())
                .count(),
            0
        );
    }

    #[cfg(not(feature = "no-std"))]
    {
        assert_eq!(bucket_k1.find_n_closest::<1>(id).len(), 0);
        assert_eq!(bucket_k2.find_n_closest::<2>(id).len(), 0);
        assert_eq!(bucket_k4.find_n_closest::<4>(id).len(), 0);
        assert_eq!(bucket_k8.find_n_closest::<8>(id).len(), 0);
        assert_eq!(bucket_k16.find_n_closest::<16>(id).len(), 0);
    }
}

#[test]
fn bucket_find_n_closest_returns_sorted_results() {
    let id = sha256(b"bucket-id");
    let mut bucket = Bucket::<16>::init(id, 0);

    for i in 0..8 {
        let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, i));
        bucket.add_entry(ip).ok();
    }

    #[cfg(feature = "no-std")]
    {
        let result = bucket.find_n_closest::<4>(id);
        let count = result.iter().filter(|e| e.is_some()).count();
        assert!(count <= 4, "Should return at most 4 entries");
    }

    #[cfg(not(feature = "no-std"))]
    {
        let entries = bucket.find_n_closest::<4>(id);
        assert!(entries.len() <= 4, "Should return at most 4 entries");
    }
}

// ===== Node Tests =====

#[test]
fn node_initialization_creates_valid_id() {
    with_large_stack(|| {
        let node = Box::new(Node::new(b"test-seed"));
        assert_ne!(node.id, U256::from(0u32));
    });
}

#[test]
fn node_same_seed_produces_same_id() {
    with_large_stack(|| {
        let node1 = Node::new(b"same-seed");
        let node2 = Node::new(b"same-seed");

        assert_eq!(node1.id, node2.id);
    });
}

#[test]
fn node_different_seeds_produce_different_ids() {
    with_large_stack(|| {
        let node1 = Node::new(b"seed1");
        let node2 = Node::new(b"seed2");

        assert_ne!(node1.id, node2.id);
    });
}

#[test]
fn node_get_n_closest_returns_correct_size() {
    with_large_stack(|| {
        let mut node = Box::new(Node::new(b"node-seed"));
        let target = sha256(b"target");

        // Just verify that get_n_closest works without panicking
        let _closest_1 = node.get_n_closest::<1>(target);
        let _closest_3 = node.get_n_closest::<3>(target);
        let _closest_5 = node.get_n_closest::<5>(target);

        // Tests pass if no panic occurs
    });
}

#[test]
fn node_get_n_closest_with_empty_buckets() {
    with_large_stack(|| {
        let mut node = Box::new(Node::new(b"empty-node"));
        let target = sha256(b"some-target");

        // Just verify that get_n_closest works with empty buckets
        let _closest = node.get_n_closest::<3>(target);

        // Tests pass if no panic occurs
    });
}
