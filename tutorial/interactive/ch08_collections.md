# Ch08: 컬렉션 — 대화형 튜토리얼

---

## 1. Vec<T> — 가변 길이 배열

> **선생님**: ch02에서 배열은 길이가 고정이라고 했죠? `Vec`은 **길이가 늘어나는 배열**이에요.
>
> ```rust
> let mut nums: Vec<i32> = Vec::new();  // 빈 Vec 생성
> nums.push(1);
> nums.push(2);
> nums.push(3);
> println!("{:?}", nums);  // [1, 2, 3]
>
> // 매크로로 간편 생성
> let nums = vec![1, 2, 3];  // 위와 같은 결과
> ```

**🙋 학생**: C++의 `std::vector`랑 같은 거야?

> **선생님**: 거의 같아요! 내부적으로 힙에 연속 메모리를 할당하고, 꽉 차면 더 큰 메모리로 이동해요.
>
> | | C++ vector | Java ArrayList | Rust Vec |
> |--|-----------|----------------|----------|
> | 접근 | `v[i]` 또는 `v.at(i)` | `list.get(i)` | `v[i]` 또는 `v.get(i)` |
> | 범위 초과 | `at()`: 예외 / `[]`: UB | 예외 | `[]`: panic / `get()`: Option |
> | 소유권 | 없음 | GC | Vec이 원소를 소유 |
>
> Rust의 `v[i]`는 범위 밖이면 **panic**(프로그램 중단)이고, `v.get(i)`는 **Option**을 반환해요.
>
> ```rust
> let v = vec![10, 20, 30];
> println!("{}", v[0]);        // 10
> println!("{:?}", v.get(5));  // None (panic 안 남)
> ```

### 💡 핵심
- `Vec<T>` = 가변 길이 배열, 힙에 저장
- `vec![1, 2, 3]` 매크로로 간편 생성
- `v[i]`는 범위 밖이면 panic, `v.get(i)`는 Option 반환
- C++ vector, Java ArrayList와 같은 역할

---

## 2. Vec과 소유권

> **선생님**: Vec에 값을 넣으면 **소유권이 Vec으로 이동**해요. ch05 복습이에요.
>
> ```rust
> let name = String::from("철수");
> let mut names = Vec::new();
> names.push(name);
> // println!("{name}");  // 에러! name은 Vec으로 이동됨
> ```

**🙋 학생**: for로 순회할 때도 소유권이 이동돼?

> **선생님**: 그래서 Rust에서 순회 방법이 세 가지예요.
>
> ```rust
> let names = vec![String::from("철수"), String::from("영희")];
>
> // 1. &로 빌려서 순회 (가장 많이 씀)
> for name in &names {
>     println!("{name}");  // 읽기만
> }
> println!("{:?}", names);  // OK! names 여전히 유효
>
> // 2. &mut로 수정하며 순회
> let mut nums = vec![1, 2, 3];
> for n in &mut nums {
>     *n *= 2;  // 각 원소 2배
> }
>
> // 3. 소유권 이동 (순회 후 Vec 사용 불가)
> for name in names {
>     println!("소비: {name}");
> }
> // println!("{:?}", names);  // 에러! 소유권 이동됨
> ```

**🙋 학생**: `&names`만 기억하면 되겠네?

> **선생님**: 맞아요, **90%는 `for item in &vec`으로 충분**해요. 수정하려면 `&mut`, 소비하려면 그냥 `vec`. ch05의 `&T`, `&mut T`, `T` 규칙이 그대로예요.

### 💡 핵심
- Vec에 push하면 소유권 이동
- `for x in &vec` = 읽기 빌림 (가장 많이 씀)
- `for x in &mut vec` = 수정 빌림
- `for x in vec` = 소유권 이동 (Vec 소비)

---

## 3. String — 생각보다 복잡한 문자열

> **선생님**: Rust에서 문자열은 두 가지예요:
>
> ```rust
> let s1: &str = "hello";              // &str — 문자열 슬라이스 (빌림)
> let s2: String = String::from("hello"); // String — 소유 문자열 (힙)
> ```
>
> | | `&str` | `String` |
> |--|--------|---------|
> | 소유권 | 빌림 (참조) | 소유 |
> | 크기 | 고정 | 가변 (push 가능) |
> | 저장 위치 | 어디든 (보통 스택/바이너리) | 힙 |
> | 비유 | `&[T]` (슬라이스) | `Vec<T>` |

**🙋 학생**: 왜 `String[0]`으로 첫 글자를 못 가져와?

> **선생님**: Rust의 String은 **UTF-8**이거든요. 한글은 한 글자가 3바이트예요.
>
> ```rust
> let s = String::from("안녕");
> // s[0]  // 에러! 뭘 원하는 거야? 바이트? 문자?
>
> // 바이트로 접근
> println!("{}", s.as_bytes()[0]);  // 236 (첫 번째 바이트)
>
> // 문자로 순회
> for c in s.chars() {
>     println!("{c}");  // '안', '녕'
> }
> ```
>
> `s[0]`이 O(1)이어야 하는데, UTF-8에서는 n번째 **문자**를 찾으려면 처음부터 세야 해요. 그래서 Rust는 **인덱싱 자체를 막아서** 성능 함정을 예방한 거예요.

**🙋 학생**: 문자열 합치기는 어떻게 해?

