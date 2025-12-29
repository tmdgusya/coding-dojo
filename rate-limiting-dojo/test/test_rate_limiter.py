from time import sleep, time
import threading
from unittest.mock import MagicMock
import pytest
from src.fixed_rate_limiter import FixedRateLimiter


def test_fixed_rate_limiter():
    """1초 윈도우에 10개 요청 제한 테스트"""
    storage = {}
    fixed_rate_limiter = FixedRateLimiter(
        window_size=1, max_requests=10, storage=storage
    )
    user_id = "User_A"

    # 요청 1-10: 0.1초 간격으로 요청
    for i in range(1, 5):
        timestamp = 0.1 * i
        result = fixed_rate_limiter.handle(
            user_id=user_id, sequence=i, timestamp=timestamp
        )
        print(result)
        assert "허용됨" in result
        assert "한도 도달" not in result or i == 10  # 10번째 요청은 "한도 도달" 포함

    result = fixed_rate_limiter.handle(user_id=user_id, sequence=5, timestamp=0.45)
    print(result)
    assert "허용됨" in result
    assert "한도 도달" not in result

    for i in range(5, 10):
        timestamp = 0.1 * i
        result = fixed_rate_limiter.handle(
            user_id=user_id, sequence=i + 1, timestamp=timestamp
        )
        print(result)
        assert "허용됨" in result
        assert (
            "한도 도달" not in result or i + 1 == 10
        )  # 10번째 요청은 "한도 도달" 포함

    # 요청 11: 0.95초 시점 (아직 1초 윈도우 내, 한도 초과)
    result = fixed_rate_limiter.handle(user_id=user_id, sequence=11, timestamp=0.95)
    print(result)
    assert "거부됨 (429 Too Many Requests)" in result

    # 요청 12: 1.1초 시점 (새로운 윈도우)
    result = fixed_rate_limiter.handle(user_id=user_id, sequence=12, timestamp=1.1)
    print(result)
    assert "허용됨" in result
    assert "새로운 윈도우" in result


def test_multiple_users_are_independent():
    """여러 유저가 서로 독립적으로 제한되는지 테스트"""
    storage = {}
    limiter = FixedRateLimiter(window_size=1, max_requests=3, storage=storage)

    # User_A: 3개 요청 모두 허용
    for i in range(1, 4):
        result = limiter.handle(user_id="User_A", sequence=i, timestamp=0.1 * i)
        print(result)
        assert "허용됨" in result

    # User_A: 4번째 요청 거부
    result = limiter.handle(user_id="User_A", sequence=4, timestamp=0.4)
    print(result)
    assert "거부됨" in result

    # User_B: User_A와 독립적으로 3개 요청 허용되어야 함
    for i in range(1, 4):
        result = limiter.handle(user_id="User_B", sequence=i, timestamp=0.1 * i)
        print(result)
        assert "허용됨" in result, (
            f"User_B의 요청 {i}이 거부되었습니다. User_A와 독립적이어야 합니다."
        )


def test_concurrent_requests_should_respect_limit():
    """
    동시성 테스트: 여러 스레드가 동시에 요청해도 max_requests를 초과하면 안 된다.

    이 테스트는 Race Condition을 검증합니다.
    - 20개의 스레드가 동시에 요청을 보냄
    - max_requests=10이므로, 정확히 10개만 허용되어야 함
    - Race Condition이 있다면 10개 이상이 허용될 수 있음
    """
    storage = {}
    limiter = FixedRateLimiter(window_size=1, max_requests=10, storage=storage)
    user_id = "User_Concurrent"

    results = []
    results_lock = threading.Lock()

    def make_request(sequence: int):
        """스레드에서 실행될 요청 함수"""
        result = limiter.handle(user_id=user_id, sequence=sequence, timestamp=0.5)
        with results_lock:
            results.append(result)

    # 20개의 스레드 생성
    threads = []
    for i in range(1, 21):
        t = threading.Thread(target=make_request, args=(i,))
        threads.append(t)

    # 모든 스레드를 거의 동시에 시작
    for t in threads:
        t.start()

    # 모든 스레드가 완료될 때까지 대기
    for t in threads:
        t.join()

    # 결과 분석
    allowed_count = sum(1 for r in results if "허용됨" in r)
    denied_count = sum(1 for r in results if "거부됨" in r)

    print(f"\n=== 동시성 테스트 결과 ===")
    print(f"허용된 요청: {allowed_count}개")
    print(f"거부된 요청: {denied_count}개")
    print(f"총 요청: {len(results)}개")

    # 핵심 검증: 허용된 요청이 max_requests(10)를 초과하면 안 됨
    assert allowed_count <= 10, (
        f"Race Condition 발생! "
        f"max_requests=10인데 {allowed_count}개가 허용되었습니다. "
        f"동시성 제어가 필요합니다."
    )
    assert allowed_count + denied_count == 20, "모든 요청이 처리되어야 합니다."


