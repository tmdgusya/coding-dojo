"""
BM-25 테스트 - TDD RED 상태

처음에는 모든 테스트가 실패합니다.
bm25.py의 메서드들을 구현하면 테스트가 통과합니다.

실행:
    uv run pytest test_bm25.py -v              # 전체 테스트
    uv run pytest test_bm25.py -v -k mission_1 # Mission 1만
    uv run pytest test_bm25.py -v -k mission_2 # Mission 2만
"""

import math
import pytest
from bm25 import BM25


# =============================================================================
# Mission 1: 역색인(Inverted Index) 구축
# =============================================================================


class TestMission1_BuildIndex:
    """
    Mission 1: _build_index() 메서드 구현

    역색인을 구축하는 것이 첫 번째 미션입니다.
    이 미션을 완료하면 다음을 달성합니다:
    - inverted_index에 단어별 문서ID와 TF 저장
    - doc_lengths에 각 문서 길이 저장
    - avgdl 계산
    """

    def test_mission_1_doc_lengths(self):
        """각 문서의 길이가 올바르게 저장되는지 확인"""
        documents = [
            "the quick brown fox",  # 4 tokens
            "the lazy dog",  # 3 tokens
            "quick brown",  # 2 tokens
        ]
        bm25 = BM25(documents)

        assert bm25.doc_lengths[0] == 4
        assert bm25.doc_lengths[1] == 3
        assert bm25.doc_lengths[2] == 2

    def test_mission_1_avgdl(self):
        """평균 문서 길이가 올바르게 계산되는지 확인"""
        documents = [
            "the quick brown fox",  # 4 tokens
            "the lazy dog",  # 3 tokens
            "quick brown",  # 2 tokens
        ]
        bm25 = BM25(documents)

        expected_avgdl = (4 + 3 + 2) / 3
        assert abs(bm25.avgdl - expected_avgdl) < 0.001

    def test_mission_1_inverted_index_contains_terms(self):
        """역색인에 단어들이 포함되는지 확인"""
        documents = [
            "python java",
            "python rust",
        ]
        bm25 = BM25(documents)

        assert "python" in bm25.inverted_index
        assert "java" in bm25.inverted_index
        assert "rust" in bm25.inverted_index

    def test_mission_1_inverted_index_term_frequency(self):
        """역색인의 TF 값이 올바른지 확인"""
        documents = [
            "python python java",  # python: 2, java: 1
            "python rust rust",  # python: 1, rust: 2
        ]
        bm25 = BM25(documents)

        assert bm25.inverted_index["python"][0] == 2
        assert bm25.inverted_index["python"][1] == 1
        assert bm25.inverted_index["java"][0] == 1
        assert bm25.inverted_index["rust"][1] == 2

    def test_mission_1_inverted_index_document_list(self):
        """역색인에서 단어를 포함하는 문서 목록이 올바른지 확인"""
        documents = [
            "python is great",
            "java is good",
            "python and java",
        ]
        bm25 = BM25(documents)

        assert set(bm25.inverted_index["python"].keys()) == {0, 2}
        assert set(bm25.inverted_index["java"].keys()) == {1, 2}
        assert set(bm25.inverted_index["is"].keys()) == {0, 1}


# =============================================================================
# Mission 2: IDF 계산
# =============================================================================


