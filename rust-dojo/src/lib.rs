use std::error::Error;
use std::fs;

// =============================================================================
// 임무 1: read_file 함수를 구현하세요
//
// String 은 소유권을 가진 문자열 임
// &str 은 문자열 슬라이스 임 => 데이터를 가지고 있는게 아니라 어딘가에 있는걸 참조하는 것
// =============================================================================
pub fn read_file(_filename: &str) -> String {
    fs::read_to_string(_filename).unwrap_or_else(|_| String::from("찾을 수 없습니다."))
}

// =============================================================================
// 임무 2: read_file_safe, find_first_match 함수를 구현하세요
// =============================================================================
pub fn read_file_safe(_filename: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(_filename)
}

pub fn find_first_match<'a>(_contents: &'a str, _query: &str) -> Option<&'a str> {
    _contents
        .lines()
        .find(|&line| line.contains(_query))
        .map(|v| v)
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
        if _args.len() < 3 {
            return Err("Not enough arguments");
        }

        let case_sensitive = match _args.get(3) {
            Some(args) => args == "true",
            None => true,
        };

        Ok(Config {
            query: _args[1].clone(),
            filename: _args[2].clone(),
            case_sensitive,
        })
    }

    pub fn display_info(&self) -> String {
        format!(
            "query: {}, filename: {}, case_sensitive: {}",
            self.query, self.filename, self.case_sensitive
        )
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
        LineMatch {
            line_number: _line_number,
            content: _content.to_owned(),
        }
    }
}

impl SearchResult for LineMatch {
    fn format(&self) -> String {
        format!("[Line{}], {}", self.line_number, self.content)
    }
}

pub fn format_results<T: SearchResult>(_results: &[T]) -> String {
    _results
        .iter()
        .map(|result| result.format())
        .collect::<Vec<String>>()
        .join("\n")
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
