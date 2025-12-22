# Legend of Darkness 에셋 리펙토링 완료 보고서

## 🎯 작업 요약

레거시 구조를 삭제하고 **스프라이트시트 기반**의 새로운 시스템으로 완전히 통합했습니다.

---

## ✅ 완료된 작업

### 1. 레거시 에셋 삭제

다음 레거시 파일들이 삭제되었습니다:
- `/assets/characters/*.png` (루트 레벨 캐릭터 이미지)
- `/assets/monsters/*.png` (루트 레벨 몬스터 이미지)
- `/assets/tiles/*.png` (루트 레벨 타일 이미지)
- `/assets/*.png` (루트 레벨 기타 이미지)

### 2. 새 에셋 구조 적용

```
assets/
├── characters/{class}/{gender}_spritesheet.png
├── monsters/{type}/spritesheet.png
├── skills/{class}/{skill}_fx.png
├── tiles/{ground,buildings,decorations}/
└── audio/{bgm,sfx}/
```

### 3. 코드 리팩토링

#### sprite_loader.rs
- `AnimationState` enum (Idle, Move, Attack, Death)
- `SpriteSheetInfo` - 프레임 크기 및 레이아웃
- `AnimationCalculator` - 프레임 계산
- `TileRenderer` - **1타일 규칙** 구현
- 새 경로 생성 함수들

#### canvas.rs
- 완전히 새로 작성
- 스프라이트시트 기반 렌더링
- 다크 판타지 색상 팔레트 적용
- 1타일 규칙 준수

#### map_renderer.rs
- 새 타일셋 구조 지원
- 대체 렌더링 (에셋 없을 때)
- 다크 판타지 색상

#### audio.rs
- BGM/SFX 관리
- 볼륨 조절
- 음소거 토글

### 4. 문서화 업데이트

- `DESIGN_GUIDELINES.md` - 1타일 규칙, 스프라이트시트 규격, 다크 판타지 가이드
- `public/assets/README.md` - AI 참조용 상세 가이드
- `public/assets/manifest.json` - 에셋 메타데이터

---

## 📐 핵심 규칙

### 1타일 규칙

> **모든 캐릭터와 몬스터는 시각적 크기와 관계없이 논리적으로 1타일만 차지합니다.**

```rust
// TileRenderer::get_draw_position() 사용
let (draw_x, draw_y) = TileRenderer::get_draw_position(
    entity.position.x,
    entity.position.y,
    sprite_width,
    sprite_height
);
```

### 스프라이트시트 레이아웃

```
Row 0: IDLE   (4 프레임, 150ms/프레임)
Row 1: MOVE   (4 프레임, 100ms/프레임)
Row 2: ATTACK (6 프레임, 83ms/프레임)
Row 3: DEATH  (4 프레임, 150ms/프레임)
```

---

## ⚠️ 필요한 에셋 파일

현재 에셋 파일이 비어 있습니다. 다음 파일들을 생성해야 합니다:

### 캐릭터 스프라이트시트
| 경로 | 크기 |
|------|------|
| `/assets/characters/warrior/male_spritesheet.png` | 384x256 px |
| `/assets/characters/warrior/female_spritesheet.png` | 384x256 px |
| `/assets/characters/mage/male_spritesheet.png` | 384x256 px |
| `/assets/characters/mage/female_spritesheet.png` | 384x256 px |
| `/assets/characters/rogue/male_spritesheet.png` | 384x256 px |
| `/assets/characters/rogue/female_spritesheet.png` | 384x256 px |
| `/assets/characters/cleric/male_spritesheet.png` | 384x256 px |
| `/assets/characters/cleric/female_spritesheet.png` | 384x256 px |
| `/assets/characters/martial_artist/male_spritesheet.png` | 384x256 px |
| `/assets/characters/martial_artist/female_spritesheet.png` | 384x256 px |

### 몬스터 스프라이트시트
| 경로 | 크기 | 카테고리 |
|------|------|---------|
| `/assets/monsters/slime/spritesheet.png` | 192x128 px | 소형 |
| `/assets/monsters/rat/spritesheet.png` | 192x128 px | 소형 |
| `/assets/monsters/bat/spritesheet.png` | 192x128 px | 소형 |
| `/assets/monsters/wolf/spritesheet.png` | 288x192 px | 중형 |
| `/assets/monsters/skeleton/spritesheet.png` | 288x192 px | 중형 |
| `/assets/monsters/goblin/spritesheet.png` | 288x192 px | 중형 |
| `/assets/monsters/ghost/spritesheet.png` | 288x192 px | 중형 |
| `/assets/monsters/dragon/spritesheet.png` | 768x512 px | 보스 |

### 타일셋
| 경로 | 설명 |
|------|------|
| `/assets/tiles/ground/tileset.png` | 바닥 타일 (5종) |
| `/assets/tiles/buildings/buildings.png` | 건물 (4종) |
| `/assets/tiles/decorations/torch.png` | 횃불 |

### 오디오
| 경로 | 설명 |
|------|------|
| `/assets/audio/bgm/village.mp3` | 마을 BGM |
| `/assets/audio/bgm/dungeon.mp3` | 던전 BGM |
| `/assets/audio/sfx/attack/sword_swing.mp3` | 칼 휘두르기 |
| `/assets/audio/sfx/attack/hit_flesh.mp3` | 타격음 |

---

## 🎨 스프라이트 생성 프롬프트 예시

### 캐릭터 (예: 전사)
```
Dark fantasy isometric 2.5D pixel art spritesheet for a male Warrior character.
64x64 pixels per frame, 6 columns x 4 rows layout.

Row 0: Idle animation (4 frames) - subtle breathing motion
Row 1: Move animation (4 frames) - walking cycle
Row 2: Attack animation (6 frames) - heavy sword slash
Row 3: Death animation (4 frames) - falling down

Style: 90s RPG, medieval dark fantasy, adult proportions.
Colors: Deep blacks, dark steel, crimson accents.
Transparent background PNG.
```

### 몬스터 (예: 슬라임)
```
Dark fantasy isometric 2.5D pixel art spritesheet for a Slime monster.
32x32 pixels per frame, 6 columns x 4 rows layout.

Row 0: Idle (4 frames) - jiggling
Row 1: Move (4 frames) - sliding
Row 2: Attack (6 frames) - lunging
Row 3: Death (4 frames) - melting

Style: Threatening, sickly green, semi-transparent, oozing.
NOT cute. Make it unsettling.
Transparent background PNG.
```

---

## 📋 AI 에이전트 참조

에셋 생성 전 반드시 확인:
1. `/DESIGN_GUIDELINES.md` - 디자인 규칙
2. `/public/assets/README.md` - 에셋 가이드
3. `/public/assets/manifest.json` - 에셋 경로

---

_생성일: 2024-12-22_
_버전: 2.0 (스프라이트시트 시스템)_
