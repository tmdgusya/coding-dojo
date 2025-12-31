use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

// =============================================================================
// 임무 1: Box<T> - 힙에 데이터 저장하기
//
// Box는 데이터를 힙에 저장합니다.
// 재귀적 타입이나 크기를 알 수 없는 타입에 필수적입니다.
// =============================================================================

/// 재귀적 리스트 (Cons List)
///
/// 예시: 1 -> 2 -> 3 -> Nil
/// Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))))
#[derive(Debug, PartialEq)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    /// 빈 리스트 생성
    pub fn new() -> Self {
        List::Nil
    }

    /// 리스트 앞에 값 추가 (prepend)
    ///
    /// 예시: Nil.prepend(1) => Cons(1, Nil)
    pub fn prepend(self, value: i32) -> Self {
        List::Cons(value, Box::new(self))
    }

    /// 리스트 길이 계산
    ///
    /// 예시: Cons(1, Cons(2, Nil)).len() => 2
    pub fn len(&self) -> usize {
        match self {
            List::Cons(_, tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }

    /// 리스트가 비어있는지 확인
    pub fn is_empty(&self) -> bool {
        match self {
            List::Cons(_, _) => false,
            List::Nil => true,
        }
    }

    /// 리스트의 모든 값 합계
    ///
    /// 예시: Cons(1, Cons(2, Cons(3, Nil))).sum() => 6
    pub fn sum(&self) -> i32 {
        match self {
            List::Cons(head, tail) => head + tail.sum(),
            List::Nil => 0,
        }
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// 임무 2: Rc<T> - 참조 카운팅으로 여러 소유자
//
// 하나의 데이터를 여러 곳에서 소유해야 할 때 사용합니다.
// 그래프 구조나 공유 데이터에 유용합니다.
// =============================================================================

/// Rc를 사용한 공유 리스트
///
/// 구조:
///     a (3) -> b (2) -> c (1) -> Nil
///              ^
///     d (4) --┘
///
/// b는 a와 d 모두에서 공유됩니다.
#[derive(Debug)]
pub enum SharedList {
    Cons(i32, Rc<SharedList>),
    Nil,
}

impl SharedList {
    pub fn new() -> Rc<Self> {
        Rc::new(SharedList::Nil)
    }

    /// Rc로 감싼 리스트 앞에 값 추가
    ///
    /// 예시: SharedList::prepend(SharedList::new(), 1)
    pub fn prepend(list: Rc<Self>, value: i32) -> Rc<Self> {
        Rc::new(SharedList::Cons(value, list))
    }

    /// 현재 참조 카운트 반환
    ///
    /// 힌트: Rc::strong_count()
    pub fn ref_count(list: &Rc<Self>) -> usize {
        Rc::strong_count(list)
    }
}

// =============================================================================
// 임무 3: RefCell<T> - 내부 가변성
//
// 불변 참조를 통해 내부 값을 변경할 수 있게 합니다.
// 빌림 규칙이 런타임에 검사됩니다.
// =============================================================================

/// 간단한 캐시 구현
///
/// 계산 결과를 캐싱하여 중복 계산을 피합니다.
pub struct Cache<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    values: RefCell<std::collections::HashMap<u32, u32>>,
}

impl<T> Cache<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calculation: T) -> Self {
        Cache {
            calculation,
            values: (RefCell::new(HashMap::new())),
        }
    }

    /// 값을 가져오거나 계산하여 캐시
    ///
    /// - 이미 계산된 값이 있으면 캐시에서 반환
    /// - 없으면 계산하고 캐시에 저장 후 반환
    ///
    /// 힌트: borrow(), borrow_mut() 사용
    pub fn get(&self, arg: u32) -> u32 {
        if self.values.borrow().contains_key(&arg) {
            return *self.values.borrow().get(&arg).unwrap();
        }

        let value = (self.calculation)(arg);
        self.values.borrow_mut().insert(arg, value);
        value
    }

    /// 캐시된 항목 수
    pub fn cached_count(&self) -> usize {
        self.values.borrow().len()
    }
}

// =============================================================================
// 임무 4: Thread - 스레드 기초
//
// Rust의 스레드는 OS 스레드입니다.
// 소유권 시스템이 데이터 레이스를 컴파일 타임에 방지합니다.
// =============================================================================

/// 여러 스레드에서 각각 계산 수행
///
/// 각 스레드는 start부터 시작해서 count개의 숫자 합을 계산합니다.
///
/// 예시: parallel_sum(vec![(0, 3), (10, 3)])
///       => 스레드1: 0+1+2=3, 스레드2: 10+11+12=33
///       => 결과: [3, 33]
pub fn parallel_sum(ranges: Vec<(i32, i32)>) -> Vec<i32> {
    todo!("임무 4-1: 각 범위를 별도 스레드에서 계산하고 결과 수집하세요")
}

/// 스레드에서 문자열 생성하여 반환
///
/// 힌트: move 클로저 필요
pub fn thread_greeting(name: String) -> String {
    todo!("임무 4-2: 스레드에서 인사말 생성하여 반환하세요")
}

