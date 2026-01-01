use rust_closure_traits_dojo::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// =============================================================================
// Mission 1: Fn, FnMut, FnOnce
// =============================================================================

#[test]
fn mission_1_1_fn_once_consume() {
    let data = String::from("consumed");
    let consume = || data;

    let result = consume_and_return(consume);
    assert_eq!(result, "consumed");
}

#[test]
fn mission_1_1_demo_fn_once() {
    let result = demo_fn_once();
    assert_eq!(result, "I will be consumed");
}

#[test]
fn mission_1_2_fn_mut_accumulate() {
    let mut count = 0;
    let increment = || {
        count += 1;
        count
    };

    let result = call_and_accumulate(increment);
    assert_eq!(result, 6);
}

#[test]
fn mission_1_2_demo_fn_mut() {
    let result = demo_fn_mut();
    assert_eq!(result, 6);
}

#[test]
fn mission_1_3_fn_many_times() {
    let value = 42;
    let get_value = || value;

    let results = call_many_times(get_value, 4);
    assert_eq!(results, vec![42, 42, 42, 42]);
}

#[test]
fn mission_1_3_demo_fn() {
    let result = demo_fn();
    assert_eq!(result, vec![20, 20, 20]);
}

#[test]
fn mission_1_4_repeater() {
    let counter = std::sync::atomic::AtomicI32::new(0);
    let repeater = Repeater::new(
        || counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1,
        5,
    );

    let results = repeater.run();
    assert_eq!(results.len(), 5);
}

#[test]
fn mission_1_5_run_once() {
    let result: i32 = run_once(|| 42);
    assert_eq!(result, 42);

    let s = String::from("hello");
    let result: String = run_once(|| s);
    assert_eq!(result, "hello");
}

// =============================================================================
// Mission 2: dyn Trait
// =============================================================================

#[test]
fn mission_2_animal_shelter() {
    let mut shelter = AnimalShelter::new();

    shelter.add(Dog {
        name: "멍멍이".to_string(),
    });
    shelter.add(Cat {
        name: "야옹이".to_string(),
    });
    shelter.add(Dog {
        name: "바둑이".to_string(),
    });

    assert_eq!(shelter.count(), 3);

    let sounds = shelter.all_speak();
    assert!(sounds.contains(&"멍멍!".to_string()));
    assert!(sounds.contains(&"야옹~".to_string()));
}

#[test]
fn mission_2_dyn_calculator() {
    let mut calc = DynCalculator::new(|a, b| a + b);
    assert_eq!(calc.calculate(2, 3), 5);

    calc.set_operation(|a, b| a * b);
    assert_eq!(calc.calculate(2, 3), 6);

    calc.set_operation(|a, b| a - b);
    assert_eq!(calc.calculate(10, 3), 7);
}

// =============================================================================
// Mission 3: 'static
// =============================================================================

#[test]
fn mission_3_static_str() {
    let s: &'static str = get_static_str();
    assert!(!s.is_empty());
}

#[test]
fn mission_3_spawn_with_static() {
    let value = String::from("Hello from thread");
    let result = spawn_with_static(value);
    assert_eq!(result, "Hello from thread");

    let number = 42i32;
    let result = spawn_with_static(number);
    assert_eq!(result, 42);
}

// =============================================================================
// Mission 4: Send + Sync
// =============================================================================

#[test]
fn mission_4_send_sync_basics() {
    require_send(String::from("hello"));
    require_send(42i32);
    require_send(vec![1, 2, 3]);

    let s = String::from("hello");
    require_sync(&s);

    let arc = Arc::new(Mutex::new(0));
    require_send(arc.clone());
    require_sync(&arc);
}

#[test]
fn mission_4_share_counter() {
    let result = share_counter_between_threads();
    assert_eq!(result, 2);
}

// =============================================================================
// Mission 5: Box<dyn FnOnce() + Send + 'static>
// =============================================================================

#[test]
fn mission_5_create_job() {
    let executed = Arc::new(Mutex::new(false));
    let executed_clone = Arc::clone(&executed);

    let job = create_job(move || {
        *executed_clone.lock().unwrap() = true;
    });

    execute_job(job);

    assert!(*executed.lock().unwrap());
}

#[test]
fn mission_5_execute_jobs_in_thread() {
    let counter = Arc::new(Mutex::new(0));

    let mut jobs: Vec<Job> = Vec::new();
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        jobs.push(create_job(move || {
            *counter.lock().unwrap() += 1;
        }));
    }

    execute_jobs_in_thread(jobs);

    thread::sleep(Duration::from_millis(100));
    assert_eq!(*counter.lock().unwrap(), 5);
}

#[test]
fn mission_5_execute_with_result() {
    let result = execute_with_result(|| {
        thread::sleep(Duration::from_millis(10));
        42
    });
    assert_eq!(result, 42);

    let result = execute_with_result(|| String::from("Hello from thread"));
    assert_eq!(result, "Hello from thread");
}
