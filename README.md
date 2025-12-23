# 어둠의전설 M (Legend of Darkness M)

넥슨의 어둠의전설(Legend of Darkness) 스타일의 **온라인 픽셀 RPG 게임**

**Rust + Bevy 0.15 + Axum 0.8 + SQLx 0.8 + PostgreSQL 18**

---

## 🎮 Bevy란?

> **Bevy는 네이티브 데스크톱 게임 엔진입니다.**

| 특징          | 설명                                 |
| ------------- | ------------------------------------ |
| **실행 방식** | 데스크톱 앱 (Windows/Mac/Linux)      |
| **빌드 필요** | `cargo build` 후 실행 파일 실행      |
| **웹 지원**   | WASM 빌드 가능 (별도 설정 필요)      |
| **장점**      | 게임 루프, 스프라이트, 물리엔진 내장 |

---

## 🛑 개발 환경 절대 원칙

> **"Host OS에는 Docker만 설치. 단, 게임 클라이언트는 로컬 Rust로 실행."**

## 🚀 빠른 시작

### 1. 환경 변수 설정

```bash
cp .env.example .env
```

### 2. 서비스 실행 (웹 게임 + API + DB)

모든 개발 및 실행은 Docker 컨테이너 내에서 이루어집니다.

```bash
docker compose up -d
```

접속 정보:

