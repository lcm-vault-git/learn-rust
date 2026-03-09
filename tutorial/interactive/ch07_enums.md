# Ch07: 열거형과 패턴 매칭 — 대화형 튜토리얼

---

## 1. 열거형 기본 — 데이터를 품는 enum

> **선생님**: C++이나 Java의 enum은 그냥 숫자에 이름 붙인 거잖아요? Rust의 enum은 **각 변형(variant)에 데이터를 넣을 수 있어요.**
>
> ```rust
> // C++ 스타일 — 그냥 구분용
> enum Direction {
>     Up,
>     Down,
>     Left,
>     Right,
> }
>
> // Rust만의 방식 — 데이터를 품음
> enum Message {
>     Quit,                        // 데이터 없음
>     Move { x: i32, y: i32 },    // 구조체처럼 이름 붙은 필드
>     Write(String),               // String 하나를 품음
>     Color(u8, u8, u8),           // 튜플처럼 여러 값
> }
> ```

**🙋 학생**: 이거 그냥 여러 struct를 하나로 묶은 거 아니야?

> **선생님**: 정확한 비유예요! C++로 치면 `union` + `enum`을 안전하게 합친 거예요. Rust에서는 이걸 **tagged union**이라고도 불러요.
>
> | | C++ enum | Java enum | Rust enum |
> |--|---------|-----------|-----------|
> | 데이터 포함 | X | 필드 가능 (제한적) | 변형별로 다른 데이터 |
> | 패턴 매칭 | switch (빈틈 가능) | switch (빈틈 가능) | match (빈틈 불가) |
> | 타입 안전성 | 약함 (정수 캐스팅) | 보통 | 강함 |
>
> **Rust enum이 강력한 이유**: 서로 다른 타입의 데이터를 하나의 타입으로 묶을 수 있어요.

### 💡 핵심
- Rust enum = 각 변형(variant)에 서로 다른 데이터를 넣을 수 있음
- C++ union + enum을 안전하게 합친 것 (tagged union)
- `Message::Write(String)` 처럼 변형마다 다른 구조 가능

---

## 2. match — 빈틈 없는 패턴 매칭

> **선생님**: enum을 쓰려면 `match`로 각 경우를 처리해요.
>
> ```rust
> fn process(msg: Message) {
>     match msg {
>         Message::Quit => println!("종료"),
>         Message::Move { x, y } => println!("이동: ({x}, {y})"),
>         Message::Write(text) => println!("메시지: {text}"),
>         Message::Color(r, g, b) => println!("색상: {r},{g},{b}"),
>     }
> }
> ```
>
> **핵심: 모든 변형을 다뤄야 해요.** 하나라도 빠뜨리면 컴파일 에러!

**🙋 학생**: 다 처리하기 귀찮은데, 나머지는 무시하면 안 돼?

> **선생님**: `_` 와일드카드를 쓰면 돼요.
>
> ```rust
> match msg {
>     Message::Quit => println!("종료"),
>     _ => println!("기타"),  // 나머지 전부
> }
> ```
>
> 근데 이게 **일부러 빡빡하게 만든 거**예요. C++의 switch에서 case를 빠뜨려서 버그 나는 경우 많잖아요? Rust는 컴파일러가 "너 이 경우 안 다뤘어"라고 알려줘요.
>
> 나중에 enum에 새 변형을 추가하면? 모든 match 문에서 컴파일 에러가 나요. 그래서 **빠뜨리는 게 불가능**해요.

**🙋 학생**: match에서 값도 꺼낼 수 있는 거야?

> **선생님**: 네! 그게 패턴 매칭의 핵심이에요. `Message::Write(text)`처럼 쓰면 안에 든 `String`이 `text` 변수에 바인딩돼요.
>
> ```rust
> let msg = Message::Move { x: 10, y: 20 };
> match msg {
>     Message::Move { x, y } => println!("x={x}, y={y}"),  // x=10, y=20
>     _ => {}
> }
> ```
>
> **match는 분기 + 값 추출을 동시에 해요.** if-else 체인보다 훨씬 강력해요.

### 💡 핵심
- `match`는 모든 경우를 다뤄야 함 (exhaustive) → 빠뜨림 방지
- `_`로 나머지 처리 가능
- 패턴 매칭으로 값 추출: `Message::Write(text)` → text에 바인딩
- enum에 변형 추가 시 → 모든 match에서 컴파일 에러 → 실수 방지

---

## 3. Option<T> — null이 없는 세상

> **선생님**: Rust에는 `null`이 없어요. 대신 **값이 있을 수도 없을 수도 있는 상황**을 `Option<T>`으로 표현해요.
>
> ```rust
> enum Option<T> {
>     Some(T),   // 값이 있음
>     None,      // 값이 없음
> }
>
> let age: Option<u32> = Some(25);
> let name: Option<String> = None;
> ```

**🙋 학생**: null이 왜 위험해? Java에서 잘 쓰고 있는데.

> **선생님**: Java에서 `NullPointerException` 안 만나봤어요? Tony Hoare(null을 발명한 사람)가 직접 **"10억 달러짜리 실수"**라고 부를 정도예요.
>
> ```rust
> // Java — 컴파일 OK, 런타임에 터짐
> // String name = null;
> // name.length();  // NullPointerException!
>
> // Rust — 컴파일 타임에 차단
> let name: Option<String> = None;
> // name.len();  // 에러! Option<String>은 String이 아님
>
> // 값을 쓰려면 반드시 꺼내야 함
> match name {
>     Some(n) => println!("길이: {}", n.len()),
>     None => println!("이름 없음"),
> }
> ```
>
> **Option은 "이 값은 없을 수도 있어"를 타입으로 명시하는 거예요.** 컴파일러가 None 처리를 강제해요.

