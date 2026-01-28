"""테스트 픽스처: 테이블 생성/정리"""

import pytest
from src.connection import get_connection


@pytest.fixture(autouse=True)
def setup_tables():
    """각 테스트 전에 필요한 테이블을 생성하고, 후에 정리합니다."""
    conn = get_connection(autocommit=True)
    cur = conn.cursor()

    cur.execute("""
        CREATE TABLE IF NOT EXISTS reader_writer_test (
            id SERIAL PRIMARY KEY,
            value TEXT
        )
    """)

    cur.execute("""
        CREATE TABLE IF NOT EXISTS accounts (
            id SERIAL PRIMARY KEY,
            balance INTEGER
        )
    """)

    cur.execute("""
        CREATE TABLE IF NOT EXISTS bloat_test (
            id SERIAL PRIMARY KEY,
            data TEXT
        )
    """)

    cur.execute("""
        CREATE TABLE IF NOT EXISTS vacuum_test (
            id SERIAL PRIMARY KEY,
            data TEXT
        )
    """)

    cur.execute("""
        CREATE TABLE IF NOT EXISTS conflict_test (
            id SERIAL PRIMARY KEY,
            value INTEGER
        )
    """)

    cur.execute("""
        CREATE TABLE IF NOT EXISTS long_tx_test (
            id SERIAL PRIMARY KEY,
            data TEXT
        )
    """)

    cur.close()
    conn.close()

    yield

    conn = get_connection(autocommit=True)
    cur = conn.cursor()

    cur.execute("DROP TABLE IF EXISTS reader_writer_test CASCADE")
    cur.execute("DROP TABLE IF EXISTS accounts CASCADE")
    cur.execute("DROP TABLE IF EXISTS bloat_test CASCADE")
    cur.execute("DROP TABLE IF EXISTS vacuum_test CASCADE")
    cur.execute("DROP TABLE IF EXISTS conflict_test CASCADE")
    cur.execute("DROP TABLE IF EXISTS long_tx_test CASCADE")

    cur.close()
    conn.close()
