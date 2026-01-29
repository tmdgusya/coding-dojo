# PostgreSQL MVCC Dojo - 작업 계획서

## TL;DR

> **Quick Summary**: PostgreSQL MVCC의 장점(읽기-쓰기 비차단, 스냅샷 격리)과 단점(dead tuple, VACUUM 필요성, XID wraparound, 쓰기 충돌, 장기 트랜잭션 문제)을 실제 PostgreSQL에 SQL을 날려가며 관찰하고 검증하는 수련장을 생성합니다.
> 
> **Deliverables**:
> - Docker Compose로 PostgreSQL 17 환경 (autovacuum=off)
> - 7개 미션의 pytest 테스트 파일 (사전 작성)
> - 7개 미션의 관찰 함수 스텁 (제자가 구현)
> - docs/README.md (미션 설명), docs/Assistance.md (조수 지시서)
> 
> **Estimated Effort**: Medium
> **Parallel Execution**: YES - 3 waves
> **Critical Path**: Task 1 (인프라) → Task 2 (공통코드) → Tasks 3~9 (미션별, 병렬 가능)

---

## Context

### Original Request
PostgreSQL MVCC에 대해 완벽하게 이해할 수 있도록 디렉토리를 생성. MVCC의 장점/단점을 완벽하게 이해할 수 있도록 세팅.

### Interview Summary
**Key Discussions**:
- 지식 수준: Vacuum/Snapshot까지 이해 (고급)
- 학습 방식: 실제 PostgreSQL 관찰 (직접 구현 아님)
- 환경: Python + uv

**Research Findings**:
- 기존 rate-limiting-dojo 패턴: `src/`, `test/` (NOT `tests/`), `docs/`, TDD 스타일, 한국어
- uv + psycopg2-binary + pytest 이미 설치됨
- Docker, psql 17 사용 가능

### Metis Review
**Identified Gaps** (addressed):
- 학생이 구현하는 것이 무엇인지 명확화 → Python 함수로 다중 연결 시나리오를 오케스트레이션하여 관찰 결과를 dict로 반환
- M5 XID wraparound는 실제 트리거 불가 → 카운터/age 관찰로 한정
- 디렉토리 패턴을 `test/`로 통일 (기존 dojo 패턴)
- conftest에 트랜잭션 롤백+연결 정리 보장 필요
- psycopg2 autocommit 설정 주의사항 명시

---

## Work Objectives

### Core Objective
PostgreSQL MVCC의 장단점 7개 측면을 실제 DB에서 관찰하고 검증하는 TDD 수련장 생성

### Concrete Deliverables
- `mvcc-dojo/` 디렉토리 (uv 프로젝트)
- `docker-compose.yml` (PG17, autovacuum=off, port 15432)
- `src/connection.py` (DB 연결 헬퍼)
- `src/observations.py` (7개 관찰 함수 스텁 - `raise NotImplementedError`)
- `test/conftest.py` (테이블 setup/teardown 픽스처)
- `test/test_mission_1_reader_not_blocking.py` ~ `test/test_mission_7_long_tx_blocks_vacuum.py`
- `docs/README.md` (미션 설명 + 예상 출력)
- `docs/Assistance.md` (조수 지시서)

### Definition of Done
- [ ] `docker compose up -d` → PostgreSQL 연결 가능
- [ ] `uv run pytest test/ -v` → 모든 테스트 실행됨 (NotImplementedError로 FAIL - RED 상태)
- [ ] 각 미션별 독립 실행 가능: `uv run pytest test/test_mission_1*.py -v`

### Must Have
- 한국어 문서 및 테스트 메시지 (기존 dojo 패턴)
- raw SQL via psycopg2 only (ORM 금지)
- 동기식 psycopg2 (async 금지)
- autovacuum=off (dead tuple 관찰을 위해)
- 미션별 독립 실행 가능
- 테스트 파일은 제자가 편집하지 않음 (src/ 만 구현)

### Must NOT Have (Guardrails)
- ORM 사용 금지 (SQLAlchemy 등)
- asyncpg/asyncio 사용 금지
- 8번째 이상의 미션 추가 금지
- connection pooling, SSL 등 프로덕션 패턴 금지
- MVCC 이론 교과서 작성 금지 (관찰 중심, 최소 이론)
- M5에서 실제 XID wraparound 트리거 금지 (카운터 관찰만)

