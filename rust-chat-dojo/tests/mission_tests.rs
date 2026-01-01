use rust_chat_dojo::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// =============================================================================
// Mission 1: User와 Room 기본 구조
// =============================================================================

#[test]
fn mission_1_create_user() {
    let user = User::new(1, "Alice");
    let borrowed = user.borrow();
    
    assert_eq!(borrowed.id, 1);
    assert_eq!(borrowed.name, "Alice");
    assert_eq!(borrowed.room_count(), 0);
}

#[test]
fn mission_1_create_room() {
    let room = Room::new(1, "general");
    let borrowed = room.borrow();
    
    assert_eq!(borrowed.id, 1);
    assert_eq!(borrowed.name, "general");
    assert_eq!(borrowed.member_count(), 0);
}

#[test]
fn mission_1_join_room() {
    let user = User::new(1, "Alice");
    let room = Room::new(1, "general");
    
    user.borrow().join_room(&room);
    room.borrow().add_member(&user);
    
    assert_eq!(user.borrow().room_count(), 1);
    assert_eq!(room.borrow().member_count(), 1);
}

#[test]
fn mission_1_leave_room() {
    let user = User::new(1, "Alice");
    let room = Room::new(1, "general");
    
    user.borrow().join_room(&room);
    room.borrow().add_member(&user);
    
    user.borrow().leave_room(1);
    room.borrow().remove_member(1);
    
    assert_eq!(user.borrow().room_count(), 0);
    assert_eq!(room.borrow().member_count(), 0);
}

#[test]
fn mission_1_weak_reference() {
    let user = User::new(1, "Alice");
    let room = Room::new(1, "general");
    
    user.borrow().join_room(&room);
    room.borrow().add_member(&user);
    
    assert_eq!(Rc::strong_count(&user), 1);
    assert_eq!(Rc::strong_count(&room), 1);
}

#[test]
fn mission_1_broadcast_message() {
    let alice = User::new(1, "Alice");
    let bob = User::new(2, "Bob");
    let room = Room::new(1, "general");
    
    alice.borrow().join_room(&room);
    bob.borrow().join_room(&room);
    room.borrow().add_member(&alice);
    room.borrow().add_member(&bob);
    
    let msg = Message::new(1, "Alice", "Hello everyone!");
    room.borrow().broadcast(msg);
    
    assert_eq!(alice.borrow().get_messages().len(), 1);
    assert_eq!(bob.borrow().get_messages().len(), 1);
}

#[test]
fn mission_1_room_history() {
    let alice = User::new(1, "Alice");
    let room = Room::new(1, "general");
    
    alice.borrow().join_room(&room);
    room.borrow().add_member(&alice);
    
    let msg = Message::new(1, "Alice", "Hello!");
    room.borrow().broadcast(msg);
    
    let history = room.borrow().get_history();
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].content, "Hello!");
}

// =============================================================================
// Mission 2: SingleThreadChatServer
// =============================================================================

#[test]
fn mission_2_create_server() {
    let server = SingleThreadChatServer::new();
    assert_eq!(server.user_count(), 0);
    assert_eq!(server.room_count(), 0);
}

#[test]
fn mission_2_create_user() {
    let mut server = SingleThreadChatServer::new();
    let user_id = server.create_user("Alice");
    
    assert_eq!(user_id, 1);
    assert_eq!(server.user_count(), 1);
}

#[test]
fn mission_2_create_room() {
    let mut server = SingleThreadChatServer::new();
    let room_id = server.create_room("general");
    
    assert_eq!(room_id, 1);
    assert_eq!(server.room_count(), 1);
}

#[test]
fn mission_2_join_and_leave() {
    let mut server = SingleThreadChatServer::new();
    let user_id = server.create_user("Alice");
    let room_id = server.create_room("general");
    
    assert!(server.join_room(user_id, room_id).is_ok());
    assert!(server.leave_room(user_id, room_id).is_ok());
}

#[test]
fn mission_2_send_message() {
    let mut server = SingleThreadChatServer::new();
    let alice_id = server.create_user("Alice");
    let bob_id = server.create_user("Bob");
    let room_id = server.create_room("general");
    
    server.join_room(alice_id, room_id).unwrap();
    server.join_room(bob_id, room_id).unwrap();
    
    assert!(server.send_message(alice_id, room_id, "Hello!").is_ok());
    
    let bob_messages = server.get_user_messages(bob_id).unwrap();
    assert_eq!(bob_messages.len(), 1);
    assert_eq!(bob_messages[0].content, "Hello!");
}

#[test]
fn mission_2_room_history() {
    let mut server = SingleThreadChatServer::new();
    let alice_id = server.create_user("Alice");
    let room_id = server.create_room("general");
    
    server.join_room(alice_id, room_id).unwrap();
    server.send_message(alice_id, room_id, "Hello!").unwrap();
    server.send_message(alice_id, room_id, "How are you?").unwrap();
    
    let history = server.get_room_history(room_id).unwrap();
    assert_eq!(history.len(), 2);
}

#[test]
fn mission_2_error_handling() {
    let mut server = SingleThreadChatServer::new();
    let user_id = server.create_user("Alice");
    
    assert!(server.join_room(user_id, 999).is_err());
    assert!(server.join_room(999, 1).is_err());
}

// =============================================================================
// Mission 3: MultiThreadChatServer
// =============================================================================

#[test]
fn mission_3_create_thread_safe_user() {
    let user = ThreadSafeUser::new(1, "Alice");
    
    assert_eq!(user.id, 1);
    assert_eq!(user.name, "Alice");
    assert_eq!(user.message_count(), 0);
}

