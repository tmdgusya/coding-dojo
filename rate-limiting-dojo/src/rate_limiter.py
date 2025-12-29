from abc import ABC, abstractmethod
from typing import Dict, Optional


class RateLimiter(ABC):
    def __init__(self, window_size: int, max_requests: int, storage: Dict) -> None:
        """
        Initialize the rate limiter with the given window size, maximum requests, and storage.

        Args:
            window_size (int): The size of the time window in seconds.
            max_requests (int): The maximum number of requests allowed within the window.
            storage (Dict): The storage to use for tracking requests.
        """
        self.window_size = window_size
        self.max_requests = max_requests
        self.storage = storage

    @abstractmethod
    def handle(
        self, user_id: str, sequence: int, timestamp: Optional[float] = None
    ) -> str: ...
