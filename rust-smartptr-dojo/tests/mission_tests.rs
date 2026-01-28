use rust_smartptr_dojo::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// =============================================================================
// Mission 1: FsNode (Rc<RefCell<T>> + Weak)
// =============================================================================

#[test]
fn mission_1_new_file() {
    let file = FsNode::new_file("readme.txt");
    assert_eq!(file.name, "readme.txt");
    assert!(!file.is_dir);
}

#[test]
fn mission_1_new_dir() {
    let dir = FsNode::new_dir("docs");
    assert_eq!(dir.name, "docs");
    assert!(dir.is_dir);
}

#[test]
fn mission_1_add_child() {
    let root = FsNode::new_dir("root");
    let file = FsNode::new_file("readme.txt");
    
    FsNode::add_child(&root, file);
    
    assert_eq!(root.children_count(), 1);
}

#[test]
fn mission_1_parent_name() {
    let root = FsNode::new_dir("root");
    let docs = FsNode::new_dir("docs");
    
    FsNode::add_child(&root, Rc::clone(&docs));
    
    assert_eq!(docs.parent_name(), Some("root".to_string()));
    assert_eq!(root.parent_name(), None);
}

#[test]
fn mission_1_full_path() {
    let root = FsNode::new_dir("root");
    let docs = FsNode::new_dir("docs");
    let readme = FsNode::new_file("readme.txt");
    
    FsNode::add_child(&root, Rc::clone(&docs));
    FsNode::add_child(&docs, Rc::clone(&readme));
    
    assert_eq!(readme.full_path(), "/root/docs/readme.txt");
    assert_eq!(docs.full_path(), "/root/docs");
    assert_eq!(root.full_path(), "/root");
}

#[test]
fn mission_1_weak_prevents_cycle() {
    let root = FsNode::new_dir("root");
    let child = FsNode::new_file("child.txt");
    
    FsNode::add_child(&root, Rc::clone(&child));
    
    // root의 strong count는 1 (원래 변수)
    // child의 strong count는 2 (원래 변수 + root의 children)
    assert_eq!(Rc::strong_count(&root), 1);
    assert_eq!(Rc::strong_count(&child), 2);
    
    // child의 parent는 Weak이므로 root의 strong count에 영향 없음
    assert_eq!(Rc::weak_count(&root), 1);
}

// =============================================================================
// Mission 2: EventBus (Rc<RefCell<...>>)
// =============================================================================

#[test]
fn mission_2_new_event_bus() {
    let bus = EventBus::new();
    assert_eq!(bus.listener_count(), 0);
}

#[test]
fn mission_2_subscribe() {
    let mut bus = EventBus::new();
    let logger: Rc<RefCell<dyn EventListener>> = Rc::new(RefCell::new(Logger::new("test")));
    
    bus.subscribe(logger);
    
    assert_eq!(bus.listener_count(), 1);
}

#[test]
fn mission_2_publish() {
    let mut bus = EventBus::new();
    let logger = Rc::new(RefCell::new(Logger::new("app")));
    let listener: Rc<RefCell<dyn EventListener>> = Rc::clone(&logger) as Rc<RefCell<dyn EventListener>>;
    
    bus.subscribe(listener);
    bus.publish("user_login");
    bus.publish("user_logout");
    
    let logs = &logger.borrow().logs;
    assert_eq!(logs.len(), 2);
    assert!(logs[0].contains("user_login"));
    assert!(logs[1].contains("user_logout"));
}

#[test]
fn mission_2_multiple_listeners() {
    let mut bus = EventBus::new();
    let logger1 = Rc::new(RefCell::new(Logger::new("logger1")));
    let logger2 = Rc::new(RefCell::new(Logger::new("logger2")));
    
    bus.subscribe(Rc::clone(&logger1) as Rc<RefCell<dyn EventListener>>);
    bus.subscribe(Rc::clone(&logger2) as Rc<RefCell<dyn EventListener>>);
    bus.publish("event");
    
    assert_eq!(logger1.borrow().logs.len(), 1);
    assert_eq!(logger2.borrow().logs.len(), 1);
}

// =============================================================================
// Mission 3: LruCache (Rc + RefCell + HashMap)
// =============================================================================

#[test]
fn mission_3_new_cache() {
    let cache: LruCache<i32> = LruCache::new(3);
    assert!(cache.is_empty());
    assert_eq!(cache.len(), 0);
}

#[test]
fn mission_3_put_and_get() {
    let cache: LruCache<String> = LruCache::new(3);
    
    cache.put("key1".to_string(), "value1".to_string());
    
    assert_eq!(cache.get("key1"), Some("value1".to_string()));
    assert_eq!(cache.get("key2"), None);
}

#[test]
fn mission_3_lru_eviction() {
    let cache: LruCache<i32> = LruCache::new(2);
    
    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    cache.put("c".to_string(), 3); // "a" should be evicted
    
    assert_eq!(cache.get("a"), None);
    assert_eq!(cache.get("b"), Some(2));
    assert_eq!(cache.get("c"), Some(3));
}

