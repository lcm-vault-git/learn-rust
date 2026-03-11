# Ch09: 에러 처리 — 대화형 튜토리얼

---

## 1. panic! vs Result — 두 종류의 에러

> **선생님**: Rust에서 에러는 두 가지로 나눠요:
>
> - **panic!** = 복구 불가능. 프로그램 중단. "이건 버그다!"
> - **Result** = 복구 가능. 호출자에게 알려줌. "이건 실패할 수 있어"
>
> ```rust
> // panic! — 프로그램이 죽음
> panic!("치명적 에러!");
>
> // Result — 호출자가 처리
> let file = std::fs::read_to_string("config.txt");
> // file은 Result<String, io::Error> 타입
> ```

**🙋 학생**: Java의 try-catch가 왜 없어?

> **선생님**: Rust는 예외(exception) 대신 **반환값으로 에러를 전달**하는 방식을 택했어요.
>
> | | C++ | Java | Rust |
> |--|-----|------|------|
> | 에러 전달 | 예외 throw | 예외 throw | Result 반환 |
> | 처리 강제 | X | checked만 | O (Result는 반드시 처리) |
> | 성능 비용 | 예외 발생 시 큼 | 예외 발생 시 큼 | 없음 (그냥 반환값) |
> | 숨겨진 흐름 | O (어디서 catch?) | O | X (코드에 다 보임) |
>
> 예외의 문제: **코드의 어디서든 갑자기 점프**할 수 있어요. try-catch를 빼먹으면 런타임에 터지고. Result는 반환값이라 **에러 경로가 코드에 명시적으로 보여요.**

### 💡 핵심
- `panic!` = 복구 불가, 프로그램 중단 (버그 상황)
- `Result<T, E>` = 복구 가능, 호출자에게 전달
- Rust에 try-catch 없음 → 반환값으로 에러 전달
- 에러 경로가 코드에 명시적으로 보임

---

## 2. Result<T, E> — Ok와 Err

> **선생님**: ch07에서 봤듯이 `Result`는 enum이에요.
>
> ```rust
> enum Result<T, E> {
>     Ok(T),    // 성공 — 결과값
>     Err(E),   // 실패 — 에러 정보
> }
> ```
>
> match로 처리하는 게 기본이에요:
>
> ```rust
> use std::fs;
>
> let result = fs::read_to_string("config.txt");
> match result {
>     Ok(content) => println!("파일 내용: {content}"),
>     Err(e) => println!("읽기 실패: {e}"),
> }
> ```

**🙋 학생**: 매번 match 쓰기 귀찮은데, 간단하게 못 해?

> **선생님**: 몇 가지 편의 메서드가 있어요:
>
> ```rust
> // unwrap — Ok면 값, Err면 panic!
> let content = fs::read_to_string("config.txt").unwrap();
>
> // expect — unwrap과 같지만 에러 메시지 지정
> let content = fs::read_to_string("config.txt")
>     .expect("config.txt를 읽을 수 없습니다");
>
> // unwrap_or — Err면 기본값 사용
> let content = fs::read_to_string("config.txt")
>     .unwrap_or(String::from("default"));
> ```

**🙋 학생**: unwrap 그냥 쓰면 안 돼?

> **선생님**: `unwrap()`은 Err일 때 **panic**이에요. 프로그램이 죽어요.
>
> - **프로토타이핑/테스트**: unwrap OK (빠르게 작성)
> - **프로덕션 코드**: unwrap 지양 (match, `?`, unwrap_or 사용)
> - **"절대 실패 안 해"를 확신할 때**: expect에 이유를 적어서 사용
>
> `expect("reason")`은 나중에 panic이 나면 **왜 실패했는지** 바로 알 수 있어서 unwrap보다 낫습니다.

### 💡 핵심
- `Result<T, E>` = `Ok(T)` 또는 `Err(E)`
- match로 처리하는 게 기본
- `unwrap()`: Err면 panic — 프로토타이핑에만
- `expect("이유")`: unwrap보다 나음 — 에러 메시지 포함
- `unwrap_or(기본값)`: 안전한 대안

---

## 3. ? 연산자 — 에러 전파의 핵심

> **선생님**: 실무에서 가장 많이 쓰는 건 **`?` 연산자**예요.
>
> ```rust
> use std::fs;
> use std::io;
>
> // ? 없이 — 장황함
> fn read_config() -> Result<String, io::Error> {
>     let result = fs::read_to_string("config.txt");
>     match result {
>         Ok(content) => Ok(content),
>         Err(e) => Err(e),      // 에러를 그대로 전파
>     }
> }
>
> // ? 사용 — 깔끔!
> fn read_config() -> Result<String, io::Error> {
>     let content = fs::read_to_string("config.txt")?;  // 에러면 바로 반환
>     Ok(content)
> }
> ```
>
> `?`는 "Ok면 값을 꺼내고, Err면 이 함수에서 바로 반환"이에요.

**🙋 학생**: `?` 이게 뭐야? 한 글자로 그렇게 많은 걸 해?

> **선생님**: 정확히 이거예요:
>
> ```rust
> // fs::read_to_string("config.txt")?
> // 는 이것과 같음:
> match fs::read_to_string("config.txt") {
>     Ok(val) => val,           // 성공이면 값을 꺼냄
>     Err(e) => return Err(e),  // 실패면 함수에서 바로 탈출
> }
> ```
>
> **체이닝**도 가능해요:
>
> ```rust
> fn setup() -> Result<Config, io::Error> {
>     let raw = fs::read_to_string("config.txt")?;  // 실패하면 여기서 반환
>     let parsed = parse_config(&raw)?;               // 실패하면 여기서 반환
>     let validated = validate(parsed)?;               // 실패하면 여기서 반환
>     Ok(validated)
> }
> ```
>
> 각 줄에서 에러가 나면 그 즉시 함수가 Err를 반환해요. try-catch 없이도 에러 처리가 깔끔하죠.