> **선생님**: 여러 방법이 있어요:
>
> ```rust
> let hello = String::from("hello");
> let world = String::from(" world");
>
> // 1. format! 매크로 (가장 깔끔)
> let s = format!("{hello}{world}");
>
> // 2. push_str
> let mut s = hello;
> s.push_str(" world");
>
> // 3. + 연산자 (소유권 주의!)
> let s = hello + &world;  // hello는 이동됨, world는 빌림
> ```
>
> **`format!`을 추천해요.** 소유권 걱정 없이 깔끔하게 합칠 수 있어요.

### 💡 핵심
- `&str` = 빌림(참조), `String` = 소유(힙)
- String은 UTF-8이라 인덱싱(`s[0]`) 불가
- `s.chars()`로 문자 순회, `s.as_bytes()`로 바이트 접근
- 문자열 합치기는 `format!` 추천

---

## 4. HashMap — 키-값 저장소

> **선생님**: Java의 `HashMap`이랑 같은 거예요.
>
> ```rust
> use std::collections::HashMap;
>
> let mut scores = HashMap::new();
> scores.insert("철수", 90);
> scores.insert("영희", 85);
>
> // 값 가져오기 — Option 반환!
> match scores.get("철수") {
>     Some(score) => println!("철수: {score}"),
>     None => println!("없음"),
> }
>
> // 순회
> for (name, score) in &scores {
>     println!("{name}: {score}");
> }
> ```

**🙋 학생**: `get`이 왜 Option이야? Java는 그냥 null 반환하잖아.

> **선생님**: ch07에서 배웠죠 — Rust에 null 없어요! 키가 없을 수 있으니 `Option`으로 표현하는 거예요. 덕분에 None 처리를 강제받아서 **KeyNotFoundException 같은 런타임 에러가 안 나요.**

> **선생님**: `entry` API도 유용해요 — "없으면 넣고, 있으면 무시":
>
> ```rust
> let mut scores = HashMap::new();
> scores.entry("철수").or_insert(90);  // 없으면 90 삽입
> scores.entry("철수").or_insert(100); // 이미 있으니 무시
> println!("{:?}", scores);  // {"철수": 90}
>
> // 카운팅 패턴
> let text = "hello world";
> let mut counts = HashMap::new();
> for c in text.chars() {
>     let count = counts.entry(c).or_insert(0);
>     *count += 1;
> }
> ```

### 💡 핵심
- `HashMap<K, V>` = 키-값 저장소, `use std::collections::HashMap`
- `get()`은 `Option` 반환 (null 없음)
- `entry().or_insert()` = 없으면 넣기 패턴
- String을 키로 쓰면 소유권 이동 (& str은 빌림)

---

## 5. 이터레이터 맛보기

> **선생님**: Rust는 **함수형 프로그래밍 스타일**의 이터레이터를 강력하게 지원해요.
>
> ```rust
> let nums = vec![1, 2, 3, 4, 5];
>
> // for 대신 이터레이터 체인
> let doubled: Vec<i32> = nums.iter()
>     .filter(|&&x| x > 2)  // 2보다 큰 것만
>     .map(|&x| x * 2)      // 2배
>     .collect();            // Vec으로 모음
>
> println!("{:?}", doubled);  // [6, 8, 10]
> ```

**🙋 학생**: for 루프로 해도 되는데 왜 이렇게 써?

> **선생님**: 두 가지 이유예요:
>
> 1. **의도가 명확**: "필터하고 변환한다"가 코드에 바로 보임
> 2. **성능이 같거나 더 좋음**: Rust 이터레이터는 **제로 코스트 추상화**예요. 컴파일러가 for 루프와 같은 기계어로 만들어줘요.
>
> 자주 쓰는 메서드:
>
> | 메서드 | 역할 | 예시 |
> |--------|------|------|
> | `.iter()` | 읽기 이터레이터 | `v.iter()` |
> | `.map()` | 각 원소 변환 | `.map(\|x\| x * 2)` |
> | `.filter()` | 조건에 맞는 것만 | `.filter(\|x\| x > 0)` |
> | `.collect()` | 결과를 컬렉션으로 | `.collect::<Vec<_>>()` |
> | `.sum()` | 합계 | `.sum::<i32>()` |
> | `.count()` | 개수 | `.count()` |
> | `.find()` | 첫 번째 매칭 | `.find(\|x\| x > 3)` → Option |
>
> 처음엔 for 루프 쓰다가, 익숙해지면 이터레이터 체인으로 바꿔보세요.

### 💡 핵심
- `.iter().map().filter().collect()` = 이터레이터 체인
- 제로 코스트 추상화 — for 루프와 성능 동일
- 의도가 코드에 드러남 (선언적 스타일)
- 처음엔 for 루프, 익숙해지면 이터레이터

---

## 6. Ch08 총정리

| 개념 | 한 줄 요약 |
|------|-----------|
| `Vec<T>` | 가변 길이 배열, `vec![]` 매크로로 생성 |
| `for x in &vec` | 읽기 빌림 순회 (가장 많이 씀) |
| `String` | 소유 문자열 (UTF-8, 인덱싱 불가) |
| `&str` | 문자열 빌림 (참조) |
| `HashMap<K, V>` | 키-값 저장소, `get()`은 Option 반환 |
| `entry().or_insert()` | 없으면 넣기 패턴 |
| 이터레이터 체인 | `.iter().map().filter().collect()` |

> **Rust 설계 철학**: 컬렉션에서도 소유권 규칙이 일관되게 적용. null 대신 Option, 예외 대신 Result. 이터레이터는 제로 코스트 추상화로 성능 손해 없이 표현력을 높임.
