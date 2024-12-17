# 큐브리드 전용 마크업 변환기

> Markdown to Jira Confluence Wiki Markup Converter

![image](https://github.com/user-attachments/assets/63db9d14-de87-4360-a0ec-f1b323557d79)

**마크다운 문서를 큐브리드 지라(CUBRID Jira) 형식으로 변환하는 변환기**

## 📋 프로젝트 개요

이 프로젝트는 마크다운 문서를 **큐브리드 전용 커스텀 마크업** 형식으로 변환해주는 변환기입니다. 노션, GitHub `README.md` 파일, 혹은 기타 마크다운 기반 문서를 **정보 손실 없이** 큐브리드 지라에 최적화된 마크업 형식으로 변환할 수 있도록 제작되었습니다.

이 변환기는 **정확도와 효율성**을 핵심 목표로 삼아 문서 작성 및 이슈 작성의 생산성을 극대화할 수 있습니다.

---

## 🚀 주요 기능

1. **Markdown → 큐브리드 지라 마크업 변환**

   - 마크다운 파일을 입력하면 큐브리드 지라 호환 마크업 형식으로 변환합니다.
   - **Heading**, **리스트**, **코드 블록**, **강조**와 같은 요소를 정확하게 처리합니다.

2. **HTML 렌더링**

   - 동일한 입력으로 HTML 형식도 렌더링하여 브라우저에서 확인할 수 있습니다.

3. **Standalone 서버 배포**
   - Warp 프레임워크를 활용한 **HTTP 서버**를 제공하여 웹 브라우저에서 변환 결과를 실시간으로 확인할 수 있습니다.
   - 기본 포트: `3030`

---

## 🛠️ 구현 방식

이 프로젝트는 다음과 같은 기술 스택과 라이브러리를 사용하여 구현되었습니다:

- **Rust**: 고성능 서버 및 마크업 변환 로직 구현
- **pulldown_cmark**: 마크다운 파서를 통해 입력을 분석 및 처리
- **Warp**: 경량 웹 서버 프레임워크를 사용하여 HTTP 요청 처리
- **HTML 템플릿 렌더링**: 동적 HTML 결과를 제공 (템플릿 내 변수 치환)
- **Tokio**: 비동기 서버 처리 및 효율적인 I/O 관리

---

## 📂 파일 구조

```plaintext
.
├── Cargo.toml               # 프로젝트 설정 및 의존성
├── src
│   ├── main.rs              # 서버 및 변환기 로직
│   └── markdown_to_confluence.rs  # 핵심 변환 로직
├── main.md                  # 테스트용 마크다운 입력 파일
└── index.html               # 결과 HTML 템플릿
```

---

## 🖥️ 사용 방법

### 1. 설치 및 실행

#### 빌드 및 실행 (Rust가 설치되어 있는 경우)

```bash
# 프로젝트 클론
git clone https://github.com/vimkim/md-to-jira-live.git
cd md-to-jira-live

# 빌드
cargo build --release

# 실행
./target/release/md-to-jira-live
```

#### 실행 시 웹 브라우저에서 확인

기본 포트는 `3030`입니다. 다음 URL로 접근하세요:

```
http://127.0.0.1:3030
```

---

### 2. 변환 파일 준비

프로젝트 루트에 `main.md` 파일을 준비하세요. 이 파일이 입력으로 사용됩니다.

**예시 `main.md` 내용:**

````markdown
# 제목입니다

**강조된 텍스트**

1. 첫 번째 항목
2. 두 번째 항목

```rust
fn main() {
    println!("Hello, world!");
}
```
````

---

### 3. 결과 확인

localhost:3030에 접속하여 변환 결과를 확인하세요.

---

## 🧩 커스텀 마크업 변환 예시

| **마크다운**        | **큐브리드 지라 마크업**        |
| ------------------- | ------------------------------- |
| `# 제목입니다`      | `h1. 제목입니다`                |
| `**강조된 텍스트**` | `*강조된 텍스트*`               |
| `1. 첫 번째 항목`   | `# 첫 번째 항목`                |
| 코드 블록           | `{code:language=rust}...{code}` |
| 인라인 코드 `\`x\`` | `{{x}}`                         |

---

## ⚙️ 옵션 및 향후 업데이트

1. **포트 설정 기능**: 서버 실행 시 커스텀 포트를 지정할 수 있도록 옵션 추가 예정입니다.
2. **Windows 및 macOS 지원**: 현재 리눅스 환경에서만 테스트되었으며, 다른 OS 환경에서도 검증을 진행할 예정입니다.

---

## 🤝 협업 및 테스트 요청

- **문서 변환 요청**: 테스트하고 싶으신 문서(예: 노션 필기, GitHub README 등)를 공유해주시면 제가 직접 변환해드립니다.
- **개선 의견**: 버그나 추가 기능에 대한 의견이 있으시면 [Issues](https://github.com/vimkim/md-to-jira-live/issues)로 알려주세요.

---

## 📝 라이선스

이 프로젝트는 **MIT 라이선스**를 따릅니다.

---

## 🙏 기여 및 지원

팀의 업무에 도움이 되기를 바랍니다. 셋업 및 사용에 어려움이 있다면 언제든지 도와드리겠습니다.
문의사항이나 추가 요청 사항은 [GitHub](https://github.com/vimkim/md-to-jira-live)를 통해 전달해 주세요.

감사합니다! 😊