#[test]
fn mission_3_access_updates_lru() {
    let cache: LruCache<i32> = LruCache::new(2);
    
    cache.put("a".to_string(), 1);
    cache.put("b".to_string(), 2);
    cache.get("a"); // "a" becomes most recently used
    cache.put("c".to_string(), 3); // "b" should be evicted (oldest)
    
    assert_eq!(cache.get("a"), Some(1));
    assert_eq!(cache.get("b"), None);
    assert_eq!(cache.get("c"), Some(3));
}

// =============================================================================
// Mission 4: ConnectionPool (Arc<Mutex<Vec<...>>>)
// =============================================================================

#[test]
fn mission_4_new_pool() {
    let pool = ConnectionPool::new(3);
    assert_eq!(pool.available_count(), 3);
}

#[test]
fn mission_4_acquire_and_release() {
    let pool = ConnectionPool::new(2);
    
    let conn1 = pool.acquire();
    assert!(conn1.is_some());
    assert_eq!(pool.available_count(), 1);
    
    let conn2 = pool.acquire();
    assert!(conn2.is_some());
    assert_eq!(pool.available_count(), 0);
    
    // No more connections available
    let conn3 = pool.acquire();
    assert!(conn3.is_none());
    
    // Release one
    pool.release(conn1.unwrap());
    assert_eq!(pool.available_count(), 1);
}

#[test]
fn mission_4_execute() {
    let pool = ConnectionPool::new(2);
    
    let result = pool.execute("SELECT * FROM users");
    
    assert!(result.is_some());
    assert!(result.unwrap().contains("SELECT * FROM users"));
    assert_eq!(pool.available_count(), 2); // Connection should be released
}

#[test]
fn mission_4_concurrent_access() {
    let pool = ConnectionPool::new(3);
    let mut handles = vec![];
    
    for i in 0..6 {
        let p = pool.clone_pool();
        let handle = thread::spawn(move || {
            if let Some(conn_id) = p.acquire() {
                thread::sleep(Duration::from_millis(10));
                p.release(conn_id);
                format!("Thread {} used connection {}", i, conn_id)
            } else {
                format!("Thread {} couldn't get connection", i)
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let _ = handle.join().unwrap();
    }
    
    assert_eq!(pool.available_count(), 3);
}

// =============================================================================
// Mission 5: ConfigManager (Arc<RwLock<T>>)
// =============================================================================

#[test]
fn mission_5_new_config_manager() {
    let manager = ConfigManager::new();
    let config = manager.get_config();
    
    assert!(!config.debug_mode);
    assert_eq!(config.max_connections, 10);
}

#[test]
fn mission_5_with_config() {
    let config = Config {
        debug_mode: true,
        max_connections: 20,
        timeout_ms: 3000,
        app_name: "TestApp".to_string(),
    };
    
    let manager = ConfigManager::with_config(config);
    
    assert!(manager.is_debug());
    assert_eq!(manager.get_config().max_connections, 20);
}

#[test]
fn mission_5_update() {
    let manager = ConfigManager::new();
    
    manager.update(|config| {
        config.debug_mode = true;
        config.max_connections = 50;
    });
    
    assert!(manager.is_debug());
    assert_eq!(manager.get_config().max_connections, 50);
}

#[test]
fn mission_5_concurrent_read() {
    let manager = ConfigManager::with_config(Config {
        debug_mode: true,
        max_connections: 100,
        timeout_ms: 5000,
        app_name: "ConcurrentApp".to_string(),
    });
    
    let mut handles = vec![];
    
    for _ in 0..10 {
        let m = manager.clone_manager();
        let handle = thread::spawn(move || {
            m.get_config().max_connections
        });
        handles.push(handle);
    }
    
    for handle in handles {
        assert_eq!(handle.join().unwrap(), 100);
    }
}

// =============================================================================
// Mission 6: ResultCollector + parallel functions
// =============================================================================

#[test]
fn mission_6_new_collector() {
    let collector: ResultCollector<i32> = ResultCollector::new();
    assert_eq!(collector.count(), 0);
}

#[test]
fn mission_6_add_results() {
    let collector: ResultCollector<String> = ResultCollector::new();
    
    collector.add("result1".to_string());
    collector.add("result2".to_string());
    
    assert_eq!(collector.count(), 2);
}

#[test]
fn mission_6_get_results() {
    let collector: ResultCollector<i32> = ResultCollector::new();
    
    collector.add(1);
    collector.add(2);
    collector.add(3);
    
    let results = collector.get_results();
    assert_eq!(results.len(), 3);
    assert!(results.contains(&1));
    assert!(results.contains(&2));
    assert!(results.contains(&3));
}

#[test]
fn mission_6_concurrent_add() {
    let collector: ResultCollector<i32> = ResultCollector::new();
    let mut handles = vec![];
    
    for i in 0..10 {
        let c = collector.clone_collector();
        let handle = thread::spawn(move || {
            c.add(i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    assert_eq!(collector.count(), 10);
}

#[test]
fn mission_6_parallel_square() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut results = parallel_square(numbers);
    results.sort();
    
    assert_eq!(results, vec![1, 4, 9, 16, 25]);
}

#[test]
fn mission_6_parallel_with_channel() {
    let items = vec!["a", "b", "c"];
    let results = parallel_with_channel(items, |s: &str| s.to_uppercase());
    
    assert_eq!(results.len(), 3);
    assert!(results.contains(&"A".to_string()));
    assert!(results.contains(&"B".to_string()));
    assert!(results.contains(&"C".to_string()));
}
