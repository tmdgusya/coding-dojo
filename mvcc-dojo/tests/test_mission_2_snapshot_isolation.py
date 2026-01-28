"""Mission 2: 스냅샷 격리(Snapshot Isolation) 테스트"""

import pytest
from src.observations import observe_snapshot_isolation


class TestMission2SnapshotIsolation:
    """스냅샷 격리 동작을 검증하는 테스트 클래스

    시나리오: Alice와 Bob의 계좌에서 Alice가 Bob에게 500원을 이체하는 상황
    - Alice 초기 잔액: 1000원
    - Bob 초기 잔액: 1000원
    - 이체 금액: 500원
    - 최종 총액: 2000원 (변하지 않음)

    스냅샷 격리는 트랜잭션이 시작 시점의 일관된 데이터를 보장합니다.
    """

    def test_reader_sees_consistent_total(self):
        """트랜잭션이 시작 시점의 일관된 총액을 봐야 합니다.

        검증: 이체 중에도 총액은 항상 2000원이어야 함
        """
        result = observe_snapshot_isolation()
        assert result["total_seen_by_reader"] == 2000, (
            "스냅샷 격리: 트랜잭션은 일관된 총액을 봐야 합니다"
        )

    def test_reader_sees_pre_transfer_alice(self):
        """트랜잭션이 시작 시점의 Alice 잔액을 봐야 합니다.

        검증: 이체 전 Alice 잔액은 1000원이어야 함
        """
        result = observe_snapshot_isolation()
        assert result["alice_before_transfer"] == 1000, (
            "스냅샷 격리: 트랜잭션은 이체 전 Alice 잔액을 봐야 합니다"
        )

    def test_reader_sees_pre_transfer_bob(self):
        """트랜잭션이 시작 시점의 Bob 잔액을 봐야 합니다.

        검증: 이체 후 커밋된 Bob 잔액은 1000원이어야 함 (스냅샷 격리)
        """
        result = observe_snapshot_isolation()
        assert result["bob_after_transfer_committed"] == 1000, (
            "스냅샷 격리: 트랜잭션은 이체 전 Bob 잔액을 봐야 합니다"
        )

    def test_isolation_level(self):
        """데이터베이스의 격리 수준이 REPEATABLE READ여야 합니다.

        검증: PostgreSQL의 기본 격리 수준은 REPEATABLE READ
        """
        result = observe_snapshot_isolation()
        assert result["isolation_level"] == "REPEATABLE READ", (
            "격리 수준이 REPEATABLE READ여야 합니다"
        )