def test_concurrent_requests_stress_test():
    """
    더 공격적인 동시성 스트레스 테스트.

    여러 번 반복하여 Race Condition이 발생할 확률을 높입니다.
    실제 분산 환경을 시뮬레이션하기 위해 barrier를 사용하여
    모든 스레드가 정확히 같은 시점에 요청을 시작하도록 합니다.
    """
    NUM_ITERATIONS = 10  # 테스트 반복 횟수
    NUM_THREADS = 50  # 동시 요청 수
    MAX_REQUESTS = 10  # 허용 최대 요청

    race_condition_detected = False

    for iteration in range(NUM_ITERATIONS):
        storage = {}
        limiter = FixedRateLimiter(
            window_size=1, max_requests=MAX_REQUESTS, storage=storage
        )
        user_id = f"User_Stress_{iteration}"

        results = []
        results_lock = threading.Lock()

        # Barrier: 모든 스레드가 준비될 때까지 대기 후 동시 시작
        barrier = threading.Barrier(NUM_THREADS)

        def make_request(sequence: int):
            barrier.wait()  # 모든 스레드가 여기서 대기 후 동시 출발
            result = limiter.handle(user_id=user_id, sequence=sequence, timestamp=0.5)
            with results_lock:
                results.append(result)

        threads = [
            threading.Thread(target=make_request, args=(i,))
            for i in range(1, NUM_THREADS + 1)
        ]

        for t in threads:
            t.start()
        for t in threads:
            t.join()

        allowed_count = sum(1 for r in results if "허용됨" in r)

        if allowed_count > MAX_REQUESTS:
            race_condition_detected = True
            print(
                f"\n[반복 {iteration + 1}] Race Condition 발생! 허용: {allowed_count}/{MAX_REQUESTS}"
            )
            break
        else:
            print(f"[반복 {iteration + 1}] OK - 허용: {allowed_count}/{MAX_REQUESTS}")

    assert not race_condition_detected, (
        f"Race Condition이 감지되었습니다! "
        f"max_requests={MAX_REQUESTS}인데 {allowed_count}개가 허용되었습니다. "
        f"힌트: storage 접근 시 Lock을 사용하거나, 원자적(atomic) 연산이 필요합니다."
    )


def test_race_condition_simulation():
    """
    Race Condition을 명시적으로 시뮬레이션하는 테스트.

    이 테스트는 실제 분산 환경에서 발생할 수 있는 상황을 재현합니다:
    1. Thread A가 storage에서 token을 읽음 (token = 1)
    2. Thread B가 storage에서 token을 읽음 (token = 1, 아직 A가 쓰기 전)
    3. Thread A가 token을 감소시키고 저장 (token = 0)
    4. Thread B도 token을 감소시키고 저장 (token = 0, 덮어쓰기!)

    결과: 두 요청 모두 허용됨 (하지만 하나만 허용되어야 함)

    이 테스트를 통과하려면 FixedRateLimiter가 thread-safe한 storage를 사용하거나,
    내부적으로 Lock을 사용해야 합니다.
    """

    # Race condition을 시뮬레이션하는 "느린" storage
    class SlowStorage(dict):
        """
        get()과 __setitem__() 사이에 다른 스레드가 끼어들 수 있는 storage.
        실제 Redis나 분산 환경에서 발생할 수 있는 상황을 시뮬레이션합니다.
        """

        def __init__(self):
            super().__init__()
            self._read_count = 0
            self._read_barrier = threading.Barrier(2)  # 2개의 스레드가 동시에 읽도록 함

        def get(self, key, default=None):
            self._read_count += 1
            value = super().get(key, default)

            # 처음 두 번의 read에서 barrier로 동기화하여
            # 두 스레드가 같은 값을 읽도록 함
            if self._read_count <= 2:
                try:
                    self._read_barrier.wait(timeout=1)
                except threading.BrokenBarrierError:
                    pass

            return value

    storage = SlowStorage()
    limiter = FixedRateLimiter(window_size=1, max_requests=1, storage=storage)
    user_id = "User_Race"

    results = []
    results_lock = threading.Lock()

    def make_request(sequence: int):
        result = limiter.handle(user_id=user_id, sequence=sequence, timestamp=0.5)
        with results_lock:
            results.append(result)

    # 정확히 2개의 스레드가 동시에 요청
    threads = [
        threading.Thread(target=make_request, args=(1,)),
        threading.Thread(target=make_request, args=(2,)),
    ]

    for t in threads:
        t.start()
    for t in threads:
        t.join()

    allowed_count = sum(1 for r in results if "허용됨" in r)

    print(f"\n=== Race Condition 시뮬레이션 결과 ===")
    print(f"허용된 요청: {allowed_count}개 (예상: 1개)")
    for i, r in enumerate(results, 1):
        print(f"  요청 {i}: {r.strip()}")

    # max_requests=1이므로 정확히 1개만 허용되어야 함
    assert allowed_count == 1, (
        f"Race Condition 발생! "
        f"max_requests=1인데 {allowed_count}개가 허용되었습니다.\n"
        f"힌트: handle() 메서드 내에서 storage 읽기-수정-쓰기 작업을 "
        f"Lock으로 보호하거나, 원자적 연산을 사용해야 합니다."
    )
