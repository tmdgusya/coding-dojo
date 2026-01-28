"""MVCC 동작 관찰 함수들"""


def observe_reader_not_blocking_writer():
    """MVCC의 핵심 장점: 읽기가 쓰기를 차단하지 않음을 관찰합니다.

    시나리오:
    1. conn1에서 BEGIN 후 SELECT (읽기 트랜잭션 유지)
    2. conn2에서 동일 행을 UPDATE (쓰기)
    3. conn2의 UPDATE가 차단 없이 성공하는지 확인

    Returns:
        dict: {
            "reader_saw": <conn1이 읽은 값>,
            "writer_blocked": <bool - conn2가 차단되었는지>,
            "writer_new_value": <conn2가 쓴 새 값>,
            "reader_still_sees": <conn1이 다시 읽은 값 (snapshot!)>,
        }
    """
    raise NotImplementedError("Mission 1: 읽기가 쓰기를 차단하지 않음을 관찰하세요")


def observe_snapshot_isolation():
    """스냅샷 격리: 트랜잭션이 시작 시점의 일관된 데이터를 봄을 관찰합니다.

    시나리오:
    1. conn1에서 BEGIN (스냅샷 생성)
    2. conn2에서 데이터 변경 후 COMMIT
    3. conn1에서 여전히 이전 값을 읽는지 확인
    4. conn1 COMMIT 후 새 트랜잭션에서 변경된 값 확인

    Returns:
        dict: {
            "initial_value": <초기 값>,
            "tx1_sees_before_commit": <tx1이 본 값 (변경 전)>,
            "tx2_committed_value": <tx2가 커밋한 값>,
            "tx1_sees_after_tx2_commit": <tx2 커밋 후에도 tx1이 본 값>,
            "new_tx_sees": <새 트랜잭션이 본 값>,
        }
    """
    raise NotImplementedError("Mission 2: 스냅샷 격리를 관찰하세요")


def observe_dead_tuples():
    """UPDATE/DELETE 시 dead tuple이 생성됨을 관찰합니다.

    시나리오:
    1. 테이블에 데이터 삽입
    2. pg_stat_user_tables에서 n_live_tup, n_dead_tup 확인
    3. UPDATE 실행
    4. dead tuple 수가 증가했는지 확인

    Returns:
        dict: {
            "before_update": {"n_live_tup": int, "n_dead_tup": int},
            "after_update": {"n_live_tup": int, "n_dead_tup": int},
            "dead_tuples_created": int,
        }
    """
    raise NotImplementedError("Mission 3: dead tuple 생성을 관찰하세요")


def observe_vacuum_effect():
    """VACUUM이 dead tuple을 정리함을 관찰합니다.

    시나리오:
    1. 데이터 삽입 후 여러 번 UPDATE (dead tuple 생성)
    2. VACUUM 실행 전 dead tuple 수 확인
    3. VACUUM 실행
    4. dead tuple이 제거되었는지 확인

    Returns:
        dict: {
            "before_vacuum": {"n_live_tup": int, "n_dead_tup": int},
            "after_vacuum": {"n_live_tup": int, "n_dead_tup": int},
            "reclaimed_tuples": int,
        }
    """
    raise NotImplementedError("Mission 4: VACUUM 효과를 관찰하세요")


def observe_xid_status():
    """트랜잭션 ID(XID)와 가시성 규칙을 관찰합니다.

    시나리오:
    1. 현재 트랜잭션 ID 확인 (txid_current())
    2. 데이터 삽입 후 xmin 확인
    3. UPDATE 후 xmin, xmax 확인
    4. xmin < xmax 관계 확인

    Returns:
        dict: {
            "current_xid": int,
            "inserted_xmin": int,
            "after_update_xmin": int,
            "after_update_xmax": int,
            "xmax_marks_deletion": bool,
        }
    """
    raise NotImplementedError("Mission 5: XID 상태를 관찰하세요")


def observe_write_write_conflict():
    """동시 쓰기 충돌 시 나중 트랜잭션이 대기함을 관찰합니다.

    시나리오:
    1. conn1에서 BEGIN 후 UPDATE (커밋하지 않음)
    2. conn2에서 동일 행 UPDATE 시도 (차단됨)
    3. conn1 COMMIT 후 conn2가 진행되는지 확인

    Returns:
        dict: {
            "tx1_updated_to": <tx1이 변경한 값>,
            "tx2_blocked": bool,
            "tx2_updated_to": <tx2가 변경한 값>,
            "final_value": <최종 값>,
        }
    """
    raise NotImplementedError("Mission 6: 쓰기-쓰기 충돌을 관찰하세요")


def observe_long_tx_blocks_vacuum():
    """오래 실행되는 트랜잭션이 VACUUM을 방해함을 관찰합니다.

    시나리오:
    1. conn1에서 BEGIN (트랜잭션 유지)
    2. 데이터 변경 후 VACUUM 실행
    3. conn1이 열려있는 동안 dead tuple이 제거되지 않음 확인
    4. conn1 COMMIT 후 VACUUM 재실행 시 정리됨 확인

    Returns:
        dict: {
            "dead_tuples_before_vacuum": int,
            "dead_tuples_after_vacuum_with_long_tx": int,
            "dead_tuples_after_long_tx_commit": int,
            "vacuum_blocked_by_long_tx": bool,
        }
    """
    raise NotImplementedError("Mission 7: 긴 트랜잭션이 VACUUM을 차단함을 관찰하세요")
