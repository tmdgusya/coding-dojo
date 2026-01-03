//! FileSystem Tools - Rust FileSystem API 학습 라이브러리
//!
//! 이 라이브러리는 Rust의 파일 시스템 API를 단계적으로 학습하기 위해 만들어졌습니다.
//! 각 임무(Mission)를 완수하면서 파일 I/O의 마스터가 되어보세요!

use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

// =============================================================================
// 임무 1: 기본 파일 읽기 (Basic File Reading)
//
// std::fs::read_to_string을 사용하여 파일 내용을 읽어오는 함수들을 구현하세요.
// 가장 간단한 방법부터 시작합니다.
// =============================================================================

/// 파일 내용을 문자열로 읽어옵니다.
///
/// # Arguments
/// * `path` - 읽을 파일의 경로
///
/// # Returns
/// * `io::Result<String>` - 파일 내용 또는 에러
pub fn read_file_to_string(path: &str) -> io::Result<String> {
    std::fs::read_to_string(path)
}

/// 파일 내용을 바이트 벡터로 읽어옵니다.
///
/// # Arguments
/// * `path` - 읽을 파일의 경로
///
/// # Returns
/// * `io::Result<Vec<u8>>` - 파일의 바이트 내용 또는 에러
pub fn read_file_to_bytes(path: &str) -> io::Result<Vec<u8>> {
    std::fs::read(path)
}

// =============================================================================
// 임무 2: 기본 파일 쓰기 (Basic File Writing)
//
// std::fs::write를 사용하여 파일에 내용을 쓰는 함수들을 구현하세요.
// =============================================================================

/// 문자열을 파일에 씁니다. (기존 내용 덮어쓰기)
///
/// # Arguments
/// * `path` - 쓸 파일의 경로
/// * `contents` - 파일에 쓸 내용
///
/// # Returns
/// * `io::Result<()>` - 성공 또는 에러
pub fn write_string_to_file(path: &str, contents: &str) -> io::Result<()> {
    std::fs::write(path, contents)
}

/// 문자열을 파일 끝에 추가합니다. (append)
///
/// # Arguments
/// * `path` - 추가할 파일의 경로
/// * `contents` - 추가할 내용
///
/// # Returns
/// * `io::Result<()>` - 성공 또는 에러
pub fn append_to_file(path: &str, contents: &str) -> io::Result<()> {
    OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?
        .write_all(contents.as_bytes())
}

// =============================================================================
// 임무 3: File 구조체와 저수준 I/O
//
// std::fs::File을 직접 다루고, Read/Write 트레이트를 이해합니다.
// =============================================================================

/// File 구조체를 사용하여 파일을 읽습니다.
///
/// # Arguments
/// * `path` - 읽을 파일의 경로
///
/// # Returns
/// * `io::Result<String>` - 파일 내용 또는 에러
pub fn read_with_file_struct(path: &str) -> io::Result<String> {
    let file = File::open(path).unwrap();
    std::io::read_to_string(file)
}

/// File 구조체를 사용하여 파일에 씁니다.
///
/// # Arguments
/// * `path` - 쓸 파일의 경로
/// * `contents` - 파일에 쓸 내용
///
/// # Returns
/// * `io::Result<()>` - 성공 또는 에러
pub fn write_with_file_struct(path: &str, contents: &str) -> io::Result<()> {
    File::create(path).unwrap().write_all(contents.as_bytes())
}

/// 파일을 청크 단위로 읽습니다.
///
/// # Arguments
/// * `path` - 읽을 파일의 경로
/// * `chunk_size` - 한 번에 읽을 바이트 수
///
/// # Returns
/// * `io::Result<Vec<Vec<u8>>>` - 청크들의 벡터 또는 에러
pub fn read_in_chunks(path: &str, chunk_size: usize) -> io::Result<Vec<Vec<u8>>> {
    let mut chunks = Vec::new();
    let mut file = File::open(path).unwrap();
    let mut buffer = vec![0; chunk_size];

    loop {
        let n = file.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }
        chunks.push(buffer[..n].to_vec());
    }

    Ok(chunks)
}

// =============================================================================
// 임무 4: 버퍼링된 I/O (Buffered I/O)
//
// BufReader와 BufWriter를 사용하여 효율적인 I/O를 구현합니다.
// =============================================================================

/// 파일을 라인 단위로 읽어 벡터로 반환합니다.
///
/// # Arguments
/// * `path` - 읽을 파일의 경로
///
/// # Returns
/// * `io::Result<Vec<String>>` - 라인들의 벡터 또는 에러
pub fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    Ok(lines)
}

