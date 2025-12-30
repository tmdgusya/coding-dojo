use rust_worker_dojo::*;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

// =============================================================================
// Mission 1: Box<T>
// =============================================================================

#[test]
fn mission_1_list_prepend() {
    let list = List::new().prepend(1);
    assert_eq!(list, List::Cons(1, Box::new(List::Nil)));
}

#[test]
fn mission_1_list_len() {
    let list = List::new().prepend(3).prepend(2).prepend(1);
    assert_eq!(list.len(), 3);
}

#[test]
fn mission_1_list_is_empty() {
    assert!(List::new().is_empty());
    assert!(!List::new().prepend(1).is_empty());
}

#[test]
fn mission_1_list_sum() {
    let list = List::new().prepend(3).prepend(2).prepend(1);
    assert_eq!(list.sum(), 6);

    let empty = List::new();
    assert_eq!(empty.sum(), 0);
}

// =============================================================================
// Mission 2: Rc<T>
// =============================================================================

#[test]
fn mission_2_shared_list_prepend() {
    let list = SharedList::new();
    let list = SharedList::prepend(list, 1);
    let list = SharedList::prepend(list, 2);
    
    match list.as_ref() {
        SharedList::Cons(val, _) => assert_eq!(*val, 2),
        SharedList::Nil => panic!("Expected Cons"),
    }
}

#[test]
fn mission_2_ref_count() {
    let a = SharedList::new();
    let a = SharedList::prepend(a, 1);
    assert_eq!(SharedList::ref_count(&a), 1);

    let b = SharedList::prepend(Rc::clone(&a), 2);
    assert_eq!(SharedList::ref_count(&a), 2);

    let c = SharedList::prepend(Rc::clone(&a), 3);
    assert_eq!(SharedList::ref_count(&a), 3);

    drop(b);
    assert_eq!(SharedList::ref_count(&a), 2);
}

// =============================================================================
// Mission 3: RefCell<T>
// =============================================================================

#[test]
fn mission_3_cache_basic() {
    let cache = Cache::new(|x| x * 2);
    
    assert_eq!(cache.get(5), 10);
    assert_eq!(cache.get(5), 10);
    assert_eq!(cache.cached_count(), 1);
}

#[test]
fn mission_3_cache_multiple_values() {
    let mut call_count = 0;
    let cache = Cache::new(|x| {
        x * x
    });
    
    assert_eq!(cache.get(2), 4);
    assert_eq!(cache.get(3), 9);
    assert_eq!(cache.get(2), 4);
    assert_eq!(cache.cached_count(), 2);
}

// =============================================================================
// Mission 4: Thread
// =============================================================================

#[test]
fn mission_4_parallel_sum() {
    let results = parallel_sum(vec![(0, 3), (10, 3)]);
    
    assert_eq!(results.len(), 2);
    assert!(results.contains(&3));
    assert!(results.contains(&33));
}

#[test]
fn mission_4_thread_greeting() {
    let greeting = thread_greeting("Rust".to_string());
    assert!(greeting.contains("Rust"));
}

// =============================================================================
// Mission 5: Channel
// =============================================================================

#[test]
fn mission_5_multi_producer() {
    let messages = multi_producer_single_consumer(3, "hello");
    
    assert_eq!(messages.len(), 3);
    for msg in messages {
        assert_eq!(msg, "hello");
    }
}

#[test]
fn mission_5_channel_map() {
    let numbers = vec![1, 2, 3, 4, 5];
    let squared = channel_map_square(numbers);
    
    assert_eq!(squared, vec![1, 4, 9, 16, 25]);
}

// =============================================================================
// Mission 6: Arc<Mutex<T>>
// =============================================================================

#[test]
fn mission_6_counter_basic() {
    let counter = Counter::new();
    assert_eq!(counter.get(), 0);
    
    counter.increment();
    counter.increment();
    assert_eq!(counter.get(), 2);
}

#[test]
fn mission_6_counter_clone() {
    let counter1 = Counter::new();
    let counter2 = counter1.clone_counter();
    
    counter1.increment();
    counter2.increment();
    
    assert_eq!(counter1.get(), 2);
    assert_eq!(counter2.get(), 2);
}

#[test]
fn mission_6_concurrent_increment() {
    let result = concurrent_increment(10, 100);
    assert_eq!(result, 1000);
}

// =============================================================================
// Mission 7: Worker Pool
// =============================================================================

#[test]
fn mission_7_thread_pool_basic() {
    use std::sync::{Arc, Mutex};
    
    let pool = ThreadPool::new(4);
    let counter = Arc::new(Mutex::new(0));
    
    for _ in 0..8 {
        let counter = Arc::clone(&counter);
        pool.execute(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
    }
    
    drop(pool);
    
    assert_eq!(*counter.lock().unwrap(), 8);
}

#[test]
fn mission_7_thread_pool_order_independent() {
    use std::sync::{Arc, Mutex};
    
    let pool = ThreadPool::new(2);
    let results = Arc::new(Mutex::new(Vec::new()));
    
    for i in 0..4 {
        let results = Arc::clone(&results);
        pool.execute(move || {
            thread::sleep(Duration::from_millis(10));
            results.lock().unwrap().push(i);
        });
    }
    
    drop(pool);
    
    let mut final_results = results.lock().unwrap().clone();
    final_results.sort();
    assert_eq!(final_results, vec![0, 1, 2, 3]);
}

#[test]
#[should_panic]
fn mission_7_thread_pool_zero_size() {
    ThreadPool::new(0);
}