---

## Verification Strategy (MANDATORY)

### Test Decision
- **Infrastructure exists**: YES (uv + pytest 이미 설치됨)
- **User wants tests**: TDD (RED → GREEN 방식)
- **Framework**: pytest

### TDD 구조
테스트 파일은 사전 작성 (RED 상태). 제자가 `src/observations.py`의 함수를 구현하면 GREEN.

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately):
├── Task 1: Docker + 프로젝트 인프라 셋업
└── Task 8: docs/README.md 작성
    Task 9: docs/Assistance.md 작성

Wave 2 (After Task 1):
├── Task 2: 공통 코드 (connection.py, conftest.py, observations.py 스텁)

Wave 3 (After Task 2):
├── Task 3: Mission 1 테스트
├── Task 4: Mission 2 테스트
├── Task 5: Mission 3 테스트
├── Task 6: Mission 4 테스트
├── Task 7: Mission 5 테스트
├── Task 10: Mission 6 테스트
└── Task 11: Mission 7 테스트
```

### Dependency Matrix

| Task | Depends On | Blocks | Can Parallelize With |
|------|-----------|--------|---------------------|
| 1 | None | 2 | 8, 9 |
| 2 | 1 | 3-7, 10-11 | None |
| 3-7, 10-11 | 2 | None | Each other |
| 8 | None | None | 1, 9 |
| 9 | None | None | 1, 8 |

---

## TODOs

- [ ] 1. Docker + 프로젝트 인프라 셋업

  **What to do**:
  - `mvcc-dojo/` 디렉토리 내 uv 프로젝트 초기화 (`uv init --no-readme`)
  - `uv add psycopg2-binary pytest` 실행
  - `docker-compose.yml` 생성:
    ```yaml
    services:
      postgres:
        image: postgres:17
        container_name: mvcc-dojo-pg
        environment:
          POSTGRES_DB: mvcc_dojo
          POSTGRES_USER: dojo
          POSTGRES_PASSWORD: dojo
        ports:
          - "15432:5432"
        command:
          - "postgres"
          - "-c"
          - "log_statement=all"
          - "-c"
          - "autovacuum=off"
    ```
  - `src/__init__.py`, `test/__init__.py` 빈 파일 생성
  - `docker compose up -d` 후 연결 확인

  **Must NOT do**:
  - production 설정 추가 금지
  - SSL, connection pooling 금지

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []
    - 단순 파일 생성 및 커맨드 실행

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 8, 9)
  - **Blocks**: Task 2
  - **Blocked By**: None

  **References**:
  - `rate-limiting-dojo/pyproject.toml` - uv 프로젝트 구조 참고
  - `rate-limiting-dojo/.gitignore` - gitignore 패턴 복사

  **Acceptance Criteria**:
  ```bash
  # 프로젝트 구조 확인
  ls mvcc-dojo/src/__init__.py mvcc-dojo/test/__init__.py mvcc-dojo/docker-compose.yml mvcc-dojo/pyproject.toml
  # Assert: 모든 파일 존재

  # Docker 실행 및 연결 확인
  cd mvcc-dojo && docker compose up -d && sleep 3
  docker compose exec postgres pg_isready -U dojo -d mvcc_dojo
  # Assert: "accepting connections"

  # autovacuum off 확인
  docker compose exec postgres psql -U dojo -d mvcc_dojo -c "SHOW autovacuum;"
  # Assert: "off"
  ```

  **Commit**: YES
  - Message: `feat(mvcc-dojo): 프로젝트 인프라 및 Docker 환경 셋업`
  - Files: `mvcc-dojo/docker-compose.yml, mvcc-dojo/pyproject.toml, mvcc-dojo/src/__init__.py, mvcc-dojo/test/__init__.py`

---

- [ ] 2. 공통 코드 생성 (connection.py, conftest.py, observations.py 스텁)

  **What to do**:
  - `src/connection.py` 생성:
    - `DB_CONFIG` dict (host=localhost, port=15432, dbname=mvcc_dojo, user=dojo, password=dojo)
    - `get_connection(autocommit=False)` → psycopg2 연결 반환
    - `execute(sql, params=None, autocommit=True)` → 단일 SQL 실행 헬퍼
  - `src/observations.py` 생성:
    - 7개 관찰 함수 스텁 (각각 `raise NotImplementedError("Mission N: ...")`)
    - 함수명: `observe_reader_not_blocking_writer`, `observe_snapshot_isolation`, `observe_dead_tuples`, `observe_vacuum_effect`, `observe_xid_status`, `observe_write_write_conflict`, `observe_long_tx_blocks_vacuum`
    - 각 함수에 상세한 docstring: 시나리오, 기대하는 반환값 dict 구조
  - `test/conftest.py` 생성:
    - `autouse` fixture: 각 테스트 전 테이블 생성, 후 DROP
    - 테이블: `reader_writer_test`, `accounts`, `bloat_test`, `vacuum_test`, `conflict_test`, `long_tx_test`
    - **중요**: fixture teardown에서 반드시 rollback + connection close 보장
    - psycopg2 autocommit=True로 DDL 실행

  **Must NOT do**:
  - ORM 사용 금지
  - async 사용 금지
  - 관찰 함수에 실제 구현 넣지 않음 (NotImplementedError만)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-low`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2 (sequential after Task 1)
  - **Blocks**: Tasks 3-7, 10-11
  - **Blocked By**: Task 1

  **References**:
  - `rate-limiting-dojo/src/__init__.py` - 패키지 구조
  - `rate-limiting-dojo/test/test_rate_limiter.py` - 테스트 패턴 참고
  - psycopg2 docs: `psycopg2.connect()`, `connection.autocommit`, `cursor.execute()`

  **Acceptance Criteria**:
  ```bash
  # 파일 존재 확인
  ls mvcc-dojo/src/connection.py mvcc-dojo/src/observations.py mvcc-dojo/test/conftest.py
  # Assert: 모든 파일 존재

  # connection 모듈 임포트 확인
  cd mvcc-dojo && uv run python -c "from src.connection import get_connection, execute; print('OK')"
  # Assert: "OK"

  # observations 모듈에 7개 함수 존재 확인
  cd mvcc-dojo && uv run python -c "
  from src.observations import (
      observe_reader_not_blocking_writer,
      observe_snapshot_isolation,
      observe_dead_tuples,
      observe_vacuum_effect,
      observe_xid_status,
      observe_write_write_conflict,
      observe_long_tx_blocks_vacuum,
  )
  print('All 7 functions imported')
  "
  # Assert: "All 7 functions imported"
  ```

  **Commit**: YES
  - Message: `feat(mvcc-dojo): 공통 연결 모듈, 관찰 함수 스텁, 테스트 픽스처 생성`
  - Files: `mvcc-dojo/src/connection.py, mvcc-dojo/src/observations.py, mvcc-dojo/test/conftest.py`

