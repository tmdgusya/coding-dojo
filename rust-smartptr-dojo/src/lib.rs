use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;

// =============================================================================
// 임무 1: 파일 시스템 트리 - Rc<RefCell<T>> + Weak
//
// 트리 구조에서 부모-자식 관계를 표현합니다.
// - 자식 → 부모: Weak (순환 참조 방지)
// - 부모 → 자식: Rc (소유권)
// =============================================================================

/// 파일 시스템 노드
///
/// 디렉토리는 자식 노드들을 가질 수 있고,
/// 모든 노드는 부모를 참조할 수 있습니다 (Weak로).
#[derive(Debug)]
pub struct FsNode {
    pub name: String,
    pub is_dir: bool,
    pub children: RefCell<Vec<Rc<FsNode>>>,
    pub parent: RefCell<Weak<FsNode>>,
}

impl FsNode {
    /// 새 파일 노드 생성
    pub fn new_file(name: &str) -> Rc<Self> {
        Rc::new(FsNode {
            name: name.to_owned(),
            is_dir: false,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
        })
    }

    /// 새 디렉토리 노드 생성
    pub fn new_dir(name: &str) -> Rc<Self> {
        Rc::new(FsNode {
            name: name.to_owned(),
            is_dir: true,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
        })
    }

    /// 디렉토리에 자식 추가
    ///
    /// - 부모(self)의 children에 child 추가
    /// - child의 parent를 self로 설정 (Weak 참조)
    ///
    /// 힌트: Rc::downgrade()로 Weak 생성
    pub fn add_child(parent: &Rc<Self>, child: Rc<FsNode>) {
        parent.children.borrow_mut().push(Rc::clone(&child));
        *child.parent.borrow_mut() = Rc::downgrade(parent);
    }

    /// 부모 노드 이름 반환
    ///
    /// 부모가 없으면 None
    ///
    /// 힌트: Weak::upgrade()로 Rc 복원 시도
    pub fn parent_name(&self) -> Option<String> {
        match self.parent.borrow().upgrade() {
            Some(parent) => Some(parent.name.to_owned()),
            None => None,
        }
    }

    /// 전체 경로 반환 (루트부터 현재 노드까지)
    ///
    /// 예: "/root/docs/readme.txt"
    pub fn full_path(&self) -> String {
        match self.parent.borrow().upgrade() {
            Some(parent) => format!("{}/{}", parent.full_path(), self.name),
            None => format!("/{}", self.name),
        }
    }

    /// 자식 수 반환
    pub fn children_count(&self) -> usize {
        self.children.borrow().iter().count()
    }
}

// =============================================================================
// 임무 2: 이벤트 버스 (옵저버 패턴) - Rc<RefCell<...>>
//
// 이벤트를 발행하면 모든 구독자에게 전달됩니다.
// =============================================================================

/// 이벤트 리스너 트레이트
pub trait EventListener {
    fn on_event(&mut self, event: &str);
    fn name(&self) -> &str;
}

/// 간단한 로거 리스너
pub struct Logger {
    pub name: String,
    pub logs: Vec<String>,
}

impl Logger {
    pub fn new(name: &str) -> Self {
        Logger {
            name: name.to_string(),
            logs: Vec::new(),
        }
    }
}

