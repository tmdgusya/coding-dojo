# PostgreSQL MVCC Dojo

## 목표
이 수련장은 PostgreSQL의 MVCC(Multi-Version Concurrency Control)를 실제 데이터베이스에서 관찰하고 체감하기 위해 만들어졌습니다.
Roach 님은 이 과정을 통해 MVCC의 장점과 단점을 완벽하게 이해하고, 프로덕션 환경에서 발생할 수 있는 문제들을 미리 경험하게 될 것입니다.

## 환경 셋업

### 1. Docker로 PostgreSQL 실행
```bash
cd mvcc-dojo
docker compose up -d
```

PostgreSQL 17이 포트 15432에서 실행됩니다. `autovacuum=off` 설정으로 dead tuple 관찰이 가능합니다.

### 2. 테스트 실행
```bash
# 전체 테스트 실행
uv run pytest tests/ -v

# 특정 미션만 실행
uv run pytest tests/test_mission_1_reader_not_blocking.py -v
```

## 임무 1: Readers Don't Block Writers (MVCC 장점)

### 설명
MVCC의 가장 큰 장점을 체감합니다. 전통적인 Lock 기반 동시성 제어에서는 읽기 잠금이 쓰기를 차단하지만, PostgreSQL MVCC에서는 읽기 트랜잭션이 쓰기 트랜잭션을 차단하지 않습니다.

### 관찰 포인트
- 두 개의 독립적인 PostgreSQL 연결 생성
- 첫 번째 연결에서 BEGIN 후 SELECT (읽기 트랜잭션 유지)
- 두 번째 연결에서 동일 행을 UPDATE
- 두 번째 연결이 차단 없이 즉시 성공하는지 확인

### 예상 출력
```python
{
    "reader_saw": "original",
    "writer_blocked": False,
    "writer_new_value": "modified",
    "reader_still_sees": "original"  # 스냅샷!
}
```

### 실행 방법
```bash
uv run pytest tests/test_mission_1_reader_not_blocking.py -v
```

---

## 임무 2: Snapshot Isolation (MVCC 장점)

### 설명
REPEATABLE READ 격리 수준에서 트랜잭션이 시작 시점의 "스냅샷"을 일관되게 봅니다. 다른 트랜잭션이 데이터를 변경하고 커밋하더라도, 현재 트랜잭션의 view는 변하지 않습니다.

### 관찰 포인트
- accounts 테이블에 Alice(1000원), Bob(1000원) 준비
- 첫 번째 연결: BEGIN (REPEATABLE READ) → Alice 잔액 조회
- 두 번째 연결: Alice에서 Bob으로 500원 이체 후 COMMIT
- 첫 번째 연결: Bob 잔액 조회 → 합계 계산
- 합계가 여전히 2000원인지 확인 (일관된 스냅샷)

### 예상 출력
```python
{
    "alice_before_transfer": 1000,
    "bob_after_transfer_committed": 1000,  # 커밋 후에도!
    "total_seen_by_reader": 2000,
    "isolation_level": "REPEATABLE READ"
}
```

### 실행 방법
```bash
uv run pytest tests/test_mission_2_snapshot_isolation.py -v
```

---

## 임무 3: Dead Tuples & Table Bloat (MVCC 단점)

### 설명
MVCC의 가장 큰 단점입니다. UPDATE나 DELETE는 실제로 행을 삭제하지 않고 "dead tuple"로 표시만 합니다. 이로 인해 테이블이 "팽창(bloat)"하여 디스크 공간을 낭비하고 성능이 저하됩니다.

### 관찰 포인트
- bloat_test 테이블에 1000행 INSERT
- 모든 행을 UPDATE (value 변경)
- `pg_stat_user_tables` 시스템 카탈로그에서 dead tuple 수 확인
- `pg_total_relation_size()`로 테이블 물리적 크기 확인
- **힌트**: `ANALYZE` 실행 후 통계가 갱신됩니다

### 예상 출력
```python
{
    "live_tuples": 1000,
    "dead_tuples": 1000,  # UPDATE로 생성된 이전 버전들
    "table_size_bytes": 73728,
    "dead_ratio": 0.5
}
```

### 실행 방법
```bash
uv run pytest tests/test_mission_3_dead_tuples.py -v
```

---

## 임무 4: VACUUM의 필요성 (MVCC 단점 → 해결책)

### 설명
VACUUM은 MVCC가 만든 dead tuple을 정리하는 메커니즘입니다. `VACUUM`은 공간을 재사용 가능하게 표시하고, `VACUUM FULL`은 테이블을 완전히 재작성하여 OS에 공간을 반환합니다.

### 관찰 포인트
- vacuum_test 테이블에 1000행 INSERT → 전체 UPDATE
- dead tuple 수 기록 (before)
- `VACUUM vacuum_test` 실행
- dead tuple 수 기록 (after) - 0에 가까워짐
- 테이블 크기 기록 (before VACUUM FULL)
- `VACUUM FULL vacuum_test` 실행
- 테이블 크기 기록 (after) - 크기 감소 확인

