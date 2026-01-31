# BM-25 도장 🥋

BM-25 (Best Match 25) 알고리즘을 **손으로 구현하며 익히는** 학습 프로젝트입니다.

## BM-25란?

BM-25는 정보 검색(Information Retrieval) 분야에서 가장 널리 사용되는 랭킹 함수입니다. TF-IDF의 한계를 극복하기 위해 개발되었으며, **Elasticsearch**, **Lucene**, **Whoosh** 등 수많은 검색 엔진의 핵심 알고리즘으로 사용됩니다.

### 핵심 공식

```
BM25(D, Q) = Σ IDF(q_i) * TF_component(q_i, D)

IDF(q_i) = log((N - n(q_i) + 0.5) / (n(q_i) + 0.5))

TF_component = (f(q_i, D) * (k1 + 1)) / (f(q_i, D) + k1 * (1 - b + b * |D| / avgdl))
```

| 변수 | 설명 |
|------|------|
| N | 전체 문서 수 |
| n(qᵢ) | 단어 qᵢ를 포함하는 문서 수 |
| f(qᵢ, D) | 문서 D에서 qᵢ의 출현 빈도 (TF) |
| \|D\| | 문서 D의 길이 (단어 수) |
| avgdl | 전체 문서의 평균 길이 |
| k1 | TF saturation 파라미터 (기본값: 1.5) |
| b | 문서 길이 정규화 파라미터 (기본값: 0.75) |

---

## 왜 BM-25가 빠른가?

### 1. 역색인(Inverted Index) 구조

```python
# 핵심 자료구조
inverted_index: Dict[str, Dict[int, int]] = {
    "python": {0: 2, 1: 1, 3: 1},    # 문서0: 2회, 문서1: 1회, 문서3: 1회
    "java": {1: 1, 4: 3},             # 문서1: 1회, 문서4: 3회
    "rust": {2: 5},                   # 문서2: 5회
    # ...
}
```

- **O(1)**로 단어가 등장하는 문서 목록 조회
- 전체 문서를 스캔할 필요 없음
- 쿼리에 포함된 단어의 포스팅 리스트만 병합

### 2. 사전 계산된 IDF 캐싱

```python
# 인덱스 구축 시 한 번만 계산
idf_cache: Dict[str, float] = {
    "python": 1.2345,
    "java": 0.8765,
    # ...
}
```

- IDF는 고정값이므로 캐싱하여 재사용
- 검색 시 **O(1)**로 조회

### 3. 효율적인 메모리 사용

- 문서는 ID로 참조 (문자열 복제 없음)
- TF만 저장 (위치 정보는 저장하지 않음)

---

## 왜 BM-25가 좋은가?

### 1. TF Saturation (빈도 포화)

단어가 많이 반복된다고 해서 점수가 무한정 증가하지 않습니다.

```
TF=1  → 점수: 1.00
TF=2  → 점수: 1.33  (+0.33)
TF=5  → 점수: 1.67  (+0.34 누적)
TF=10 → 점수: 1.82  (+0.15 누적)
TF=20 → 점수: 1.91  (+0.09 누적)
```

**의미**: 키워드 스태핑(keyword stuffing)에 덜 민감합니다.

### 2. 문서 길이 정규화

긴 문서가 불리하지 않도록 조정합니다.

| 문서 | TF | 길이 | 정규화 전 | 정규화 후 |
|------|-----|------|----------|----------|
| "python" | 1 | 1 | 1.0 | 1.00 |
| "python programming" | 1 | 2 | 0.5 | 0.88 |
| "python is great..." | 1 | 10 | 0.1 | 0.55 |

**의미**: 주제에 집중한 짧은 문서와 포괄적인 긴 문서를 공정하게 평가합니다.

### 3. 안정적인 IDF

TF-IDF의 IDF는 모든 문서에 등장하는 단어에서 음수가 되지만, BM-25는 그렇지 않습니다.

```
# 모든 문서에 등장하는 단어
TF-IDF IDF: log(10/10) = 0
BM-25 IDF:  log((10-10+0.5)/(10+0.5)) ≈ -0.02  (거의 0에 가까움)
```

---

## 파일 구조

```
bm25-dojo/
├── bm25.py          # BM-25 핵심 구현
├── test_bm25.py     # 테스트 및 학습 스크립트
├── README.md        # 이 파일
└── pyproject.toml   # 프로젝트 설정
```

---

## 실행 방법

### 기본 데모 실행

```bash
uv run python bm25.py
```

### 테스트 실행

```bash
uv run python test_bm25.py
```

---

## 학습 경로

### 🔰 입문자
1. `bm25.py`의 `BM25` 클래스를 읽어보기
2. `_build_index()` 메서드 이해하기
3. `_compute_idf()`와 `_compute_tf_component()` 공식 이해하기
4. 기본 데모 실행핳기

### 🥋 중급자
1. `test_bm25.py`의 `test_tf_saturation()` 분석하기
2. `k1` 파라미터 변경핳며 실험하기
3. `b` 파라미터 변경핳며 실험하기
4. 역색인 구조 직접 출력핳기 (`get_term_info()` 활용)

### 🥷 고급자
1. 대용량 데이터셋으로 성능 테스트
2. 쿼리 최적화 (and/or/not 연산 구현)
3. 멀티스레딩 검색 구현
4. 디스크 기반 인덱싱 구현

---

## 파라미터 튜닝 가이드

| 파라미터 | 기본값 | 효과 | 조정 팁 |
|---------|-------|------|--------|
| k1 | 1.5 | TF saturation 정도 | 키워드 매칭 중시 → 2.0, 의미 중시 → 1.2 |
| b | 0.75 | 문서 길이 정규화 강도 | 짧은 문서 선호 → 0.3, 긴 문서 허용 → 0.9 |

---

## 참고 자료

- [Okapi BM25 - Wikipedia](https://en.wikipedia.org/wiki/Okapi_BM25)
- [The Probabilistic Relevance Framework: BM25 and Beyond](https://www.staff.city.ac.uk/~sb317/papers/foundations_bm25_review.pdf)
- [Elasticsearch BM25 Similarity](https://www.elastic.co/guide/en/elasticsearch/reference/current/index-modules-similarity.html#bm25)

---

## 도전 과제 🎯

1. **한글 문서 검색**: 한글 문서로 테스트핳기
2. **실시간 인덱싱**: 문서 추가/삭제 기능 구현
3. **Boosting**: 특정 필드에 가중치 부여
4. **Fuzzy Search**: 오타 허용 검색 추가

---

**행운을 빕니다, 사부! 🥋**
