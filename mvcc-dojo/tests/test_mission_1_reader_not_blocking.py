"""Mission 1: 읽기가 쓰기를 차단하지 않음 (Readers Don't Block Writers)

MVCC의 핵심 장점을 테스트합니다.
- 읽기 트랜잭션이 쓰기 트랜잭션을 차단하지 않음
- 각 트랜잭션은 독립적인 스냅샷을 봄
"""

import pytest
from src.observations import observe_reader_not_blocking_writer


class TestMission1ReaderNotBlocking:
    """MVCC: 읽기가 쓰기를 차단하지 않음을 검증합니다."""

    def test_writer_is_not_blocked_by_reader(self):
        """쓰기 트랜잭션이 읽기 트랜잭션에 의해 차단되지 않아야 합니다.

        MVCC의 핵심 장점: 읽기와 쓰기가 서로 간섭하지 않음
        """
        result = observe_reader_not_blocking_writer()
        assert result["writer_blocked"] is False, (
            "쓰기가 읽기에 의해 차단되면 안 됩니다"
        )

    def test_reader_sees_original_value(self):
        """읽기 트랜잭션이 원본 값을 봐야 합니다.

        트랜잭션 시작 시점의 스냅샷을 유지합니다.
        """
        result = observe_reader_not_blocking_writer()
        assert result["reader_saw"] == "original", "읽기는 원본 값을 봐야 합니다"

    def test_reader_still_sees_old_value_after_write(self):
        """읽기 트랜잭션이 쓰기 후에도 여전히 원본 값을 봐야 합니다.

        스냅샷 격리: 트랜잭션 중간에 다른 트랜잭션의 변경을 보지 않음
        """
        result = observe_reader_not_blocking_writer()
        assert result["reader_still_sees"] == "original", (
            "읽기는 스냅샷을 유지해야 합니다"
        )

    def test_writer_successfully_updated(self):
        """쓰기 트랜잭션이 성공적으로 값을 변경해야 합니다.

        쓰기가 차단되지 않으므로 즉시 완료됩니다.
        """
        result = observe_reader_not_blocking_writer()
        assert result["writer_new_value"] == "modified", (
            "쓰기는 새 값으로 업데이트해야 합니다"
        )
