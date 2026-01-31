"""
BM-25 (Best Match 25) 알고리즘 핵심 구현
=======================================

BM-25는 정보 검색(Information Retrieval)에서 사용되는 랭킹 함수입니다.
TF-IDF의 한계를 극복하기 위해 문서 길이 정규화와 TF saturation을 도입했습니다.

핵심 공식:
---------
BM25(D, Q) = Σ IDF(q_i) * TF_component(q_i, D)

IDF(q_i) = log((N - n(q_i) + 0.5) / (n(q_i) + 0.5))

TF_component = (f(q_i, D) * (k1 + 1)) / (f(q_i, D) + k1 * (1 - b + b * |D| / avgdl))

where:
  - N: 전체 문서 수
  - n(q_i): q_i를 포함하는 문서 수
  - f(q_i, D): 문서 D에서 q_i의 출현 빈도 (TF)
  - |D|: 문서 D의 길이 (단어 수)
  - avgdl: 전체 문서의 평균 길이
  - k1: TF saturation 파라미터 (기본값: 1.5)
  - b: 문서 길이 정규화 파라미터 (기본값: 0.75)

TODO: 아래 클래스의 메서드들을 직접 구현하세요!
"""

import math
import re
from collections import defaultdict
from typing import List, Dict, Tuple, Optional