---

- [ ] 3. Mission 1 테스트: Readers Don't Block Writers (MVCC 장점)

  **What to do**:
  - `test/test_mission_1_reader_not_blocking.py` 생성
  - 테스트 클래스 `TestMission1ReaderNotBlocking` with 4 tests:
    - `test_writer_is_not_blocked_by_reader`: `result["writer_blocked"]` is False
    - `test_reader_sees_original_value`: `result["reader_saw"]` == "original"
    - `test_reader_still_sees_old_value_after_write`: `result["reader_still_sees"]` == "original"
    - `test_writer_successfully_updated`: `result["writer_new_value"]` == "modified"
  - 모든 테스트는 `observe_reader_not_blocking_writer()` 호출
  - 한국어 assertion 메시지

  **Must NOT do**:
  - 테스트에 구현 로직 넣지 않음
  - conftest에서 이미 테이블 생성하므로 중복 DDL 금지

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Tasks 4-7, 10-11)
  - **Blocks**: None
  - **Blocked By**: Task 2

  **References**:
  - `rate-limiting-dojo/test/test_rate_limiter.py` - 테스트 작성 패턴 (한국어 메시지)
  - Task 2의 `observations.py` 스텁 - `observe_reader_not_blocking_writer` 시그니처

  **Acceptance Criteria**:
  ```bash
  cd mvcc-dojo && uv run pytest test/test_mission_1_reader_not_blocking.py -v 2>&1 | grep -E "FAILED|PASSED|ERROR"
  # Assert: 4 FAILED (NotImplementedError) - RED 상태
  ```

  **Commit**: NO (groups with Tasks 4-7, 10-11)