impl EventListener for Logger {
    fn on_event(&mut self, event: &str) {
        self.logs.push(format!("[{}] {}", self.name, event));
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// 이벤트 버스
///
/// 여러 리스너를 등록하고 이벤트를 브로드캐스트합니다.
pub struct EventBus {
    listeners: Vec<Rc<RefCell<dyn EventListener>>>,
}

impl EventBus {
    pub fn new() -> Self {
        todo!("임무 2-1: EventBus 생성")
    }

    /// 리스너 등록
    ///
    /// 힌트: Rc<RefCell<dyn EventListener>>로 다양한 리스너 타입 저장
    pub fn subscribe(&mut self, listener: Rc<RefCell<dyn EventListener>>) {
        todo!("임무 2-2: 리스너 등록")
    }

    /// 모든 리스너에게 이벤트 전달
    ///
    /// 힌트: borrow_mut()로 각 리스너의 on_event 호출
    pub fn publish(&self, event: &str) {
        todo!("임무 2-3: 이벤트 브로드캐스트")
    }

    /// 등록된 리스너 수
    pub fn listener_count(&self) -> usize {
        todo!("임무 2-4: 리스너 수 반환")
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// 임무 3: LRU 캐시 - Rc + RefCell + HashMap
//
// 가장 최근에 사용된 항목을 추적하는 캐시입니다.
// 용량 초과 시 가장 오래된 항목을 제거합니다.
// =============================================================================

/// LRU 캐시 노드
#[derive(Debug)]
pub struct CacheNode<V> {
    pub key: String,
    pub value: V,
    pub prev: RefCell<Option<Rc<CacheNode<V>>>>,
    pub next: RefCell<Option<Rc<CacheNode<V>>>>,
}

/// LRU 캐시
///
/// - get: 항목 조회 + 최근 사용으로 이동
/// - put: 항목 추가 (용량 초과 시 LRU 제거)
pub struct LruCache<V> {
    capacity: usize,
    map: RefCell<HashMap<String, Rc<CacheNode<V>>>>,
    // head: 가장 최근 사용
    head: RefCell<Option<Rc<CacheNode<V>>>>,
    // tail: 가장 오래된 (제거 대상)
    tail: RefCell<Option<Rc<CacheNode<V>>>>,
}

impl<V: Clone> LruCache<V> {
    /// 새 캐시 생성
    pub fn new(capacity: usize) -> Self {
        todo!("임무 3-1: LruCache 생성")
    }

    /// 값 조회 (사용 시 최근으로 이동)
    pub fn get(&self, key: &str) -> Option<V> {
        todo!("임무 3-2: 캐시 조회")
    }

    /// 값 저장 (용량 초과 시 LRU 제거)
    pub fn put(&self, key: String, value: V) {
        todo!("임무 3-3: 캐시 저장")
    }

    /// 현재 캐시 크기
    pub fn len(&self) -> usize {
        todo!("임무 3-4: 캐시 크기")
    }

    /// 캐시가 비어있는지
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// =============================================================================
// 임무 4: 연결 풀 - Arc<Mutex<Vec<...>>>
//
// 여러 스레드에서 공유하는 연결 풀입니다.
// =============================================================================

/// 가상의 데이터베이스 연결
#[derive(Debug)]
pub struct Connection {
    pub id: usize,
    pub in_use: bool,
}

impl Connection {
    pub fn new(id: usize) -> Self {
        Connection { id, in_use: false }
    }

    pub fn query(&self, sql: &str) -> String {
        format!("Connection {}: executed '{}'", self.id, sql)
    }
}

/// 연결 풀
///
/// 여러 스레드에서 안전하게 연결을 가져오고 반납합니다.
pub struct ConnectionPool {
    connections: Arc<Mutex<Vec<Connection>>>,
    max_size: usize,
}

impl ConnectionPool {
    /// 풀 생성 (초기 연결 수 지정)
    pub fn new(size: usize) -> Self {
        todo!("임무 4-1: ConnectionPool 생성")
    }

    /// 연결 획득
    ///
    /// 사용 가능한 연결이 있으면 반환, 없으면 None
    ///
    /// 힌트: in_use = false인 연결 찾아서 true로 변경
    pub fn acquire(&self) -> Option<usize> {
        todo!("임무 4-2: 연결 획득")
    }

    /// 연결 반납
    pub fn release(&self, conn_id: usize) {
        todo!("임무 4-3: 연결 반납")
    }

    /// 사용 가능한 연결 수
    pub fn available_count(&self) -> usize {
        todo!("임무 4-4: 사용 가능 연결 수")
    }

    /// 쿼리 실행 (연결 획득 → 실행 → 반납)
    pub fn execute(&self, sql: &str) -> Option<String> {
        todo!("임무 4-5: 쿼리 실행")
    }

    /// Arc 클론 반환 (다른 스레드에서 사용)
    pub fn clone_pool(&self) -> Self {
        ConnectionPool {
            connections: Arc::clone(&self.connections),
            max_size: self.max_size,
        }
    }
}

// =============================================================================
// 임무 5: 공유 설정 - Arc<RwLock<T>>
//
// 읽기가 많고 쓰기가 적은 설정을 공유합니다.
// RwLock은 여러 읽기 또는 하나의 쓰기를 허용합니다.
// =============================================================================

/// 애플리케이션 설정
#[derive(Debug, Clone)]
pub struct Config {
    pub debug_mode: bool,
    pub max_connections: usize,
    pub timeout_ms: u64,
    pub app_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            debug_mode: false,
            max_connections: 10,
            timeout_ms: 5000,
            app_name: "MyApp".to_string(),
        }
    }
}

/// 공유 설정 관리자
pub struct ConfigManager {
    config: Arc<RwLock<Config>>,
}

impl ConfigManager {
    /// 기본 설정으로 생성
    pub fn new() -> Self {
        todo!("임무 5-1: ConfigManager 생성")
    }

    /// 설정으로 생성
    pub fn with_config(config: Config) -> Self {
        todo!("임무 5-2: 설정과 함께 생성")
    }

    /// 설정 읽기 (읽기 락)
    ///
    /// 힌트: read().unwrap()
    pub fn get_config(&self) -> Config {
        todo!("임무 5-3: 설정 읽기")
    }

    /// debug_mode만 읽기
    pub fn is_debug(&self) -> bool {
        todo!("임무 5-4: debug_mode 읽기")
    }

    /// 설정 업데이트 (쓰기 락)
    ///
    /// 힌트: write().unwrap()
    pub fn update<F>(&self, updater: F)
    where
        F: FnOnce(&mut Config),
    {
        todo!("임무 5-5: 설정 업데이트")
    }

    /// Arc 클론 반환
    pub fn clone_manager(&self) -> Self {
        ConfigManager {
            config: Arc::clone(&self.config),
        }
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// 임무 6: 병렬 작업 수집기 - Arc<Mutex<T>> + channel
//
// 여러 스레드에서 작업을 수행하고 결과를 수집합니다.
// =============================================================================

/// 작업 결과 수집기
pub struct ResultCollector<T> {
    results: Arc<Mutex<Vec<T>>>,
}

impl<T: Send + 'static> ResultCollector<T> {
    pub fn new() -> Self {
        todo!("임무 6-1: ResultCollector 생성")
    }

    /// 결과 추가 (스레드 안전)
    pub fn add(&self, result: T) {
        todo!("임무 6-2: 결과 추가")
    }

    /// 모든 결과 반환
    pub fn get_results(&self) -> Vec<T>
    where
        T: Clone,
    {
        todo!("임무 6-3: 결과 반환")
    }

    /// 결과 수
    pub fn count(&self) -> usize {
        todo!("임무 6-4: 결과 수")
    }

    /// Arc 클론
    pub fn clone_collector(&self) -> Self {
        ResultCollector {
            results: Arc::clone(&self.results),
        }
    }
}

impl<T: Send + 'static> Default for ResultCollector<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 병렬로 작업 실행하고 결과 수집
///
/// 각 숫자의 제곱을 계산하는 작업을 병렬로 실행합니다.
pub fn parallel_square(numbers: Vec<i32>) -> Vec<i32> {
    todo!("임무 6-5: 병렬 제곱 계산")
}

/// 채널을 사용한 병렬 작업
///
/// 각 작업의 결과를 채널로 전송하고 수집합니다.
pub fn parallel_with_channel<T, F>(items: Vec<T>, worker: F) -> Vec<String>
where
    T: Send + 'static,
    F: Fn(T) -> String + Send + 'static + Clone,
{
    todo!("임무 6-6: 채널로 병렬 작업")
}

// =============================================================================
// 테스트 헬퍼
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fs_node_basic() {
        let root = FsNode::new_dir("root");
        assert_eq!(root.name, "root");
        assert!(root.is_dir);
    }
}
