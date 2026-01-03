use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

// =============================================================================
// 채팅 서버 시뮬레이터
//
// 이 프로젝트에서는 간단한 채팅 서버를 구현합니다.
// 모든 스마트 포인터 개념을 하나의 맥락에서 통합 연습합니다.
//
// 구조:
// - ChatServer: 전체 서버 관리
// - Room: 채팅방 (여러 사용자가 참여)
// - User: 사용자 (여러 방에 참여 가능)
// - Message: 메시지
// =============================================================================

// =============================================================================
// 임무 1: User와 Room 기본 구조
//
// Rc<RefCell<T>>로 단일 스레드에서 사용자-방 관계를 관리합니다.
// =============================================================================

pub type UserId = u64;
pub type RoomId = u64;

#[derive(Debug, Clone)]
pub struct Message {
    pub sender_id: UserId,
    pub sender_name: String,
    pub content: String,
    pub timestamp: u64,
}

impl Message {
    pub fn new(sender_id: UserId, sender_name: &str, content: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Message {
            sender_id,
            sender_name: sender_name.to_string(),
            content: content.to_string(),
            timestamp,
        }
    }
}

#[derive(Debug)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub joined_rooms: RefCell<Vec<Weak<RefCell<Room>>>>,
    pub inbox: RefCell<Vec<Message>>,
}

impl User {
    pub fn new(id: UserId, name: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(User {
            id: id,
            name: name.to_string(),
            joined_rooms: RefCell::new(Vec::new()),
            inbox: RefCell::new(Vec::new()),
        }))
    }

    pub fn join_room(&self, room: &Rc<RefCell<Room>>) {
        self.joined_rooms.borrow_mut().push(Rc::downgrade(room));
    }

    pub fn leave_room(&self, room_id: RoomId) {
        self.joined_rooms
            .borrow_mut()
            .retain(|room| room.upgrade().unwrap().borrow_mut().id != room_id);
    }

    pub fn room_count(&self) -> usize {
        self.joined_rooms
            .borrow()
            .iter()
            .filter(|room| room.upgrade().is_some())
            .count()
    }

    pub fn receive_message(&self, message: Message) {
        self.inbox.borrow_mut().push(message);
    }

    pub fn get_messages(&self) -> Vec<Message> {
        self.inbox.borrow().clone()
    }
}

#[derive(Debug)]
pub struct Room {
    pub id: RoomId,
    pub name: String,
    pub members: RefCell<Vec<Weak<RefCell<User>>>>,
    pub history: RefCell<Vec<Message>>,
}

impl Room {
    pub fn new(id: RoomId, name: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Room {
            id,
            name: name.to_owned(),
            members: RefCell::new(Vec::new()),
            history: RefCell::new(Vec::new()),
        }))
    }

    pub fn add_member(&self, user: &Rc<RefCell<User>>) {
        self.members.borrow_mut().push(Rc::downgrade(user))
    }

    pub fn remove_member(&self, user_id: UserId) {
        self.members
            .borrow_mut()
            .retain(|member| member.upgrade().unwrap().borrow().id != user_id);
    }

    pub fn member_count(&self) -> usize {
        self.members
            .borrow()
            .iter()
            .filter(|member| member.upgrade().is_some())
            .count()
    }

    pub fn broadcast(&self, message: Message) {
        self.members
            .borrow()
            .iter()
            .filter_map(|member| member.upgrade())
            .for_each(|member| member.borrow_mut().receive_message(message.clone()));

        self.history.borrow_mut().push(message);
    }

    pub fn get_history(&self) -> Vec<Message> {
        self.history.borrow().clone()
    }
}

// =============================================================================
// 임무 2: 싱글 스레드 ChatServer
//
// Rc<RefCell<T>>로 사용자와 방을 관리합니다.
// =============================================================================

pub struct SingleThreadChatServer {
    users: HashMap<UserId, Rc<RefCell<User>>>,
    rooms: HashMap<RoomId, Rc<RefCell<Room>>>,
    next_user_id: UserId,
    next_room_id: RoomId,
}

impl SingleThreadChatServer {
    pub fn new() -> Self {
        SingleThreadChatServer {
            users: HashMap::new(),
            rooms: HashMap::new(),
            next_user_id: 1,
            next_room_id: 1,
        }
    }

    pub fn create_user(&mut self, name: &str) -> UserId {
        let user = User::new(self.next_user_id, name);
        self.users.insert(self.next_user_id, user);
        self.next_user_id += 1;
        self.next_user_id - 1
    }

