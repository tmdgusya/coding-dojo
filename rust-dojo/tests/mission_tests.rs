//! minigrep 학습 테스트
//!
//! 이 테스트들은 현재 실패(RED) 상태입니다.
//! 각 임무를 완수하면서 테스트를 통과(GREEN)시키세요.
//!
//! 실행 방법:
//! ```bash
//! cargo test                    # 모든 테스트
//! cargo test mission_1          # 임무 1만
//! cargo test mission_2          # 임무 2만
//! ```

use minigrep::*;

// ============================================================================
// 임무 1: 기본 문법과 소유권 (Ownership)
// ============================================================================

mod mission_1_ownership {
    use super::*;

    #[test]
    fn test_read_file_success() {
        // poem.txt 파일을 읽어서 내용을 반환해야 합니다.
        // TODO: src/lib.rs에 read_file 함수를 구현하세요.
        let contents = read_file("tests/fixtures/poem.txt");
        assert!(contents.contains("I'm nobody! Who are you?"));
    }

    #[test]
    fn test_read_file_contains_multiple_lines() {
        let contents = read_file("tests/fixtures/poem.txt");
        assert!(contents.contains("Are you nobody, too?"));
        assert!(contents.contains("How dreary to be somebody!"));
    }

    #[test]
    fn test_read_file_not_found() {
        // 파일이 없을 때 에러 메시지를 반환해야 합니다.
        let contents = read_file("nonexistent_file.txt");
        assert!(
            contents.to_lowercase().contains("error") || contents.contains("찾을 수 없습니다.")
        );
    }

    #[test]
    fn test_ownership_string_returned() {
        // read_file은 String을 반환해야 합니다 (소유권 이동)
        let contents: String = read_file("tests/fixtures/poem.txt");
        // 반환된 String의 소유권을 가지고 있으므로 수정 가능
        let mut owned = contents;
        owned.push_str("\n-- End of poem --");
        assert!(owned.contains("-- End of poem --"));
    }
}

// ============================================================================
// 임무 2: 에러 처리 (Result & Option)
// ============================================================================

mod mission_2_error_handling {
    use super::*;

    #[test]
    fn test_read_file_safe_success() {
        // Result<String, std::io::Error>를 반환해야 합니다.
        // TODO: src/lib.rs에 read_file_safe 함수를 구현하세요.
        let result = read_file_safe("tests/fixtures/poem.txt");
        assert!(result.is_ok());
        let contents = result.unwrap();
        assert!(contents.contains("I'm nobody"));
    }

    #[test]
    fn test_read_file_safe_error() {
        let result = read_file_safe("nonexistent.txt");
        assert!(result.is_err());
        // io::Error 타입인지 확인
        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    }

    #[test]
    fn test_find_first_match_found() {
        // 첫 번째 매칭 라인을 Option<&str>로 반환해야 합니다.
        // TODO: src/lib.rs에 find_first_match 함수를 구현하세요.
        let contents = "hello\nworld\nhello rust\ngoodbye";
        let result = find_first_match(contents, "rust");
        assert_eq!(result, Some("hello rust"));
    }

    #[test]
    fn test_find_first_match_not_found() {
        let contents = "hello\nworld\ngoodbye";
        let result = find_first_match(contents, "rust");
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_first_match_first_line() {
        let contents = "rust is great\nhello rust\nrust again";
        let result = find_first_match(contents, "rust");
        // 첫 번째 매칭만 반환
        assert_eq!(result, Some("rust is great"));
    }
}

// ============================================================================
// 임무 3: 구조체와 메서드 (Struct & impl)
// ============================================================================

mod mission_3_struct {
    use super::*;

    #[test]
    fn test_config_build_success() {
        // Config 구조체와 build 메서드를 구현하세요.
        // TODO: src/lib.rs에 Config 구조체를 정의하세요.
        let args = vec![
            String::from("minigrep"),
            String::from("needle"),
            String::from("haystack.txt"),
        ];
        let config = Config::build(&args);
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.query, "needle");
        assert_eq!(config.filename, "haystack.txt");
    }

    #[test]
    fn test_config_build_not_enough_args() {
        let args = vec![String::from("minigrep")];
        let config = Config::build(&args);
        assert!(config.is_err());
    }

    #[test]
    fn test_config_build_missing_filename() {
        let args = vec![String::from("minigrep"), String::from("query")];
        let config = Config::build(&args);
        assert!(config.is_err());
    }

    #[test]
    fn test_config_case_sensitive_default() {
        let args = vec![
            String::from("minigrep"),
            String::from("query"),
            String::from("file.txt"),
        ];
        let config = Config::build(&args).unwrap();
        // 기본값은 대소문자 구분 (true)
        assert!(config.case_sensitive);
    }