**🙋 학생**: Java의 Optional이랑 비슷한 거네?

> **선생님**: 맞아요, 개념은 같아요. 차이는:
>
> | | Java Optional | Rust Option |
> |--|--------------|-------------|
> | 사용 강제 | X (그냥 null 써도 됨) | O (null 자체가 없음) |
> | 성능 비용 | 힙 할당 (객체) | 제로 코스트 (컴파일 타임) |
> | 패턴 매칭 | X | match/if let으로 추출 |
>
> Java는 Optional을 권장하지만 강제가 아니라서, 결국 null을 쓰는 코드가 섞여요. Rust는 **null 자체가 없으니까 무조건 Option**이에요.

### 💡 핵심
- Rust에 null 없음 → `Option<T>` = `Some(값)` 또는 `None`
- Option을 쓰면 None 처리를 컴파일러가 강제
- Java Optional과 같은 개념이지만, Rust는 강제 + 제로 코스트
- null의 "10억 달러 실수"를 타입 시스템으로 해결

---

## 4. if let — match의 간편 버전

> **선생님**: match에서 하나의 패턴만 관심 있을 때, `if let`이 더 깔끔해요.
>
> ```rust
> let msg = Some("hello");
>
> // match — 좀 길다
> match msg {
>     Some(text) => println!("{text}"),
>     None => {}
> }
>
> // if let — 간결!
> if let Some(text) = msg {
>     println!("{text}");
> }
> ```
>
> `if let 패턴 = 값` — "이 값이 이 패턴에 맞으면 실행해"라는 뜻이에요. `else`도 붙일 수 있어요.
>
> ```rust
> if let Some(text) = msg {
>     println!("있음: {text}");
> } else {
>     println!("없음");
> }
> ```

**🙋 학생**: 그러면 match랑 if let 중에 언제 뭘 써야 해?

> **선생님**: 간단한 기준:
> - **경우가 2-3개 이상** → `match` (전부 다루니까 안전)
> - **하나만 확인** → `if let` (간결)
> - **모든 경우를 빠짐없이 다뤄야 할 때** → 반드시 `match`

### 💡 핵심
- `if let 패턴 = 값` — 하나의 패턴만 확인할 때 간결한 문법
- match의 축약 버전, else도 가능
- 여러 경우 → match, 하나만 → if let

---

## 5. 실전 패턴 — enum으로 상태 표현하기

> **선생님**: enum의 진짜 위력은 **상태를 타입으로 표현**할 때 나와요.
>
> ```rust
> enum HttpStatus {
>     Ok(String),                   // 200 — 응답 본문
>     NotFound,                     // 404
>     ServerError { code: u16, msg: String },  // 500 — 에러 정보
> }
>
> fn handle_response(status: HttpStatus) {
>     match status {
>         HttpStatus::Ok(body) => println!("성공: {body}"),
>         HttpStatus::NotFound => println!("404 Not Found"),
>         HttpStatus::ServerError { code, msg } => {
>             println!("서버 에러 {code}: {msg}");
>         }
>     }
> }
> ```
>
> 각 상태마다 **필요한 데이터가 다르잖아요?** Ok이면 본문이 필요하고, ServerError면 코드와 메시지가 필요하고. enum으로 이걸 하나의 타입에 안전하게 담을 수 있어요.

**🙋 학생**: ch09에서 배울 Result도 이런 거야?

> **선생님**: 날카로워요! `Result`는 사실 이렇게 생긴 enum이에요:
>
> ```rust
> enum Result<T, E> {
>     Ok(T),    // 성공 — 결과값
>     Err(E),   // 실패 — 에러 정보
> }
> ```
>
> Rust의 에러 처리 전체가 enum + match 위에 만들어진 거예요. ch09에서 자세히 다룰 거예요.

**🙋 학생**: Option에도 편한 메서드가 있어?

> **선생님**: 많아요! 자주 쓰는 것들:
>
> ```rust
> let x: Option<i32> = Some(5);
>
> x.unwrap();          // Some이면 값 꺼냄, None이면 panic!
> x.unwrap_or(0);      // Some이면 값, None이면 기본값 0
> x.is_some();         // true
> x.is_none();         // false
> x.map(|v| v * 2);    // Some(10) — 안의 값을 변환
> ```
>
> `unwrap()`은 **프로토타이핑에만** 쓰세요. 프로덕션 코드에서는 match나 `unwrap_or`를 쓰는 게 안전해요.

### 💡 핵심
- enum으로 상태별로 다른 데이터를 하나의 타입에 담기
- `Result<T, E>`도 enum — Rust 에러 처리의 기반
- Option 유용한 메서드: `unwrap_or`, `map`, `is_some`
- `unwrap()`은 프로토타이핑에만 (프로덕션에서는 위험)

---

## 6. Ch07 총정리

| 개념 | 한 줄 요약 |
|------|-----------|
| `enum` | 변형(variant)별로 다른 데이터를 품는 타입 |
| `match` | 모든 경우를 빠짐없이 처리 (exhaustive) |
| `_` | 와일드카드 — 나머지 전부 |
| `Option<T>` | `Some(값)` 또는 `None` — null 대체 |
| `if let` | match의 간편 버전 (하나만 확인) |
| `Result<T, E>` | `Ok(값)` 또는 `Err(에러)` — 에러 처리 기반 |

> **Rust 설계 철학**: null을 타입 시스템으로 대체하고, match의 exhaustive 검사로 빠뜨림을 컴파일 타임에 잡는다. 모든 것이 "컴파일 타임에 잡자"로 연결.
