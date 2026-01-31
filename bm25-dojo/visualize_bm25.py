#!/usr/bin/env python3
"""
BM-25 Algorithm Visualization Tool

Works independently of bm25.py implementation.
Understand algorithm principles visually before implementing!

Usage:
    uv run python visualize_bm25.py          # All visualizations
    uv run python visualize_bm25.py idf      # IDF only
    uv run python visualize_bm25.py tf       # TF Saturation only
    uv run python visualize_bm25.py length   # Document length normalization only
    uv run python visualize_bm25.py index    # Inverted index structure only
"""

import math
import sys
from typing import List, Dict
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
import numpy as np


def compute_idf(N: int, n_q: int) -> float:
    """BM-25 IDF formula"""
    if n_q == 0:
        return 0.0
    numerator = N - n_q + 0.5
    denominator = n_q + 0.5
    return max(0.0, math.log(numerator / denominator))


def compute_tf_component(
    tf: float, k1: float, b: float, doc_len: int, avgdl: float
) -> float:
    """BM-25 TF component formula"""
    if tf == 0:
        return 0.0
    length_norm = 1 - b + b * (doc_len / avgdl)
    numerator = tf * (k1 + 1)
    denominator = tf + k1 * length_norm
    return numerator / denominator


def visualize_idf():
    """
    IDF (Inverse Document Frequency) visualization

    Key concept:
    - Rare terms have higher IDF
    - Terms appearing in all documents have IDF ~ 0
    """
    print("\n" + "=" * 60)
    print("IDF (Inverse Document Frequency) Visualization")
    print("=" * 60)
    print()
    print("Formula: IDF(q) = log((N - n(q) + 0.5) / (n(q) + 0.5))")
    print("  - N: Total number of documents")
    print("  - n(q): Number of documents containing term q")
    print()

    fig, axes = plt.subplots(1, 2, figsize=(14, 5))

    N_values = [10, 100, 1000]
    colors = ["#e74c3c", "#3498db", "#2ecc71"]

    for N, color in zip(N_values, colors):
        n_q_values = np.arange(1, N + 1)
        idf_values = [compute_idf(N, int(n)) for n in n_q_values]

        axes[0].plot(n_q_values, idf_values, label=f"N={N}", color=color, linewidth=2)

    axes[0].set_xlabel("n(q): Documents containing term", fontsize=11)
    axes[0].set_ylabel("IDF Value", fontsize=11)
    axes[0].set_title("IDF Curve by Document Count (N)", fontsize=13)
    axes[0].legend()
    axes[0].grid(True, alpha=0.3)
    axes[0].set_xscale("log")

    N = 100
    examples = [
        ("Rare term\n(1 doc)", 1, "#e74c3c"),
        ("Uncommon\n(10 docs)", 10, "#f39c12"),
        ("Common\n(50 docs)", 50, "#3498db"),
        ("Very common\n(90 docs)", 90, "#2ecc71"),
        ("All docs\n(100 docs)", 100, "#95a5a6"),
    ]

    terms = [e[0] for e in examples]
    idf_vals = [compute_idf(N, e[1]) for e in examples]
    bar_colors = [e[2] for e in examples]

    bars = axes[1].bar(terms, idf_vals, color=bar_colors, edgecolor="black")
    axes[1].set_ylabel("IDF Value", fontsize=11)
    axes[1].set_title(f"IDF by Term Rarity (N=100)", fontsize=13)

    for bar, val in zip(bars, idf_vals):
        axes[1].text(
            bar.get_x() + bar.get_width() / 2,
            bar.get_height() + 0.1,
            f"{val:.2f}",
            ha="center",
            va="bottom",
            fontsize=10,
        )

    axes[1].set_ylim(0, max(idf_vals) * 1.3)

    plt.tight_layout()
    plt.savefig("viz_idf.png", dpi=150, bbox_inches="tight")
    print("Saved: viz_idf.png")
    plt.show()


