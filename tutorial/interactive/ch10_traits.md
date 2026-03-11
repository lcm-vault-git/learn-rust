# Ch10: 트레이트 — 대화형 튜토리얼

---

## 1. 트레이트 기본 — 행동을 정의하는 계약

> **선생님**: ch06에서 Rust에 상속이 없다고 했죠? 대신 **트레이트(trait)**로 "이 타입이 무엇을 할 수 있는지"를 정의해요.
>
> ```rust
> trait Greet {
>     fn hello(&self) -> String;
> }
>
> struct User { name: String }
> struct Bot { id: u32 }
>
> impl Greet for User {
>     fn hello(&self) -> String {
>         format!("안녕하세요, {}입니다", self.name)
>     }
> }
>
> impl Greet for Bot {
>     fn hello(&self) -> String {
>         format!("봇 #{}입니다", self.id)
>     }
> }
> ```

**🙋 학생**: Java interface랑 뭐가 달라?

> **선생님**: 거의 같은 개념이에요! 차이점:
>
> | | Java interface | Rust trait |
> |--|---------------|------------|
> | 기본 메서드 | O (default method) | O |
> | 다중 구현 | O (implements 여러 개) | O (impl 여러 개) |
> | 필드 | X (상수만) | X |
> | 제네릭 제약 | extends (`<T extends Comparable>`) | 트레이트 바운드 (`T: Ord`) |
> | 상속 | 클래스 상속과 공존 | 상속 없음, trait만 |
> | 외부 타입에 구현 | X | O (orphan rule 있음) |
>
> 가장 큰 차이: Rust에서는 **내가 만들지 않은 타입에도 trait을 구현**할 수 있어요.
>
> ```rust
> // 외부 타입 i32에 내가 만든 trait 구현
> trait Describe {
>     fn desc(&self) -> String;
> }
>
> impl Describe for i32 {
>     fn desc(&self) -> String {
>         format!("정수 {}", self)
>     }
> }
>
> println!("{}", 42.desc());  // "정수 42"
> ```

### 💡 핵심
- `trait` = 타입이 할 수 있는 행동의 계약 (Java interface와 유사)
- `impl Trait for Type`으로 구현
- 외부 타입에도 trait 구현 가능 (Java보다 유연)
- Rust에서 상속 대신 사용하는 핵심 도구

---

## 2. 기본 구현과 derive

> **선생님**: trait 메서드에 **기본 구현**을 넣을 수 있어요. Java의 default method랑 같아요.
>
> ```rust
> trait Summary {
>     fn title(&self) -> String;            // 필수 구현
>     fn preview(&self) -> String {          // 기본 구현 있음
>         format!("{}...", &self.title()[..20])
>     }
> }
>
> struct Article { title: String, content: String }
>
> impl Summary for Article {
>     fn title(&self) -> String {
>         self.title.clone()  // title만 구현하면 preview는 기본 구현 사용
>     }
> }
> ```

**🙋 학생**: derive가 정확히 뭐야? ch06에서 `#[derive(Debug)]` 쓰긴 했는데.

> **선생님**: `#[derive]`는 **컴파일러가 trait 구현을 자동으로 만들어주는** 거예요. 직접 구현하지 않아도 되니까 편해요.
>
> ```rust
> #[derive(Debug, Clone, PartialEq)]
> struct Point { x: f64, y: f64 }
>
> let p1 = Point { x: 1.0, y: 2.0 };
> let p2 = p1.clone();          // Clone이 구현됨
> println!("{:?}", p1);         // Debug가 구현됨
> println!("{}", p1 == p2);     // PartialEq가 구현됨 → true
> ```
>
> 자주 쓰는 derive 목록:
>
> | derive | 효과 |
> |--------|------|
> | `Debug` | `{:?}` 출력 가능 |
> | `Clone` | `.clone()` 복사 가능 |
> | `Copy` | 자동 복사 (ch05의 Copy 트레이트!) |
> | `PartialEq` | `==` 비교 가능 |
> | `Eq` | 완전 비교 (NaN 없는 타입) |
> | `Hash` | HashMap 키로 사용 가능 |
> | `Default` | `Type::default()` 기본값 생성 |
>
> **`Debug`는 거의 항상 붙이세요.** 디버깅할 때 `println!("{:?}", x)`로 바로 확인 가능.

