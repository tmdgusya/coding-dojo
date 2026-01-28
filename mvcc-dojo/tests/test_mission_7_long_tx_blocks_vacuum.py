"""Mission 7: 오래된 트랜잭션이 VACUUM 방해 (Long TX Blocks VACUUM)

MVCC의 중요한 제약을 테스트합니다.
- 오래 실행되는 트랜잭션이 VACUUM을 차단함
- VACUUM은 모든 트랜잭션이 볼 수 있는 dead tuple만 제거 가능
- 오래된 트랜잭션이 종료될 때까지 dead tuple이 누적됨
"""

import pytest
from src.observations import observe_long_tx_blocks_vacuum


class TestMission7LongTxBlocksVacuum:
    """오래된 트랜잭션이 VACUUM을 방해함을 검증합니다."""

    def test_vacuum_cannot_clean_with_old_tx(self):
        """오래 실행되는 트랜잭션이 있을 때 VACUUM이 dead tuple을 정리하지 못해야 합니다.

        MVCC의 제약: VACUUM은 모든 활성 트랜잭션이 볼 수 있는 dead tuple만 제거 가능
        """
        result = observe_long_tx_blocks_vacuum()
        assert result["dead_after_first_vacuum"] >= 400, (
            "오래된 트랜잭션이 있을 때 VACUUM은 dead tuple을 정리하지 못해야 합니다"
        )

    def test_old_tx_is_open_during_first_vacuum(self):
        """첫 번째 VACUUM 실행 시 오래된 트랜잭션이 열려있어야 합니다.

        검증: 오래된 트랜잭션이 활성 상태여야 VACUUM이 차단됨
        """
        result = observe_long_tx_blocks_vacuum()
        assert result["old_tx_still_open"] is True, (
            "첫 번째 VACUUM 실행 시 오래된 트랜잭션이 열려있어야 합니다"
        )

    def test_vacuum_works_after_old_tx_closes(self):
        """오래된 트랜잭션이 종료된 후 VACUUM이 dead tuple을 정리해야 합니다.

        검증: 트랜잭션 종료 후 VACUUM은 정상 작동
        """
        result = observe_long_tx_blocks_vacuum()
        assert result["dead_after_close_and_vacuum"] < 100, (
            "오래된 트랜잭션이 종료된 후 VACUUM은 dead tuple을 정리해야 합니다"
        )

    def test_vacuum_could_finally_clean(self):
        """오래된 트랜잭션 종료 후 VACUUM이 성공적으로 정리했는지 확인합니다.

        검증: VACUUM이 dead tuple을 효과적으로 제거
        """
        result = observe_long_tx_blocks_vacuum()
        assert result["vacuum_could_clean"] is True, (
            "오래된 트랜잭션 종료 후 VACUUM은 성공적으로 정리해야 합니다"
        )