class BM25:
    """
    BM-25 알고리즘 구현체

    핵심 자료구조:
    -------------
    1. inverted_index: {term: {doc_id: tf, ...}, ...}
       - 역색인: 각 단어가 어떤 문서에 몇 번 등장하는지 저장
       - 빠른 검색을 위해 필수적인 자료구조

    2. doc_lengths: {doc_id: length, ...}
       - 각 문서의 길이 저장

    3. doc_count: 전체 문서 수 (N)

    4. avgdl: 평균 문서 길이

    5. idf_cache: {term: idf_score, ...}
       - IDF는 고정되므로 한 번 계산 후 캐싱
    """

    def __init__(self, documents: List[str], k1: float = 1.5, b: float = 0.75):
        """
        BM-25 초기화

        Args:
            documents: 문서 문자열 리스트
            k1: TF saturation 파라미터 (클수록 TF가 score에 미치는 영향이 선형에 가까워짐)
            b: 문서 길이 정규화 파라미터 (0이면 정규화 없음, 1이면 완전 정규화)
        """
        self.k1 = k1
        self.b = b
        self.documents = documents
        self.doc_count = len(documents)

        # 핵심 자료구조 초기화
        self.inverted_index: Dict[str, Dict[int, int]] = defaultdict(dict)
        self.doc_lengths: Dict[int, int] = {}
        self.total_length = 0
        self.avgdl: float = 0.0
        self.idf_cache: Dict[str, float] = {}

        # 인덱스 구축
        self._build_index()

    def _tokenize(self, text: str) -> List[str]:
        """
        텍스트를 토큰화 (소문자 변환 + 알파벳/숫자만 추출)

        실제 시스템에서는 더 정교한 토큰화가 필요하지만,
        학습을 위해 간단한 방식을 사용합니다.
        """
        text = text.lower()
        # 알파벳, 숫자, 한글만 추출
        tokens = re.findall(r"[a-z0-9가-힣]+", text)
        return tokens

    def _build_index(self):
        """
        역색인(Inverted Index) 및 문서 길이 정보 구축

        구현해야 할 내용:
        1. 각 문서를 토큰화
        2. 각 문서의 길이(토큰 수)를 self.doc_lengths에 저장
        3. 전체 토큰 수를 self.total_length에 누적
        4. 각 단어의 문서별 출현 빈도를 self.inverted_index에 저장
        5. 평균 문서 길이(self.avgdl) 계산

        힌트:
        - self._tokenize(doc)으로 문서를 토큰화
        - self.inverted_index[term][doc_id] = freq 형태로 저장
        """
        raise NotImplementedError("_build_index 메서드를 구현하세요!")

    def _compute_idf(self, term: str) -> float:
        """
        IDF (Inverse Document Frequency) 계산

        공식: log((N - n(q) + 0.5) / (n(q) + 0.5))

        where:
          - N: 전체 문서 수 (self.doc_count)
          - n(q): 해당 단어를 포함하는 문서 수

        구현해야 할 내용:
        1. 캐시(self.idf_cache)에 있으면 재사용
        2. 단어를 포함하는 문서 수(n_q) 계산
        3. BM-25 IDF 공식 적용 (음수 방지: max(0, ...))
        4. 결과를 캐시에 저장

        힌트:
        - 단어가 없으면 IDF = 0
        - len(self.inverted_index.get(term, {}))로 문서 수 확인
        """
        raise NotImplementedError("_compute_idf 메서드를 구현하세요!")

    def _compute_tf_component(self, term: str, doc_id: int) -> float:
        """
        TF 컴포넌트 계산 (문서 길이 정규화가 적용된 TF)

        공식: (f(q, D) * (k1 + 1)) / (f(q, D) + k1 * (1 - b + b * |D| / avgdl))

        구현해야 할 내용:
        1. 해당 문서에서의 단어 빈도(f_q_D) 확인
        2. 단어가 없으면 0 반환
        3. 문서 길이 정규화 계수 계산
        4. TF 컴포넌트 공식 적용

        핵심 아이디어:
        - TF Saturation: 단어 빈도가 높아져도 점수가 선형적으로 증가하지 않음
        - 문서 길이 정규화: 긴 문서의 TF를 패널티 처리

        힌트:
        - self.inverted_index.get(term, {}).get(doc_id, 0)으로 TF 확인
        - self.doc_lengths[doc_id]로 문서 길이 확인
        """
        raise NotImplementedError("_compute_tf_component 메서드를 구현하세요!")

    def score(self, query: str, doc_id: int) -> float:
        """
        특정 문서에 대한 쿼리의 BM-25 점수 계산

        공식: score(D, Q) = Σ IDF(q_i) * TF_component(q_i, D)

        구현해야 할 내용:
        1. 쿼리를 토큰화
        2. 각 토큰에 대해 IDF * TF_component 계산
        3. 합계 반환
        """
        raise NotImplementedError("score 메서드를 구현하세요!")

    def search(self, query: str, top_k: int = 5) -> List[Tuple[int, float]]:
        """
        쿼리에 대해 가장 관련성 높은 문서 검색

        구현해야 할 내용:
        1. 모든 문서에 대해 score 계산
        2. 점수 내림차순 정렬
        3. 상위 top_k개 반환

        Returns:
            List of (doc_id, score) tuples, sorted by score descending
        """
        raise NotImplementedError("search 메서드를 구현하세요!")

    def get_term_info(self, term: str) -> Optional[Dict]:
        """
        특정 단어에 대한 정보 조회 (디버깅/학습용)

        이 메서드는 구현되어 있습니다. _build_index를 구현하면 작동합니다.
        """
        term = term.lower()
        if term not in self.inverted_index:
            return None

        doc_freqs = self.inverted_index[term]

        # _compute_idf가 구현되지 않았으면 0 반환
        try:
            idf = self._compute_idf(term)
        except NotImplementedError:
            idf = 0.0

        return {
            "term": term,
            "document_frequency": len(doc_freqs),
            "idf": idf,
            "postings": dict(doc_freqs),  # 어떤 문서에 몇 번 등장하는지
        }

    def explain_score(self, query: str, doc_id: int) -> Dict:
        """
        점수 계산 과정 상세 설명 (디버깅/학습용)

        이 메서드는 구현되어 있습니다. 관련 메서드들을 구현하면 작동합니다.
        """
        query_tokens = self._tokenize(query)
        doc = self.documents[doc_id]
        doc_length = self.doc_lengths.get(doc_id, 0)

        terms_breakdown = []
        total_score = 0.0

        for term in set(query_tokens):  # 중복 제거
            try:
                idf = self._compute_idf(term)
                tf_component = self._compute_tf_component(term, doc_id)
                term_score = idf * tf_component
            except NotImplementedError:
                idf = 0.0
                tf_component = 0.0
                term_score = 0.0

            f_q_D = self.inverted_index.get(term, {}).get(doc_id, 0)

            terms_breakdown.append(
                {
                    "term": term,
                    "tf_in_doc": f_q_D,
                    "idf": idf,
                    "tf_component": tf_component,
                    "term_score": term_score,
                }
            )

            total_score += term_score

        return {
            "doc_id": doc_id,
            "document": doc[:100] + "..." if len(doc) > 100 else doc,
            "doc_length": doc_length,
            "query": query,
            "avgdl": self.avgdl,
            "k1": self.k1,
            "b": self.b,
            "terms": sorted(
                terms_breakdown, key=lambda x: x["term_score"], reverse=True
            ),
            "total_score": total_score,
        }
