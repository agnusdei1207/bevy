//! Anchor Point System
//!
//! 앵커 포인트 시스템은 캐릭터 애니메이션의 각 프레임에 장비 부착 위치를 정의합니다.
//! 이를 통해 캐릭터 본체와 장비를 별도로 관리하면서도 자연스럽게 결합할 수 있습니다.
//!
//! ## 장점
//! - **용량 절감**: 캐릭터 1세트 + 무기 N개만 필요 (조합별 스프라이트 불필요)
//! - **유연성**: 새 무기 추가 시 무기 스프라이트만 추가하면 됨
//! - **일관성**: 모든 캐릭터가 동일한 앵커 규격을 공유
//!
//! ## 사용 예시
//! ```text
//! 캐릭터 프레임 (48x64)
//! ┌──────────────────┐
//! │      ●head       │ ← 투구 부착점
//! │                  │
//! │   ●left  ●right  │ ← 무기/방패 부착점
//! │      ●body       │ ← 갑옷 중심점
//! │                  │
//! │      ●feet       │ ← 신발/그림자 부착점
//! └──────────────────┘
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::manifest::{AnimationState, SpriteDirection};

/// 앵커 포인트 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnchorType {
    /// 머리 위치 (투구, 모자 부착)
    Head,
    /// 몸통 중심 (갑옷 기준점)
    Body,
    /// 오른손 (주무기 부착)
    RightHand,
    /// 왼손 (방패, 보조무기 부착)
    LeftHand,
    /// 등 (망토, 날개 부착)
    Back,
    /// 발 위치 (그림자, 이펙트 기준점)
    Feet,
    /// 이펙트 중심 (스킬 이펙트 발생 위치)
    EffectCenter,
}

impl AnchorType {
    pub fn all() -> &'static [AnchorType] {
        &[
            AnchorType::Head,
            AnchorType::Body,
            AnchorType::RightHand,
            AnchorType::LeftHand,
            AnchorType::Back,
            AnchorType::Feet,
            AnchorType::EffectCenter,
        ]
    }
}

/// 2D 좌표 (프레임 내 상대 위치, 픽셀 단위)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// 미러링 (좌우 반전 시 X 좌표 반전)
    pub fn mirror(&self, frame_width: f32) -> Self {
        Self {
            x: frame_width - self.x,
            y: self.y,
        }
    }
}

/// 단일 프레임의 앵커 포인트들
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FrameAnchors {
    /// 각 앵커 타입별 위치
    pub anchors: HashMap<AnchorType, Point2D>,
    
    /// 장비 회전 각도 (도 단위, 무기 휘두르기 등에 사용)
    #[serde(default)]
    pub rotation: f32,
    
    /// 장비 Z-순서 오프셋 (앞/뒤 렌더링 조정)
    #[serde(default)]
    pub z_offset: i32,
}

impl FrameAnchors {
    pub fn new() -> Self {
        Self::default()
    }

    /// 앵커 추가
    pub fn with_anchor(mut self, anchor_type: AnchorType, x: f32, y: f32) -> Self {
        self.anchors.insert(anchor_type, Point2D::new(x, y));
        self
    }

    /// 특정 앵커 위치 가져오기
    pub fn get(&self, anchor_type: AnchorType) -> Option<Point2D> {
        self.anchors.get(&anchor_type).copied()
    }

    /// 미러링된 앵커 반환
    pub fn mirrored(&self, frame_width: f32) -> Self {
        let mut mirrored_anchors = HashMap::new();
        
        for (anchor_type, point) in &self.anchors {
            // 좌우 손은 서로 교체
            let new_type = match anchor_type {
                AnchorType::RightHand => AnchorType::LeftHand,
                AnchorType::LeftHand => AnchorType::RightHand,
                other => *other,
            };
            mirrored_anchors.insert(new_type, point.mirror(frame_width));
        }
        
        Self {
            anchors: mirrored_anchors,
            rotation: -self.rotation, // 회전도 반전
            z_offset: self.z_offset,
        }
    }
}

/// 애니메이션 시퀀스의 모든 프레임 앵커
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationAnchors {
    /// 애니메이션 상태
    pub state: AnimationState,
    
    /// 방향
    pub direction: SpriteDirection,
    
    /// 각 프레임별 앵커 (프레임 인덱스 -> 앵커 데이터)
    pub frames: Vec<FrameAnchors>,
}

impl AnimationAnchors {
    pub fn new(state: AnimationState, direction: SpriteDirection, frame_count: usize) -> Self {
        Self {
            state,
            direction,
            frames: vec![FrameAnchors::default(); frame_count],
        }
    }

