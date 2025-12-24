# 🎮 게임 규격 및 크기 설정 가이드

## 📋 목차
1. [문제 상황](#문제-상황)
2. [전체 규격 개요](#전체-규격-개요)
3. [타일 시스템](#타일-시스템)
4. [캐릭터 규격](#캐릭터-규격)
5. [몬스터 규격](#몬스터-규격)
6. [스프라이트시트 레이아웃](#스프라이트시트-레이아웃)
7. [실제 적용 방법](#실제-적용-방법)
8. [체크리스트](#체크리스트)

---

## 🚨 문제 상황

현재 게임에서 발생하고 있는 문제들:

### 1. 크기 불일치
- **증상**: 몬스터가 캐릭터보다 훨씬 크게 표시됨 (특히 Giant Rat)
- **원인**: 
  - 스프라이트시트는 1024x1024 크기로 생성됨
  - 각 프레임은 256x256 크기로 잘라냄
  - 하지만 실제 스프라이트 내용물은 32~128px 크기
  - 투명 여백이 포함된 256x256 전체를 렌더링하면 너무 큼

### 2. 투명도 문제
- **증상**: 스프라이트 주변에 검은색/체크무늬 배경 보임
- **원인**: 
  - Alpha channel이 없거나 불완전
  - PNG colorkey 방식 사용 (WebP alpha 필요)
  - 스프라이트 rect 설정이 잘못됨

### 3. 타일 정렬 문제
- **증상**: 타일들이 제대로 연결되지 않고 흩어짐
- **원인**: Isometric projection이 일부 타일에만 적용됨

---

## 📐 전체 규격 개요

```
┌─────────────────────────────────────────────────┐
│  게임 전체 크기 체계 (모두 픽셀 단위)             │
├─────────────────────────────────────────────────┤
│  타일              : 64 x 32   (2:1 ratio)      │
│  캐릭터 (모든 클래스): 48 x 64                    │
│  몬스터 Small      : 32 x 32                    │
│  몬스터 Medium     : 48 x 48                    │
│  몬스터 Large      : 64 x 64                    │
│  몬스터 Boss       : 128 x 128                  │
│  스프라이트시트     : 1024 x 1024               │
│  프레임 그리드     : 4 x 4 = 256px/프레임       │
└─────────────────────────────────────────────────┘
```

### 중요 원칙

1. **스프라이트시트 vs 실제 스프라이트 크기**
   ```
   스프라이트시트: 1024x1024
   ├─ 프레임 그리드: 4x4 (256x256 각 칸)
   └─ 실제 스프라이트: 중앙에 배치된 실제 크기
      ├─ 캐릭터: 48x64 (256x256 칸 내부 중앙)
      ├─ Small: 32x32 (256x256 칸 내부 중앙)
      └─ Boss: 128x128 (256x256 칸 내부 중앙)
   ```

2. **렌더링 순서**
   ```
   Z-Index 구조:
   0     : 타일
   1     : 그림자
   5-99  : 캐릭터/몬스터 (Y 위치에 따라 동적)
   100   : 이펙트
   1000  : UI
   ```

---

## 🗺️ 타일 시스템

### Isometric 2:1 Ratio

```
        (64px)
    ┌──────────┐
   ╱            ╲  (32px)
  ╱              ╲
 ╱                ╲
└──────────────────┘
```

### 코드 상수

```rust
/// src/shared/constants.rs 참조
pub const TILE_WIDTH: f32 = 64.0;
pub const TILE_HEIGHT: f32 = 32.0;
pub const GRID_UNIT: f32 = TILE_WIDTH;
```

### 타일 에셋 제작 가이드

1. **크기**: 64 x 32 픽셀
2. **앵커**: 하단 중앙 (발 위치)
3. **포맷**: WebP (투명 배경)
4. **예시**: 
   - `grass_64x32.webp`
   - `stone_64x32.webp`
   - `water_64x32.webp`

---

## 🧑 캐릭터 규격

### 크기 규격

```
캐릭터 스프라이트: 48 x 64 픽셀
```

### 스프라이트시트 구조

```
1024 x 1024 스프라이트시트
═══════════════════════════════════════
║  Down-Idle │ Right-Idle │ Up-Idle  │ [비어있음] ║
║  (256x256) │ (256x256)  │(256x256) │(256x256)   ║
║  내부:     │ 내부:      │ 내부:    │            ║
║  48x64     │ 48x64      │ 48x64    │            ║
═══════════════════════════════════════
║  Down-Walk │ Right-Walk │ Up-Walk  │ [비어있음] ║
║  (256x256) │ (256x256)  │(256x256) │(256x256)   ║
║  내부:     │ 내부:      │ 내부:    │            ║
║  48x64     │ 48x64      │ 48x64    │            ║
═══════════════════════════════════════
║Down-Attack │Right-Attack│Up-Attack │ [비어있음] ║
║  (256x256) │ (256x256)  │(256x256) │(256x256)   ║
║  내부:     │ 내부:      │ 내부:    │            ║
║  48x64     │ 48x64      │ 48x64    │            ║
═══════════════════════════════════════
║  Down-Die  │ Right-Die  │ Up-Die   │ [비어있음] ║
║  (256x256) │ (256x256)  │(256x256) │(256x256)   ║
║  내부:     │ 내부:      │ 내부:    │            ║
║  48x64     │ 48x64      │ 48x64    │            ║
═══════════════════════════════════════
```

### 중요: 프레임 내부 배치

각 256x256 프레임 안에서:
- 실제 48x64 스프라이트는 **중앙에 배치**
- 나머지는 **투명 여백**
- Rect 설정 시 **실제 스프라이트 영역만 잘라야 함**

### 잘못된 방법 vs 올바른 방법

```
❌ 잘못됨: 256x256 전체를 렌더링
sprite.rect = Rect::new(0, 0, 256, 256)      // 너무 큼!
sprite.custom_size = Some(Vec2::new(256, 256))

✅ 올바름: 실제 스프라이트 영역만 렌더링
sprite.rect = Rect::new(104, 96, 152, 160)   // 48x64 중앙 부분
sprite.custom_size = Some(Vec2::new(48, 64))
```

또는 더 나은 방법:

```
✅ 최선: 스프라이트를 48x64로 처음부터 제작
- 스프라이트시트: 192x256 (4x4 프레임, 각 48x64)
- 또는 배경 제거 후 crop
```

---

## 👾 몬스터 규격

### 크기별 분류

| 등급 | 크기 | 예시 |
|------|------|------|
| Small | 32x32 | 쥐, 박쥐|
| Medium | 48x48 | 슬라임, 여우 |
| Large | 64x64 | 늑대, 스켈레톤, 고블린 |
| Boss | 128x128 | 드래곤, 리치 |

### 스프라이트시트 예시 (Small 32x32)

```
1024 x 1024 스프라이트시트
═══════════════════════════════════════
║Down-Idle(256) │Right-Idle │ Up-Idle │ [비어있음] ║
║  내부: 32x32  │ 내부:32x32│내부:32x32│           ║
═══════════════════════════════════════
```

### 현재 문제: Giant Rat

```
현재 상황:
- 스프라이트시트: 1024x1024
- 프레임: 256x256로 설정
- 실제 쥐 이미지: 32x32 또는 48x48 정도
- 렌더링: 256x256 전체를 렌더링 → 너무 큼!

해결 방법:
1. Rect를 실제 쥐 크기로 crop
2. custom_size를 32x32로 설정
3. 또는 스프라이트 재생성 시 여백 제거
```

---

## 📊 스프라이트시트 레이아웃

### 표준 레이아웃

```rust
// src/shared/domain/sprite/manifest.rs

SpriteLayout {
    image_width: 1024,           // 이미지 전체 크기
    image_height: 1024,
    frame_width: 256,            // ⚠️ 문제: 프레임 그리드 크기
    frame_height: 256,
    columns: 4,
    rows: 4,
    padding: 0,
    offset_x: 0,
    offset_y: 0,
}
```

### ⚠️ 핵심 문제

`frame_width`: 256은 **그리드 칸의 크기**이지 **실제 스프라이트 크기**가 아님!

### 해결 방법 1: Sprite Rect Cropping

```rust
// 각 프레임 내에서 실제 스프라이트 영역을 계산
let frame_rect = layout.get_frame_rect(index);  // (0, 0, 256, 256)
let actual_sprite_rect = crop_to_content(frame_rect, character_size);  // (104, 96, 152, 160)

sprite.rect = Some(Rect::new(
    actual_sprite_rect.x,
    actual_sprite_rect.y,
    actual_sprite_rect.x + actual_sprite_rect.width,
    actual_sprite_rect.y + actual_sprite_rect.height,
));
sprite.custom_size = Some(Vec2::new(CHARACTER_SPRITE_WIDTH, CHARACTER_SPRITE_HEIGHT));
```

### 해결 방법 2: 에셋 재생성 (권장)

```
기존:
1024x1024 스프라이트시트
├─ 4x4 그리드 (256x256 각 칸)
└─ 실제 스프라이트는 중앙에 작게

개선:
192x256 스프라이트시트 (캐릭터)
├─ 4x4 그리드 (48x64 각 칸)
└─ 여백 없이 꽉 찬 스프라이트

128x128 스프라이트시트 (Small 몬스터)
├─ 4x4 그리드 (32x32 각 칸)
└─ 여백 없이 꽉 찬 스프라이트
```

---

## 🔧 실제 적용 방법

### 1. Constants 사용

```rust
use crate::shared::constants::*;

// 타일 생성
Sprite {
    custom_size: Some(Vec2::new(TILE_WIDTH, TILE_HEIGHT)),
    ..default()
}

// 캐릭터 생성
Sprite {
    custom_size: Some(Vec2::new(CHARACTER_RENDER_WIDTH, CHARACTER_RENDER_HEIGHT)),
    ..default()
}

// 몬스터 생성 (Small)
Sprite {
    custom_size: Some(Vec2::new(MONSTER_SPRITE_SMALL, MONSTER_SPRITE_SMALL)),
    ..default()
}
```

### 2. SpriteLayout 수정 (임시 해결책)

**⚠️ 주의**: 이 방법은 스프라이트가 256x256 칸 내부에 중앙 배치되어 있을 때만 작동

```rust
// 캐릭터 매니패스트 (임시)
layout: SpriteLayout {
    image_width: 1024,
    image_height: 1024,
    frame_width: 256,    // 그리드 크기
    frame_height: 256,
    columns: 4,
    rows: 4,
    ..Default::default()
},

// 렌더링 시 크기 조정
sprite.custom_size = Some(Vec2::new(48.0, 64.0));  // 실제 크기로 축소
```

### 3. 새 스프라이트 생성 (영구 해결책)

```bash
# ImageMagick으로 여백 제거
magick input.png -trim output_trimmed.png

# 4x4 그리드로 재배치 (48x64 기준)
magick montage frame_*.png -tile 4x4 -geometry 48x64+0+0 -background none output_192x256.webp
```

---

## ✅ 체크리스트

### 에셋 제작
- [ ] 투명 배경 (Alpha Channel) 사용
- [ ] WebP 포맷 사용
- [ ] 타일: 64x32
- [ ] 캐릭터: 48x64 (스프라이트시트 192x256 권장)
- [ ] 몬스터: 크기별 정확한 규격 준수
- [ ] 앵커: 하단 중앙 (0.5, 1.0)

### 코드 적용
- [ ] `src/shared/constants.rs` 모든 크기 정의 확인
- [ ] `SpriteLayout` 프레임 크기 정확히 설정
- [ ] `Sprite.custom_size` 실제 렌더링 크기 설정
- [ ] `Sprite.rect` 필요 시 crop 영역 설정
- [ ] Z-Index 레이어 순서 확인

### 테스트
- [ ] 웹에서 실행 시 모든 스프라이트 크기 정상
- [ ] 타일이 정렬되어 연결됨
- [ ] 캐릭터와 몬스터 크기 비율 자연스러움
- [ ] 투명 배경이 정상적으로 표시됨
- [ ] 애니메이션이 자연스럽게 재생됨

---

## 🔗 참고 문서

- `src/shared/constants.rs` - 모든 크기 상수 정의
- `ASSETS.md` - 에셋 제작 가이드
- `src/shared/domain/sprite/manifest.rs` - 스프라이트 매니페스트 시스템
- `src/client/animation.rs` - 애니메이션 시스템

---

**마지막 업데이트**: 2025-12-24  
**작성자**: Antigravity AI
