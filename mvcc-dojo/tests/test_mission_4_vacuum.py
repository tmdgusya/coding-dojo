"""Mission 4: VACUUM 효과(Dead Tuple 정리) 테스트"""

import pytest
from src.observations import observe_vacuum_effect


class TestMission4Vacuum:
    """VACUUM이 dead tuple을 정리하는 동작을 검증하는 테스트 클래스

    시나리오: 데이터 변경으로 인한 dead tuple 생성 및 정리
    - 초기 데이터 삽입
    - 여러 번의 UPDATE로 dead tuple 생성
    - VACUUM 실행 전후 비교
    - VACUUM FULL로 디스크 공간 회수

    VACUUM vs VACUUM FULL:
    - VACUUM: dead tuple을 표시하여 재사용 가능하게 함 (온라인 작업)
    - VACUUM FULL: 테이블을 재작성하여 디스크 공간을 회수 (배타적 잠금)
    """

    def test_dead_tuples_exist_before_vacuum(self):
        """VACUUM 실행 전에 dead tuple이 존재해야 합니다.

        검증: UPDATE 작업으로 인해 900개 이상의 dead tuple이 생성되어야 함
        """
        result = observe_vacuum_effect()
        assert result["dead_before_vacuum"] >= 900, (
            "VACUUM 전: 900개 이상의 dead tuple이 존재해야 합니다"
        )

    def test_vacuum_clears_dead_tuples(self):
        """VACUUM 실행 후 dead tuple이 정리되어야 합니다.

        검증: VACUUM 후 dead tuple 수가 100개 미만으로 감소해야 함
        """
        result = observe_vacuum_effect()
        assert result["dead_after_vacuum"] < 100, (
            "VACUUM 후: dead tuple이 100개 미만으로 정리되어야 합니다"
        )

    def test_vacuum_full_reclaims_space(self):
        """VACUUM FULL이 디스크 공간을 회수해야 합니다.

        검증: VACUUM FULL 실행 후 공간 회수 여부 확인
        """
        result = observe_vacuum_effect()
        assert result["space_reclaimed"] is True, (
            "VACUUM FULL: 디스크 공간이 회수되어야 합니다"
        )

    def test_vacuum_full_reduces_size(self):
        """VACUUM FULL이 테이블 크기를 감소시켜야 합니다.

        검증: VACUUM FULL 후 테이블 크기가 감소해야 함
        """
        result = observe_vacuum_effect()
        assert result["size_after_vacuum_full"] < result["size_before_vacuum_full"], (
            "VACUUM FULL: 테이블 크기가 감소해야 합니다"
        )