def visualize_tf_saturation():
    """
    TF Saturation visualization

    Key concept:
    - Score doesn't increase infinitely with term frequency
    - k1 parameter controls saturation speed
    """
    print("\n" + "=" * 60)
    print("TF Saturation Visualization")
    print("=" * 60)
    print()
    print("Formula: TF_component = (tf * (k1 + 1)) / (tf + k1 * length_norm)")
    print("  - Smaller k1 = faster saturation")
    print("  - Larger k1 = more linear")
    print()

    fig, axes = plt.subplots(1, 2, figsize=(14, 5))

    tf_values = np.arange(0, 21, 0.5)
    k1_values = [0.5, 1.2, 1.5, 2.0, 5.0]
    colors = ["#e74c3c", "#f39c12", "#3498db", "#9b59b6", "#2ecc71"]

    for k1, color in zip(k1_values, colors):
        tf_comp_values = [compute_tf_component(tf, k1, 0, 10, 10) for tf in tf_values]
        axes[0].plot(
            tf_values, tf_comp_values, label=f"k1={k1}", color=color, linewidth=2
        )

    axes[0].plot(
        tf_values, tf_values, "--", color="gray", alpha=0.5, label="Linear (ref)"
    )

    axes[0].set_xlabel("TF (Term Frequency)", fontsize=11)
    axes[0].set_ylabel("TF Component", fontsize=11)
    axes[0].set_title("TF Saturation by k1 Parameter", fontsize=13)
    axes[0].legend(loc="lower right")
    axes[0].grid(True, alpha=0.3)
    axes[0].set_xlim(0, 20)
    axes[0].set_ylim(0, 5)

    k1 = 1.5
    tf_range = [1, 2, 3, 5, 10, 20]
    tf_comps = [compute_tf_component(tf, k1, 0, 10, 10) for tf in tf_range]

    increases = [0] + [tf_comps[i] - tf_comps[i - 1] for i in range(1, len(tf_comps))]

    x = np.arange(len(tf_range))
    width = 0.35

    bars1 = axes[1].bar(
        x - width / 2,
        tf_comps,
        width,
        label="TF Component",
        color="#3498db",
        edgecolor="black",
    )
    bars2 = axes[1].bar(
        x + width / 2,
        increases,
        width,
        label="Increase",
        color="#e74c3c",
        edgecolor="black",
    )

    axes[1].set_xlabel("TF Value", fontsize=11)
    axes[1].set_ylabel("Value", fontsize=11)
    axes[1].set_title(f"Score Change with TF Increase (k1={k1})", fontsize=13)
    axes[1].set_xticks(x)
    axes[1].set_xticklabels([str(tf) for tf in tf_range])
    axes[1].legend()
    axes[1].grid(True, alpha=0.3, axis="y")

    for bar, val in zip(bars1, tf_comps):
        axes[1].text(
            bar.get_x() + bar.get_width() / 2,
            bar.get_height() + 0.02,
            f"{val:.2f}",
            ha="center",
            va="bottom",
            fontsize=9,
        )

    plt.tight_layout()
    plt.savefig("viz_tf_saturation.png", dpi=150, bbox_inches="tight")
    print("Saved: viz_tf_saturation.png")
    plt.show()


