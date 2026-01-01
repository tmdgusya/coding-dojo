use std::thread;

// =============================================================================
// 임무 1: 클로저 트레이트 - Fn, FnMut, FnOnce
//
// 모든 클로저는 자동으로 이 트레이트 중 하나 이상을 구현합니다.
// 어떤 트레이트를 구현하는지는 클로저가 캡처한 값을 어떻게 사용하는지에 따라 결정됩니다.
//
// FnOnce: 캡처한 값의 소유권을 가져감 (한 번만 호출 가능)
// FnMut:  캡처한 값을 변경함 (&mut로 빌림)
// Fn:     캡처한 값을 읽기만 함 (&로 빌림)
//
// 포함 관계: Fn ⊂ FnMut ⊂ FnOnce
// (Fn을 구현하면 FnMut도 구현, FnMut을 구현하면 FnOnce도 구현)
// =============================================================================

/// FnOnce만 구현하는 클로저를 받아 실행
///
/// FnOnce: 캡처한 값을 소비(consume)하므로 한 번만 호출 가능
///
/// 예시:
/// ```ignore
/// let name = String::from("Rust");
/// let consume = || {
///     drop(name);  // name의 소유권을 가져가서 drop
/// };
/// // consume은 FnOnce만 구현 (name을 소비하므로)
/// ```
pub fn call_once<F>(f: F) -> String
where
    F: FnOnce() -> String,
{
    todo!("임무 1-1: FnOnce 클로저를 호출하세요")
}

/// FnMut을 구현하는 클로저를 받아 여러 번 실행
///
/// FnMut: 캡처한 값을 변경할 수 있으므로 &mut self 필요
///
/// 예시:
/// ```ignore
/// let mut count = 0;
/// let mut increment = || {
///     count += 1;  // count를 변경
/// };
/// // increment는 FnMut 구현 (count를 &mut로 캡처)
/// ```
pub fn call_mut_twice<F>(mut f: F) -> i32
where
    F: FnMut() -> i32,
{
    todo!("임무 1-2: FnMut 클로저를 두 번 호출하고 결과를 더하세요")
}

/// Fn을 구현하는 클로저를 받아 여러 번 실행
///
/// Fn: 캡처한 값을 읽기만 하므로 &self로 충분
///
/// 예시:
/// ```ignore
/// let name = String::from("Rust");
/// let greet = || {
///     println!("Hello, {}", name);  // name을 읽기만 함
/// };
/// // greet은 Fn 구현 (name을 &로 캡처)
/// ```
pub fn call_fn_three_times<F>(f: F) -> Vec<i32>
where
    F: Fn() -> i32,
{
    todo!("임무 1-3: Fn 클로저를 세 번 호출하고 결과를 Vec으로 반환하세요")
}

/// 클로저를 저장하는 구조체
///
/// 제네릭 타입 F에 적절한 트레이트 바운드를 추가하세요.
/// 힌트: 이 구조체의 calculate 메서드는 여러 번 호출될 수 있어야 합니다.
pub struct Calculator<F> {
    operation: F,
}

impl<F> Calculator<F>
where
    F: Fn(i32, i32) -> i32,
{
    pub fn new(operation: F) -> Self {
        Calculator { operation }
    }

    pub fn calculate(&self, a: i32, b: i32) -> i32 {
        todo!("임무 1-4: 저장된 클로저를 호출하세요")
    }
}

// =============================================================================
// 임무 2: 트레이트 객체 - dyn Trait
//
// 제네릭: 컴파일 타임에 타입 결정 (정적 디스패치, 빠름)
// 트레이트 객체: 런타임에 타입 결정 (동적 디스패치, 유연함)
//
// dyn Trait은 "이 트레이트를 구현하는 어떤 타입"을 의미합니다.
// 크기를 알 수 없으므로 항상 포인터 뒤에 있어야 합니다: &dyn Trait, Box<dyn Trait>
// =============================================================================

pub trait Animal {
    fn speak(&self) -> &str;
    fn name(&self) -> &str;
}

pub struct Dog {
    pub name: String,
}

pub struct Cat {
    pub name: String,
}

