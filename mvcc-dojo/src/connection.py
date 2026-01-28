"""PostgreSQL 연결 헬퍼"""

import psycopg2

DB_CONFIG = {
    "dbname": "mvcc_dojo",
    "user": "dojo",
    "password": "dojo",
    "host": "localhost",
    "port": 15432,
}


def get_connection(autocommit=False):
    """새 PostgreSQL 연결을 반환합니다.

    Args:
        autocommit: True이면 자동 커밋 모드로 설정

    Returns:
        psycopg2.connection: PostgreSQL 연결 객체
    """
    conn = psycopg2.connect(**DB_CONFIG)
    conn.autocommit = autocommit
    return conn


def execute(sql, params=None, autocommit=True):
    """단일 SQL을 실행하고 결과를 반환합니다.

    Args:
        sql: 실행할 SQL 문
        params: SQL 파라미터 (선택)
        autocommit: True이면 자동 커밋 모드로 실행

    Returns:
        list: SELECT 쿼리의 경우 fetchall() 결과, 그 외 None
    """
    conn = get_connection(autocommit=autocommit)
    try:
        cur = conn.cursor()
        cur.execute(sql, params)

        if cur.description:
            result = cur.fetchall()
        else:
            result = None

        cur.close()

        if not autocommit:
            conn.commit()

        return result
    finally:
        conn.close()
