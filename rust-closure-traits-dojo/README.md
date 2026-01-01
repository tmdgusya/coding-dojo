# Rust Closure & Traits Dojo

`Box<dyn FnOnce() + Send + 'static>`을 완전히 이해하기 위한 수련장입니다.

## 선수 조건

- rust-dojo (minigrep) 완료
- 기본 클로저 문법 (`|x| x + 1`)

## 빠른 시작

```bash
cd rust-closure-traits-dojo

cargo test             # 전체 테스트 (처음엔 모두 실패)
cargo test mission_1   # Fn, FnMut, FnOnce
cargo test mission_2   # dyn Trait
cargo test mission_3   # 'static
cargo test mission_4   # Send + Sync
cargo test mission_5   # 통합
```

## 학습 경로

| 임무 | 주제 | 핵심 질문 |
|------|------|----------|
| 1 | 클로저 트레이트 | 왜 Fn, FnMut, FnOnce 3개? |
| 2 | 트레이트 객체 | 제네릭 vs dyn Trait? |
| 3 | 'static | 영원히 산다는 뜻? |
| 4 | Send + Sync | Rc는 왜 스레드로 못 보내? |
| 5 | **통합** | 모든 조각 맞추기 |

## 다음 단계

이 dojo 완료 후 → rust-worker-dojo 임무 6.5, 7

## 상세 안내

- 학습 계획: [docs/README.md](docs/README.md)
- 도움 요청: [docs/Assistance.md](docs/Assistance.md)