---

- [ ] 4. Mission 2 테스트: Snapshot Isolation (MVCC 장점)

  **What to do**:
  - `test/test_mission_2_snapshot_isolation.py` 생성
  - 테스트 클래스 `TestMission2SnapshotIsolation` with 4 tests:
    - `test_reader_sees_consistent_total`: `result["total_seen_by_reader"]` == 2000
    - `test_reader_sees_pre_transfer_alice`: `result["alice_before_transfer"]` == 1000
    - `test_reader_sees_pre_transfer_bob`: `result["bob_after_transfer_committed"]` == 1000
    - `test_isolation_level`: `result["isolation_level"]` == "REPEATABLE READ"
  - 시나리오: accounts 테이블, Alice→Bob 500원 이체 중 reader가 일관된 스냅샷을 보는지

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3
  - **Blocks**: None
  - **Blocked By**: Task 2

  **References**:
  - Task 2의 `observations.py` - `observe_snapshot_isolation` 시그니처
  - PostgreSQL docs: `SET TRANSACTION ISOLATION LEVEL REPEATABLE READ`

  **Acceptance Criteria**:
  ```bash
  cd mvcc-dojo && uv run pytest test/test_mission_2_snapshot_isolation.py -v 2>&1 | grep -E "FAILED|PASSED|ERROR"
  # Assert: 4 FAILED (NotImplementedError) - RED 상태
  ```

  **Commit**: NO (groups with other mission tests)

---

- [ ] 5. Mission 3 테스트: Dead Tuples & Table Bloat (MVCC 단점)

  **What to do**:
  - `test/test_mission_3_dead_tuples.py` 생성
  - 테스트 클래스 `TestMission3DeadTuples` with 4 tests:
    - `test_dead_tuples_created_after_update`: `result["dead_tuples"]` >= 900
    - `test_live_tuples_remain`: `result["live_tuples"]` >= 900
    - `test_dead_ratio_is_significant`: `result["dead_ratio"]` >= 0.4
    - `test_table_has_physical_size`: `result["table_size_bytes"]` > 0
  - 시나리오: 1000행 INSERT → 전체 UPDATE → pg_stat_user_tables로 dead tuple 확인
  - **주의**: `pg_stat_user_tables` 통계는 `ANALYZE` 후에 갱신됨을 docstring에 힌트

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3
  - **Blocks**: None
  - **Blocked By**: Task 2

  **References**:
  - Task 2의 `observations.py` - `observe_dead_tuples` 시그니처
  - PostgreSQL docs: `pg_stat_user_tables`, `pg_total_relation_size()`

  **Acceptance Criteria**:
  ```bash
  cd mvcc-dojo && uv run pytest test/test_mission_3_dead_tuples.py -v 2>&1 | grep -E "FAILED|PASSED|ERROR"
  # Assert: 4 FAILED - RED 상태
  ```

  **Commit**: NO

---

- [ ] 6. Mission 4 테스트: VACUUM의 필요성 (MVCC 단점 → 해결책)

  **What to do**:
  - `test/test_mission_4_vacuum.py` 생성
  - 테스트 클래스 `TestMission4Vacuum` with 4 tests:
    - `test_dead_tuples_exist_before_vacuum`: `result["dead_before_vacuum"]` >= 900
    - `test_vacuum_clears_dead_tuples`: `result["dead_after_vacuum"]` < 100
    - `test_vacuum_full_reclaims_space`: `result["space_reclaimed"]` is True
    - `test_vacuum_full_reduces_size`: `result["size_after_vacuum_full"]` < `result["size_before_vacuum_full"]`
  - 시나리오: INSERT→UPDATE→dead tuple 확인→VACUUM→재확인→VACUUM FULL→크기 비교

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3
  - **Blocks**: None
  - **Blocked By**: Task 2

  **References**:
  - Task 2의 `observations.py` - `observe_vacuum_effect` 시그니처
  - PostgreSQL docs: `VACUUM`, `VACUUM FULL`, `pg_total_relation_size()`

  **Acceptance Criteria**:
  ```bash
  cd mvcc-dojo && uv run pytest test/test_mission_4_vacuum.py -v 2>&1 | grep -E "FAILED|PASSED|ERROR"
  # Assert: 4 FAILED - RED 상태
  ```

  **Commit**: NO

