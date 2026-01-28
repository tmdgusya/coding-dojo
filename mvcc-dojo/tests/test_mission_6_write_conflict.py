"""Mission 6: 쓰기-쓰기 충돌(Write-Write Conflict) 테스트

SERIALIZABLE 충돌을 테스트합니다.
- 두 트랜잭션이 동시에 같은 행을 수정하려 할 때
- 나중 트랜잭션이 대기하거나 실패함
- 직렬화 가능성(Serializability) 보장
"""

import pytest
from src.observations import observe_write_write_conflict


class TestMission6WriteConflict:
    """쓰기-쓰기 충돌 동작을 검증하는 테스트 클래스

    시나리오: 두 트랜잭션이 동일한 행을 동시에 수정하려는 상황
    - conn1: 먼저 UPDATE 시작 (커밋하지 않음)
    - conn2: 동일 행 UPDATE 시도 (차단 또는 실패)
    - conn1: COMMIT
    - conn2: 진행 또는 실패

    SERIALIZABLE 충돌: 나중 트랜잭션이 대기하거나 serialization_failure 발생
    """

    def test_both_transactions_read_same_value(self):
        """두 트랜잭션이 같은 초기값을 읽어야 합니다.

        검증: 충돌 전 두 연결이 동일한 값을 읽음
        """
        result = observe_write_write_conflict()
        assert result["conn1_read"] == 0, "conn1이 초기값 0을 읽어야 합니다"
        assert result["conn2_read"] == 0, "conn2가 초기값 0을 읽어야 합니다"

    def test_first_committer_wins(self):
        """먼저 커밋한 트랜잭션이 성공해야 합니다.

        검증: conn1의 UPDATE가 성공적으로 커밋됨
        """
        result = observe_write_write_conflict()
        assert result["conn1_commit"] == "success", "conn1의 커밋이 성공해야 합니다"

    def test_second_committer_fails(self):
        """나중 커밋 시도가 실패해야 합니다.

        검증: conn2의 UPDATE가 serialization_failure로 실패
        """
        result = observe_write_write_conflict()
        assert result["conn2_commit"] == "serialization_failure", (
            "conn2의 커밋이 serialization_failure로 실패해야 합니다"
        )

    def test_error_code_is_40001(self):
        """PostgreSQL 직렬화 실패 에러 코드가 40001이어야 합니다.

        검증: SQLSTATE 40001 (serialization_failure)
        """
        result = observe_write_write_conflict()
        assert result["error_code"] == "40001", (
            "에러 코드가 40001(serialization_failure)이어야 합니다"
        )