    pub fn create_room(&mut self, name: &str) -> RoomId {
        let room = Room::new(self.next_room_id, name);
        self.rooms.insert(self.next_room_id, room);
        self.next_room_id += 1;
        self.next_room_id - 1
    }

    pub fn join_room(&self, user_id: UserId, room_id: RoomId) -> Result<(), String> {
        let user = self.users.get(&user_id).ok_or("User not found")?;
        let room = self.rooms.get(&room_id).ok_or("Room not found")?;
        room.borrow_mut().add_member(user);
        user.borrow_mut().join_room(room);
        Ok(())
    }

    pub fn leave_room(&self, user_id: UserId, room_id: RoomId) -> Result<(), String> {
        let user = self.users.get(&user_id).ok_or("User not found")?;
        let room = self.rooms.get(&room_id).ok_or("Room not found")?;
        room.borrow_mut().remove_member(user_id);
        user.borrow_mut().leave_room(room_id);
        Ok(())
    }

    pub fn send_message(
        &self,
        user_id: UserId,
        room_id: RoomId,
        content: &str,
    ) -> Result<(), String> {
        let user = self.users.get(&user_id).ok_or("User not found")?;
        let room = self.rooms.get(&room_id).ok_or("Room not found")?;
        let sender_name = user.borrow().name.clone();
        room.borrow_mut()
            .broadcast(Message::new(user_id, &sender_name, content));
        Ok(())
    }

    pub fn get_user_messages(&self, user_id: UserId) -> Result<Vec<Message>, String> {
        let user = self.users.get(&user_id).ok_or("User not found")?;
        let messages = user.borrow().get_messages();
        Ok(messages)
    }

    pub fn get_room_history(&self, room_id: RoomId) -> Result<Vec<Message>, String> {
        let room = self.rooms.get(&room_id).ok_or("Room not found")?;
        let history = room.borrow().get_history();
        Ok(history)
    }

    pub fn user_count(&self) -> usize {
        self.users.len()
    }

    pub fn room_count(&self) -> usize {
        println!("Rooms: {:?}", self.rooms);
        self.rooms.len()
    }
}

impl Default for SingleThreadChatServer {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// 임무 3: 멀티 스레드 ChatServer
//
// Arc<RwLock<T>>와 Arc<Mutex<T>>로 스레드 안전하게 관리합니다.
// =============================================================================

#[derive(Debug, Clone)]
pub struct ThreadSafeUser {
    pub id: UserId,
    pub name: String,
    pub inbox: Arc<Mutex<Vec<Message>>>,
}

impl ThreadSafeUser {
    pub fn new(id: UserId, name: &str) -> Self {
        todo!("임무 3-1: ThreadSafeUser 생성")
    }

    pub fn receive_message(&self, message: Message) {
        todo!("임무 3-2: 메시지 수신 (스레드 안전)")
    }

    pub fn get_messages(&self) -> Vec<Message> {
        todo!("임무 3-3: 받은 메시지 조회")
    }

    pub fn message_count(&self) -> usize {
        todo!("임무 3-4: 받은 메시지 수")
    }
}

#[derive(Debug)]
pub struct ThreadSafeRoom {
    pub id: RoomId,
    pub name: String,
    pub members: RwLock<Vec<UserId>>,
    pub history: Mutex<Vec<Message>>,
}

impl ThreadSafeRoom {
    pub fn new(id: RoomId, name: &str) -> Self {
        todo!("임무 3-5: ThreadSafeRoom 생성")
    }

    pub fn add_member(&self, user_id: UserId) {
        todo!("임무 3-6: 멤버 추가")
    }

    pub fn remove_member(&self, user_id: UserId) {
        todo!("임무 3-7: 멤버 제거")
    }

    pub fn member_count(&self) -> usize {
        todo!("임무 3-8: 멤버 수")
    }

    pub fn has_member(&self, user_id: UserId) -> bool {
        todo!("임무 3-9: 멤버 여부 확인")
    }

    pub fn add_to_history(&self, message: Message) {
        todo!("임무 3-10: 히스토리에 메시지 추가")
    }

    pub fn get_history(&self) -> Vec<Message> {
        todo!("임무 3-11: 히스토리 조회")
    }

    pub fn get_member_ids(&self) -> Vec<UserId> {
        todo!("임무 3-12: 멤버 ID 목록")
    }
}

pub struct MultiThreadChatServer {
    users: Arc<RwLock<HashMap<UserId, Arc<ThreadSafeUser>>>>,
    rooms: Arc<RwLock<HashMap<RoomId, Arc<ThreadSafeRoom>>>>,
    next_user_id: Arc<Mutex<UserId>>,
    next_room_id: Arc<Mutex<RoomId>>,
}

impl MultiThreadChatServer {
    pub fn new() -> Self {
        todo!("임무 3-13: MultiThreadChatServer 생성")
    }

