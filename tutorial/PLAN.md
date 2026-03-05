# Rust 실무 학습 플랜 (4주)

## 최종 목표 (Vision)
Rust로 DPS 프로세스 구축: wasmtime으로 hwp.wasm을 호스팅하고, tonic gRPC 서버로 QueryService와 통신하는 바이너리 개발

---

## 아키텍처 전체 상 (옴니포커스 — 먼저 머릿속에 그리기)

```
[QueryService (WebFlux)]
        │
        │ gRPC (protobuf)
        ▼
[DPS 프로세스 (Rust Binary)]
  ├── tonic gRPC Server     ← 요청 수신/응답
  ├── tokio async runtime   ← 비동기 처리
  └── wasmtime 런타임        ← WASM 실행 엔진
       ├── hwp.wasm #1 (메모리 샌드박스)
       └── hwp.wasm #2 (격리된 인스턴스)
```

### wasmtime 내부 구조 (개념 자르기)
```
Engine (설정/컴파일러)
  └── Module (컴파일된 .wasm)
       └── Store (상태 컨테이너)
            └── Instance (실행 인스턴스)
                 ├── Memory (선형 메모리 = 샌드박스)
                 └── Func (호출 가능한 함수)
```

### 핵심 크레이트 맵
| 크레이트 | 역할 | 우선순위 |
|---------|------|---------|
| `wasmtime` | WASM 런타임, 모듈 로드/실행 | ★★★ |
| `tonic` | gRPC 서버/클라이언트 | ★★★ |
| `tokio` | 비동기 런타임 (tonic 기반) | ★★★ |
| `prost` | protobuf 코드 생성 | ★★☆ |
| `anyhow`/`thiserror` | 에러 처리 | ★★☆ |
| `tracing` | 로깅/디버깅 | ★☆☆ |

---

## 시간 배분 원칙 (뇌과학 루틴)

```
[평일 낮 — 업무 중]  인풋: 문서 읽기, 아키텍처 구조 파악, 개념 이해
[평일 저녁 1시간]    아웃풋: 직접 코드 타이핑, cargo run 확인
[주말 2~3시간]       PR/TR 집중: 깊은 실습, 에러 분석, 실전 구현
```

> 낮에 읽은 것을 저녁에 치면 → 수면 중 해마가 기억 통합 → 다음날 업무 중 문서 읽을 때 더 잘 이해됨

---

## 1주차: Rust 기초 문법 속성

**주중 낮(업무)**: Rust Book 해당 챕터 훑어보기 (인풋)
**주중 저녁(1시간)**: practice.rs에 직접 타이핑 (아웃풋)

| 일차 | 낮 (인풋) | 저녁 1시간 (아웃풋) |
|------|----------|-------------------|
| 월 | ch02 변수/타입 문서 읽기 | study.md 보며 practice.rs 작성 → cargo run |
| 화 | ch03 함수 + ch04 제어흐름 읽기 | 함수 3개 + for/while 각 1개 직접 구현 |
| 수 | ch05 소유권 문서 읽기 (핵심!) | 이동/빌림/참조 코드 작성, 일부러 에러 내보기 |
| 목 | ch06 구조체 + ch07 열거형 읽기 | struct + impl, enum + match 각 1개 구현 |
| 금 | ch08~10 (컬렉션, 에러, 트레이트) 읽기 | Result + ?, Vec + HashMap 예제 각 1개 구현 |

**주말 (2~3시간)**:
- PR: 소유권 에러 3가지를 일부러 만들고 컴파일러 메시지 분석 (시간 무제한)
- TR: 1주차 전체 내용으로 "간단한 학생 성적 관리 프로그램" 60분 안에 구현
  - struct, enum, Vec, match, Result 모두 사용

> **C++ 경험자 PR 포인트**: 소유권은 C++의 unique_ptr/shared_ptr과 비교하며 이해

---

## 2주차: tokio 비동기 + wasmtime 기초

