# Rust 코드 설명

## 1. `markdown_to_confluence` 함수

마크다운 문서를 **큐브리드 지라 마크업** 형식으로 변환합니다.

```rust
let parser = Parser::new(input);
```

- **역할**: `pulldown_cmark`의 `Parser`를 이용해 입력된 Markdown 문자열을 이벤트로 분해합니다.

```rust
Tag::Heading { level, .. } => {
    let heading_level = match level {
        HeadingLevel::H1 => "h1.",
        HeadingLevel::H2 => "h2.",
        HeadingLevel::H3 => "h3.",
        _ => "h6.",
    };
    output.push_str(&format!("\n\n{} ", heading_level));
}
```

- **역할**: 헤딩 레벨(H1 ~ H6)을 큐브리드 지라 형식(`h1.`, `h2.`)으로 변환합니다.

```rust
Tag::List(Some(_)) => list_stack.push(true); // 순서 있는 리스트
Tag::List(None) => list_stack.push(false);  // 순서 없는 리스트
```

- **역할**: 리스트의 타입(순서 있는 리스트와 없는 리스트)을 스택에 저장합니다.

```rust
Tag::CodeBlock(CodeBlockKind::Fenced(lang)) => {
    let l = if lang.as_ref() == "plaintext" { CowStr::from("sh") } else { lang };
    output.push_str(&format!("\n{{code:language={}}}\n", l));
}
```

- **역할**: 코드 블록을 `{code:language=LANG}` 형식으로 변환합니다. 기본 언어는 `sh`로 설정됩니다.

```rust
Event::Text(text) => output.push_str(&text);
```

- **역할**: 텍스트 내용을 결과 문자열에 추가합니다.

---

## 2. 서버 실행 부분

**Warp 프레임워크**를 사용하여 웹 서버를 실행합니다.

```rust
warp::serve(markdown_route)
    .run(([127, 0, 0, 1], 3030))
    .await;
```

- **역할**: `127.0.0.1:3030` 주소에서 HTTP 서버를 시작합니다.

---

## 3. 동적 템플릿 처리

`index.html` 템플릿에 변환된 결과를 삽입합니다.

```rust
html_template = html_template
    .replace("{{ rendered_html }}", &rendered_html)
    .replace("{{ confluence_content }}", &confluence_content);
```

- **역할**: HTML 템플릿의 특정 플레이스홀더(`{{ ... }}`)를 변환된 결과로 치환합니다.
