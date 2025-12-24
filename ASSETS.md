# 📦 Legend of Darkness M: Asset Master Guide

이 문서는 **몬스터, 캐릭터, 그리고 캐릭터가 착용한 장착 아이템**에 대한 통합 에셋 관리 방식, 기술적 사양, 제작 가이드라인을 정의합니다.

---

## 🎨 1. 에셋 관리 철학

1.  **경량화 (Lightweight Core)**: 모든 이미지는 **WebP (.webp)** 포맷을 사용하여 PNG 대비 용량을 50% 이상 절감합니다.
2.  **효율적 방향성 (3-Way Mirroring)**: 4방향을 모두 제작하지 않고 **Down, Right, Up** 3방향만 제작합니다. **Left는 실시간 미러링** 처리합니다.
3.  **조합형 구조 (Paper Doll & Anchors)**: 캐릭터와 장비를 분리하여 **앵커 포인트(Anchor Point)** 기반으로 실시간 합성합니다.
4.  **투명 배경 (Perfect Transparency)**: 배경색(컬러키) 방식이 아닌 **Alpha Channel을 포함한 투명 배경** WebP를 사용합니다.

---

## 📂 2. 디렉토리 구조

```
public/assets/
├── characters/             # 캐릭터 본체 (성별/종족별)
│   └── warrior_male/
│       ├── spritesheet.webp
│       ├── manifest.json   # 스프라이트 애니메이션 정의
│       └── anchors.json    # 장비 부착 좌표 정의
├── equipment/              # 장비 아이템 (단일 이미지)
│   ├── weapons/            # sword_01.webp (단일 웹피)
│   ├── armor/
│   └── hair/               # 머리카락 스타일별 .webp
├── monsters/               # 몬스터 (종류별)
│   └── rat/
│       ├── spritesheet.webp
│       └── manifest.json
├── tiles/                  # 맵 타일 (쿼터뷰 2:1 ratio)
├── ui/                     # UI 요소
└── audio/                  # BGM(mp3), SFX(wav)
```

---

## ⚙️ 3. 스프라이트 & 페이퍼돌 시스템 (Technical)

### 매니페스트 시스템 (`manifest.json`)
현대판 EPF/SPF입니다. 이미지의 영역별 애니메이션 상태를 정의합니다.
- **Row 순서**: Idle(0), Walk(1), Attack(2), Die(3)
- **Column 순서**: Down, Right, Up (Left는 Right 반전)

### 앵커 포인트 시스템 (`anchors.json`)
장비가 캐릭터의 움직임을 따라가는 핵심 기술입니다.
- **Head**: 투구/머리카락 부착점 (48x64 기준 기본 24, 8)
- **RightHand**: 무기 부착점 (공격 모션 시 위치/회전값 포함)
- **Feet**: 그림자 및 발 밑 이펙트 기준점

### 레이어링 순서 (Z-Index)
1. `Shadow` (0) → 2. `Body` (1) → 3. `Armor` (3) → 4. `Hair` (4) → 5. `Weapon/Shield` (6,7) → 6. `Effect` (9)

---

## 🤖 4. AI 에셋 생성 가이드 (Generation)

### 캐릭터 본체 프롬프트 (WebP 기반)
```text
Dark fantasy 2D pixel art sprite sheet, 3 columns x 4 rows.
Col 1: Down, Col 2: Right, Col 3: Up. 
Rows: Idle, Walk, Attack, Death.
Isometric 2.5D, 16-bit retro style, transparent background.
```

### 후처리 워크플로우
1.  **배경 제거**: 생성된 이미지에서 배경을 완벽하게 제거 (remove.bg 등 사용).
2.  **WebP 변환**: `magick input.png output.webp` (투명도 유지 필수).
3.  **검수**: 3방향 레이아웃(Down, Right, Up)이 정확한 그리드에 배치되었는지 확인.

---

## 📐 5. 상세 규격표

| 구분 | 규격 | 상세 |
|------|------|------|
| **포맷** | **WebP** | 알파 채널 포함, 배경 투명 |
| **캐릭터 크기** | 48x64 pixel | 프레임당 크기 |
| **몬스터 크기** | 32/48/64/128 | 등급별 상이 |
| **타일** | 64x32 pixel | 쿼터뷰 (2:1 비율) |
| **프레임수** | 4 frames | 모든 동작은 4프레임 루프 |

---

## ✅ 6. 체크리스트

- [ ] 배경이 투명한가? (분홍색/흰색 배경 안됨)
- [ ] 확장자가 `.webp`인가?
- [ ] 3방향(Down, Right, Up) 순서가 맞는가?
- [ ] `manifest.json`과 `anchors.json`이 이미지와 쌍을 이루는가?

---

## 🔗 참조 (Internal Docs)
- **[README.md](./README.md)**: 프로젝트 개요 및 실시간 실행 가이드
