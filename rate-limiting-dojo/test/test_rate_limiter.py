from time import sleep, time
import pytest
from src.fixed_rate_limiter import FixedRateLimiter


def test_fixed_rate_limiter():
    """1초 윈도우에 10개 요청 제한 테스트"""
    storage = {}
    fixed_rate_limiter = FixedRateLimiter(window_size=1, max_requests=10, storage=storage)
    user_id = "User_A"

    # 요청 1-10: 0.1초 간격으로 요청
    for i in range(1, 5):
        timestamp = 0.1 * i
        result = fixed_rate_limiter.handle(user_id=user_id, sequence=i, timestamp=timestamp)
        print(result)
        assert "허용됨" in result
        assert "한도 도달" not in result or i == 10  # 10번째 요청은 "한도 도달" 포함

    result = fixed_rate_limiter.handle(user_id=user_id, sequence=5, timestamp=0.45)
    print(result)
    assert "허용됨" in result
    assert "한도 도달" not in result

    for i in range(5, 10):
        timestamp = 0.1 * i
        result = fixed_rate_limiter.handle(user_id=user_id, sequence=i+1, timestamp=timestamp)
        print(result)
        assert "허용됨" in result
        assert "한도 도달" not in result or i + 1 == 10  # 10번째 요청은 "한도 도달" 포함

    # 요청 11: 0.95초 시점 (아직 1초 윈도우 내, 한도 초과)
    result = fixed_rate_limiter.handle(user_id=user_id, sequence=11, timestamp=0.95)
    print(result)
    assert "거부됨 (429 Too Many Requests)" in result

    # 요청 12: 1.1초 시점 (새로운 윈도우)
    result = fixed_rate_limiter.handle(user_id=user_id, sequence=12, timestamp=1.1)
    print(result)
    assert "허용됨" in result
    assert "새로운 윈도우" in result