---

- [ ] 7. Mission 5 테스트: Transaction ID Wraparound (MVCC 단점)

  **What to do**:
  - `test/test_mission_5_xid_wraparound.py` 생성
  - 테스트 클래스 `TestMission5XidWraparound` with 4 tests:
    - `test_current_xid_is_valid`: `result["current_xid"]` > 0
    - `test_max_xid_is_2_billion`: `result["max_xid"]` == 2**31
    - `test_wraparound_remaining_calculated`: `result["remaining_before_wraparound"]` > 0
    - `test_wraparound_percentage_is_low`: `result["wraparound_pct"]` < 1.0
  - **절대 금지**: 실제 XID wraparound 트리거 시도. 카운터/age 관찰만.

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3
  - **Blocks**: None
  - **Blocked By**: Task 2

  **References**:
  - Task 2의 `observations.py` - `observe_xid_status` 시그니처
  - PostgreSQL docs: `txid_current()`, `age()`, `datfrozenxid`, `autovacuum_freeze_max_age`

  **Acceptance Criteria**:
  ```bash
  cd mvcc-dojo && uv run pytest test/test_mission_5_xid_wraparound.py -v 2>&1 | grep -E "FAILED|PASSED|ERROR"
  # Assert: 4 FAILED - RED 상태
  ```

  **Commit**: NO

---

- [ ] 8. docs/README.md 작성

  **What to do**:
  - `docs/README.md` 생성 (한국어)
  - 내용:
    - 수련장 목표: MVCC 장단점 체감
    - 환경 셋업 가이드 (`docker compose up -d`, `uv run pytest test/ -v`)
    - 7개 미션 요약 (미션명, 장점/단점 분류, 핵심 관찰 포인트)
    - 각 미션별:
      - 설명 (2-3문장)
      - 관찰해야 할 PostgreSQL 시스템 카탈로그/함수
      - 예상 출력 결과 (반환 dict 구조)
      - 실행 방법: `uv run pytest test/test_mission_N_*.py -v`
    - 사용법 섹션
  - 기존 `rate-limiting-dojo/docs/README.md` 스타일 따름

  **Must NOT do**:
  - MVCC 교과서 작성 금지 (관찰 포인트만 안내)
  - 정답/구현 힌트 넣지 않음

  **Recommended Agent Profile**:
  - **Category**: `writing`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 9)
  - **Blocks**: None
  - **Blocked By**: None

  **References**:
  - `rate-limiting-dojo/docs/README.md` - 기존 README 스타일/톤 참고
  - Task 2의 observations.py 스텁 - 각 함수의 docstring에서 시나리오/반환값 참고

  **Acceptance Criteria**:
  ```bash
  test -f mvcc-dojo/docs/README.md && wc -l mvcc-dojo/docs/README.md
  # Assert: 파일 존재, 100줄 이상
  grep -c "Mission\|미션\|임무" mvcc-dojo/docs/README.md
  # Assert: 7개 이상 미션 언급
  ```

  **Commit**: NO (groups with Task 9)

---

