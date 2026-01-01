//! Sprite Manifest System
//!
//! EPF/SPF 스타일의 스프라이트 매니페스트 시스템입니다.
//! 어둠의전설처럼 하나의 파일에 모든 방향/모션 정보를 담는 컨테이너 방식을 지원합니다.
//!
//! ## 특징
//! - **선언적 관리**: JSON/TOML로 스프라이트 메타데이터 정의
//! - **방향별 관리**: 4방향/8방향 지원, 좌우 미러링으로 메모리 절약
//! - **모션별 관리**: idle, walk, attack, hit, die 등 상태별 프레임 정의
//! - **인덱스 참조**: 게임 엔진에서 상태값으로 정확한 프레임 위치 계산

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::shared::constants::*;

#[cfg(feature = "client")]
use bevy::{asset::Asset, reflect::TypePath};

/// 스프라이트 방향 (어둠의전설 스타일 4방향)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SpriteDirection {
    #[default]
    Down = 0,
    Left = 1,
    Right = 2,
    Up = 3,
}

impl SpriteDirection {
    /// 좌우 미러링 시 사용할 반대 방향 반환
    pub fn mirrored(&self) -> Option<SpriteDirection> {
        match self {
            SpriteDirection::Left => Some(SpriteDirection::Right),
            SpriteDirection::Right => Some(SpriteDirection::Left),
            _ => None,
        }
    }

    /// 미러링이 필요한지 (Left 방향은 Right를 미러링해서 사용 가능)
    pub fn needs_mirror(&self) -> bool {
        matches!(self, SpriteDirection::Left)
    }

    /// 모든 방향 반복자
    pub fn all() -> impl Iterator<Item = SpriteDirection> {
        [
            SpriteDirection::Down,
            SpriteDirection::Left,
            SpriteDirection::Right,
            SpriteDirection::Up,
        ]
        .into_iter()
    }

    /// 열 인덱스로 변환 (스프라이트시트에서의 열 위치)
    pub fn column_index(&self) -> usize {
        *self as usize
    }
}

/// 애니메이션 상태 (모션 종류)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AnimationState {
    #[default]
    Idle = 0,
    Walk = 1,
    Attack = 2,
    Hit = 3,
    Die = 4,
    Cast = 5,     // 마법 시전
    Emote = 6,    // 감정표현
    Special = 7,  // 특수 동작
}

impl AnimationState {
    /// 행 인덱스로 변환 (스프라이트시트에서의 행 위치)
    pub fn row_index(&self) -> usize {
        *self as usize
    }

    /// 기본 상태들
    pub fn basic_states() -> impl Iterator<Item = AnimationState> {
        [
            AnimationState::Idle,
            AnimationState::Walk,
            AnimationState::Attack,
            AnimationState::Die,
        ]
        .into_iter()
    }
}

/// 개별 애니메이션 시퀀스 정의
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationSequence {
    /// 애니메이션 상태
    pub state: AnimationState,
    
    /// 시작 프레임 인덱스
    pub start_frame: usize,
    
    /// 프레임 수
    pub frame_count: usize,
    
    /// 초당 프레임 수 (FPS)
    #[serde(default = "default_fps")]
    pub fps: f32,
    
    /// 루프 여부
    #[serde(default = "default_true")]
    pub looping: bool,
    
    /// 각 방향별 프레임 시작 오프셋 (없으면 자동 계산)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_offsets: Option<HashMap<SpriteDirection, usize>>,
}

fn default_fps() -> f32 { 8.0 }
fn default_true() -> bool { true }

impl Default for AnimationSequence {
    fn default() -> Self {
        Self {
            state: AnimationState::Idle,
            start_frame: 0,
            frame_count: 4,
            fps: 8.0,
            looping: true,
            direction_offsets: None,
        }
    }
}

/// 스프라이트시트 레이아웃 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "client", derive(TypePath))]
pub struct SpriteLayout {
    /// 이미지 전체 너비 (픽셀)
    pub image_width: u32,
    
    /// 이미지 전체 높이 (픽셀)
    pub image_height: u32,
    
    /// 개별 프레임 너비
    pub frame_width: u32,
    
    /// 개별 프레임 높이
    pub frame_height: u32,
    
