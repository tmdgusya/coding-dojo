use fstools::*;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

// ============================================================================
// 임무 1: 기본 파일 읽기 (Basic File Reading)
// ============================================================================

mod mission_1_basic_reading {
    use super::*;

    #[test]
    fn test_read_file_to_string_success() {
        let contents = read_file_to_string("tests/fixtures/hello.txt").unwrap();
        assert!(contents.contains("Hello, Rust!"));
        assert!(contents.contains("FileSystem Dojo"));
    }

    #[test]
    fn test_read_file_to_string_multiline() {
        let contents = read_file_to_string("tests/fixtures/poem.txt").unwrap();
        assert!(contents.contains("The Road Not Taken"));
        assert!(contents.contains("Robert Frost"));
        assert!(contents.contains("Two roads diverged"));
    }

    #[test]
    fn test_read_file_to_string_not_found() {
        let result = read_file_to_string("nonexistent_file.txt");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    }

    #[test]
    fn test_read_file_to_bytes() {
        let bytes = read_file_to_bytes("tests/fixtures/hello.txt").unwrap();
        assert!(!bytes.is_empty());
        let as_string = String::from_utf8(bytes).unwrap();
        assert!(as_string.contains("Hello, Rust!"));
    }

    #[test]
    fn test_read_file_to_bytes_preserves_content() {
        let bytes = read_file_to_bytes("tests/fixtures/numbers.txt").unwrap();
        let content = String::from_utf8(bytes).unwrap();
        assert!(content.contains("1"));
        assert!(content.contains("10"));
    }
}

// ============================================================================
// 임무 2: 기본 파일 쓰기 (Basic File Writing)
// ============================================================================

mod mission_2_basic_writing {
    use super::*;

    #[test]
    fn test_write_string_to_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_write.txt");
        let path_str = file_path.to_str().unwrap();

        write_string_to_file(path_str, "Hello, World!").unwrap();

        let contents = fs::read_to_string(&file_path).unwrap();
        assert_eq!(contents, "Hello, World!");
    }

    #[test]
    fn test_write_string_overwrites() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_overwrite.txt");
        let path_str = file_path.to_str().unwrap();

        write_string_to_file(path_str, "First content").unwrap();
        write_string_to_file(path_str, "Second content").unwrap();

        let contents = fs::read_to_string(&file_path).unwrap();
        assert_eq!(contents, "Second content");
    }

    #[test]
    fn test_append_to_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_append.txt");
        let path_str = file_path.to_str().unwrap();

        write_string_to_file(path_str, "Line 1\n").unwrap();
        append_to_file(path_str, "Line 2\n").unwrap();
        append_to_file(path_str, "Line 3\n").unwrap();

        let contents = fs::read_to_string(&file_path).unwrap();
        assert!(contents.contains("Line 1"));
        assert!(contents.contains("Line 2"));
        assert!(contents.contains("Line 3"));
    }

    #[test]
    fn test_append_creates_file_if_not_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("new_file.txt");
        let path_str = file_path.to_str().unwrap();

        append_to_file(path_str, "New content").unwrap();

        let contents = fs::read_to_string(&file_path).unwrap();
        assert_eq!(contents, "New content");
    }
}

// ============================================================================
// 임무 3: File 구조체와 저수준 I/O
// ============================================================================

mod mission_3_file_struct {
    use super::*;

    #[test]
    fn test_read_with_file_struct() {
        let contents = read_with_file_struct("tests/fixtures/hello.txt").unwrap();
        assert!(contents.contains("Hello, Rust!"));
    }

    #[test]
    fn test_write_with_file_struct() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("file_struct_write.txt");
        let path_str = file_path.to_str().unwrap();

        write_with_file_struct(path_str, "Written with File struct").unwrap();

        let contents = fs::read_to_string(&file_path).unwrap();
        assert_eq!(contents, "Written with File struct");
    }

    #[test]
    fn test_read_in_chunks_small_file() {
        let chunks = read_in_chunks("tests/fixtures/hello.txt", 10).unwrap();
        assert!(!chunks.is_empty());

        let combined: Vec<u8> = chunks.into_iter().flatten().collect();
        let content = String::from_utf8(combined).unwrap();
        assert!(content.contains("Hello, Rust!"));
    }

    #[test]
    fn test_read_in_chunks_exact_size() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("chunk_test.txt");
        fs::write(&file_path, "0123456789").unwrap();

        let chunks = read_in_chunks(file_path.to_str().unwrap(), 5).unwrap();
        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0], b"01234");
        assert_eq!(chunks[1], b"56789");
    }

    #[test]
    fn test_read_in_chunks_larger_than_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("small.txt");
        fs::write(&file_path, "small").unwrap();

        let chunks = read_in_chunks(file_path.to_str().unwrap(), 100).unwrap();
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], b"small");
    }
}

