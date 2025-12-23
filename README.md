# 어둠의전설 M (Legend of Darkness M)

어둠의전설(Legend of Darkness) 스타일의 **완전한 온라인 픽셀 RPG 게임**

**Rust + Leptos 0.8.7 + Axum 0.8.7 + SQLx 0.8 + PostgreSQL 18 apline**

# Dev Container 환경에서 개발합니다. OS레벨에 어떠한 것도 설치하지 않습니다. 볼륨만 OS 로컬에 연결합니다.

> ⚠️ **통합 문서**: 이 문서는 프로젝트의 모든 정보(사용법, 구조, 에셋 가이드, 디자인 가이드)를 포함하는 단일 진실 공급원(SSOT)입니다.

---

## 📖 목차

1. [개요 및 특징](#-개요-및-특징)
2. [기술 스택](#-기술-스택)
3. [시스템 아키텍처](#-시스템-아키텍처)
4. [필수 요구사항 및 설치](#-필수-요구사항-및-설치)
5. [프로젝트 구조](#-프로젝트-구조)
6. [게임 조작법](#-게임-조작법)
7. [에셋 및 디자인 가이드](#-에셋-및-디자인-가이드)
8. [데이터베이스 스키마](#-데이터베이스-스키마)
9. [로드맵](#-로드맵)

---

## 🎮 개요 및 특징

**어둠의전설 M**은 90년대 클래식 RPG의 감성을 현대적인 웹 기술로 재현한 프로젝트입니다.

### 핵심 시스템

- ✅ **5개 직업**: 전사, 도적, 마법사, 성직자, 무도가
- ✅ **1서클 시스템**: 레벨 1-11 성장 구간
- ✅ **틱(Tick) 시스템**: 15초마다 HP/MP 자동 회복 (물약 없음)
- ✅ **전투 시스템**: 아이소메트릭(쿼터뷰) 액션, 타겟팅
- ✅ **경제/아이템**: 몬스터 골드 드롭, 상점, 장비 착용

### 비주얼 & 테마 (다크 판타지)

- 🎨 **시점**: 아이소메트릭 2.5D (대각선 30도)
- 🎨 **스타일**: 90년대 RPG 픽셀 아트 (SD 캐릭터 2등신)
- 🎨 **분위기**: 어둡고, 우울하고, 무겁고, 신비로운 중세 판타지
- 🎨 **금지 사항**: 귀여운/치비 스타일, 밝은 파스텔 톤, 만화 스타일 절대 금지

---

## 🛠 기술 스택

| 분류           | 기술           | 버전            |
| -------------- | -------------- | --------------- |
| **프론트엔드** | Leptos         | 0.8 (CSR)       |
| **백엔드**     | Axum           | 0.8 (REST API)  |
| **DBMS**       | PostgreSQL     | 18 (Alpine)     |
| **ORM**        | SQLx           | 0.8             |
| **언어**       | Rust           | 1.85+ (Nightly) |
| **인프라**     | Docker Compose | -               |

---

## 🏗️ 시스템 아키텍처

이 프로젝트는 **CSR(Client-Side Rendering) + REST API** 구조로 설계되었습니다.

```mermaid
graph LR
    User[Client Browser] -->|Port 8080| Frontend[Frontend (CSR)]
    Frontend -->|REST API (Port 3000/api)| Backend[Backend (Axum)]
    Backend -->|Port 5432| DB[PostgreSQL 18]
```

### 구조적 특징

- **Frontend (CSR)**: Leptos WASM 클라이언트. `trunk`를 통해 서빙됩니다. (Port 8080)
- **Backend (API)**: Axum REST API 서버. 순수 데이터 처리를 담당합니다. (Port 3000)
- **Database**: PostgreSQL 18 Alpine. (Port 5433 Host / 5432 Internal)

---

## 🚀 필수 요구사항 및 설치

**"Host OS에는 아무것도 설치하지 마세요."**

이 프로젝트는 **완전한 컨테이너 기반 개발 환경(Dev Container)**을 지향합니다. 로컬 머신에 Rust, PostgreSQL, Node.js 등을 설치할 필요가 전혀 없습니다.

### 필수 요구사항

- **Docker Only**: Docker Desktop 또는 Docker Engine만 있으면 됩니다.

### 개발 환경 구조 (Dev Container)

`docker compose up` 명령 하나로 다음 환경이 즉시 구축됩니다:

1.  **Frontend Container** (`legend-frontend`)
    - **Port**: 8080
    - **Role**: CSR 클라이언트 개발 서버 (`trunk serve`)
    - **Tools**: Rust Nightly, WASM Target, Trunk
2.  **API Container** (`legend-api`)
    - **Port**: 3000
    - **Role**: Axum REST API 서버 (`cargo run`)
    - **Tools**: Rust Nightly, SQLx CLI
3.  **Database Container** (`legend-db`)
    - **Port**: 5433 (Host) / 5432 (Internal)
    - **Image**: `postgres:18-alpine`
    - **Role**: 게임 데이터 저장소

### 빠른 시작 (Quick Start)

**1. 환경 변수 설정**

```bash
cp .env.example .env
```

_(기본값: `postgres://legend:legend@db:5432/legend`)_

**2. 전체 개발 환경 시작**

```bash
docker compose up --build
```

- 최초 실행 시 Rust 의존성 컴파일로 인해 시간이 소요될 수 있습니다.
- 빌드가 완료되면 터미널에 로그가 출력됩니다.

**3. 접속 정보**

- 🎮 **게임 클라이언트 (CSR)**: [http://localhost:8080](http://localhost:8080)
- 🔌 **API 서버**: [http://localhost:3000](http://localhost:3000)
- 🗄️ **DB 관리자 (Adminer)**: [http://localhost:8081](http://localhost:8081)
- 🐘 **DB 접속 (Local Client)**: `localhost:5433` (User/PW/DB: `legend`)

### 주요 개발 명령어

```bash
# DB만 백그라운드 실행
docker compose up -d db adminer

# API 서버 로그 확인
docker compose logs -f api

# 프론트엔드 로그 확인
docker compose logs -f frontend

# 컨테이너 내부 쉘 접속
docker exec -it legend-dev bash

# (컨테이너 내부) DB 마이그레이션
sqlx migrate run
```

---

## 📁 프로젝트 구조

```
legend/
├── src/
│   ├── main.rs                 # 서버 엔트리 (Axum 0.8)
│   ├── lib.rs                  # 라이브러리/WASM 엔트리
│   ├── app.rs                  # Leptos 앱 라우팅
│   │
│   ├── domain/                 # 도메인 주도 설계(DDD)
│   │   ├── character/          # 캐릭터/직업
│   │   ├── item/               # 아이템/인벤토리
│   │   ├── skill/              # 스킬 로직
│   │   └── map/                # 맵 데이터
│   │
│   ├── game/                   # 게임 클라이언트 로직
│   │   ├── canvas.rs           # 캔버스 렌더링 (Sprite/Tile)
│   │   ├── systems/            # 게임 시스템 (Audio, SpriteLoader)
│   │   └── mod.rs              # 게임 루프
│   │
│   └── components/             # UI 컴포넌트 (HUD, Windows)
│
├── public/
│   └── assets/                 # 게임 리소스 (스프라이트, 오디오)
└── style/                      # CSS (다크 판타지 테마)
```

---

## 🎯 게임 조작법

### 이동 및 전투

- **이동**: `W`, `A`, `S`, `D` 또는 방향키 (대각선/조합키 지원)
- **공격/상호작용**: `Space`

### 인터페이스

- `C`: 캐릭터 창 (스탯/정보)
- `I`: 인벤토리
- `K`: 스킬 창

---

## 🎨 에셋 및 디자인 가이드

이 섹션은 디자이너 및 AI 에이전트가 게임 에셋을 생성하거나 수정할 때 반드시 준수해야 하는 규칙입니다.

### 1. 핵심 디자인 규칙

#### 1타일 규칙 (One-Tile Rule) ⚠️ 중요

> **모든 캐릭터와 몬스터는 시각적 크기와 관계없이 논리적으로 1타일(1칸)만 차지합니다.**

- **적용 범위**: 충돌 판정, 이동 경로, 타겟팅, 스폰 위치 등 모든 게임 로직
- **시각적 처리**: 드래곤(128px)처림 큰 스프라이트는 타일 중심에서 위로 확장되어 그려지지만, 발 밑의 1칸(32x32)이 실제 위치입니다.

#### 색상 팔레트 (다크 판타지)

- **기본/어둠**: `#0a0a0a` (배경), `#1a1a2e`, `#16213e`
- **피/경고**: `#8b0000`, `#660000`
- **마법/신비**: `#4a0080` (보라), `#2d1b4e`
- **금지 색상**: 순수 흰색(#ffffff), 파스텔 핑크/하늘색/민트색

### 2. 스프라이트시트 규격

모든 움직이는 엔티티는 아래의 **행(Row) 기반 레이아웃**을 따릅니다. 프레임 순서는 왼쪽에서 오른쪽입니다.

```
┌────────────────────────────────────────────────────────┐
│ Frame 0  │ Frame 1  │ Frame 2  │ Frame 3  │ ...      │
├────────────────────────────────────────────────────────┤
│ Row 0: IDLE   (대기)     - 4 프레임 (600ms, 호흡)      │
│ Row 1: MOVE   (이동)     - 4 프레임 (400ms, 걷기)      │
│ Row 2: ATTACK (공격)     - 6 프레임 (500ms, 타격)      │
│ Row 3: DEATH  (사망)     - 4 프레임 (600ms, 쓰러짐)    │
└────────────────────────────────────────────────────────┘
```

#### 엔티티별 프레임 크기

| 엔티티 타입     | 프레임 크기 (px) | 시트 크기 (예상) | 비고                        |
| --------------- | ---------------- | ---------------- | --------------------------- |
| **캐릭터**      | 64x64            | 384x256          | 모든 직업 동일              |
| **소형 몬스터** | 32x32            | 192x128          | 슬라임, 쥐 (Lv 1-10)        |
| **중형 몬스터** | 48x48            | 288x192          | 스켈레톤, 고블린 (Lv 11-50) |
| **대형 몬스터** | 64x64            | 384x256          | 엘리트 몬스터 (Lv 51+)      |
| **보스**        | 128x128          | 768x512          | 드래곤 등                   |

### 3. 캐릭터 및 몬스터 디자인 가이드

#### 캐릭터 (직업별 테마)

- **전사**: 무거운 판금 갑옷, 전투로 닳은 느낌
- **마법사**: 어두운 로브, 후드, 빛나는 눈
- **도적**: 가벼운 가죽, 후드 망토, 암살자 미학
- **성직자**: 낡은 성스러운 의복, 우아하지만 어두운 사제 로브
- **무도가**: 맨가슴, 붕대, 흉터, 타이트한 전투복
- **비율**: 성인 신체 비율 (7-8등신), SD 스타일(2등신)을 따르되 치비(Chibi) 비율은 지양

### 4. 기술적 구현 (개발자 참고)

**스프라이트 경로 로딩**

```rust
use crate::client::game::systems::{SpriteLoader, AnimationState};
let path = SpriteLoader::get_character_sprite_path("warrior", "male", AnimationState::Idle);
```

**애니메이션 프레임 계산**

```rust
use crate::client::game::systems::AnimationCalculator;
let frame = AnimationCalculator::get_frame_index(current_time_ms, frame_count, frame_duration_ms);
```

### 5. 폴더 구조

모든 에셋은 `public/assets/` 내에 위치합니다.

- **캐릭터**: `/assets/characters/{class}/{gender}_spritesheet.png`
- **몬스터**: `/assets/monsters/{name}/spritesheet.png`
- **스킬**: `/assets/skills/{class}/{skill_name}_fx.png`
- **타일**: `/assets/tiles/ground/tileset.png`
- **오디오**: `/assets/audio/bgm/`, `/assets/audio/sfx/`

---

## 🗄️ 데이터베이스 스키마 요약

1. **players**: 닉네임, 성별, 직업, 레벨(1-99 서클), HP/MP
2. **classes**: 직업별 기본 스탯 정의
3. **skills**: 직업별 마법/기술 정보 (MP 소모, 데미지)
4. **items**: 무기, 방어구, 악세서리 정보
5. **monsters**: 몬스터 AI 타입 및 능력치

---

## 🎯 로드맵

- [ ] **멀티플레이어**: WebSocket 실시간 동기화
- [ ] **채팅/파티/길드**: 커뮤니티 기능
- [ ] **PVP**: 플레이어 간 전투
- [ ] **사운드 구현**: BGM 및 효과음 시스템 연동
- [ ] **모바일**: 터치 컨트롤 최적화

---

**Made with 🦀 Rust + ⚡ Leptos 0.8 + 🚀 Axum 0.8 + 🐘 PostgreSQL**