- 🎮 **웹 게임 접속**: [http://localhost:8080](http://localhost:8080)
- 🔌 **API 서버**: [http://localhost:3000](http://localhost:3000)
- 🗄️ **DB 관리자**: [http://localhost:8081](http://localhost:8081)

---

## 🛠️ 개발 및 빌드 가이드 (Dev Container)

개발 환경에 아무것도 설치할 필요 없이 `docker compose` 명령만으로 모든 작업을 수행할 수 있습니다.

### 1. 웹(Web/WASM) 개발 및 실시간 확인

`docker compose up`을 실행하면 이미 웹 서버가 8080 포트에서 실행 중입니다. 코드를 수정하면 자동으로 다시 빌드되어 브라우저에 반영됩니다.

만약 로그를 실시간으로 보거나 독립적으로 실행하고 싶다면:

```bash
docker compose logs -f web
```

### 2. 네이티브(Native) 빌드

데스크톱 실행 파일(Windows/Linux)을 빌드할 때 사용합니다.

```bash
# 네이티브 바이너리 빌드 (target/debug/legend-game)
docker compose run --rm game cargo build --bin legend-game --features client

# 릴리즈 빌드 (최적화 버전)
docker compose run --rm game cargo build --bin legend-game --features client --release
```

### 3. 데이터베이스 관리 (Migration)

DB 스키마를 변경하거나 데이터를 업데이트할 때 사용합니다.

```bash
# 새로운 마이그레이션 파일 생성
docker compose run --rm api sqlx migrate add [이름]

# 마이그레이션 적용
docker compose run --rm api sqlx migrate run
```

---

## 🌐 웹 배포용 정적 빌드

최종 배포를 위한 정적 파일(HTML/JS/WASM)을 생성합니다.

```bash
docker compose run --rm game trunk build --release
```

빌드 결과물은 `dist/` 폴더에 생성됩니다. 이 폴더의 내용물을 정적 웹 서버(Nginx, S3 등)에 올리면 배포가 완료됩니다.

---

## 📁 프로젝트 구조

```
legend/
├── src/
│   ├── game_main.rs          # 🎮 Bevy 게임 엔트리포인트
│   ├── server_main.rs        # 🔌 Axum API 서버 엔트리포인트
│   ├── lib.rs                # 라이브러리 루트
│   │
│   ├── client/               # 🎮 Bevy 게임 클라이언트
│   │   ├── mod.rs            # LegendGamePlugin (메인 플러그인)
│   │   ├── states.rs         # GameState (Loading, Menu, Playing...)
│   │   ├── components.rs     # ECS 컴포넌트들
│   │   ├── resources.rs      # 게임 리소스들 (스프라이트 핸들 포함)
│   │   ├── systems.rs        # 카메라, 에셋 로딩
│   │   ├── ui.rs             # 메뉴, HUD
│   │   └── game.rs           # 월드, 플레이어, 몬스터, 전투
│   │
│   ├── server/               # 🔌 Axum REST API
│   │   ├── auth.rs           # 로그인/회원가입
│   │   ├── db.rs             # DB 연결
│   │   └── monsters.rs       # 몬스터 API
│   │
│   └── shared/               # 📦 공용 모듈
│       ├── domain/           # 도메인 모델
│       │   ├── character/    # Player, PlayerClass, StatType
│       │   ├── monster/      # Monster, MonsterAI, SpriteSize
│       │   ├── item/         # Item, Equipment
│       │   ├── skill/        # Skill
│       │   ├── map/          # GameMap, Tile
│       │   └── shared/       # Stats, Position, Direction
│       │
│       └── data/             # 📊 정적 데이터 상수 (const)
│           ├── mod.rs        # 데이터 모듈 루트
│           ├── assets.rs     # 에셋 경로 상수
│           ├── monsters.rs   # 몬스터 정의 (스탯, 스프라이트, 드롭)
│           ├── characters.rs # 클래스 정의 (기본 스탯, 스프라이트)
│           ├── skills.rs     # 스킬 정의 (데미지, 힐, 버프)
│           ├── items.rs      # 아이템 정의 (무기, 방어구, 소모품)
│           └── maps.rs       # 맵 정의 (타일 레이아웃, 스폰)
│
├── public/assets/            # 게임 에셋
│   ├── characters/           # 캐릭터 스프라이트시트
│   ├── monsters/             # 몬스터 스프라이트시트
│   ├── tiles/                # 타일/빌딩/장식 스프라이트
│   ├── fonts/                # 폰트
│   └── audio/                # BGM, 효과음
│
├── migrations/               # SQLx 마이그레이션
└── compose.yml               # Docker Compose
```

### 📚 관련 문서

| 문서 | 설명 |
|------|------|
| [ASSETS.md](./ASSETS.md) | 에셋 폴더 구조 및 스프라이트 규격 |
| [ASSET_GENERATION.md](./ASSET_GENERATION.md) | 에셋 생성 프롬프트 및 상태 추적 |
| [DESIGN_GUIDELINES.md](./DESIGN_GUIDELINES.md) | 비주얼 스타일 가이드 |

---

## 🛠 기술 스택

| 분류          | 기술           | 버전                 |
| ------------- | -------------- | -------------------- |
| **게임 엔진** | Bevy           | 0.15                 |
| **백엔드**    | Axum           | 0.8 (REST API)       |
| **DBMS**      | PostgreSQL     | 18 (Alpine)          |
| **ORM**       | SQLx           | 0.8                  |
| **언어**      | Rust           | 1.85+ (Edition 2024) |
| **인프라**    | Docker Compose | -                    |

---

## 📊 데이터 아키텍처

### 정적 데이터 (Rust const) vs 동적 데이터 (DB)

| 구분 | 저장 위치 | 데이터 종류 |
|------|----------|------------|
| **정적 (const)** | `src/shared/data/*.rs` | 몬스터, 스킬, 아이템, 맵, 경험치 테이블 |
| **동적 (DB)** | PostgreSQL | 유저, 캐릭터, 인벤토리, 진행도 |

### Characters 테이블 주요 필드

```sql
-- 레벨/경험치 (만렙: 99)
level INT (1-99)
exp BIGINT              -- 현재 레벨 경험치
total_exp BIGINT        -- 누적 경험치

-- 추가 스탯 (기본값은 클래스 const에서)
bonus_str_stat INT      -- 힘 보너스
bonus_dex_stat INT      -- 민첩 보너스
bonus_int_stat INT      -- 지능 보너스
bonus_wis_stat INT      -- 지혜 보너스
bonus_con_stat INT      -- 체력 보너스
stat_points INT         -- 미배분 포인트
```

### 스탯 명명 규칙

모든 스탯 필드는 `_stat` 접미사 사용:

| 필드명 | 설명 |
|--------|------|
| `str_stat` | Strength (힘) - 물리 공격력 |
| `dex_stat` | Dexterity (민첩) - 크리티컬, 회피 |
| `int_stat` | Intelligence (지능) - 마법 공격력 |
| `wis_stat` | Wisdom (지혜) - MP, 마법 방어력 |
| `con_stat` | Constitution (체력) - HP, 물리 방어력 |

### 경험치 시스템

- **만렙**: 99
- **공식**: `exp_to_next = floor(100 * level^1.5)`
- **만렙 총 경험치**: 3,715,799
- **테이블**: `src/shared/data/characters.rs::EXP_TABLE`

---

## 🏗️ 아키텍처

```
┌─────────────────────────────────────────────────────────┐
│              Bevy Game Client (Native App)               │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐    │
│  │ States  │  │   UI    │  │  Game   │  │ Systems │    │
│  │ Machine │  │ (Menu,  │  │ (World, │  │(Camera, │    │
│  │         │  │  HUD)   │  │ Combat) │  │ Input)  │    │
│  └─────────┘  └─────────┘  └─────────┘  └─────────┘    │
└────────────────────────┬────────────────────────────────┘
                         │ HTTP (REST API)
                         ▼
┌─────────────────────────────────────────────────────────┐
│               Axum REST API (Docker)                     │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐                 │
│  │  Auth   │  │Monsters │  │  Items  │                 │
│  │ Handler │  │ Handler │  │ Handler │                 │
│  └─────────┘  └─────────┘  └─────────┘                 │
└────────────────────────┬────────────────────────────────┘
                         │ SQLx
                         ▼
┌─────────────────────────────────────────────────────────┐
│                PostgreSQL 18 (Docker)                    │
└─────────────────────────────────────────────────────────┘
```

---

## 🎯 게임 조작법

| 키                       | 동작               |
| ------------------------ | ------------------ |
| `W` `A` `S` `D` / 화살표 | 이동 (대각선 지원) |
| `Space`                  | 공격               |
| `C`                      | 캐릭터 창 (예정)   |
| `I`                      | 인벤토리 (예정)    |
| `K`                      | 스킬 창 (예정)     |

---

## ✨ Leptos → Bevy 마이그레이션 이유

| Leptos (이전)       | Bevy (현재)                   |
| ------------------- | ----------------------------- |
| UI 프레임워크       | **게임 엔진**                 |
| 게임 루프 직접 구현 | 내장 게임 루프                |
| Canvas 2D 수동 관리 | 스프라이트/렌더링 자동화      |
| 웹 브라우저만 지원  | **네이티브 + WASM 모두 지원** |
| 디버깅 어려움       | 풍부한 디버깅 도구            |

---

## 📋 로드맵

- [x] Bevy로 마이그레이션
- [x] 기본 이동/전투 시스템
- [x] 메인 메뉴 / 직업 선택
- [x] 정적 데이터 const 분리 (몬스터, 스킬, 아이템, 맵)
- [x] 경험치 테이블 (Level 1-99)
- [x] 스프라이트 시스템 기반 구축
- [ ] 스프라이트 애니메이션 (진행 중)
- [ ] 멀티플레이어 (WebSocket)
- [ ] 인벤토리 시스템
- [ ] 스킬 시스템
- [ ] WASM 웹 빌드

---

**Made with 🦀 Rust + 🎮 Bevy 0.15 + 🚀 Axum 0.8 + 🐘 PostgreSQL**