class TestMission2_ComputeIDF:
    """
    Mission 2: _compute_idf() 메서드 구현

    IDF(Inverse Document Frequency)를 계산합니다.
    공식: log((N - n(q) + 0.5) / (n(q) + 0.5))
    """

    def test_mission_2_idf_rare_term(self):
        """희귀 단어의 IDF가 높은지 확인"""
        documents = [
            "common word here",
            "common word there",
            "common word everywhere",
            "common word and rare",  # 'rare'는 1개 문서에만
        ]
        bm25 = BM25(documents)

        idf_rare = bm25._compute_idf("rare")
        idf_common = bm25._compute_idf("common")

        assert idf_rare > idf_common

    def test_mission_2_idf_formula(self):
        """IDF 공식이 올바르게 적용되는지 확인"""
        documents = ["a", "b", "c", "d", "e"]  # 5 documents
        bm25 = BM25(documents)

        # 'a'는 1개 문서에 등장
        # IDF = log((5 - 1 + 0.5) / (1 + 0.5)) = log(4.5 / 1.5) = log(3) ≈ 1.0986
        expected_idf = math.log((5 - 1 + 0.5) / (1 + 0.5))
        actual_idf = bm25._compute_idf("a")

        assert abs(actual_idf - expected_idf) < 0.001

    def test_mission_2_idf_unknown_term(self):
        """존재하지 않는 단어의 IDF는 0"""
        documents = ["python java", "rust go"]
        bm25 = BM25(documents)

        assert bm25._compute_idf("nonexistent") == 0.0

    def test_mission_2_idf_caching(self):
        """IDF가 캐싱되는지 확인"""
        documents = ["python java", "python rust"]
        bm25 = BM25(documents)

        bm25._compute_idf("python")
        assert "python" in bm25.idf_cache

    def test_mission_2_idf_all_docs_term(self):
        """모든 문서에 등장하는 단어의 IDF는 0에 가까움"""
        documents = [
            "the quick fox",
            "the lazy dog",
            "the brown bear",
        ]
        bm25 = BM25(documents)

        idf_the = bm25._compute_idf("the")
        # N=3, n(q)=3 → log((3-3+0.5)/(3+0.5)) = log(0.5/3.5) < 0
        # max(0, ...) 적용하면 0
        assert idf_the >= 0


# =============================================================================
# Mission 3: TF 컴포넌트 계산
# =============================================================================


class TestMission3_ComputeTFComponent:
    """
    Mission 3: _compute_tf_component() 메서드 구현

    TF 컴포넌트를 계산합니다 (문서 길이 정규화 포함).
    공식: (tf * (k1 + 1)) / (tf + k1 * (1 - b + b * |D| / avgdl))
    """

    def test_mission_3_tf_component_basic(self):
        """기본 TF 컴포넌트 계산"""
        documents = [
            "python python python",  # tf=3
            "java",
        ]
        bm25 = BM25(documents, k1=1.5, b=0.75)

        tf_comp = bm25._compute_tf_component("python", 0)
        assert tf_comp > 0

    def test_mission_3_tf_component_zero_for_missing_term(self):
        """단어가 없는 문서의 TF 컴포넌트는 0"""
        documents = ["python java", "rust go"]
        bm25 = BM25(documents)

        assert bm25._compute_tf_component("python", 1) == 0.0

    def test_mission_3_tf_saturation(self):
        """TF가 증가할수록 TF 컴포넌트 증가 속도가 감소 (Saturation)"""
        documents = [
            "python",
            "python python",
            "python python python",
            "python python python python",
        ]
        bm25 = BM25(documents, k1=1.5, b=0)

        tf_comps = [bm25._compute_tf_component("python", i) for i in range(4)]

        # TF 컴포넌트는 증가하지만 증가폭은 감소
        increases = [tf_comps[i + 1] - tf_comps[i] for i in range(3)]
        assert increases[0] > increases[1] > increases[2]

    def test_mission_3_length_normalization(self):
        """긴 문서는 TF 컴포넌트가 낮아짐 (b > 0일 때)"""
        documents = [
            "python",  # 짧은 문서
            "python and some other words",  # 긴 문서 (동일 TF)
        ]
        bm25 = BM25(documents, k1=1.5, b=0.75)

        tf_short = bm25._compute_tf_component("python", 0)
        tf_long = bm25._compute_tf_component("python", 1)

        assert tf_short > tf_long

    def test_mission_3_no_length_norm_when_b_zero(self):
        """b=0이면 문서 길이 정규화 없음"""
        documents = [
            "python",
            "python and some other words extra",
        ]
        bm25 = BM25(documents, k1=1.5, b=0)

        tf_short = bm25._compute_tf_component("python", 0)
        tf_long = bm25._compute_tf_component("python", 1)

        assert abs(tf_short - tf_long) < 0.001


# =============================================================================
# Mission 4: 점수 계산
# =============================================================================