### 예상 출력
```python
{
    "dead_before_vacuum": 1000,
    "dead_after_vacuum": 0,
    "size_before_vacuum_full": 73728,
    "size_after_vacuum_full": 40960,
    "space_reclaimed": True
}
```

### 실행 방법
```bash
uv run pytest tests/test_mission_4_vacuum.py -v
```

---

## 임무 5: Transaction ID Wraparound (MVCC 단점)

### 설명
MVCC는 각 트랜잭션에 32비트 XID를 부여합니다. 최대 약 21억(2^31)개까지 사용 가능하며, 이를 초과하면 "wraparound" 위험이 발생합니다. PostgreSQL은 이를 방지하기 위해 aggressive vacuum을 수행하며, 실패 시 DB가 셧다운됩니다.

### 관찰 포인트
- `txid_current()` 함수로 현재 트랜잭션 ID 확인
- `pg_database` 카탈로그에서 `age(datfrozenxid)` 확인
- `autovacuum_freeze_max_age` 설정값 확인
- wraparound까지 남은 트랜잭션 수 계산
- **주의**: 실제 wraparound를 트리거하지 않습니다 (카운터 관찰만)

### 예상 출력
```python
{
    "current_xid": 735,
    "oldest_frozen_xid": 726,
    "xid_age": 9,
    "max_xid": 2147483648,  # 2^31
    "remaining_before_wraparound": 2147483639,
    "wraparound_pct": 0.0001
}
```

### 실행 방법
```bash
uv run pytest tests/test_mission_5_xid_wraparound.py -v
```

---

## 임무 6: Write-Write Conflict (MVCC 한계)

### 설명
MVCC는 읽기-쓰기 충돌은 해결하지만, 쓰기-쓰기 충돌은 피할 수 없습니다. SERIALIZABLE 격리 수준에서 두 트랜잭션이 같은 행을 수정하면, 먼저 커밋한 쪽이 이기고 나중 쪽은 serialization failure (40001) 에러가 발생합니다.

### 관찰 포인트
- conflict_test 테이블에 counter=0인 행 준비
- 첫 번째 연결: BEGIN (SERIALIZABLE) → SELECT counter → UPDATE counter+1
- 두 번째 연결: BEGIN (SERIALIZABLE) → SELECT counter → UPDATE counter+1
- 첫 번째 연결: COMMIT (성공)
- 두 번째 연결: COMMIT → `psycopg2.errors.SerializationFailure` 발생
- 에러 코드 40001 확인

### 예상 출력
```python
{
    "conn1_read": 0,
    "conn2_read": 0,
    "conn1_commit": "success",
    "conn2_commit": "serialization_failure",
    "error_code": "40001"
}
```

### 실행 방법
```bash
uv run pytest tests/test_mission_6_write_conflict.py -v
```

---

## 임무 7: Long Transaction Blocks VACUUM (MVCC 단점)

### 설명
프로덕션에서 가장 흔한 MVCC 관련 장애 원인입니다. 오래 열려있는 트랜잭션이 있으면 VACUUM이 그 시점 이후의 dead tuple을 제거할 수 없습니다. 이로 인해 테이블 bloat가 계속 증가합니다.

### 관찰 포인트
- long_tx_test 테이블에 500행 INSERT
- 첫 번째 연결: BEGIN → SELECT (오래된 트랜잭션 유지)
- 두 번째 연결: 모든 행 UPDATE (dead tuple 생성)
- `VACUUM long_tx_test` 실행
- dead tuple이 여전히 남아있는지 확인 (첫 번째 연결의 스냅샷 때문!)
- 첫 번째 연결: COMMIT (트랜잭션 종료)
- `VACUUM long_tx_test` 다시 실행
- 이제 dead tuple이 정리되었는지 확인

### 예상 출력
```python
{
    "dead_after_first_vacuum": 500,  # 정리 안됨!
    "old_tx_still_open": True,
    "dead_after_close_and_vacuum": 0,  # 정리됨
    "vacuum_could_clean": True
}
```

### 실행 방법
```bash
uv run pytest tests/test_mission_7_long_tx_blocks_vacuum.py -v
```

---

## 사용법

### 전체 테스트 실행
```bash
cd mvcc-dojo
uv run pytest tests/ -v
```

### 특정 미션만 실행
```bash
uv run pytest tests/test_mission_3_dead_tuples.py -v
```

### Docker 환경 정리
```bash
docker compose down -v
```

## 학습 경로

1. **장점 체감** (M1, M2): MVCC가 왜 강력한지 이해
2. **단점 인식** (M3, M5, M7): 프로덕션에서 발생할 문제들 미리 경험
3. **해결책 학습** (M4): VACUUM의 중요성과 동작 원리
4. **한계 이해** (M6): MVCC로도 해결할 수 없는 충돌

모든 미션을 완료하면 PostgreSQL MVCC의 장단점을 완벽하게 이해하게 됩니다!