    /// 특정 프레임의 앵커 가져오기
    pub fn get_frame(&self, frame_index: usize) -> Option<&FrameAnchors> {
        self.frames.get(frame_index)
    }

    /// 특정 프레임의 특정 앵커 위치 가져오기
    pub fn get_anchor(&self, frame_index: usize, anchor_type: AnchorType) -> Option<Point2D> {
        self.frames.get(frame_index)?.get(anchor_type)
    }
}

/// 캐릭터 전체 앵커 데이터
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAnchors {
    /// 캐릭터 ID (예: "warrior_male")
    pub character_id: String,
    
    /// 프레임 크기 (미러링 계산용)
    pub frame_width: f32,
    pub frame_height: f32,
    
    /// 모든 애니메이션의 앵커 데이터
    /// 키: "state_direction" (예: "walk_right")
    pub animations: HashMap<String, AnimationAnchors>,
}

impl CharacterAnchors {
    pub fn new(character_id: &str, frame_width: f32, frame_height: f32) -> Self {
        Self {
            character_id: character_id.to_string(),
            frame_width,
            frame_height,
            animations: HashMap::new(),
        }
    }

    /// 애니메이션 키 생성
    fn animation_key(state: AnimationState, direction: SpriteDirection) -> String {
        format!("{:?}_{:?}", state, direction).to_lowercase()
    }

    /// 애니메이션 앵커 추가
    pub fn add_animation(&mut self, animation: AnimationAnchors) {
        let key = Self::animation_key(animation.state, animation.direction);
        self.animations.insert(key, animation);
    }

    /// 특정 상태/방향/프레임의 앵커 가져오기
    pub fn get_anchors(
        &self,
        state: AnimationState,
        direction: SpriteDirection,
        frame_index: usize,
    ) -> Option<FrameAnchors> {
        let key = Self::animation_key(state, direction);
        
        // 직접 매칭 시도
        if let Some(anim) = self.animations.get(&key) {
            return anim.get_frame(frame_index).cloned();
        }
        
        // 미러링 시도 (Left <-> Right)
        let mirror_direction = match direction {
            SpriteDirection::Left => Some(SpriteDirection::Right),
            SpriteDirection::Right => Some(SpriteDirection::Left),
            _ => None,
        };
        
        if let Some(mirror_dir) = mirror_direction {
            let mirror_key = Self::animation_key(state, mirror_dir);
            if let Some(anim) = self.animations.get(&mirror_key) {
                if let Some(frame) = anim.get_frame(frame_index) {
                    return Some(frame.mirrored(self.frame_width));
                }
            }
        }
        
        None
    }

    /// 기본 캐릭터 앵커 생성 (48x64 기준)
    pub fn default_character(character_id: &str) -> Self {
        let mut anchors = Self::new(character_id, 48.0, 64.0);
        
        // 기본 앵커 위치 (48x64 프레임 기준)
        let default_frame = FrameAnchors::new()
            .with_anchor(AnchorType::Head, 24.0, 8.0)      // 머리 상단 중앙
            .with_anchor(AnchorType::Body, 24.0, 32.0)     // 몸통 중앙
            .with_anchor(AnchorType::RightHand, 38.0, 36.0) // 오른손 (무기)
            .with_anchor(AnchorType::LeftHand, 10.0, 36.0)  // 왼손 (방패)
            .with_anchor(AnchorType::Back, 24.0, 28.0)      // 등 (망토)
            .with_anchor(AnchorType::Feet, 24.0, 62.0)      // 발
            .with_anchor(AnchorType::EffectCenter, 24.0, 32.0); // 이펙트 중심
        
        // 모든 상태/방향에 대해 기본 앵커 설정
        for state in AnimationState::basic_states() {
            for direction in SpriteDirection::all() {
                // Right와 Down 방향만 정의 (Left는 미러링으로 처리)
                if direction == SpriteDirection::Left {
                    continue;
                }
                
                let mut anim = AnimationAnchors::new(state, direction, 4);
                for i in 0..4 {
                    anim.frames[i] = default_frame.clone();
                    
                    // 공격 애니메이션은 무기 회전 추가
                    if state == AnimationState::Attack {
                        anim.frames[i].rotation = match i {
                            0 => -30.0,
                            1 => 0.0,
                            2 => 45.0,
                            3 => 30.0,
                            _ => 0.0,
                        };
                        
                        // 공격 프레임별 손 위치 조정
                        let hand_offset = match i {
                            0 => (2.0, 0.0),
                            1 => (4.0, -4.0),
                            2 => (6.0, -2.0),
                            3 => (3.0, 2.0),
                            _ => (0.0, 0.0),
                        };
                        
                        if let Some(right_hand) = anim.frames[i].anchors.get_mut(&AnchorType::RightHand) {
                            right_hand.x += hand_offset.0;
                            right_hand.y += hand_offset.1;
                        }
                    }
                }
                anchors.add_animation(anim);
            }
        }
        
        anchors
    }

