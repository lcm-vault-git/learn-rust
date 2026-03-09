# Ch06: 구조체 — 대화형 튜토리얼

---

## 1. 구조체 기본 — struct로 데이터 묶기

> **선생님**: ch02에서 튜플로 여러 값을 묶었죠? 근데 `person.0`, `person.1` 이러면 뭐가 뭔지 모르잖아요. 그래서 **이름 붙은 필드**로 묶는 게 구조체예요.
>
> ```rust
> struct User {
>     name: String,
>     age: u32,
>     active: bool,
> }
>
> let user = User {
>     name: String::from("철수"),
>     age: 25,
>     active: true,
> };
> println!("{}", user.name);  // "철수"
> ```
>
> C++의 `struct`나 `class`랑 비슷해요. 다만 Rust는 **`struct`만 있고 `class`가 없어요.** 접근 제어는 `pub` 키워드로 따로 해요.

**🙋 학생**: C++ class랑 뭐가 달라?

> **선생님**: 핵심 차이를 정리하면:
>
> | | C++ class | Rust struct |
> |--|-----------|-------------|
> | 기본 접근 | private | private (같은 모듈 내에선 접근 가능) |
> | 상속 | O (class 상속) | X (trait으로 대체) |
> | 메서드 | class 안에 정의 | `impl` 블록에 따로 정의 |
> | 생성자 | constructor | 연관 함수 (관례: `new()`) |
> | 소멸자 | destructor | `Drop` trait |
>
> 가장 큰 차이: **Rust에는 상속이 없어요.** 대신 trait(ch10)과 합성(composition)을 써요.

**🙋 학생**: mut은 필드별로 설정 못 해?

> **선생님**: 아뇨, Rust는 **인스턴스 단위**로 mut을 정해요.
>
> ```rust
> let mut user = User { name: String::from("철수"), age: 25, active: true };
> user.age = 26;        // OK — 인스턴스가 mut이니까 전부 수정 가능
>
> let user2 = User { name: String::from("영희"), age: 30, active: true };
> // user2.age = 31;    // 에러! user2는 불변
> ```
>
> C++처럼 필드별로 `const` 붙이는 건 안 돼요. 전체가 불변이거나 전체가 가변이에요.

### 💡 핵심
- `struct`로 이름 붙은 필드로 데이터 묶기
- Rust에는 class 없음 — struct + impl로 대체
- mut은 인스턴스 단위 (필드별 X)
- 상속 없음 → trait + 합성으로 대체

---

## 2. 메서드와 impl

> **선생님**: 구조체에 메서드를 붙이려면 `impl` 블록을 써요.
>
> ```rust
> struct Rectangle {
>     width: f64,
>     height: f64,
> }
>
> impl Rectangle {
>     fn area(&self) -> f64 {           // &self = 읽기 빌림
>         self.width * self.height
>     }
>
>     fn scale(&mut self, factor: f64) { // &mut self = 수정 빌림
>         self.width *= factor;
>         self.height *= factor;
>     }
> }
>
> let mut rect = Rectangle { width: 10.0, height: 5.0 };
> println!("넓이: {}", rect.area());  // 50.0
> rect.scale(2.0);
> println!("넓이: {}", rect.area());  // 200.0
> ```

**🙋 학생**: `&self`가 뭐야? ch05에서 배운 빌림이랑 관련 있어?

> **선생님**: 정확해요! `self`의 세 가지 형태가 ch05 소유권과 직결돼요:
>
> | 매개변수 | 의미 | ch05 대응 |
> |---------|------|-----------|
> | `&self` | 읽기 빌림 | `&T` — 값 안 바꿈 |
> | `&mut self` | 수정 빌림 | `&mut T` — 값 바꿈 |
> | `self` | 소유권 이동 | `T` — 호출 후 사용 불가 |
>
> ```rust
> impl User {
>     fn into_name(self) -> String {  // self = 소유권 가져감
>         self.name
>     }
> }
>
> let user = User { name: String::from("철수"), age: 25, active: true };
> let name = user.into_name();
> // println!("{}", user.age);  // 에러! user의 소유권은 이미 이동됨
> ```
>
> **90%는 `&self`를 써요.** 수정이 필요하면 `&mut self`, 값을 소비할 때만 `self`.

### 💡 핵심
- `impl 구조체명 { }` 블록 안에 메서드 정의
- `&self` = 읽기, `&mut self` = 수정, `self` = 소유권 이동
- ch05 소유권 규칙이 그대로 적용됨
- 90%는 `&self`로 충분

---

## 3. 연관 함수 — ::과 .의 차이

> **선생님**: `String::from("hello")` 기억나죠? `::`로 호출하는 건 **연관 함수(associated function)**예요. 인스턴스 없이 호출하는 거예요.
>
> ```rust
> impl Rectangle {
>     // 연관 함수 — self 없음, :: 로 호출
>     fn new(width: f64, height: f64) -> Rectangle {
>         Rectangle { width, height }  // 필드명과 변수명 같으면 축약 가능
>     }
>
>     // 메서드 — &self 있음, . 로 호출
>     fn area(&self) -> f64 {
>         self.width * self.height
>     }
> }
>
> let rect = Rectangle::new(10.0, 5.0);  // :: 로 호출
> let a = rect.area();                    // .  로 호출
> ```

**🙋 학생**: `new`는 생성자인 거야?

