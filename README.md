# Macbook M1 Air Rust 설치

## 참고

    - VSCode 공식문서 https://code.visualstudio.com/docs/languages/rust
    - MS 공식문서 https://docs.microsoft.com/ko-kr/learn/paths/rust-first-steps/
    - Rust 공식문서 https://www.rust-lang.org/learn/get-started
    - 한국 Rust 사용자 그룹 https://rust-kr.org/pages/install/
    - 자바스크립트와 비교 https://yceffort.kr/2022/02/rust-for-javascript-developer-chapter1
    - 한국어 유튜브 강의 https://www.youtube.com/watch?v=W9DO6m8JSSs&list=PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE&index=1
    - Rust STD https://doc.rust-lang.org/std/index.html

### rustup으로 컴파일러와 패키지 매니저 설치

    https://rustup.rs/

### Rust 버전 확인하기

    rustc --version

### rustup 업데이트

    rustup --update

### VSC Extension

    rust-analyzer, Better TOML, crates, CodeLLDB 설치

### 새 프로젝트 시작하기

    cargo new <프로젝트 이름>

### 러스트 빌드, 실행

    cargo build
    cargo run

### Clippy 실행 ( same as eslint )

    cargo clippy // 찾기만 하는 명령어
    cargo clippy --fix // 찾고 수정까지 해주는 명령어
