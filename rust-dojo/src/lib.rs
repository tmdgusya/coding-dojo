use std::error::Error;
use std::fs;

// =============================================================================
// 임무 1: read_file 함수를 구현하세요
//
// String 은 소유권을 가진 문자열 임
// &str 은 문자열 슬라이스 임 => 데이터를 가지고 있는게 아니라 어딘가에 있는걸 참조하는 것
// =============================================================================
pub fn read_file(_filename: &str) -> String {
    let result = fs::read_to_string(_filename);
    return match result {
        Ok(contents) => contents,
        Err(_) => String::from("찾을 수 없습니다."),
    };
}

// =============================================================================
// 임무 2: read_file_safe, find_first_match 함수를 구현하세요
// =============================================================================
pub fn read_file_safe(_filename: &str) -> Result<String, std::io::Error> {
    todo!("임무 2: Result 타입을 반환하는 안전한 파일 읽기 함수를 구현하세요")
}

pub fn find_first_match<'a>(_contents: &'a str, _query: &str) -> Option<&'a str> {
    todo!("임무 2: 첫 번째 매칭 라인을 Option으로 반환하세요")
}

// =============================================================================
// 임무 3: Config 구조체를 구현하세요
// =============================================================================
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(_args: &[String]) -> Result<Config, &'static str> {
        todo!("임무 3: 커맨드라인 인자로 Config를 생성하는 함수를 구현하세요")
    }

    pub fn display_info(&self) -> String {
        todo!("임무 3: Config 정보를 문자열로 반환하는 메서드를 구현하세요")
    }
}

// =============================================================================
// 임무 4: SearchResult 트레이트와 LineMatch 구조체를 구현하세요
// =============================================================================
pub trait SearchResult {
    fn format(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineMatch {
    pub line_number: usize,
    pub content: String,
}

impl LineMatch {
    pub fn new(_line_number: usize, _content: &str) -> Self {
        todo!("임무 4: LineMatch 생성자를 구현하세요")
    }
}

impl SearchResult for LineMatch {
    fn format(&self) -> String {
        todo!("임무 4: SearchResult 트레이트를 구현하세요")
    }
}

pub fn format_results<T: SearchResult>(_results: &[T]) -> String {
    todo!("임무 4: 제네릭 함수를 구현하세요")
}

// =============================================================================
// 임무 5: search, search_case_insensitive 함수를 구현하세요
// 생명주기 어노테이션에 주목하세요!
// =============================================================================
pub fn search<'a>(_query: &str, _contents: &'a str) -> Vec<&'a str> {
    todo!("임무 5: 검색 함수를 구현하세요. 생명주기 어노테이션이 필요합니다!")
}

pub fn search_case_insensitive<'a>(_query: &str, _contents: &'a str) -> Vec<&'a str> {
    todo!("임무 5: 대소문자 무시 검색 함수를 구현하세요")
}

// =============================================================================
// 최종 임무: run 함수로 모든 것을 통합하세요
// =============================================================================
pub fn run(_config: Config) -> Result<(), Box<dyn Error>> {
    todo!("최종 임무: 전체 로직을 통합하는 run 함수를 구현하세요")
}