def visualize_length_normalization():
    """
    Document length normalization visualization

    Key concept:
    - Longer documents naturally have higher TF -> need penalty
    - b parameter controls normalization strength
    """
    print("\n" + "=" * 60)
    print("Document Length Normalization Visualization")
    print("=" * 60)
    print()
    print("Normalization factor: 1 - b + b * (|D| / avgdl)")
    print("  - b=0: No normalization (long docs favored)")
    print("  - b=1: Full normalization")
    print("  - Docs longer than avg -> factor > 1 -> penalty")
    print()

    fig, axes = plt.subplots(1, 2, figsize=(14, 5))

    avgdl = 50
    doc_lengths = np.arange(10, 201, 5)
    b_values = [0.0, 0.25, 0.5, 0.75, 1.0]
    colors = ["#2ecc71", "#f39c12", "#3498db", "#9b59b6", "#e74c3c"]

    for b, color in zip(b_values, colors):
        norm_factors = [1 - b + b * (dl / avgdl) for dl in doc_lengths]
        axes[0].plot(
            doc_lengths, norm_factors, label=f"b={b}", color=color, linewidth=2
        )

    axes[0].axhline(y=1, color="gray", linestyle="--", alpha=0.5)
    axes[0].axvline(x=avgdl, color="gray", linestyle="--", alpha=0.5)
    axes[0].text(avgdl + 2, 0.5, f"avgdl={avgdl}", fontsize=10, alpha=0.7)

    axes[0].set_xlabel("Document Length (words)", fontsize=11)
    axes[0].set_ylabel("Normalization Factor", fontsize=11)
    axes[0].set_title("Length Normalization by b Parameter", fontsize=13)
    axes[0].legend(loc="upper left")
    axes[0].grid(True, alpha=0.3)

    axes[0].fill_between(
        doc_lengths,
        0,
        1,
        where=[dl < avgdl for dl in doc_lengths],
        alpha=0.1,
        color="green",
    )
    axes[0].fill_between(
        doc_lengths,
        1,
        2.5,
        where=[dl > avgdl for dl in doc_lengths],
        alpha=0.1,
        color="red",
    )
    axes[0].text(
        25, 0.6, "Bonus\nZone", fontsize=10, ha="center", color="green", alpha=0.7
    )
    axes[0].text(
        150, 1.8, "Penalty\nZone", fontsize=10, ha="center", color="red", alpha=0.7
    )

    tf = 3
    k1 = 1.5
    avgdl = 50
    doc_lengths_example = [20, 50, 100, 150]

    x = np.arange(len(doc_lengths_example))
    width = 0.25

    for i, b in enumerate([0.0, 0.5, 1.0]):
        scores = [
            compute_tf_component(tf, k1, b, dl, avgdl) for dl in doc_lengths_example
        ]
        color = colors[int(i * 2)]
        bars = axes[1].bar(
            x + (i - 1) * width,
            scores,
            width,
            label=f"b={b}",
            color=color,
            edgecolor="black",
        )

    axes[1].set_xlabel("Document Length", fontsize=11)
    axes[1].set_ylabel("TF Component", fontsize=11)
    axes[1].set_title(f"Score by Doc Length (same TF={tf}, avgdl={avgdl})", fontsize=13)
    axes[1].set_xticks(x)
    axes[1].set_xticklabels([str(dl) for dl in doc_lengths_example])
    axes[1].legend()
    axes[1].grid(True, alpha=0.3, axis="y")

    plt.tight_layout()
    plt.savefig("viz_length_norm.png", dpi=150, bbox_inches="tight")
    print("Saved: viz_length_norm.png")
    plt.show()