class TestMission4_Score:
    """
    Mission 4: score() 메서드 구현

    문서에 대한 쿼리의 BM-25 점수를 계산합니다.
    공식: Σ IDF(q_i) * TF_component(q_i, D)
    """

    def test_mission_4_score_basic(self):
        """기본 점수 계산"""
        documents = [
            "python machine learning",
            "java enterprise",
        ]
        bm25 = BM25(documents)

        score = bm25.score("python", 0)
        assert score > 0

    def test_mission_4_score_zero_for_unrelated(self):
        """관련 없는 쿼리는 0점"""
        documents = [
            "python machine learning",
            "java enterprise",
        ]
        bm25 = BM25(documents)

        score = bm25.score("rust", 0)
        assert score == 0.0

    def test_mission_4_score_multi_term(self):
        """다중 단어 쿼리의 점수는 각 단어 점수의 합"""
        documents = [
            "python machine learning data science",
        ]
        bm25 = BM25(documents)

        score_python = bm25.score("python", 0)
        score_machine = bm25.score("machine", 0)
        score_both = bm25.score("python machine", 0)

        assert abs(score_both - (score_python + score_machine)) < 0.001

    def test_mission_4_relevant_doc_higher_score(self):
        """관련성 높은 문서가 높은 점수"""
        documents = [
            "python is great for data science and machine learning",
            "java is used for enterprise applications",
            "python python python",
        ]
        bm25 = BM25(documents)

        score_0 = bm25.score("python data science", 0)
        score_1 = bm25.score("python data science", 1)

        assert score_0 > score_1


# =============================================================================
# Mission 5: 검색
# =============================================================================


class TestMission5_Search:
    """
    Mission 5: search() 메서드 구현

    쿼리에 대해 가장 관련성 높은 문서를 검색합니다.
    """

    def test_mission_5_search_returns_list(self):
        """검색 결과는 리스트 형태"""
        documents = ["python java", "rust go"]
        bm25 = BM25(documents)

        results = bm25.search("python", top_k=2)
        assert isinstance(results, list)

    def test_mission_5_search_returns_tuples(self):
        """검색 결과는 (doc_id, score) 튜플"""
        documents = ["python java", "rust go"]
        bm25 = BM25(documents)

        results = bm25.search("python", top_k=2)
        for result in results:
            assert isinstance(result, tuple)
            assert len(result) == 2

    def test_mission_5_search_sorted_by_score(self):
        """검색 결과는 점수 내림차순 정렬"""
        documents = [
            "python",
            "python python",
            "python python python",
        ]
        bm25 = BM25(documents, b=0)

        results = bm25.search("python", top_k=3)
        scores = [score for _, score in results]

        assert scores == sorted(scores, reverse=True)

    def test_mission_5_search_top_k(self):
        """top_k 개수만큼 결과 반환"""
        documents = ["a", "b", "c", "d", "e"]
        bm25 = BM25(documents)

        results = bm25.search("a", top_k=2)
        assert len(results) <= 2

    def test_mission_5_search_correct_ranking(self):
        """관련성 높은 문서가 상위 랭크"""
        documents = [
            "the quick brown fox",
            "the lazy dog sleeps",
            "the quick brown dog jumps",
            "a fox and a dog are friends",
            "quick brown foxes are cute",
        ]
        bm25 = BM25(documents)

        results = bm25.search("quick brown fox", top_k=3)
        top_doc_ids = [doc_id for doc_id, _ in results]

        # "quick brown fox"를 모두 포함하는 문서 0이 최상위여야 함
        assert 0 in top_doc_ids[:2]


# =============================================================================
# 통합 테스트
# =============================================================================


class TestIntegration:
    """통합 테스트: 모든 미션이 완료되었을 때"""

    def test_full_workflow(self):
        """전체 워크플로우 테스트"""
        documents = [
            "Python is a high-level programming language",
            "Python is widely used for data science and machine learning",
            "JavaScript is used for web development alongside HTML and CSS",
            "Python and JavaScript are both popular programming languages",
            "Machine learning algorithms require Python programming skills",
        ]

        bm25 = BM25(documents, k1=1.5, b=0.75)

        # 검색 테스트
        results = bm25.search("python programming", top_k=3)

        assert len(results) == 3
        assert all(score >= 0 for _, score in results)

        # Python 관련 문서가 상위에
        top_ids = [doc_id for doc_id, _ in results]
        python_docs = [0, 1, 3, 4]
        assert any(doc_id in python_docs for doc_id in top_ids)

    def test_explain_score_works(self):
        """explain_score 메서드가 작동하는지 확인"""
        documents = ["python java", "rust go"]
        bm25 = BM25(documents)

        explanation = bm25.explain_score("python", 0)

        assert "doc_id" in explanation
        assert "total_score" in explanation
        assert "terms" in explanation