- [ ] 9. docs/Assistance.md 작성

  **What to do**:
  - `docs/Assistance.md` 생성 (한국어)
  - 조수 지시서:
    - 원칙: 정답 제공 금지, 소크라테스식 문답
    - 각 미션별 가이드:
      - M1: "두 커넥션을 어떻게 동시에 유지할 수 있을까요?" "autocommit 설정의 의미는?"
      - M2: "REPEATABLE READ와 READ COMMITTED의 차이는?" "conn.set_session() 사용법 확인해보셨나요?"
      - M3: "pg_stat_user_tables에서 어떤 컬럼이 dead tuple을 보여줄까요?" "ANALYZE 실행 필요성?"
      - M4: "VACUUM과 VACUUM FULL의 차이는?" "왜 VACUUM FULL은 exclusive lock이 필요할까요?"
      - M5: "txid_current()와 age()의 관계는?" "datfrozenxid는 어디서 조회할까요?"
      - M6: "SERIALIZABLE에서 왜 UPDATE 순서가 중요할까요?" "psycopg2.errors.SerializationFailure 예외 처리는?"
      - M7: "pg_stat_activity에서 오래된 트랜잭션을 어떻게 찾을까요?" "xmin horizon이란?"
  - 기존 `rate-limiting-dojo/docs/Assistance.md` 스타일 따름

  **Must NOT do**:
  - 직접적인 SQL 쿼리 답변 금지
  - 완성된 코드 제공 금지

  **Recommended Agent Profile**:
  - **Category**: `writing`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 8)
  - **Blocks**: None
  - **Blocked By**: None

  **References**:
  - `rate-limiting-dojo/docs/Assistance.md` - 기존 조수 지시서 스타일/톤 참고

  **Acceptance Criteria**:
  ```bash
  test -f mvcc-dojo/docs/Assistance.md && wc -l mvcc-dojo/docs/Assistance.md
  # Assert: 파일 존재, 40줄 이상
  grep -c "Mission\|미션\|임무" mvcc-dojo/docs/Assistance.md
  # Assert: 7개 이상 미션 가이드
  ```

  **Commit**: NO (groups with Task 8)

---

- [ ] 10. Mission 6 테스트: Write-Write Conflict (MVCC 한계)

  **What to do**:
  - `test/test_mission_6_write_conflict.py` 생성
  - 테스트 클래스 `TestMission6WriteConflict` with 4 tests:
    - `test_both_transactions_read_same_value`: conn1_read==0, conn2_read==0
    - `test_first_committer_wins`: `result["conn1_commit"]` == "success"
    - `test_second_committer_fails`: `result["conn2_commit"]` == "serialization_failure"
    - `test_error_code_is_40001`: `result["error_code"]` == "40001"
  - 시나리오: SERIALIZABLE 격리수준, 같은 행 동시 UPDATE, 먼저 커밋한 쪽 승리
  - **주의**: psycopg2.errors.SerializationFailure 예외의 pgcode 활용

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3
  - **Blocks**: None
  - **Blocked By**: Task 2

  **References**:
  - Task 2의 `observations.py` - `observe_write_write_conflict` 시그니처
  - PostgreSQL docs: SERIALIZABLE isolation, error code 40001

  **Acceptance Criteria**:
  ```bash
  cd mvcc-dojo && uv run pytest test/test_mission_6_write_conflict.py -v 2>&1 | grep -E "FAILED|PASSED|ERROR"
  # Assert: 4 FAILED - RED 상태
  ```

  **Commit**: NO

---

- [ ] 11. Mission 7 테스트: Long Transaction Blocks VACUUM (MVCC 단점)

  **What to do**:
  - `test/test_mission_7_long_tx_blocks_vacuum.py` 생성
  - 테스트 클래스 `TestMission7LongTxBlocksVacuum` with 4 tests:
    - `test_vacuum_cannot_clean_with_old_tx`: `result["dead_after_first_vacuum"]` >= 400
    - `test_old_tx_is_open_during_first_vacuum`: `result["old_tx_still_open"]` is True
    - `test_vacuum_works_after_old_tx_closes`: `result["dead_after_close_and_vacuum"]` < 100
    - `test_vacuum_could_finally_clean`: `result["vacuum_could_clean"]` is True
  - 시나리오: 오래된 트랜잭션 유지 → UPDATE → VACUUM 실패 → TX 종료 → VACUUM 성공

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3
  - **Blocks**: None
  - **Blocked By**: Task 2

  **References**:
  - Task 2의 `observations.py` - `observe_long_tx_blocks_vacuum` 시그니처
  - PostgreSQL docs: `pg_stat_activity`, xmin horizon, VACUUM behavior with open transactions

  **Acceptance Criteria**:
  ```bash
  cd mvcc-dojo && uv run pytest test/test_mission_7_long_tx_blocks_vacuum.py -v 2>&1 | grep -E "FAILED|PASSED|ERROR"
  # Assert: 4 FAILED - RED 상태
  ```

  **Commit**: NO