    /// 열 수 (가로 방향 프레임 개수)
    pub columns: usize,
    
    /// 행 수 (세로 방향 프레임 개수)
    pub rows: usize,
    
    /// 프레임 간 간격 (픽셀, 선택사항)
    #[serde(default)]
    pub padding: u32,
    
    /// 오프셋 X (첫 프레임 시작 위치)
    #[serde(default)]
    pub offset_x: u32,
    
    /// 오프셋 Y (첫 프레임 시작 위치)
    #[serde(default)]
    pub offset_y: u32,
}

impl SpriteLayout {
    /// 프레임 인덱스로 픽셀 영역(Rect) 계산
    pub fn get_frame_rect(&self, frame_index: usize) -> (u32, u32, u32, u32) {
        let col = (frame_index % self.columns) as u32;
        let row = (frame_index / self.columns) as u32;
        
        let x = self.offset_x + col * (self.frame_width + self.padding);
        let y = self.offset_y + row * (self.frame_height + self.padding);
        
        (x, y, self.frame_width, self.frame_height)
    }

    /// 특정 상태와 방향에 대한 프레임 인덱스 계산
    pub fn get_sprite_index(
        &self,
        state: AnimationState,
        direction: SpriteDirection,
        frame: usize,
    ) -> usize {
        // 어둠의전설 스타일 8x8 시트 규격 (정사각형 이미지 최적화)
        // 각 상태(Idle, Walk, Attack, Die)는 2개 행을 차지함.
        // 한 행(8열)은 2개 방향(각 4프레임)을 포함함.
        
        let dir_idx = match direction {
            SpriteDirection::Down => 0,
            SpriteDirection::Left => 1,
            SpriteDirection::Right => 2,
            SpriteDirection::Up => 3,
        };

        let state_row_start = match state {
            AnimationState::Idle => 0,
            AnimationState::Walk => 2,
            AnimationState::Attack => 4,
            AnimationState::Die => 6,
            _ => 0, // 기본값 또는 에러 처리
        };

        // 8x8 (SPRITESHEET_COLUMNS = 8) 기준 계산
        let final_row = state_row_start + (dir_idx / 2);
        let final_col = (dir_idx % 2) * 4 + (frame % 4);
        
        final_row * 8 + final_col
    }
}

impl Default for SpriteLayout {
    fn default() -> Self {
        Self {
            image_width: 256,
            image_height: 256,
            frame_width: 64,
            frame_height: 64,
            columns: 4,
            rows: 4,
            padding: 0,
            offset_x: 0,
            offset_y: 0,
        }
    }
}

/// 색상 팔레트 정의 (팔레트 스왑용)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    /// 팔레트 이름 (예: "fire", "ice", "poison")
    pub name: String,
    
    /// 원본 색상 -> 교체 색상 매핑 (hex 문자열)
    pub color_map: HashMap<String, String>,
}

/// 미러링 설정
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MirrorConfig {
    /// 미러링 활성화 여부
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    /// 미러링 소스 방향 -> 타겟 방향
    /// 예: Right -> Left (오른쪽 스프라이트를 뒤집어 왼쪽으로 사용)
    pub source_to_target: HashMap<SpriteDirection, SpriteDirection>,
}

impl MirrorConfig {
    /// 기본 좌우 미러링 설정 생성
    pub fn default_horizontal() -> Self {
        let source_to_target = HashMap::new();
        // source_to_target.insert(SpriteDirection::Right, SpriteDirection::Left);
        
        Self {
            enabled: false, // 4방향 시트를 직접 사용하므로 미러링 비활성화
            source_to_target,
        }
    }

    /// 주어진 방향에 대해 실제 스프라이트 시트에서 참조해야 할 방향과 미러링 여부를 반환
    pub fn get_actual_direction(&self, direction: SpriteDirection) -> (SpriteDirection, bool) {
        if self.enabled {
            // target -> source 매핑을 찾아야 함
            // 즉, Left 방향을 요청했는데 Right를 미러링해서 사용한다면 (Right -> Left)
            // 실제 방향은 Right가 되고, 미러링 플래그는 true가 된다.
            if let Some((source_dir, _)) = self.source_to_target.iter()
                .find(|(_, target_dir)| **target_dir == direction)
            {
                (*source_dir, true)
            } else {
                (direction, false)
            }
        } else {
            (direction, false)
        }
    }
}