**주중 낮(업무)**: wasmtime 공식 문서, tokio 튜토리얼 조사
**주중 저녁(1시간)**: 짧은 예제 직접 구현

| 일차 | 낮 (인풋) | 저녁 1시간 (아웃풋) |
|------|----------|-------------------|
| 월 | tokio 공식 튜토리얼 구조 파악 | async fn + .await 기본 예제 작성 |
| 화 | tokio::spawn, 채널 문서 읽기 | spawn으로 동시 작업 2개 실행하는 코드 작성 |
| 수 | wasmtime 공식 문서: Engine→Module→Store→Instance 흐름 파악 | wasmtime hello world: .wat 로드 → 함수 호출 |
| 목 | wasmtime Memory, fuel, 리소스 제한 문서 읽기 | 메모리 제한 설정 + OOM 발생시키기 실험 |
| 금 | WASI 인터페이스 문서 조사 | wasi 설정 추가하여 .wasm 실행 |

**주말 (2~3시간)**:
- PR: wasmtime으로 .wasm 2개 동시 로드 → 메모리 격리 확인 (시간 무제한)
- TR: "wasmtime으로 wasm 로드 + 함수 호출 + 결과 출력" 처음부터 60분 안에 구현

---

## 3주차: tonic gRPC + 통합

**주중 낮(업무)**: tonic 문서, protobuf 스키마 설계 검토
**주중 저녁(1시간)**: gRPC 서버 점진적 구축

| 일차 | 낮 (인풋) | 저녁 1시간 (아웃풋) |
|------|----------|-------------------|
| 월 | tonic 공식 예제, .proto 문법 파악 | tonic hello world: .proto 정의 → 서버 구현 |
| 화 | DPS용 .proto 스키마 설계 (업무 겸) | 설계한 .proto로 서버 스켈레톤 생성 |
| 수 | tonic + wasmtime 통합 방법 조사 | gRPC 요청 → wasmtime 호출 → 응답 반환 코드 작성 |
| 목 | 에러 처리 패턴 조사 (anyhow, tonic::Status) | wasmtime 에러 → gRPC Status 변환 구현 |
| 금 | 동시 요청 처리 방법 조사 | tokio + wasmtime 인스턴스 풀 기초 구현 |

**주말 (2~3시간)**:
- PR: gRPC → wasmtime 전체 파이프라인 에러 케이스 분석
- TR: 전체 서버를 처음부터 다시 구현 (시간 제한 2시간)

---

## 4주차: 프로토타입 완성

**주중 낮(업무)**: 실제 hwp.wasm 관련 WASI 인터페이스 검토
**주중 저녁(1시간)**: 실무 코드 다듬기

| 일차 | 낮 (인풋) | 저녁 1시간 (아웃풋) |
|------|----------|-------------------|
| 월 | hwp.wasm 실제 WASI 요구사항 분석 | hwp.wasm 로드 테스트 |
| 화 | 메모리/fuel 튜닝 조사 | 리소스 제한 설정 최적화 |
| 수 | Docker/Pod 배포 방법 조사 | Dockerfile 작성 + 빌드 테스트 |
| 목 | 성능 벤치마크 방법 조사 | Node.js vs Rust(wasmtime) 비교 측정 |
| 금 | 결과 정리 | 팀 공유용 문서 작성 |

**주말 (2~3시간)**:
- 최종 코드 정리 + 엣지 케이스 테스트
- 전체 회고 및 추가 과제 정리

---

## 2분 법칙 (시작이 어려울 때)

저녁에 피곤해서 시작하기 싫으면:
> "일단 `cargo new` 하나만 치자" 또는 "practice.rs 열고 fn main() 만 적자"

시작만 하면 뇌가 각성 → 몰입으로 이어짐

---

## 지금 당장 할 첫 번째 행동

ch02_variables의 `study.md`를 열고, practice.rs에 불변 변수 3줄만 타이핑 → `cargo run --bin practice`