    #[test]
    fn test_config_display_info() {
        // Config에 정보를 출력하는 메서드 구현
        let args = vec![
            String::from("minigrep"),
            String::from("test"),
            String::from("sample.txt"),
        ];
        let config = Config::build(&args).unwrap();
        let info = config.display_info();
        assert!(info.contains("test"));
        assert!(info.contains("sample.txt"));
    }
}

// ============================================================================
// 임무 4: 트레이트와 제네릭 (Traits & Generics)
// ============================================================================

mod mission_4_traits {
    use super::*;

    #[test]
    fn test_line_match_new() {
        // LineMatch 구조체를 구현하세요.
        // TODO: src/lib.rs에 LineMatch를 정의하세요.
        let m = LineMatch::new(1, "Hello, world!");
        assert_eq!(m.line_number, 1);
        assert_eq!(m.content, "Hello, world!");
    }

    #[test]
    fn test_line_match_format() {
        // SearchResult 트레이트의 format 메서드를 구현하세요.
        let m = LineMatch::new(42, "The answer");
        let formatted = m.format();
        assert!(formatted.contains("42"));
        assert!(formatted.contains("The answer"));
        // 예상 형식: "[Line 42] The answer"
        assert!(formatted.starts_with("[Line"));
    }

    #[test]
    fn test_search_result_trait() {
        // SearchResult 트레이트를 정의하고 LineMatch에 구현하세요.
        let m = LineMatch::new(1, "test");
        // 트레이트 객체로 사용 가능해야 함
        let result: &dyn SearchResult = &m;
        assert!(result.format().contains("test"));
    }

    #[test]
    fn test_print_results_generic() {
        // 제네릭 함수 format_results를 구현하세요.
        let matches = vec![LineMatch::new(1, "first"), LineMatch::new(5, "second")];
        let output = format_results(&matches);
        assert!(output.contains("first"));
        assert!(output.contains("second"));
        assert!(output.contains("1"));
        assert!(output.contains("5"));
    }

    #[test]
    fn test_line_match_equality() {
        // PartialEq 트레이트 derive
        let m1 = LineMatch::new(1, "test");
        let m2 = LineMatch::new(1, "test");
        let m3 = LineMatch::new(2, "test");
        assert_eq!(m1, m2);
        assert_ne!(m1, m3);
    }
}

// ============================================================================
// 임무 5: 생명주기 (Lifetimes)
// ============================================================================

mod mission_5_lifetimes {
    use super::*;

    #[test]
    fn test_search_single_result() {
        // search 함수를 구현하세요.
        // 생명주기 어노테이션이 필요합니다!
        // TODO: src/lib.rs에 search 함수를 정의하세요.
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";
        let results = search("safe", contents);
        assert_eq!(results, vec!["safe, fast, productive."]);
    }

    #[test]
    fn test_search_multiple_results() {
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me, it's safe.";
        let results = search("safe", contents);
        assert_eq!(results.len(), 2);
        assert!(results.contains(&"safe, fast, productive."));
        assert!(results.contains(&"Trust me, it's safe."));
    }

    #[test]
    fn test_search_no_results() {
        let contents = "Hello\nWorld\nRust";
        let results = search("Python", contents);
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_case_insensitive() {
        // search_case_insensitive 함수를 구현하세요.
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        let results = search_case_insensitive("rust", contents);
        assert_eq!(results, vec!["Rust:", "Trust me."]);
    }

    #[test]
    fn test_search_case_insensitive_multiple() {
        let contents = "HELLO\nhello\nHeLLo\nworld";
        let results = search_case_insensitive("hello", contents);
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_lifetime_reference_validity() {
        // 반환된 참조가 원본 contents의 일부임을 확인
        let contents = String::from("line one\nline two\nline three");
        let results = search("two", &contents);

        // results의 &str들은 contents에서 빌려온 것
        assert_eq!(results[0], "line two");

        // contents가 유효한 동안 results도 유효
        assert!(contents.contains(results[0]));
    }
}

// ============================================================================
// 최종 임무: 통합 테스트
// ============================================================================

mod final_mission_integration {
    use super::*;

    #[test]
    fn test_run_function() {
        // run 함수로 전체 로직 통합
        // TODO: src/lib.rs에 run 함수를 구현하세요.
        let args = vec![
            String::from("minigrep"),
            String::from("nobody"),
            String::from("tests/fixtures/poem.txt"),
        ];
        let config = Config::build(&args).unwrap();
        let result = run(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_with_case_insensitive() {
        // 대소문자 무시 옵션 테스트
        let mut config = Config::build(&vec![
            String::from("minigrep"),
            String::from("Nobody"),
            String::from("tests/fixtures/poem.txt"),
        ])
        .unwrap();
        config.case_sensitive = false;

        let result = run(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_full_workflow() {
        // 전체 워크플로우 테스트
        let contents = read_file_safe("tests/fixtures/poem.txt").unwrap();
        let results = search("you", &contents);

        assert!(!results.is_empty());
        for line in &results {
            assert!(line.to_lowercase().contains("you"));
        }
    }
}
