//! Game Constants - 게임 규격 정의
//!
//! 모든 에셋 크기, 타일 크기, 스케일 등을 중앙에서 관리합니다.
//! 이 파일의 값들은 게임 전체의 기준이 됩니다.

// ============================================================
// 📐 1. 타일 시스템 (Tile System)
// ============================================================

/// 타일 너비 (픽셀) - Isometric 2:1 ratio 기준
pub const TILE_WIDTH: f32 = 64.0;

/// 타일 높이 (픽셀) - Isometric 2:1 ratio 기준
pub const TILE_HEIGHT: f32 = 32.0;

/// 게임 월드의 그리드 단위 (논리적 그리드 1칸 = 실제 픽셀)
/// 모든 캐릭터, 몬스터, 객체는 이 그리드에 정렬됩니다.
pub const GRID_UNIT: f32 = TILE_WIDTH;

// ============================================================
// 🎮 2. 캐릭터 규격 (Character Specifications)
// ============================================================

/// 캐릭터 스프라이트 크기 (픽셀)
/// Standard: 64x64 프레임, 256x256 시트 (4x4 그리드)
pub const CHARACTER_SPRITE_WIDTH: f32 = 64.0;

/// 캐릭터 스프라이트 높이 (픽셀)
/// Standard: 64x64 프레임, 256x256 시트 (4x4 그리드)
pub const CHARACTER_SPRITE_HEIGHT: f32 = 64.0;

/// 캐릭터 렌더링 스케일 (게임 내 표시 크기)
/// 1.0 = 원본 크기, 1.5 = 1.5배 확대
pub const CHARACTER_RENDER_SCALE: f32 = 1.0;

/// 캐릭터 실제 렌더링 너비 (스케일 적용 후)
pub const CHARACTER_RENDER_WIDTH: f32 = CHARACTER_SPRITE_WIDTH * CHARACTER_RENDER_SCALE;

/// 캐릭터 실제 렌더링 높이 (스케일 적용 후)
pub const CHARACTER_RENDER_HEIGHT: f32 = CHARACTER_SPRITE_HEIGHT * CHARACTER_RENDER_SCALE;

// ============================================================
// 👾 3. 몬스터 규격 (Monster Specifications)
// ============================================================

/// 소형 몬스터 (예: 쥐, 박쥐)
pub const MONSTER_SPRITE_SMALL: f32 = 32.0;

/// 중형 몬스터 (예: 슬라임, 여우)
pub const MONSTER_SPRITE_MEDIUM: f32 = 48.0;

/// 대형 몬스터 (예: 늑대, 스켈레톤)
pub const MONSTER_SPRITE_LARGE: f32 = 64.0;

/// 보스 몬스터 (예: 드래곤)
pub const MONSTER_SPRITE_BOSS: f32 = 128.0;

/// 몬스터 렌더링 스케일
pub const MONSTER_RENDER_SCALE: f32 = 1.0;

// ============================================================
// 🖼️ 4. 스프라이트시트 규격 (Sprite Sheet Layout)
// ============================================================

/// 캐릭터 스프라이트시트 이미지 크기 (8x8 그리드 기준, 정사각형)
/// 64 픽셀 * 8 열 = 512 픽셀
pub const CHARACTER_SPRITESHEET_SIZE: u32 = 512;

/// 스프라이트시트 열 수 (8열)
pub const SPRITESHEET_COLUMNS: usize = 8;

/// 스프라이트시트 행 수 (8행)
pub const SPRITESHEET_ROWS: usize = 8;

/// 캐릭터 프레임 크기 (64x64)
pub const CHARACTER_FRAME_SIZE: u32 = 64;

// ============================================================
// 🎬 5. 애니메이션 규격 (Animation Specifications)
// ============================================================

/// 일관성 있는 애니메이션 FPS (모든 동작 공통)
pub const ANIMATION_FPS_DEFAULT: f32 = 10.0;

/// 일관성 있는 프레임 수 (각 방향/상태별)
pub const ANIMATION_FRAMES_DEFAULT: usize = 4;

// ============================================================
// 🌍 6. 맵 규격 (Map Specifications)
// ============================================================

/// 기본 맵 너비 (그리드 단위)
pub const MAP_WIDTH_DEFAULT: i32 = 16;

/// 기본 맵 높이 (그리드 단위)
pub const MAP_HEIGHT_DEFAULT: i32 = 16;

// ============================================================
// 📊 7. Z-Index / 레이어 순서 (Rendering Layers)
// ============================================================

/// 타일 레이어 Z 오프셋
pub const Z_LAYER_TILE: f32 = 0.0;

/// 그림자 레이어 Z 오프셋
pub const Z_LAYER_SHADOW: f32 = 1.0;

/// 캐릭터/몬스터 레이어 Z 오프셋 (Y 위치 기반 동적 계산)
pub const Z_LAYER_ENTITY_BASE: f32 = 5.0;

/// 이펙트 레이어 Z 오프셋
pub const Z_LAYER_EFFECT: f32 = 100.0;

/// UI 레이어 Z 오프셋
pub const Z_LAYER_UI: f32 = 1000.0;

// ============================================================
// ⚙️ 8. 게임플레이 상수 (Gameplay Constants)
// ============================================================

/// 이동 속도 (초당 이동하는 타일 수)
pub const MOVE_SPEED: f32 = 5.0;

/// 한 타일 이동에 걸리는 시간 (초)
pub const MOVE_DURATION: f32 = 1.0 / MOVE_SPEED;

// ============================================================
// 🔧 9. 디버그 설정 (Debug Settings)
// ============================================================

/// 디버그 그리드 표시 여부
pub const DEBUG_SHOW_GRID: bool = false;

/// 디버그 히트박스 표시 여부
pub const DEBUG_SHOW_HITBOX: bool = false;

// ============================================================
// 📖 10. 문서 및 가이드 (Documentation)
// ============================================================

/// 에셋 제작 시 참고할 규격 요약 (ASSETS.md 참조)
///
/// ## 파일 포맷
/// - **PNG 전용** (무손실, 알파 채널, 투명 배경 필수)
///
/// ## 타일
/// - 크기: 64x32 픽셀 (2:1 isometric)
///
/// ## 캐릭터
/// - 프레임: 64x64 픽셀
/// - 시트: 256x256 (4x4 그리드)
/// - 열: Down(0), Left(1), Right(2), Up(3)
/// - 행: Idle(0), Walk(1), Attack(2), Die(3)
///
/// ## 몬스터
/// - Small: 32x32 프레임, 128x128 시트
/// - Medium: 48x48 프레임, 192x192 시트
/// - Large: 64x64 프레임, 256x256 시트
/// - Boss: 128x128 프레임, 512x512 시트
///
/// ## 장비 (Paper Doll)
/// - 캐릭터와 동일한 레이아웃 (256x256, 64x64 프레임)
/// - 앵커 포인트 기반 합성
pub const ASSET_GUIDELINES: &str = "See ASSETS.md for detailed asset creation guidelines";
