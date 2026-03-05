fn main() {
    // 1. 불변 변수 (기본)
    let x = 5;
    println!("x = {x}");
    // x = 10;  // 에러! 기본적으로 변수는 변경 불가

    // 2. 가변 변수 (mut)
    let mut y = 5;
    println!("y = {y}");
    y = 10;
    println!("y 변경 후 = {y}");

    // 3. 상수 (const) — 반드시 타입 명시, 대문자 관례
    const MAX_SCORE: u32 = 100;
    println!("최대 점수: {MAX_SCORE}");

    // 4. 섀도잉 (shadowing) — 같은 이름으로 다시 let 가능
    let z = 5;
    let z = z + 1;       // 이전 z를 가리고 새 z 생성
    let z = z * 2;
    println!("z = {z}"); // 12

    // 5. 기본 타입들
    let integer: i32 = 42;       // 정수 (기본 i32)
    let float: f64 = 3.14;       // 실수 (기본 f64)
    let is_true: bool = true;    // 불리언
    let letter: char = 'A';      // 문자 (작은따옴표)
    let text: &str = "hello";    // 문자열 슬라이스

    println!("정수: {integer}");
    println!("실수: {float}");
    println!("불리언: {is_true}");
    println!("문자: {letter}");
    println!("문자열: {text}");

    // 6. 타입 추론 — Rust가 알아서 추론
    let a = 100;       // i32로 추론
    let b = 3.14;      // f64로 추론
    let c = true;      // bool로 추론
    println!("추론: {a}, {b}, {c}");

    // 7. 튜플과 배열
    let tuple = (1, 2.0, 'a');
    println!("튜플: {}, {}, {}", tuple.0, tuple.1, tuple.2);

    let arr = [1, 2, 3, 4, 5];
    println!("배열 첫번째: {}", arr[0]);
    println!("배열 길이: {}", arr.len());
}