// ============================================================================
// 임무 4: 버퍼링된 I/O (Buffered I/O)
// ============================================================================

mod mission_4_buffered_io {
    use super::*;

    #[test]
    fn test_read_lines() {
        let lines = read_lines("tests/fixtures/numbers.txt").unwrap();
        assert_eq!(lines.len(), 10);
        assert_eq!(lines[0], "1");
        assert_eq!(lines[9], "10");
    }

    #[test]
    fn test_read_lines_poem() {
        let lines = read_lines("tests/fixtures/poem.txt").unwrap();
        assert!(lines.len() > 20);
        assert_eq!(lines[0], "The Road Not Taken");
    }

    #[test]
    fn test_grep_lines_found() {
        let matches = grep_lines("tests/fixtures/poem.txt", "road").unwrap();
        assert!(!matches.is_empty());
        for (_, line) in &matches {
            assert!(line.to_lowercase().contains("road"));
        }
    }

    #[test]
    fn test_grep_lines_with_line_numbers() {
        let matches = grep_lines("tests/fixtures/numbers.txt", "5").unwrap();
        let line_numbers: Vec<usize> = matches.iter().map(|(n, _)| *n).collect();
        assert!(line_numbers.contains(&5));
    }

    #[test]
    fn test_grep_lines_not_found() {
        let matches = grep_lines("tests/fixtures/hello.txt", "xyz123").unwrap();
        assert!(matches.is_empty());
    }

    #[test]
    fn test_write_lines() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("lines.txt");
        let path_str = file_path.to_str().unwrap();

        let lines = ["Line 1", "Line 2", "Line 3"];
        write_lines(path_str, &lines).unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("Line 1"));
        assert!(content.contains("Line 2"));
        assert!(content.contains("Line 3"));
    }
}

// ============================================================================
// 임무 5: Path와 PathBuf
// ============================================================================

mod mission_5_path {
    use super::*;

    #[test]
    fn test_get_filename() {
        assert_eq!(
            get_filename("/home/user/file.txt"),
            Some("file.txt".to_string())
        );
        assert_eq!(
            get_filename("document.pdf"),
            Some("document.pdf".to_string())
        );
        assert_eq!(get_filename("/"), None);
    }

    #[test]
    fn test_get_extension() {
        assert_eq!(get_extension("file.txt"), Some("txt".to_string()));
        assert_eq!(get_extension("archive.tar.gz"), Some("gz".to_string()));
        assert_eq!(get_extension("no_extension"), None);
        assert_eq!(get_extension(".gitignore"), None);
    }

    #[test]
    fn test_change_extension() {
        let new_path = change_extension("document.txt", "pdf");
        assert_eq!(new_path, PathBuf::from("document.pdf"));

        let new_path = change_extension("/home/user/file.rs", "exe");
        assert!(new_path.to_str().unwrap().ends_with("file.exe"));
    }

    #[test]
    fn test_join_paths() {
        let joined = join_paths("/home/user", "documents");
        assert_eq!(joined, PathBuf::from("/home/user/documents"));

        let joined = join_paths("base", "sub/dir");
        assert_eq!(joined, PathBuf::from("base/sub/dir"));
    }

    #[test]
    fn test_is_absolute_path() {
        assert!(is_absolute_path("/home/user"));
        assert!(is_absolute_path("/"));
        assert!(!is_absolute_path("relative/path"));
        assert!(!is_absolute_path("./current"));
    }
}

// ============================================================================
// 임무 6: 디렉토리 순회 (Directory Traversal)
// ============================================================================

mod mission_6_directory {
    use super::*;

    #[test]
    fn test_list_directory() {
        let entries = list_directory("tests/fixtures").unwrap();
        assert!(entries.len() >= 3);

        let filenames: Vec<String> = entries
            .iter()
            .filter_map(|p| p.file_name())
            .filter_map(|n| n.to_str())
            .map(|s| s.to_string())
            .collect();

        assert!(filenames.contains(&"hello.txt".to_string()));
        assert!(filenames.contains(&"numbers.txt".to_string()));
        assert!(filenames.contains(&"poem.txt".to_string()));
    }

    #[test]
    fn test_list_directory_not_found() {
        let result = list_directory("nonexistent_directory");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_files() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        fs::write(dir.path().join("file1.txt"), "content").unwrap();
        fs::write(dir.path().join("file2.txt"), "content").unwrap();

        let files = list_files(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.iter().all(|p| p.is_file()));
    }

    #[test]
    fn test_find_files_by_extension() {
        let files = find_files_by_extension("tests/fixtures", "txt").unwrap();
        assert!(files.len() >= 3);
        assert!(files
            .iter()
            .all(|p| { p.extension().map(|e| e == "txt").unwrap_or(false) }));
    }

