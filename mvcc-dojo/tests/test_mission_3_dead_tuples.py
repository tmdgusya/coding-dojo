"""Mission 3: Dead Tuples & Table Bloat 테스트"""

import pytest
from src.observations import observe_dead_tuples


class TestMission3DeadTuples:
    """Dead Tuple 생성과 테이블 블로트를 검증하는 테스트 클래스

    시나리오: 1000행 UPDATE → dead tuple 관찰
    - 초기 데이터: 1000행 삽입
    - UPDATE 실행: 모든 행을 UPDATE (각 행마다 dead tuple 생성)
    - 검증: dead tuple 수, live tuple 수, dead ratio, 테이블 물리 크기

    Dead Tuple은 UPDATE/DELETE 시 생성되며, VACUUM이 실행될 때까지 유지됩니다.
    이는 테이블 블로트(Table Bloat)의 주요 원인입니다.
    """

    def test_dead_tuples_created_after_update(self):
        """UPDATE 후 dead tuple이 생성되어야 합니다.

        검증: 1000행 UPDATE 후 dead tuple 수는 900개 이상이어야 함
        """
        result = observe_dead_tuples()
        assert result["dead_tuples"] >= 900, (
            "Dead Tuple: UPDATE 후 dead tuple이 충분히 생성되어야 합니다"
        )

    def test_live_tuples_remain(self):
        """UPDATE 후에도 live tuple이 유지되어야 합니다.

        검증: UPDATE 후 live tuple 수는 900개 이상이어야 함
        """
        result = observe_dead_tuples()
        assert result["live_tuples"] >= 900, (
            "Dead Tuple: UPDATE 후에도 live tuple이 유지되어야 합니다"
        )

    def test_dead_ratio_is_significant(self):
        """Dead tuple의 비율이 유의미해야 합니다.

        검증: dead_ratio = dead_tuples / (dead_tuples + live_tuples) >= 0.4
        """
        result = observe_dead_tuples()
        assert result["dead_ratio"] >= 0.4, (
            "Dead Tuple: dead tuple 비율이 40% 이상이어야 합니다"
        )

    def test_table_has_physical_size(self):
        """테이블의 물리적 크기가 측정되어야 합니다.

        검증: table_size_bytes > 0
        """
        result = observe_dead_tuples()
        assert result["table_size_bytes"] > 0, (
            "Dead Tuple: 테이블의 물리적 크기가 0보다 커야 합니다"
        )