// =============================================================================
// 임무 5: Channel - 메시지 패싱
//
// "공유 메모리로 통신하지 말고, 통신으로 메모리를 공유하라"
// 채널은 스레드 간 안전한 통신 방법입니다.
// =============================================================================

/// 여러 생산자가 하나의 소비자에게 메시지 전송
///
/// num_producers개의 스레드가 각각 message를 전송합니다.
/// 모든 메시지를 수집하여 반환합니다.
///
/// 예시: multi_producer_single_consumer(3, "hello")
///       => ["hello", "hello", "hello"] (순서는 다를 수 있음)
pub fn multi_producer_single_consumer(num_producers: usize, message: &str) -> Vec<String> {
    todo!("임무 5-1: mpsc 채널로 여러 생산자 구현하세요")
}

/// 스레드에서 작업 결과를 채널로 전송
///
/// numbers의 각 숫자를 제곱하여 채널로 전송합니다.
/// 결과를 수집하여 정렬된 Vec으로 반환합니다.
pub fn channel_map_square(numbers: Vec<i32>) -> Vec<i32> {
    todo!("임무 5-2: 채널을 사용한 map 연산 구현하세요")
}

// =============================================================================
// 임무 6: Arc<Mutex<T>> - 스레드 안전한 공유 상태
//
// Arc = Atomic Reference Counting (Rc의 스레드 안전 버전)
// Mutex = Mutual Exclusion (상호 배제 락)
// =============================================================================

/// 스레드 안전한 카운터
pub struct Counter {
    count: Arc<Mutex<i32>>,
}

impl Counter {
    pub fn new() -> Self {
        todo!("임무 6-1: Counter 생성자 구현하세요")
    }

    /// 카운터 증가 (스레드 안전)
    pub fn increment(&self) {
        todo!("임무 6-2: lock()을 사용해 카운터 증가하세요")
    }

    /// 현재 값 반환
    pub fn get(&self) -> i32 {
        todo!("임무 6-3: 현재 카운터 값 반환하세요")
    }

    /// Arc 클론 반환 (다른 스레드에서 사용)
    pub fn clone_counter(&self) -> Self {
        Counter {
            count: Arc::clone(&self.count),
        }
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

/// 여러 스레드에서 동시에 카운터 증가
///
/// num_threads개의 스레드가 각각 increments_per_thread번 증가시킵니다.
/// 최종 카운터 값을 반환합니다.
///
/// 예시: concurrent_increment(4, 100) => 400
pub fn concurrent_increment(num_threads: usize, increments_per_thread: i32) -> i32 {
    todo!("임무 6-4: 여러 스레드에서 안전하게 카운터 증가시키세요")
}

// =============================================================================
// 임무 7: Worker Pool (최종 임무)
//
// 지금까지 배운 모든 개념을 통합합니다:
// - Arc, Mutex: 공유 상태
// - Channel: 작업 전달
// - Thread: 워커 스레드
// =============================================================================

/// 작업 타입 (채널로 전송할 클로저)
type Job = Box<dyn FnOnce() + Send + 'static>;

/// 워커 - 작업을 받아 실행하는 스레드
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

/// 스레드 풀
///
/// 사용 예시:
/// ```ignore
/// let pool = ThreadPool::new(4);
///
/// pool.execute(|| {
///     println!("작업 1 실행!");
/// });
///
/// pool.execute(|| {
///     println!("작업 2 실행!");
/// });
/// ```
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl Worker {
    /// 새 워커 생성
    ///
    /// receiver에서 작업을 받아 실행하는 스레드를 생성합니다.
    ///
    /// 힌트:
    /// - receiver는 Arc<Mutex<mpsc::Receiver<Job>>>
    /// - 루프에서 lock(), recv() 사용
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        todo!("임무 7-1: Worker 생성 - 작업을 받아 실행하는 스레드")
    }
}

impl ThreadPool {
    /// 지정된 크기의 스레드 풀 생성
    ///
    /// 힌트:
    /// - mpsc::channel()로 작업 전달용 채널 생성
    /// - receiver를 Arc<Mutex<...>>로 감싸서 워커들이 공유
    /// - size개의 Worker 생성
    ///
    /// # Panics
    /// size가 0이면 panic
    pub fn new(size: usize) -> ThreadPool {
        todo!("임무 7-2: ThreadPool 생성")
    }

    /// 작업 실행 요청
    ///
    /// 클로저를 받아 워커 스레드에서 실행합니다.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        todo!("임무 7-3: 작업을 채널로 전송하세요")
    }
}

impl Drop for ThreadPool {
    /// 풀 정리 - Graceful shutdown
    ///
    /// 힌트:
    /// 1. sender를 drop하여 채널 닫기
    /// 2. 모든 워커 스레드 join
    fn drop(&mut self) {
        todo!("임무 7-4: Graceful shutdown 구현하세요")
    }
}

// =============================================================================
// 테스트 헬퍼
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_basic() {
        let list = List::new().prepend(3).prepend(2).prepend(1);
        assert_eq!(list.len(), 3);
        assert_eq!(list.sum(), 6);
    }
}
