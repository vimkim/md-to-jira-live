# Description

CREATE TABLE 구문을 이용하여 VECTOR 타입 컬럼을 갖는 테이블 생성 지원

## VECTOR 타입 컬럼 지원

`CREATE TABLE` 구문을 이용하여 VECTOR 타입 컬럼을 갖는 테이블을 생성할 수 있다.  
예: `CREATE TABLE vt (v VECTOR);`

## 테이블 스키마 조회 가능

테이블 생성 후, 다음 명령어들을 통해 테이블의 VECTOR 타입 컬럼 속성을 조회할 수 있어야 합니다.

- `;sc vt` (테이블 구조 간략 조회)
- `show columns from vt` (테이블 컬럼 상세 조회)
- `desc vt` (테이블 설명 조회)

---

# Specification Changes

- `CREATE TABLE` 시 VECTOR 타입 지원
- csql로 다음 명령 수행하여 VECTOR 타입 관련 스키마 정보 조회 지원:
  - `;sc class_name`
  - `show columns from class_name;`
  - `desc class_name`

---

# Implementation

- `Lexer/Parser`와 `parse_tree*.c` 함수에 `PT_TYPE_VECTOR` 추가 - `CREATE TABLE` 가능
- `DB_TYPE`와 `tp_domain`에 `DB_TYPE_VECTOR`, `tp_VECTOR`, `tp_domain_Vector` 추가 - `desc` 명령 지원

---

# Acceptance Criteria

1. `CREATE TABLE` 구문으로 VECTOR 타입 컬럼을 갖는 테이블 생성 가능한지 확인:
   - `CREATE TABLE vt (v VECTOR);`
   - `CREATE TABLE vt (v VECTOR(1024));`
   - `CREATE TABLE vt (v VECTOR(1024, 32));`
   - `CREATE TABLE vt (c int primary key, v VECTOR(1024, 32));`
2. csql의 다음 명령어들로 위에서 생성한 스키마 정보가 정상 출력됨을 확인:
   - `;sc vt`
   - `show columns from vt;`
   - `desc vt`

- arstarstarst
- arstarstarst
  - arstarst
  - arstarst
    1. arstarst
    1. why, after this, new line?
  - arstarst end
    1. arstartsst why new lines?
- arstarst

---

# Definition of Done

- Acceptance Criteria를 모두 통과.