    #[test]
    fn test_find_files_by_extension_none_found() {
        let files = find_files_by_extension("tests/fixtures", "xyz").unwrap();
        assert!(files.is_empty());
    }

    #[test]
    fn test_walk_directory() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("sub");
        let subsubdir = subdir.join("subsub");
        fs::create_dir_all(&subsubdir).unwrap();

        fs::write(dir.path().join("root.txt"), "root").unwrap();
        fs::write(subdir.join("sub.txt"), "sub").unwrap();
        fs::write(subsubdir.join("deep.txt"), "deep").unwrap();

        let files = walk_directory(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(files.len(), 3);
    }
}

// ============================================================================
// 임무 7: 파일 메타데이터 (File Metadata)
// ============================================================================

mod mission_7_metadata {
    use super::*;

    #[test]
    fn test_file_info_from_path() {
        let info = FileInfo::from_path("tests/fixtures/hello.txt").unwrap();
        assert!(info.is_file);
        assert!(!info.is_dir);
        assert!(info.size > 0);
    }

    #[test]
    fn test_file_info_directory() {
        let info = FileInfo::from_path("tests/fixtures").unwrap();
        assert!(!info.is_file);
        assert!(info.is_dir);
    }

    #[test]
    fn test_get_file_size() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("sized.txt");
        fs::write(&file_path, "12345").unwrap();

        let size = get_file_size(file_path.to_str().unwrap()).unwrap();
        assert_eq!(size, 5);
    }

    #[test]
    fn test_is_file() {
        assert!(is_file("tests/fixtures/hello.txt"));
        assert!(!is_file("tests/fixtures"));
        assert!(!is_file("nonexistent"));
    }

    #[test]
    fn test_is_directory() {
        assert!(is_directory("tests/fixtures"));
        assert!(!is_directory("tests/fixtures/hello.txt"));
        assert!(!is_directory("nonexistent"));
    }

    #[test]
    fn test_path_exists() {
        assert!(path_exists("tests/fixtures/hello.txt"));
        assert!(path_exists("tests/fixtures"));
        assert!(!path_exists("definitely_not_here.txt"));
    }
}

// ============================================================================
// 최종 임무: 통합 파일 유틸리티
// ============================================================================

mod final_mission_utilities {
    use super::*;

    #[test]
    fn test_copy_file() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("source.txt");
        let dst = dir.path().join("destination.txt");

        fs::write(&src, "Copy me!").unwrap();

        let bytes = copy_file(src.to_str().unwrap(), dst.to_str().unwrap()).unwrap();
        assert_eq!(bytes, 8);

        let dst_content = fs::read_to_string(&dst).unwrap();
        assert_eq!(dst_content, "Copy me!");

        assert!(src.exists());
    }

    #[test]
    fn test_move_file() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("to_move.txt");
        let dst = dir.path().join("moved.txt");

        fs::write(&src, "Move me!").unwrap();

        move_file(src.to_str().unwrap(), dst.to_str().unwrap()).unwrap();

        assert!(!src.exists());
        assert!(dst.exists());

        let content = fs::read_to_string(&dst).unwrap();
        assert_eq!(content, "Move me!");
    }

    #[test]
    fn test_remove_dir_recursive() {
        let dir = tempdir().unwrap();
        let target = dir.path().join("to_remove");
        let subdir = target.join("subdir");
        fs::create_dir_all(&subdir).unwrap();
        fs::write(target.join("file.txt"), "delete me").unwrap();
        fs::write(subdir.join("nested.txt"), "delete me too").unwrap();

        remove_dir_recursive(target.to_str().unwrap()).unwrap();

        assert!(!target.exists());
    }

    #[test]
    fn test_replace_in_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("replace.txt");
        fs::write(&file_path, "Hello World! Hello Rust! Hello Everyone!").unwrap();

        let count = replace_in_file(file_path.to_str().unwrap(), "Hello", "Hi").unwrap();
        assert_eq!(count, 3);

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Hi World! Hi Rust! Hi Everyone!");
    }

    #[test]
    fn test_replace_in_file_no_match() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("no_replace.txt");
        fs::write(&file_path, "Nothing to replace here").unwrap();

        let count = replace_in_file(file_path.to_str().unwrap(), "XYZ", "ABC").unwrap();
        assert_eq!(count, 0);

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Nothing to replace here");
    }

    #[test]
    fn test_calculate_dir_size() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("a.txt"), "12345").unwrap();
        fs::write(dir.path().join("b.txt"), "67890").unwrap();

        let subdir = dir.path().join("sub");
        fs::create_dir(&subdir).unwrap();
        fs::write(subdir.join("c.txt"), "abc").unwrap();

        let size = calculate_dir_size(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(size, 13);
    }
}