/// 스프라이트 매니페스트 - 하나의 스프라이트시트에 대한 완전한 메타데이터
/// 스프라이트 매니페스트 (스프라이트시트 메타데이터)
#[derive(Debug, Clone, Serialize, Deserialize, Asset, TypePath)]
pub struct SpriteManifest {
    /// 매니페스트 버전
    #[serde(default = "default_version")]
    pub version: String,
    
    /// 스프라이트 고유 식별자
    pub id: String,
    
    /// 스프라이트 이름 (표시용)
    pub name: String,
    
    /// 스프라이트 타입 (character, monster, effect, tile 등)
    pub sprite_type: SpriteType,
    
    /// 이미지 파일 경로 (public 기준 상대 경로)
    pub image_path: String,
    
    /// 레이아웃 정보
    pub layout: SpriteLayout,
    
    /// 애니메이션 시퀀스 목록
    pub animations: Vec<AnimationSequence>,
    
    /// 미러링 설정
    #[serde(default)]
    pub mirror: MirrorConfig,
    
    /// 색상 팔레트 변형들 (선택사항)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub palettes: Vec<ColorPalette>,
    
    /// 앵커 포인트 (렌더링 기준점, 0.0-1.0)
    #[serde(default = "default_anchor")]
    pub anchor: (f32, f32),
    
    /// 추가 메타데이터
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
}

fn default_version() -> String { "1.0.0".to_string() }
fn default_anchor() -> (f32, f32) { (0.5, 1.0) } // 하단 중앙이 기본값

/// 스프라이트 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpriteType {
    Character,
    Monster,
    Effect,
    Tile,
    Item,
    Ui,
}

impl Default for SpriteManifest {
    fn default() -> Self {
        Self {
            version: default_version(),
            id: "unknown".to_string(),
            name: "Unknown Sprite".to_string(),
            sprite_type: SpriteType::Character,
            image_path: "/assets/placeholder.png".to_string(),
            layout: SpriteLayout::default(),
            animations: vec![AnimationSequence::default()],
            mirror: MirrorConfig::default_horizontal(),
            palettes: Vec::new(),
            anchor: default_anchor(),
            metadata: HashMap::new(),
        }
    }
}

impl SpriteManifest {
    /// 특정 애니메이션 시퀀스 찾기
    pub fn get_animation(&self, state: AnimationState) -> Option<&AnimationSequence> {
        self.animations.iter().find(|a| a.state == state)
    }

    /// 상태, 방향, 현재 프레임 번호를 기반으로 실제 프레임 인덱스와 반전 여부 반환
    pub fn get_frame_index(
        &self,
        state: AnimationState,
        direction: SpriteDirection,
        frame: usize,
    ) -> (usize, bool) {
        // 미러링 설정 확인
        let (actual_direction, needs_flip) = self.mirror.get_actual_direction(direction);

        let index = self.layout.get_sprite_index(state, actual_direction, frame);
        (index, needs_flip)
    }

    /// JSON 문자열로 직렬화
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// JSON 문자열에서 역직렬화
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// 캐릭터용 기본 매니페스트 생성
    /// Standard: 256x256 sheet, 64x64 frames, 4x4 grid
    pub fn new_character(id: &str, name: &str, image_path: &str) -> Self {
        let animations = vec![
            AnimationSequence {
                state: AnimationState::Idle,
                start_frame: 0,
                frame_count: ANIMATION_FRAMES_DEFAULT,
                fps: ANIMATION_FPS_DEFAULT,
                looping: true,
                direction_offsets: None,
            },
            AnimationSequence {
                state: AnimationState::Walk,
                start_frame: 0, // get_sprite_index handles the row offset
                frame_count: ANIMATION_FRAMES_DEFAULT,
                fps: ANIMATION_FPS_DEFAULT,
                looping: true,
                direction_offsets: None,
            },
            AnimationSequence {
                state: AnimationState::Attack,
                start_frame: 0,
                frame_count: ANIMATION_FRAMES_DEFAULT,
                fps: ANIMATION_FPS_DEFAULT,
                looping: false,
                direction_offsets: None,
            },
            AnimationSequence {
                state: AnimationState::Die,
                start_frame: 0,
                frame_count: ANIMATION_FRAMES_DEFAULT,
                fps: ANIMATION_FPS_DEFAULT,
                looping: false,
                direction_offsets: None,
            },
        ];

        Self {
            id: id.to_string(),
            name: name.to_string(),
            sprite_type: SpriteType::Character,
            image_path: image_path.to_string(),
            layout: SpriteLayout {
                image_width: CHARACTER_SPRITESHEET_SIZE,
                image_height: CHARACTER_SPRITESHEET_SIZE,
                frame_width: CHARACTER_FRAME_SIZE,
                frame_height: CHARACTER_FRAME_SIZE,
                columns: SPRITESHEET_COLUMNS,
                rows: SPRITESHEET_ROWS,
                ..Default::default()
            },
            animations,
            mirror: MirrorConfig::default_horizontal(),
            ..Default::default()
        }
    }

