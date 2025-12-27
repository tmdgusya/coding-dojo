from time import time
from typing import Dict, Optional

from src.rate_limiter import RateLimiter


class FixedRateLimiter(RateLimiter):

    def __init__(self, window_size: int, max_requests: int, storage: Dict) -> None:
        """
        Initialize the rate limiter with the given window size, maximum requests, and storage.

        Args:
            window_size (int): The size of the time window in seconds.
            max_requests (int): The maximum number of requests allowed within the window.
            storage (Dict): The storage to use for tracking requests.
        """
        super().__init__(window_size, max_requests, storage)

    def handle(self, user_id: str, sequence: int, timestamp: Optional[float] = None) -> str:
        if timestamp is None:
            timestamp = time()
        current_window = int(timestamp // self.window_size)
        key = f"{user_id}:{current_window}"

        token = self.storage.get(key, self.max_requests)
        if token == 0:
            return self._print(
                timestamp,
                user_id,
                sequence,
                False,
                True,
                token == self.max_requests - 1
            )

        token -= 1
        self.storage[key] = token

        is_allowed = token >= 0
        return self._print(
            timestamp,
            user_id,
            sequence,
            is_allowed,
            token == 0, # 한도 도달 여부
            token == self.max_requests - 1
        )

    def _print(
        self,
        current_time: float,
        user_id: str,
        sequence: int,
        is_allowed: bool,
        is_limited: bool,
        is_new_window: bool
    ) -> str:
        text = "허용됨" if is_allowed else "거부됨 (429 Too Many Requests)"
        template = f"\n[시간 {current_time:.2f}s] {user_id} 요청 {sequence}: {text}"

        if is_limited:
            template += " (한도 도달)"

        if is_new_window:
            template += "(새로운 윈도우)"

        return template