def visualize_inverted_index():
    """
    Inverted Index structure visualization

    Key concept:
    - Inverted index maps "term -> documents"
    - Forward index maps "document -> terms"
    - O(1) lookup for all documents containing a term
    """
    print("\n" + "=" * 60)
    print("Inverted Index Structure Visualization")
    print("=" * 60)
    print()
    print("Inverted Index: {term: {doc_id: tf, ...}, ...}")
    print("  - Stores which documents contain each term")
    print("  - O(1) lookup for relevant documents")
    print()

    documents = [
        "python is great for data science",
        "java is used for enterprise apps",
        "python and java are both popular",
        "data science uses python extensively",
    ]

    fig, axes = plt.subplots(1, 2, figsize=(14, 6))

    axes[0].set_xlim(0, 10)
    axes[0].set_ylim(0, len(documents) + 1)
    axes[0].set_title("Forward Index (Document -> Terms)", fontsize=13)
    axes[0].axis("off")

    for i, doc in enumerate(documents):
        y = len(documents) - i
        axes[0].text(0.5, y, f"Doc {i}:", fontsize=11, fontweight="bold", va="center")
        axes[0].text(
            1.5,
            y,
            doc[:30] + ("..." if len(doc) > 30 else ""),
            fontsize=10,
            va="center",
            family="monospace",
        )

    inverted_index: Dict[str, Dict[int, int]] = {}
    for doc_id, doc in enumerate(documents):
        words = doc.lower().split()
        for word in words:
            if word not in inverted_index:
                inverted_index[word] = {}
            inverted_index[word][doc_id] = inverted_index[word].get(doc_id, 0) + 1

    highlight_terms = ["python", "java", "data", "is", "science"]
    term_colors = {
        "python": "#e74c3c",
        "java": "#3498db",
        "data": "#2ecc71",
        "is": "#95a5a6",
        "science": "#9b59b6",
    }

    axes[1].set_xlim(0, 10)
    axes[1].set_ylim(0, len(highlight_terms) + 2)
    axes[1].set_title("Inverted Index (Term -> Documents)", fontsize=13)
    axes[1].axis("off")

    axes[1].text(0.5, len(highlight_terms) + 1, "Term", fontsize=11, fontweight="bold")
    axes[1].text(
        2.5,
        len(highlight_terms) + 1,
        "Postings (doc_id: tf)",
        fontsize=11,
        fontweight="bold",
    )
    axes[1].text(7.0, len(highlight_terms) + 1, "DF", fontsize=11, fontweight="bold")

    axes[1].axhline(y=len(highlight_terms) + 0.5, color="black", linewidth=0.5)

    for i, term in enumerate(highlight_terms):
        y = len(highlight_terms) - i
        color = term_colors.get(term, "black")

        axes[1].add_patch(
            mpatches.Rectangle(
                (0.3, y - 0.3), 1.5, 0.6, facecolor=color, alpha=0.2, edgecolor=color
            )
        )
        axes[1].text(
            1.0,
            y,
            term,
            fontsize=11,
            fontweight="bold",
            va="center",
            ha="center",
            color=color,
        )

        postings = inverted_index.get(term, {})
        posting_str = ", ".join(
            [f"{doc_id}:{tf}" for doc_id, tf in sorted(postings.items())]
        )
        axes[1].text(
            2.5, y, f"{{{posting_str}}}", fontsize=10, va="center", family="monospace"
        )

        df = len(postings)
        axes[1].text(7.0, y, str(df), fontsize=11, va="center", fontweight="bold")

        bar_width = (df / len(documents)) * 2.5
        axes[1].add_patch(
            mpatches.Rectangle(
                (7.5, y - 0.2), bar_width, 0.4, facecolor=color, alpha=0.5
            )
        )

    plt.tight_layout()
    plt.savefig("viz_inverted_index.png", dpi=150, bbox_inches="tight")
    print("Saved: viz_inverted_index.png")
    plt.show()