---

- [ ] 12. 최종 커밋 및 검증

  **What to do**:
  - 모든 테스트/문서 파일을 하나의 커밋으로 묶기
  - 전체 테스트 RED 상태 확인
  - Docker 환경 정상 동작 확인

  **Recommended Agent Profile**:
  - **Category**: `quick`
  - **Skills**: [`git-master`]

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential (final)
  - **Blocks**: None
  - **Blocked By**: All tasks

  **Acceptance Criteria**:
  ```bash
  # Docker 실행 상태 확인
  cd mvcc-dojo && docker compose exec postgres pg_isready -U dojo -d mvcc_dojo
  # Assert: "accepting connections"

  # 전체 테스트 실행 (모두 FAIL = RED 상태가 정상)
  cd mvcc-dojo && uv run pytest test/ -v 2>&1 | tail -5
  # Assert: "7 failed" 또는 "28 failed" (7 미션 × 4 테스트)
  # Assert: 0 errors (import/구조 에러 없어야 함)

  # 미션별 독립 실행 확인
  cd mvcc-dojo && uv run pytest test/test_mission_1_reader_not_blocking.py -v 2>&1 | grep "FAILED"
  # Assert: 4 FAILED
  ```

  **Commit**: YES
  - Message: `feat(mvcc-dojo): MVCC 장단점 관찰 수련장 - 7개 미션 테스트 및 문서`
  - Files: all test files, docs/README.md, docs/Assistance.md
  - Pre-commit: `cd mvcc-dojo && uv run pytest test/ --co -q` (collect only - 구조 확인)

---

## Commit Strategy

| After Task | Message | Files | Verification |
|-----------|---------|-------|-------------|
| 1 | `feat(mvcc-dojo): 프로젝트 인프라 및 Docker 환경 셋업` | docker-compose.yml, pyproject.toml, __init__.py files | docker compose exec postgres pg_isready |
| 2 | `feat(mvcc-dojo): 공통 연결 모듈, 관찰 함수 스텁, 테스트 픽스처` | src/connection.py, src/observations.py, test/conftest.py | uv run python -c "from src.observations import ..." |
| 12 | `feat(mvcc-dojo): 7개 미션 테스트 및 문서` | test/test_mission_*.py, docs/*.md | uv run pytest test/ -v |

---

## Success Criteria

### Verification Commands
```bash
# 1. Docker 환경
cd mvcc-dojo && docker compose up -d && sleep 3
docker compose exec postgres pg_isready -U dojo -d mvcc_dojo
# Expected: "accepting connections"

# 2. autovacuum 비활성화
docker compose exec postgres psql -U dojo -d mvcc_dojo -c "SHOW autovacuum;"
# Expected: "off"

# 3. 전체 테스트 수집 (구조 확인)
cd mvcc-dojo && uv run pytest test/ --co -q
# Expected: 28 tests collected (7 missions × 4 tests)

# 4. 전체 테스트 실행 (RED 상태)
cd mvcc-dojo && uv run pytest test/ -v
# Expected: 28 failed, 0 errors

# 5. 미션별 독립 실행
cd mvcc-dojo && uv run pytest test/test_mission_3_dead_tuples.py -v
# Expected: 4 failed, 0 errors
```

### Final Checklist
- [ ] 7개 미션 테스트 파일 존재 (test/test_mission_N_*.py)
- [ ] src/observations.py에 7개 함수 스텁 (모두 NotImplementedError)
- [ ] Docker PostgreSQL 17 + autovacuum=off
- [ ] docs/README.md 미션 설명 포함
- [ ] docs/Assistance.md 조수 지시서 포함
- [ ] 모든 테스트 RED 상태 (0 errors, 28 failures)
- [ ] 한국어 문서 및 메시지