### 💡 핵심
- trait에 기본 구현 가능 (Java default method와 동일)
- `#[derive(...)]` = 컴파일러가 자동으로 trait 구현
- `Debug`는 거의 항상 붙이기 (디버깅 필수)
- ch05의 Copy, ch06의 Clone이 전부 trait이었음

---

## 3. 트레이트 바운드 — 제네릭 + 트레이트

> **선생님**: 함수에서 "아무 타입이나 받되, 특정 능력이 있는 타입만"이라고 제한하는 게 **트레이트 바운드**예요.
>
> ```rust
> // "Display를 구현한 타입만 받겠다"
> fn print_it<T: std::fmt::Display>(item: T) {
>     println!("{item}");
> }
>
> print_it(42);         // i32는 Display 구현됨 → OK
> print_it("hello");    // &str도 Display 구현됨 → OK
> // print_it(vec![1]); // Vec은 Display 없음 → 컴파일 에러!
> ```

**🙋 학생**: C++의 템플릿이랑 비슷한 거 아니야?

> **선생님**: 비슷하지만 핵심 차이가 있어요:
>
> | | C++ 템플릿 | Rust 제네릭 + 트레이트 바운드 |
> |--|----------|----------------------------|
> | 제약 명시 | X (C++20 concept 전까지) | O (항상 명시) |
> | 에러 시점 | 인스턴스화할 때 (긴 에러) | 함수 정의 시점 (명확한 에러) |
> | 에러 메시지 | 읽기 어려움 | 읽기 쉬움 |
>
> C++ 템플릿은 아무 타입이나 넣고 "이게 되나 보자" 하는 거고, Rust는 **"이 능력이 있는 타입만 받겠다"**를 미리 선언하는 거예요.
>
> 여러 트레이트를 요구할 때:
>
> ```rust
> // + 로 여러 트레이트 요구
> fn log<T: Display + Debug>(item: T) {
>     println!("Display: {item}");
>     println!("Debug: {item:?}");
> }
>
> // where 절로 읽기 좋게
> fn process<T, U>(t: T, u: U)
> where
>     T: Display + Clone,
>     U: Debug + Default,
> {
>     // ...
> }
> ```

### 💡 핵심
- `fn foo<T: Trait>(x: T)` = T는 Trait를 구현해야 함
- `+`로 여러 트레이트 요구: `T: Display + Debug`
- `where` 절로 복잡한 바운드를 읽기 좋게
- C++ 템플릿과 달리 제약을 미리 명시 → 명확한 에러

---

## 4. 자주 쓰는 표준 트레이트

> **선생님**: Rust 표준 라이브러리에 자주 쓰는 trait들이 있어요. 이걸 알면 라이브러리 문서 읽기가 쉬워져요.
>
> ```rust
> use std::fmt;
>
> struct Point { x: f64, y: f64 }
>
> // Display — println!("{}", p)로 출력
> impl fmt::Display for Point {
>     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
>         write!(f, "({}, {})", self.x, self.y)
>     }
> }
>
> // From — 타입 변환
> impl From<(f64, f64)> for Point {
>     fn from((x, y): (f64, f64)) -> Self {
>         Point { x, y }
>     }
> }
>
> let p = Point::from((1.0, 2.0));
> let p2: Point = (3.0, 4.0).into();  // From을 구현하면 Into는 자동
> println!("{p}");  // "(1, 2)"
> ```

**🙋 학생**: 다 외워야 해?

> **선생님**: 외울 필요 없고, 자주 만나는 것만 알면 돼요:
>
> | 트레이트 | 역할 | 어디서 쓰나 |
> |---------|------|------------|
> | `Display` | 사용자용 출력 (`{}`) | `println!`, 에러 메시지 |
> | `Debug` | 개발자용 출력 (`{:?}`) | 디버깅, 로그 |
> | `Clone` / `Copy` | 값 복사 | ch05에서 배움 |
> | `From` / `Into` | 타입 변환 | 생성자 대용, 에러 변환 |
> | `Iterator` | 이터레이터 | ch08에서 배움 |
> | `Default` | 기본값 | 설정 구조체 초기화 |
> | `PartialEq` / `Eq` | 동등 비교 | `==`, HashMap 키 |
> | `PartialOrd` / `Ord` | 순서 비교 | 정렬 |
>
> 처음엔 `Debug`, `Clone`, `Display`, `From` 네 개만 기억하세요.