/// 파일에서 특정 패턴을 포함하는 라인들을 찾습니다.
///
/// # Arguments
/// * `path` - 검색할 파일의 경로
/// * `pattern` - 검색할 패턴
///
/// # Returns
/// * `io::Result<Vec<(usize, String)>>` - (라인번호, 라인내용) 튜플의 벡터
pub fn grep_lines(path: &str, pattern: &str) -> io::Result<Vec<(usize, String)>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.contains(pattern) {
            lines.push((i + 1, line));
        }
    }

    Ok(lines)
}

/// 여러 줄을 효율적으로 파일에 씁니다.
///
/// # Arguments
/// * `path` - 쓸 파일의 경로
/// * `lines` - 쓸 라인들의 슬라이스
///
/// # Returns
/// * `io::Result<()>` - 성공 또는 에러
pub fn write_lines(path: &str, lines: &[&str]) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    let mut writer = BufWriter::new(file);
    for line in lines {
        writer.write_all(line.as_bytes())?;
        writer.write_all(b"\n")?;
    }
    writer.flush()?;
    Ok(())
}

// =============================================================================
// 임무 5: Path와 PathBuf
//
// 플랫폼 독립적인 경로 처리 방법을 학습합니다.
// =============================================================================

/// 경로에서 파일명을 추출합니다.
///
/// # Arguments
/// * `path` - 파일 경로
///
/// # Returns
/// * `Option<String>` - 파일명 또는 None
pub fn get_filename(path: &str) -> Option<String> {
    Path::new(path)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
}

/// 경로에서 확장자를 추출합니다.
///
/// # Arguments
/// * `path` - 파일 경로
///
/// # Returns
/// * `Option<String>` - 확장자 또는 None
pub fn get_extension(path: &str) -> Option<String> {
    Path::new(path)
        .extension()
        .map(|s| s.to_string_lossy().to_string())
}

/// 경로의 확장자를 변경합니다.
///
/// # Arguments
/// * `path` - 원본 파일 경로
/// * `new_ext` - 새 확장자
///
/// # Returns
/// * `PathBuf` - 새 경로
pub fn change_extension(path: &str, new_ext: &str) -> PathBuf {
    Path::new(path).with_extension(new_ext)
}

/// 두 경로를 결합합니다.
///
/// # Arguments
/// * `base` - 기본 경로
/// * `relative` - 상대 경로
///
/// # Returns
/// * `PathBuf` - 결합된 경로
pub fn join_paths(base: &str, relative: &str) -> PathBuf {
    Path::new(base).join(relative)
}

/// 경로가 절대 경로인지 확인합니다.
///
/// # Arguments
/// * `path` - 확인할 경로
///
/// # Returns
/// * `bool` - 절대 경로 여부
pub fn is_absolute_path(path: &str) -> bool {
    Path::new(path).is_absolute()
}

// =============================================================================
// 임무 6: 디렉토리 순회 (Directory Traversal)
//
// 디렉토리 내용을 읽고 탐색하는 방법을 학습합니다.
// =============================================================================

/// 디렉토리 내의 모든 엔트리를 나열합니다.
///
/// # Arguments
/// * `dir_path` - 디렉토리 경로
///
/// # Returns
/// * `io::Result<Vec<PathBuf>>` - 엔트리들의 경로 벡터
pub fn list_directory(dir_path: &str) -> io::Result<Vec<PathBuf>> {
    std::fs::read_dir(dir_path)?
        .map(|entry| entry.map(|e| e.path()))
        .collect()
}

/// 디렉토리 내의 파일들만 나열합니다. (하위 디렉토리 제외)
///
/// # Arguments
/// * `dir_path` - 디렉토리 경로
///
/// # Returns
/// * `io::Result<Vec<PathBuf>>` - 파일들의 경로 벡터
pub fn list_files(dir_path: &str) -> io::Result<Vec<PathBuf>> {
    Ok(std::fs::read_dir(dir_path)?
        .filter_map(|entry| {
            entry
                .ok()
                .filter(|e| e.file_type().map_or(false, |t| t.is_file()))
                .map(|e| e.path())
        })
        .collect())
}

/// 특정 확장자를 가진 파일들만 찾습니다.
///
/// # Arguments
/// * `dir_path` - 디렉토리 경로
/// * `extension` - 찾을 확장자 (예: "txt", "rs")
///
/// # Returns
/// * `io::Result<Vec<PathBuf>>` - 해당 확장자 파일들의 경로 벡터
pub fn find_files_by_extension(dir_path: &str, extension: &str) -> io::Result<Vec<PathBuf>> {
    Ok(std::fs::read_dir(dir_path)?
        .filter_map(|entry| {
            entry
                .ok()
                .filter(|e| e.file_type().map_or(false, |t| t.is_file()))
                .filter(|e| e.path().extension().map_or(false, |ext| ext == extension))
                .map(|e| e.path())
        })
        .collect())
}