> **선생님**: Rust에 공식적인 생성자는 없지만, `new`라는 이름의 연관 함수를 만드는 게 **관례**예요. C++의 constructor 역할이에요.
>
> 정리하면:
> - `::` = 타입에 붙는 함수 (연관 함수). `self` 없음. 예: `String::from()`, `Vec::new()`
> - `.` = 인스턴스에 붙는 함수 (메서드). `self` 있음. 예: `rect.area()`, `s.len()`

### 💡 핵심
- 연관 함수: `self` 없음, `Type::func()`으로 호출
- 메서드: `self` 있음, `instance.method()`로 호출
- `new()` = Rust의 관례적 생성자
- 필드명과 변수명 같으면 축약: `Rectangle { width, height }`

---

## 4. 구조체와 소유권

> **선생님**: 구조체 필드에 `String`을 넣으면, 구조체가 그 데이터를 **소유**하는 거예요.
>
> ```rust
> struct User {
>     name: String,    // User가 문자열 데이터를 소유
>     age: u32,
> }
>
> let name = String::from("철수");
> let user = User { name: name, age: 25 };  // name의 소유권 → User로 이동
> // println!("{name}");  // 에러! name은 이미 이동됨
> ```

**🙋 학생**: 그러면 `&str`을 필드로 쓰면 안 돼?

> **선생님**: 쓸 수는 있는데, **라이프타임**이라는 걸 명시해야 해요. 지금은 복잡하니까 규칙만 알면 돼요:
>
> ```rust
> // 간단한 규칙:
> // 구조체가 데이터를 소유해야 하면 → String
> // 잠깐 빌려서 보기만 할 때     → &str (라이프타임 필요)
>
> struct User {
>     name: String,     // ← 지금은 이걸 쓰세요
> }
> ```
>
> **처음엔 구조체 필드에 `String`, `Vec<T>` 등 소유 타입을 쓰세요.** 참조(`&`)를 필드에 넣는 건 라이프타임을 배운 뒤에 해도 돼요.

### 💡 핵심
- 구조체 필드에 값을 넣으면 소유권이 구조체로 이동
- 필드에는 소유 타입 사용: `String` (O), `&str` (라이프타임 필요)
- 처음엔 `String`, `Vec<T>` 등 소유 타입으로 시작
- 참조 필드는 라이프타임 배운 뒤에

---

## 5. 실전 패턴

> **선생님**: 자주 쓰는 패턴 몇 가지를 알려드릴게요.
>
> **1. `#[derive(Debug)]` — 디버그 출력**
>
> ```rust
> #[derive(Debug)]
> struct Point { x: f64, y: f64 }
>
> let p = Point { x: 1.0, y: 2.0 };
> println!("{:?}", p);   // Point { x: 1.0, y: 2.0 }
> println!("{p:#?}");    // 예쁘게 출력 (줄바꿈 포함)
> ```
>
> `#[derive]`는 컴파일러가 자동으로 구현을 만들어주는 거예요. `Debug`를 붙이면 `{:?}`로 출력 가능.

**🙋 학생**: derive로 뭘 더 붙일 수 있어?

> **선생님**: 자주 쓰는 것들:
>
> | derive | 기능 |
> |--------|------|
> | `Debug` | `{:?}` 출력 |
> | `Clone` | `.clone()` 사용 가능 |
> | `PartialEq` | `==` 비교 가능 |
> | `Default` | 기본값 생성 `Type::default()` |
>
> **2. 구조체 업데이트 문법 (..)**
>
> ```rust
> let user1 = User { name: String::from("철수"), age: 25, active: true };
> let user2 = User {
>     age: 30,
>     ..user1  // 나머지 필드는 user1에서 가져옴
> };
> // 주의: user1.name은 String이라 이동됨! user1.name 사용 불가
> ```
>
> `..`는 "나머지는 여기서 가져와"라는 뜻이에요. 단, String 같은 Move 타입 필드는 소유권이 이동돼요.

**🙋 학생**: 그러면 user1을 아예 못 쓰는 거야?

> **선생님**: `name`(String, Move 타입)은 이동됐으니 못 쓰지만, `age`(u32, Copy 타입)나 `active`(bool, Copy 타입)는 복사된 거라 `user1.age`는 여전히 쓸 수 있어요. ch05에서 배운 Copy vs Move 규칙이 여기서도 적용돼요.

### 💡 핵심
- `#[derive(Debug)]`로 디버그 출력 활성화
- `..other` 문법으로 나머지 필드 복사/이동
- derive로 Clone, PartialEq, Default 등 자동 구현
- 업데이트 문법에서도 소유권 규칙 적용 (Move 타입 주의)

---

## 6. Ch06 총정리

| 개념 | 한 줄 요약 |
|------|-----------|
| `struct` | 이름 붙은 필드로 데이터 묶기 |
| `impl` | 메서드와 연관 함수 정의 블록 |
| `&self` / `&mut self` / `self` | 읽기 빌림 / 수정 빌림 / 소유권 이동 |
| `Type::func()` | 연관 함수 (:: 호출, self 없음) |
| `instance.method()` | 메서드 (. 호출, self 있음) |
| `#[derive(...)]` | 컴파일러 자동 구현 (Debug, Clone 등) |
| `..other` | 구조체 업데이트 문법 |

> **Rust 설계 철학**: 상속 대신 합성과 trait. 구조체는 데이터만 담고, 행동은 impl과 trait으로 분리. 소유권 규칙이 구조체에도 동일하게 적용.