def visualize_full_bm25():
    """
    Full BM-25 score calculation visualization

    Shows how scores are calculated for each document given a query.
    """
    print("\n" + "=" * 60)
    print("Full BM-25 Score Calculation Visualization")
    print("=" * 60)
    print()
    print("BM25(D, Q) = SUM[ IDF(q_i) * TF_component(q_i, D) ]")
    print()

    documents = [
        "python machine learning",
        "python data science machine learning deep learning",
        "java enterprise development",
        "machine learning with python and tensorflow",
    ]
    query = "python machine learning"
    query_terms = query.lower().split()

    k1, b = 1.5, 0.75

    doc_lengths = [len(doc.split()) for doc in documents]
    avgdl = sum(doc_lengths) / len(doc_lengths)
    N = len(documents)

    inverted_index: Dict[str, Dict[int, int]] = {}
    for doc_id, doc in enumerate(documents):
        words = doc.lower().split()
        for word in words:
            if word not in inverted_index:
                inverted_index[word] = {}
            inverted_index[word][doc_id] = inverted_index[word].get(doc_id, 0) + 1

    fig = plt.figure(figsize=(15, 10))
    gs = fig.add_gridspec(3, 2, height_ratios=[1, 2, 1], hspace=0.4, wspace=0.3)

    ax_query = fig.add_subplot(gs[0, :])
    ax_query.set_xlim(0, 10)
    ax_query.set_ylim(0, 2)
    ax_query.axis("off")
    ax_query.set_title(f'Query: "{query}"', fontsize=14, fontweight="bold")

    term_info_text = []
    for term in query_terms:
        n_q = len(inverted_index.get(term, {}))
        idf = compute_idf(N, n_q)
        term_info_text.append(f"{term}: DF={n_q}, IDF={idf:.3f}")

    ax_query.text(
        5,
        1,
        " | ".join(term_info_text),
        fontsize=11,
        ha="center",
        va="center",
        bbox=dict(boxstyle="round", facecolor="lightblue", alpha=0.5),
    )

    ax_scores = fig.add_subplot(gs[1, 0])

    doc_scores = []
    score_breakdown = []

    for doc_id, doc in enumerate(documents):
        term_scores = []
        for term in query_terms:
            tf = inverted_index.get(term, {}).get(doc_id, 0)
            n_q = len(inverted_index.get(term, {}))
            idf = compute_idf(N, n_q)
            tf_comp = compute_tf_component(tf, k1, b, doc_lengths[doc_id], avgdl)
            term_score = idf * tf_comp
            term_scores.append((term, tf, idf, tf_comp, term_score))

        total_score = sum([ts[4] for ts in term_scores])
        doc_scores.append(total_score)
        score_breakdown.append(term_scores)

    colors = ["#e74c3c", "#3498db", "#2ecc71", "#f39c12"]
    bars = ax_scores.barh(range(len(documents)), doc_scores, color=colors)
    ax_scores.set_yticks(range(len(documents)))
    ax_scores.set_yticklabels([f"Doc {i}" for i in range(len(documents))])
    ax_scores.set_xlabel("BM-25 Score", fontsize=11)
    ax_scores.set_title("BM-25 Score by Document", fontsize=13)
    ax_scores.invert_yaxis()

    for i, (bar, score) in enumerate(zip(bars, doc_scores)):
        ax_scores.text(
            bar.get_width() + 0.02,
            bar.get_y() + bar.get_height() / 2,
            f"{score:.3f}",
            va="center",
            fontsize=10,
        )

    ax_detail = fig.add_subplot(gs[1, 1])
    ax_detail.axis("off")
    ax_detail.set_title("Score Calculation Detail", fontsize=13)

    y_pos = 1.0
    line_height = 0.12

    for doc_id, breakdown in enumerate(score_breakdown):
        total = sum([ts[4] for ts in breakdown])
        ax_detail.text(
            0,
            y_pos,
            f"Doc {doc_id} (len={doc_lengths[doc_id]}): Total={total:.3f}",
            fontsize=10,
            fontweight="bold",
            transform=ax_detail.transAxes,
        )
        y_pos -= line_height

        for term, tf, idf, tf_comp, term_score in breakdown:
            if tf > 0:
                ax_detail.text(
                    0.05,
                    y_pos,
                    f'  "{term}": TF={tf}, IDF={idf:.3f}, TF_comp={tf_comp:.3f} -> {term_score:.3f}',
                    fontsize=9,
                    transform=ax_detail.transAxes,
                    family="monospace",
                )
                y_pos -= line_height * 0.8

        y_pos -= line_height * 0.5

    ax_docs = fig.add_subplot(gs[2, :])
    ax_docs.axis("off")
    ax_docs.set_title(f"Documents (avgdl={avgdl:.1f}, k1={k1}, b={b})", fontsize=13)

    for i, doc in enumerate(documents):
        ax_docs.text(
            0.02,
            0.8 - i * 0.25,
            f'Doc {i}: "{doc}"',
            fontsize=10,
            transform=ax_docs.transAxes,
        )

    plt.tight_layout()
    plt.savefig("viz_full_bm25.png", dpi=150, bbox_inches="tight")
    print("Saved: viz_full_bm25.png")
    plt.show()


def main():
    visualizations = {
        "idf": ("IDF Curve", visualize_idf),
        "tf": ("TF Saturation", visualize_tf_saturation),
        "length": ("Document Length Normalization", visualize_length_normalization),
        "index": ("Inverted Index Structure", visualize_inverted_index),
        "full": ("Full BM-25 Calculation", visualize_full_bm25),
    }

    if len(sys.argv) > 1:
        choice = sys.argv[1].lower()
        if choice in visualizations:
            name, func = visualizations[choice]
            func()
        else:
            print(f"Unknown option: {choice}")
            print(f"Available: {', '.join(visualizations.keys())}")
    else:
        print("=" * 60)
        print("BM-25 Algorithm Visualization Tool")
        print("=" * 60)
        print()
        print("Running all visualizations sequentially.")
        print("Individual: uv run python visualize_bm25.py [idf|tf|length|index|full]")
        print()

        for key in ["idf", "tf", "length", "index", "full"]:
            name, func = visualizations[key]
            print(f"\n>>> Starting {name} visualization...")
            input("(Press Enter to continue...)")
            func()


if __name__ == "__main__":
    main()