/// 디렉토리를 재귀적으로 순회하며 모든 파일을 찾습니다.
///
/// # Arguments
/// * `dir_path` - 시작 디렉토리 경로
///
/// # Returns
/// * `io::Result<Vec<PathBuf>>` - 모든 파일들의 경로 벡터
pub fn walk_directory(dir_path: &str) -> io::Result<Vec<PathBuf>> {
    let dirs = std::fs::read_dir(dir_path)?;
    let mut files = Vec::new();
    for dir in dirs {
        let dir = dir?;
        if dir.file_type()?.is_dir() {
            files.extend(walk_directory(dir.path().as_os_str().to_str().unwrap())?);
        } else {
            files.push(dir.path());
        }
    }
    Ok(files)
}

// =============================================================================
// 임무 7: 파일 메타데이터 (File Metadata)
//
// 파일의 속성과 메타데이터를 다루는 방법을 학습합니다.
// =============================================================================

/// 파일 정보를 담는 구조체
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub is_file: bool,
    pub is_dir: bool,
    pub is_readonly: bool,
}

impl FileInfo {
    /// 경로로부터 FileInfo를 생성합니다.
    pub fn from_path(path: &str) -> io::Result<Self> {
        let metadata = fs::metadata(path)?;
        Ok(Self {
            path: PathBuf::from(path),
            size: metadata.len(),
            is_file: metadata.is_file(),
            is_dir: metadata.is_dir(),
            is_readonly: metadata.permissions().readonly(),
        })
    }
}

/// 파일 크기를 반환합니다.
///
/// # Arguments
/// * `path` - 파일 경로
///
/// # Returns
/// * `io::Result<u64>` - 파일 크기 (바이트)
pub fn get_file_size(path: &str) -> io::Result<u64> {
    Ok(fs::metadata(path)?.len())
}

/// 경로가 파일인지 확인합니다.
///
/// # Arguments
/// * `path` - 확인할 경로
///
/// # Returns
/// * `bool` - 파일 여부
pub fn is_file(path: &str) -> bool {
    Path::new(path).is_file()
}

/// 경로가 디렉토리인지 확인합니다.
///
/// # Arguments
/// * `path` - 확인할 경로
///
/// # Returns
/// * `bool` - 디렉토리 여부
pub fn is_directory(path: &str) -> bool {
    Path::new(path).is_dir()
}

/// 경로가 존재하는지 확인합니다.
///
/// # Arguments
/// * `path` - 확인할 경로
///
/// # Returns
/// * `bool` - 존재 여부
pub fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

// =============================================================================
// 최종 임무: 통합 파일 유틸리티
//
// 지금까지 배운 모든 것을 통합하여 실용적인 파일 유틸리티를 구현합니다.
// =============================================================================

/// 파일을 복사합니다.
///
/// # Arguments
/// * `src` - 원본 파일 경로
/// * `dst` - 대상 파일 경로
///
/// # Returns
/// * `io::Result<u64>` - 복사된 바이트 수
pub fn copy_file(src: &str, dst: &str) -> io::Result<u64> {
    fs::copy(src, dst)
}

/// 파일을 이동합니다. (복사 후 삭제)
///
/// # Arguments
/// * `src` - 원본 파일 경로
/// * `dst` - 대상 파일 경로
///
/// # Returns
/// * `io::Result<()>` - 성공 또는 에러
pub fn move_file(src: &str, dst: &str) -> io::Result<()> {
    fs::rename(src, dst)
}

/// 디렉토리와 그 내용을 재귀적으로 삭제합니다.
///
/// # Arguments
/// * `dir_path` - 삭제할 디렉토리 경로
///
/// # Returns
/// * `io::Result<()>` - 성공 또는 에러
pub fn remove_dir_recursive(dir_path: &str) -> io::Result<()> {
    fs::remove_dir_all(dir_path)
}

/// 파일 내용에서 문자열을 찾아 치환합니다.
///
/// # Arguments
/// * `path` - 파일 경로
/// * `from` - 찾을 문자열
/// * `to` - 치환할 문자열
///
/// # Returns
/// * `io::Result<usize>` - 치환된 횟수
pub fn replace_in_file(path: &str, from: &str, to: &str) -> io::Result<usize> {
    let content = fs::read_to_string(path)?;
    let count = content.matches(from).count();
    let new_content = content.replace(from, to);
    fs::write(path, new_content)?;
    Ok(count)
}

/// 디렉토리 내 모든 파일의 총 크기를 계산합니다.
///
/// # Arguments
/// * `dir_path` - 디렉토리 경로
///
/// # Returns
/// * `io::Result<u64>` - 총 크기 (바이트)
pub fn calculate_dir_size(dir_path: &str) -> io::Result<u64> {
    let files = walk_directory(dir_path)?;
    let mut total_size = 0u64;
    for file in files {
        total_size += get_file_size(file.to_str().unwrap())?;
    }
    Ok(total_size)
}