impl Animal for Dog {
    fn speak(&self) -> &str {
        "멍멍!"
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Animal for Cat {
    fn speak(&self) -> &str {
        "야옹~"
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// 동물 쉼터 - 다양한 종류의 동물을 저장
///
/// 문제: Vec<Dog>? Vec<Cat>? 둘 다 저장하려면?
/// 해결: Vec<Box<dyn Animal>>
pub struct AnimalShelter {
    animals: Vec<Box<dyn Animal>>,
}

impl AnimalShelter {
    pub fn new() -> Self {
        AnimalShelter {
            animals: Vec::new(),
        }
    }

    /// 동물 추가
    ///
    /// 힌트: Box::new()로 감싸서 추가
    pub fn add<A: Animal + 'static>(&mut self, animal: A) {
        todo!("임무 2-1: 동물을 shelter에 추가하세요")
    }

    /// 모든 동물이 말하게 하기
    ///
    /// 각 동물의 speak() 결과를 Vec<String>으로 반환
    pub fn all_speak(&self) -> Vec<String> {
        todo!("임무 2-2: 모든 동물의 speak() 결과를 수집하세요")
    }

    pub fn count(&self) -> usize {
        self.animals.len()
    }
}

impl Default for AnimalShelter {
    fn default() -> Self {
        Self::new()
    }
}

/// 함수를 저장하는 구조체 (트레이트 객체 버전)
///
/// 제네릭 Calculator와 비교:
/// - 제네릭: Calculator<F> - F는 컴파일 타임에 고정
/// - 트레이트 객체: 런타임에 다른 함수로 교체 가능
pub struct DynCalculator {
    operation: Box<dyn Fn(i32, i32) -> i32>,
}

impl DynCalculator {
    pub fn new<F>(operation: F) -> Self
    where
        F: Fn(i32, i32) -> i32 + 'static,
    {
        DynCalculator {
            operation: Box::new(operation),
        }
    }

    pub fn calculate(&self, a: i32, b: i32) -> i32 {
        todo!("임무 2-3: 저장된 클로저를 호출하세요")
    }

    /// 연산을 다른 것으로 교체
    pub fn set_operation<F>(&mut self, operation: F)
    where
        F: Fn(i32, i32) -> i32 + 'static,
    {
        todo!("임무 2-4: 연산을 새 것으로 교체하세요")
    }
}

// =============================================================================
// 임무 3: 'static 생명주기
//
// 'static은 두 가지 의미가 있습니다:
//
// 1. 참조의 생명주기: &'static str
//    - 프로그램 전체 동안 유효한 참조
//    - 예: 문자열 리터럴 "hello"
//
// 2. 타입 바운드: T: 'static
//    - T가 'static이 아닌 참조를 포함하지 않음
//    - 즉, T는 소유된 데이터이거나 'static 참조만 포함
//    - 예: String, i32, Vec<String> 모두 'static 만족
// =============================================================================

/// 'static 참조 반환
///
/// 문자열 리터럴은 프로그램 바이너리에 포함되어 있어서 'static입니다.
pub fn get_static_str() -> &'static str {
    todo!("임무 3-1: 'static 문자열을 반환하세요")
}

/// 'static 바운드가 필요한 이유 - 스레드
///
/// 스레드는 언제 끝날지 모릅니다.
/// 따라서 스레드로 전달되는 데이터는:
/// - 소유권이 이동되거나 (move)
/// - 'static 참조여야 합니다
///
/// T: 'static은 "T가 dangling reference를 포함하지 않음"을 보장합니다.
pub fn spawn_with_static<T>(value: T) -> T
where
    T: Send + 'static,
{
    todo!("임무 3-2: 값을 스레드로 보냈다가 받아오세요")
}

use std::cell::RefCell;
use std::rc::Rc;
/// 이 함수는 왜 컴파일되지 않을까요? (주석 해제하면 에러)
///
/// ```compile_fail
/// fn spawn_with_reference(s: &String) -> String {
///     std::thread::spawn(move || s.clone()).join().unwrap()
/// }
/// ```
///
/// 답: &String은 'static이 아닙니다. 스레드가 실행되는 동안
/// 원본 String이 drop될 수 있습니다.
// =============================================================================
// 임무 4: Send와 Sync 마커 트레이트
//
// Send: 소유권을 다른 스레드로 이동할 수 있음
// Sync: 여러 스레드에서 &T로 동시에 접근해도 안전함
//
// 대부분의 타입은 자동으로 Send + Sync입니다.
// 예외: Rc<T> (Send 아님), RefCell<T> (Sync 아님), *mut T (둘 다 아님)
//
// T: Send이면 &T: Send (참조도 보낼 수 있음)
// T: Sync이면 &T: Send (공유 참조를 다른 스레드로 보낼 수 있음)
// =============================================================================
use std::sync::{Arc, Mutex};

/// Send 트레이트 확인
///
/// 이 함수는 T: Send인 경우에만 호출 가능합니다.
pub fn require_send<T: Send>(_: T) {}

/// Sync 트레이트 확인
///
/// 이 함수는 T: Sync인 경우에만 호출 가능합니다.
pub fn require_sync<T: Sync>(_: &T) {}

/// Send가 아닌 타입을 스레드로 보내려면?
///
/// Rc<T>는 Send가 아닙니다. (참조 카운트가 atomic이 아님)
/// Arc<T>를 사용하면 됩니다. (Atomic Reference Count)
pub fn share_counter_between_threads() -> i32 {
    todo!("임무 4-1: Arc<Mutex<i32>>를 사용해 두 스레드에서 카운터를 증가시키세요")
}

/// 왜 Rc<T>는 Send가 아닐까요?
///
/// ```compile_fail
/// use std::rc::Rc;
/// use std::thread;
///
/// let rc = Rc::new(5);
/// thread::spawn(move || {
///     println!("{}", rc);  // 컴파일 에러!
/// });
/// ```
///
/// Rc의 참조 카운트 증감이 atomic이 아니라서
/// 동시에 여러 스레드에서 접근하면 데이터 레이스 발생

// =============================================================================
// 임무 5: 통합 - Box<dyn FnOnce() + Send + 'static>
//
// 이제 모든 조각을 맞춰봅시다:
//
// Box<dyn FnOnce() + Send + 'static>
// │    │   │         │      │
// │    │   │         │      └── 참조가 없거나 'static 참조만 포함
// │    │   │         └── 다른 스레드로 보낼 수 있음
// │    │   └── 한 번만 호출 가능한 클로저
// │    └── 트레이트 객체 (런타임 다형성)
// └── 힙에 저장 (크기를 모르므로)
//
// 이것은 "스레드로 보낼 수 있는, 한 번 실행할 작업"입니다.
// 바로 Worker Pool의 Job 타입!
// =============================================================================

/// 작업 타입 정의
pub type Job = Box<dyn FnOnce() + Send + 'static>;

/// Job을 생성하는 함수
///
/// 클로저를 받아서 Job으로 변환합니다.
pub fn create_job<F>(f: F) -> Job
where
    F: FnOnce() + Send + 'static,
{
    todo!("임무 5-1: 클로저를 Job으로 변환하세요")
}

/// Job을 실행하는 함수
pub fn execute_job(job: Job) {
    todo!("임무 5-2: Job을 실행하세요")
}

/// 여러 Job을 스레드에서 실행
///
/// jobs를 받아서 새 스레드에서 모두 실행합니다.
pub fn execute_jobs_in_thread(jobs: Vec<Job>) {
    todo!("임무 5-3: 새 스레드에서 모든 Job을 실행하세요")
}

/// 결과를 반환하는 Job
///
/// 이번에는 결과가 있습니다: Box<dyn FnOnce() -> T + Send + 'static>
pub fn execute_with_result<T, F>(f: F) -> T
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    todo!("임무 5-4: 클로저를 스레드에서 실행하고 결과를 반환하세요")
}

// =============================================================================
// 테스트 헬퍼
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_traits_compile() {
        // Fn은 여러 번 호출 가능
        let x = 5;
        let fn_closure = || x;
        assert_eq!(fn_closure(), 5);
        assert_eq!(fn_closure(), 5);

        // FnMut은 상태 변경 가능
        let mut count = 0;
        let mut fn_mut_closure = || {
            count += 1;
            count
        };
        assert_eq!(fn_mut_closure(), 1);
        assert_eq!(fn_mut_closure(), 2);
    }
}