    pub fn create_user(&self, name: &str) -> UserId {
        todo!("임무 3-14: 사용자 생성 (스레드 안전)")
    }

    pub fn create_room(&self, name: &str) -> RoomId {
        todo!("임무 3-15: 방 생성 (스레드 안전)")
    }

    pub fn join_room(&self, user_id: UserId, room_id: RoomId) -> Result<(), String> {
        todo!("임무 3-16: 방 참여")
    }

    pub fn leave_room(&self, user_id: UserId, room_id: RoomId) -> Result<(), String> {
        todo!("임무 3-17: 방 나가기")
    }

    pub fn send_message(
        &self,
        user_id: UserId,
        room_id: RoomId,
        content: &str,
    ) -> Result<(), String> {
        todo!("임무 3-18: 메시지 보내기 (모든 멤버에게 전달)")
    }

    pub fn get_user(&self, user_id: UserId) -> Option<Arc<ThreadSafeUser>> {
        todo!("임무 3-19: 사용자 조회")
    }

    pub fn get_room(&self, room_id: RoomId) -> Option<Arc<ThreadSafeRoom>> {
        todo!("임무 3-20: 방 조회")
    }

    pub fn user_count(&self) -> usize {
        todo!("임무 3-21: 총 사용자 수")
    }

    pub fn room_count(&self) -> usize {
        todo!("임무 3-22: 총 방 수")
    }

    pub fn clone_server(&self) -> Self {
        MultiThreadChatServer {
            users: Arc::clone(&self.users),
            rooms: Arc::clone(&self.rooms),
            next_user_id: Arc::clone(&self.next_user_id),
            next_room_id: Arc::clone(&self.next_room_id),
        }
    }
}

impl Default for MultiThreadChatServer {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// 임무 4: 메시지 브로커 (Channel 기반)
//
// 별도 스레드에서 메시지를 처리하는 브로커를 구현합니다.
// =============================================================================

#[derive(Debug, Clone)]
pub enum ChatCommand {
    SendMessage {
        user_id: UserId,
        room_id: RoomId,
        content: String,
    },
    JoinRoom {
        user_id: UserId,
        room_id: RoomId,
    },
    LeaveRoom {
        user_id: UserId,
        room_id: RoomId,
    },
    Shutdown,
}

pub struct MessageBroker {
    sender: mpsc::Sender<ChatCommand>,
    handle: Option<thread::JoinHandle<()>>,
}

impl MessageBroker {
    pub fn new(server: MultiThreadChatServer) -> Self {
        todo!("임무 4-1: MessageBroker 생성 (워커 스레드 시작)")
    }

    pub fn send_command(&self, command: ChatCommand) {
        todo!("임무 4-2: 커맨드 전송")
    }

    pub fn shutdown(self) {
        todo!("임무 4-3: Graceful shutdown")
    }
}

// =============================================================================
// 임무 5: 통계 수집기
//
// 서버 통계를 실시간으로 수집합니다.
// =============================================================================

#[derive(Debug, Default, Clone)]
pub struct ServerStats {
    pub total_messages: u64,
    pub total_joins: u64,
    pub total_leaves: u64,
    pub active_users: u64,
    pub active_rooms: u64,
}

pub struct StatsCollector {
    stats: Arc<RwLock<ServerStats>>,
}

impl StatsCollector {
    pub fn new() -> Self {
        todo!("임무 5-1: StatsCollector 생성")
    }

    pub fn record_message(&self) {
        todo!("임무 5-2: 메시지 카운트 증가")
    }

    pub fn record_join(&self) {
        todo!("임무 5-3: 참여 카운트 증가")
    }

    pub fn record_leave(&self) {
        todo!("임무 5-4: 나가기 카운트 증가")
    }

    pub fn set_active_users(&self, count: u64) {
        todo!("임무 5-5: 활성 사용자 수 설정")
    }

    pub fn set_active_rooms(&self, count: u64) {
        todo!("임무 5-6: 활성 방 수 설정")
    }

    pub fn get_stats(&self) -> ServerStats {
        todo!("임무 5-7: 통계 조회")
    }

    pub fn clone_collector(&self) -> Self {
        StatsCollector {
            stats: Arc::clone(&self.stats),
        }
    }
}

impl Default for StatsCollector {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// 테스트 헬퍼
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let msg = Message::new(1, "Alice", "Hello!");
        assert_eq!(msg.sender_id, 1);
        assert_eq!(msg.sender_name, "Alice");
        assert_eq!(msg.content, "Hello!");
    }
}