#[test]
fn mission_3_create_thread_safe_room() {
    let room = ThreadSafeRoom::new(1, "general");
    
    assert_eq!(room.id, 1);
    assert_eq!(room.name, "general");
    assert_eq!(room.member_count(), 0);
}

#[test]
fn mission_3_room_operations() {
    let room = ThreadSafeRoom::new(1, "general");
    
    room.add_member(1);
    room.add_member(2);
    
    assert_eq!(room.member_count(), 2);
    assert!(room.has_member(1));
    assert!(room.has_member(2));
    assert!(!room.has_member(3));
    
    room.remove_member(1);
    assert_eq!(room.member_count(), 1);
    assert!(!room.has_member(1));
}

#[test]
fn mission_3_multi_thread_server() {
    let server = MultiThreadChatServer::new();
    
    assert_eq!(server.user_count(), 0);
    assert_eq!(server.room_count(), 0);
    
    let user_id = server.create_user("Alice");
    let room_id = server.create_room("general");
    
    assert_eq!(server.user_count(), 1);
    assert_eq!(server.room_count(), 1);
    assert!(server.get_user(user_id).is_some());
    assert!(server.get_room(room_id).is_some());
}

#[test]
fn mission_3_concurrent_user_creation() {
    let server = MultiThreadChatServer::new();
    let mut handles = vec![];
    
    for i in 0..10 {
        let s = server.clone_server();
        let handle = thread::spawn(move || {
            s.create_user(&format!("User{}", i))
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    assert_eq!(server.user_count(), 10);
}

#[test]
fn mission_3_concurrent_messaging() {
    let server = MultiThreadChatServer::new();
    
    let alice_id = server.create_user("Alice");
    let bob_id = server.create_user("Bob");
    let room_id = server.create_room("general");
    
    server.join_room(alice_id, room_id).unwrap();
    server.join_room(bob_id, room_id).unwrap();
    
    let mut handles = vec![];
    
    for i in 0..5 {
        let s = server.clone_server();
        let handle = thread::spawn(move || {
            s.send_message(alice_id, room_id, &format!("Message {}", i)).unwrap();
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let bob = server.get_user(bob_id).unwrap();
    assert_eq!(bob.message_count(), 5);
}

#[test]
fn mission_3_concurrent_read() {
    let server = MultiThreadChatServer::new();
    let user_id = server.create_user("Alice");
    let room_id = server.create_room("general");
    
    server.join_room(user_id, room_id).unwrap();
    
    let mut handles = vec![];
    
    for _ in 0..10 {
        let s = server.clone_server();
        let handle = thread::spawn(move || {
            s.get_user(user_id).is_some()
        });
        handles.push(handle);
    }
    
    for handle in handles {
        assert!(handle.join().unwrap());
    }
}

// =============================================================================
// Mission 4: MessageBroker
// =============================================================================

#[test]
fn mission_4_create_broker() {
    let server = MultiThreadChatServer::new();
    let broker = MessageBroker::new(server);
    
    broker.shutdown();
}

#[test]
fn mission_4_send_commands() {
    let server = MultiThreadChatServer::new();
    let user_id = server.create_user("Alice");
    let room_id = server.create_room("general");
    
    let broker = MessageBroker::new(server.clone_server());
    
    broker.send_command(ChatCommand::JoinRoom { user_id, room_id });
    thread::sleep(Duration::from_millis(50));
    
    let room = server.get_room(room_id).unwrap();
    assert!(room.has_member(user_id));
    
    broker.shutdown();
}

#[test]
fn mission_4_send_message_via_broker() {
    let server = MultiThreadChatServer::new();
    let alice_id = server.create_user("Alice");
    let bob_id = server.create_user("Bob");
    let room_id = server.create_room("general");
    
    server.join_room(alice_id, room_id).unwrap();
    server.join_room(bob_id, room_id).unwrap();
    
    let broker = MessageBroker::new(server.clone_server());
    
    broker.send_command(ChatCommand::SendMessage {
        user_id: alice_id,
        room_id,
        content: "Hello via broker!".to_string(),
    });
    
    thread::sleep(Duration::from_millis(50));
    
    let bob = server.get_user(bob_id).unwrap();
    assert!(bob.message_count() > 0);
    
    broker.shutdown();
}

// =============================================================================
// Mission 5: StatsCollector
// =============================================================================

#[test]
fn mission_5_create_collector() {
    let collector = StatsCollector::new();
    let stats = collector.get_stats();
    
    assert_eq!(stats.total_messages, 0);
    assert_eq!(stats.total_joins, 0);
    assert_eq!(stats.total_leaves, 0);
}

#[test]
fn mission_5_record_stats() {
    let collector = StatsCollector::new();
    
    collector.record_message();
    collector.record_message();
    collector.record_join();
    collector.record_leave();
    
    let stats = collector.get_stats();
    assert_eq!(stats.total_messages, 2);
    assert_eq!(stats.total_joins, 1);
    assert_eq!(stats.total_leaves, 1);
}

#[test]
fn mission_5_concurrent_recording() {
    let collector = StatsCollector::new();
    let mut handles = vec![];
    
    for _ in 0..10 {
        let c = collector.clone_collector();
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                c.record_message();
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let stats = collector.get_stats();
    assert_eq!(stats.total_messages, 1000);
}

#[test]
fn mission_5_set_active_counts() {
    let collector = StatsCollector::new();
    
    collector.set_active_users(42);
    collector.set_active_rooms(5);
    
    let stats = collector.get_stats();
    assert_eq!(stats.active_users, 42);
    assert_eq!(stats.active_rooms, 5);
}