### 💡 핵심
- `Display`: 사용자용 출력, `Debug`: 개발자용 출력
- `From`/`Into`: 타입 변환 (From 구현하면 Into 자동)
- `Clone`/`Copy`: 값 복사 (ch05 소유권과 연결)
- 처음엔 Debug, Clone, Display, From 네 개만

---

## 5. impl Trait과 dyn Trait

> **선생님**: 트레이트를 함수 인자/반환에 쓰는 방법이 두 가지예요.
>
> ```rust
> // 1. impl Trait — 정적 디스패치 (컴파일 타임에 타입 결정)
> fn greet(item: &impl Greet) {
>     println!("{}", item.hello());
> }
>
> // 2. dyn Trait — 동적 디스패치 (런타임에 타입 결정)
> fn greet_dynamic(item: &dyn Greet) {
>     println!("{}", item.hello());
> }
> ```

**🙋 학생**: 뭐가 달라? 둘 다 같은 거 아니야?

> **선생님**: 내부 동작이 달라요:
>
> | | `impl Trait` (정적) | `dyn Trait` (동적) |
> |--|--------------------|--------------------|
> | 타입 결정 | 컴파일 타임 | 런타임 |
> | 성능 | 인라인 최적화 가능, 빠름 | vtable 간접 호출, 약간 느림 |
> | 코드 크기 | 타입마다 복사 (monomorphization) | 하나의 코드 공유 |
> | 유연성 | 하나의 구체 타입만 | 여러 타입 섞기 가능 |
> | C++ 대응 | 템플릿 | virtual function |
>
> ```rust
> // dyn이 필요한 경우 — 여러 타입을 하나의 Vec에
> let greeters: Vec<Box<dyn Greet>> = vec![
>     Box::new(User { name: String::from("철수") }),
>     Box::new(Bot { id: 1 }),
> ];
>
> for g in &greeters {
>     println!("{}", g.hello());
> }
> ```
>
> `impl Trait`으로 안 되는 게 있어요 — 서로 다른 타입을 한 컬렉션에 담을 때는 `dyn Trait`이 필요해요.

**🙋 학생**: 언제 뭘 써야 해?

> **선생님**: 간단한 기준:
> - **기본**: `impl Trait` (성능 좋고 간단)
> - **여러 타입 섞어야 할 때**: `Box<dyn Trait>`
> - **trait object를 저장해야 할 때**: `Box<dyn Trait>`
>
> **90%는 `impl Trait`이면 충분**해요. dyn은 정말 필요할 때만.

### 💡 핵심
- `impl Trait` = 정적 디스패치, 컴파일 타임 결정, 빠름
- `dyn Trait` = 동적 디스패치, 런타임 결정, 유연함
- C++ 비유: impl = 템플릿, dyn = virtual
- 기본은 impl, 여러 타입 섞을 때만 dyn

---

## 6. Ch10 총정리

| 개념 | 한 줄 요약 |
|------|-----------|
| `trait` | 타입의 행동을 정의하는 계약 (Java interface) |
| `impl Trait for Type` | 타입에 trait 구현 |
| `#[derive(...)]` | 컴파일러 자동 구현 (Debug, Clone 등) |
| 트레이트 바운드 `T: Trait` | 제네릭에 "이 능력 필수" 제약 |
| `impl Trait` | 정적 디스패치 (빠름, 90% 사용) |
| `dyn Trait` | 동적 디스패치 (유연함, 필요시만) |
| `From` / `Into` | 타입 변환 trait |
| `Display` / `Debug` | 출력 trait |

> **Rust 설계 철학**: 상속 대신 trait으로 행동을 정의하고 합성(composition)으로 조합. 제네릭 + 트레이트 바운드로 컴파일 타임에 제약을 검사하고, 제로 코스트 추상화로 성능 손해 없이 추상화.