**🙋 학생**: `?`를 쓰려면 함수 반환타입이 Result여야 해?

> **선생님**: 네! `?`는 반환타입이 `Result` (또는 `Option`)인 함수에서만 쓸 수 있어요. `main`에서도 쓸 수 있어요:
>
> ```rust
> fn main() -> Result<(), Box<dyn std::error::Error>> {
>     let content = fs::read_to_string("config.txt")?;
>     println!("{content}");
>     Ok(())
> }
> ```

### 💡 핵심
- `?` = Ok면 값 꺼냄, Err면 바로 반환 (에러 전파)
- 체이닝 가능: 각 줄에서 실패 시 즉시 반환
- 반환타입이 `Result` 또는 `Option`인 함수에서만 사용 가능
- 실무에서 가장 많이 쓰는 에러 처리 방법

---

## 4. 커스텀 에러 타입

> **선생님**: 실무에서는 자기만의 에러 타입을 만들어요. ch07의 enum이 여기서 빛을 발해요.
>
> ```rust
> #[derive(Debug)]
> enum AppError {
>     FileNotFound(String),
>     ParseError { line: usize, msg: String },
>     NetworkTimeout,
> }
>
> fn load_config(path: &str) -> Result<Config, AppError> {
>     let content = std::fs::read_to_string(path)
>         .map_err(|_| AppError::FileNotFound(path.to_string()))?;
>
>     let config = parse(&content)
>         .map_err(|e| AppError::ParseError { line: e.line, msg: e.msg })?;
>
>     Ok(config)
> }
> ```

**🙋 학생**: 매번 저렇게 만들어야 해? 귀찮은데.

> **선생님**: `thiserror` 크레이트를 쓰면 편해요:
>
> ```rust
> use thiserror::Error;
>
> #[derive(Debug, Error)]
> enum AppError {
>     #[error("파일을 찾을 수 없음: {0}")]
>     FileNotFound(String),
>
>     #[error("{line}번째 줄 파싱 에러: {msg}")]
>     ParseError { line: usize, msg: String },
>
>     #[error("네트워크 타임아웃")]
>     NetworkTimeout,
> }
> ```
>
> `#[error("...")]`로 에러 메시지를 지정하면 `Display` 트레이트가 자동 구현돼요.

### 💡 핵심
- enum으로 커스텀 에러 타입 정의 (ch07 enum 활용)
- `map_err()`로 에러 타입 변환
- `thiserror` 크레이트로 보일러플레이트 줄이기
- 에러 메시지는 `#[error("...")]`로 선언적 정의

---

## 5. 실전 에러 처리 전략

> **선생님**: 실무에서 언제 뭘 쓸지 정리해드릴게요.

**🙋 학생**: panic이랑 Result 중에 언제 뭘 써야 해?

> **선생님**: 기준이 명확해요:
>
> | 상황 | 방법 |
> |------|------|
> | 프로그래밍 버그 (절대 일어나면 안 됨) | `panic!`, `unreachable!()` |
> | 외부 요인으로 실패 가능 (파일, 네트워크) | `Result` |
> | 프로토타이핑 중 | `unwrap()` / `expect()` |
> | 라이브러리 코드 | `Result` (호출자가 결정) |
> | 애플리케이션 최상위 | `anyhow` 크레이트 |
>
> **`anyhow`** 크레이트는 애플리케이션 코드에서 편하게 쓸 수 있어요:
>
> ```rust
> use anyhow::{Context, Result};
>
> fn setup() -> Result<()> {  // anyhow::Result = 어떤 에러든 OK
>     let config = std::fs::read_to_string("config.txt")
>         .context("config.txt 읽기 실패")?;  // 맥락 추가
>
>     let port: u16 = config.trim().parse()
>         .context("포트 번호 파싱 실패")?;
>
>     Ok(())
> }
> ```

> **선생님**: 크레이트 선택 기준:
>
> | 크레이트 | 용도 |
> |---------|------|
> | `thiserror` | 라이브러리 — 구체적인 에러 타입 정의 |
> | `anyhow` | 애플리케이션 — 편하게 에러 전파 |
>
> PLAN.md에서 볼 DPS 프로세스는 **tonic 서버(애플리케이션)**니까 `anyhow`를 주로 쓰게 될 거예요. wasmtime 래퍼 같은 라이브러리 부분은 `thiserror`를 쓰면 좋고요.

### 💡 핵심
- 버그 = panic, 실패 가능 = Result
- 라이브러리: `thiserror` (구체적 에러 타입)
- 애플리케이션: `anyhow` (편한 에러 전파)
- `.context("설명")`으로 에러에 맥락 추가

---

## 6. Ch09 총정리

| 개념 | 한 줄 요약 |
|------|-----------|
| `panic!` | 복구 불가 에러 — 프로그램 중단 |
| `Result<T, E>` | 복구 가능 에러 — Ok 또는 Err |
| `?` 연산자 | Ok면 값 꺼냄, Err면 바로 반환 |
| `unwrap()` / `expect()` | 빠른 처리 (Err면 panic) |
| `map_err()` | 에러 타입 변환 |
| `thiserror` | 라이브러리용 커스텀 에러 |
| `anyhow` | 애플리케이션용 편한 에러 처리 |

> **Rust 설계 철학**: 예외 대신 반환값으로 에러 전달. 에러 경로가 코드에 명시적으로 보이고, 처리를 컴파일러가 강제. `?` 연산자로 간결함까지 확보.
