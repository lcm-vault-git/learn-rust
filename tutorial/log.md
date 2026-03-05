# Rust 학습 일지

---

## 2026-03-04 — Ch01: Hello World + 프로젝트 세팅

### 사용한 명령어
```bash
cargo new ch01_hello       # 새 Rust 프로젝트 생성
cargo build                # 컴파일만 (실행 안 함)
cargo run                  # 컴파일 + 실행
cargo check                # 빠른 컴파일 검사 (바이너리 안 만듦)
```

### 배운 내용
- `fn main()` — Rust 프로그램 진입점
- `println!("텍스트")` — 출력 매크로 (`!` = 매크로 표시)
- 모든 문장은 세미콜론 `;`으로 끝남

### 트러블슈팅
- PowerShell에서 `cargo` 명령 안 됨 → 환경변수 PATH에 `C:\Users\ham\.cargo\bin` 등록 필요
- 환경변수 등록 후에도 안 되면 → **Cursor(IDE)를 껐다 다시 켜야** 새 환경변수 적용됨

### 연습 과제
```bash
# 직접 해보기
cargo new my_test          # 새 프로젝트 만들어보기
cd my_test
cargo run                  # 실행 확인
cargo check                # build 대신 빠른 검사 해보기
```