    /// 몬스터용 기본 매니페스트 생성
    /// Standard sizes: Small(32/128), Medium(48/192), Large(64/256), Boss(128/512)
    pub fn new_monster(id: &str, name: &str, image_path: &str, size: MonsterSpriteSize) -> Self {
        // Calculate frame and sheet size based on monster size tier
        let frame_size = match size {
            MonsterSpriteSize::Small => 32,
            MonsterSpriteSize::Medium => 48,
            MonsterSpriteSize::Large => 64,
            MonsterSpriteSize::Boss => 128,
        };
        let image_size = frame_size * 8; // 8x8 grid

        let animations = vec![
            AnimationSequence {
                state: AnimationState::Idle,
                start_frame: 0,
                frame_count: ANIMATION_FRAMES_DEFAULT,
                fps: ANIMATION_FPS_DEFAULT,
                looping: true,
                direction_offsets: None,
            },
            AnimationSequence {
                state: AnimationState::Walk,
                start_frame: 0,
                frame_count: ANIMATION_FRAMES_DEFAULT,
                fps: ANIMATION_FPS_DEFAULT,
                looping: true,
                direction_offsets: None,
            },
            AnimationSequence {
                state: AnimationState::Attack,
                start_frame: 0,
                frame_count: ANIMATION_FRAMES_DEFAULT,
                fps: ANIMATION_FPS_DEFAULT,
                looping: false,
                direction_offsets: None,
            },
            AnimationSequence {
                state: AnimationState::Die,
                start_frame: 0,
                frame_count: ANIMATION_FRAMES_DEFAULT,
                fps: ANIMATION_FPS_DEFAULT,
                looping: false,
                direction_offsets: None,
            },
        ];

        Self {
            id: id.to_string(),
            name: name.to_string(),
            sprite_type: SpriteType::Monster,
            image_path: image_path.to_string(),
            layout: SpriteLayout {
                image_width: image_size,
                image_height: image_size,
                frame_width: frame_size,
                frame_height: frame_size,
                columns: SPRITESHEET_COLUMNS,
                rows: SPRITESHEET_ROWS,
                ..Default::default()
            },
            animations,
            mirror: MirrorConfig::default(),
            ..Default::default()
        }
    }
}

/// 몬스터 스프라이트 크기 (레이아웃 생성용)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonsterSpriteSize {
    Small,   // 32x32
    Medium,  // 48x48
    Large,   // 64x64
    Boss,    // 128x128
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_serialization() {
        let manifest = SpriteManifest::new_character(
            "warrior_male",
            "Warrior (Male)",
            "/assets/characters/warrior/male/spritesheet.png",
        );

        let json = manifest.to_json().unwrap();
        let parsed: SpriteManifest = SpriteManifest::from_json(&json).unwrap();
        
        assert_eq!(manifest.id, parsed.id);
        assert_eq!(manifest.name, parsed.name);
    }

    #[test]
    fn test_frame_index_calculation() {
        let manifest = SpriteManifest::new_character(
            "test",
            "Test",
            "/test.png",
        );

        // Idle, Down, Frame 0 -> index 0
        let (index, flip) = manifest.get_frame_index(
            AnimationState::Idle,
            SpriteDirection::Down,
            0,
        );
        assert_eq!(index, 0);
        assert!(!flip);
    }
}