    /// JSON으로 직렬화
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// JSON에서 역직렬화
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// 장비 렌더링 정보
#[derive(Debug, Clone)]
pub struct EquipmentRenderInfo {
    /// 장비 스프라이트 경로
    pub sprite_path: String,
    /// 렌더링 위치 (월드 좌표)
    pub position: Point2D,
    /// 회전 각도 (도)
    pub rotation: f32,
    /// Z 순서
    pub z_order: i32,
    /// 좌우 반전 여부
    pub flip_x: bool,
}

/// 장비 렌더러 - 앵커를 기반으로 장비 렌더링 정보 계산
pub struct EquipmentRenderer {
    /// 캐릭터 앵커 데이터 캐시
    character_anchors: HashMap<String, CharacterAnchors>,
}

impl Default for EquipmentRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl EquipmentRenderer {
    pub fn new() -> Self {
        Self {
            character_anchors: HashMap::new(),
        }
    }

    /// 캐릭터 앵커 데이터 등록
    pub fn register_character(&mut self, anchors: CharacterAnchors) {
        self.character_anchors.insert(anchors.character_id.clone(), anchors);
    }

    /// 장비 렌더링 정보 계산
    pub fn calculate_equipment_position(
        &self,
        character_id: &str,
        character_world_pos: Point2D,
        state: AnimationState,
        direction: SpriteDirection,
        frame_index: usize,
        anchor_type: AnchorType,
    ) -> Option<EquipmentRenderInfo> {
        let anchors = self.character_anchors.get(character_id)?;
        let frame_anchors = anchors.get_anchors(state, direction, frame_index)?;
        let anchor_point = frame_anchors.get(anchor_type)?;
        
        // 미러링 필요 여부
        let needs_flip = direction == SpriteDirection::Left;
        
        // 월드 좌표 계산 (캐릭터 위치 + 앵커 오프셋)
        let world_x = character_world_pos.x + anchor_point.x - anchors.frame_width / 2.0;
        let world_y = character_world_pos.y + anchor_point.y - anchors.frame_height;
        
        Some(EquipmentRenderInfo {
            sprite_path: String::new(), // 호출자가 설정
            position: Point2D::new(world_x, world_y),
            rotation: frame_anchors.rotation,
            z_order: frame_anchors.z_offset,
            flip_x: needs_flip,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anchor_mirroring() {
        let frame = FrameAnchors::new()
            .with_anchor(AnchorType::RightHand, 40.0, 30.0)
            .with_anchor(AnchorType::LeftHand, 8.0, 30.0);
        
        let mirrored = frame.mirrored(48.0);
        
        // 미러링 후 좌우 손이 교체되어야 함
        let right = mirrored.get(AnchorType::RightHand).unwrap();
        let left = mirrored.get(AnchorType::LeftHand).unwrap();
        
        // 원래 LeftHand(8.0) -> 미러링 -> 새 RightHand(48-8=40)
        assert!((right.x - 40.0).abs() < 0.01);
        // 원래 RightHand(40.0) -> 미러링 -> 새 LeftHand(48-40=8)
        assert!((left.x - 8.0).abs() < 0.01);
    }

    #[test]
    fn test_default_character_anchors() {
        let anchors = CharacterAnchors::default_character("warrior_male");
        
        // Walk Right 애니메이션이 있어야 함
        let walk_right = anchors.get_anchors(
            AnimationState::Walk,
            SpriteDirection::Right,
            0,
        );
        assert!(walk_right.is_some());
        
        // Walk Left는 Right의 미러링으로 가져와야 함
        let walk_left = anchors.get_anchors(
            AnimationState::Walk,
            SpriteDirection::Left,
            0,
        );
        assert!(walk_left.is_some());
    }

    #[test]
    fn test_attack_rotation() {
        let anchors = CharacterAnchors::default_character("test");
        
        // 공격 애니메이션은 프레임별 회전이 있어야 함
        let attack_frame_0 = anchors.get_anchors(
            AnimationState::Attack,
            SpriteDirection::Right,
            0,
        ).unwrap();
        
        assert!(attack_frame_0.rotation.abs() > 0.0);
    }
}
