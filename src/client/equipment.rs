//! Equipment Rendering System
//!
//! Bevy 기반의 장비 렌더링 시스템입니다.
//! 앵커 포인트를 사용하여 캐릭터 애니메이션에 맞춰 장비를 렌더링합니다.

use bevy::prelude::*;
use std::collections::HashMap;

use crate::shared::domain::sprite::{
    AnchorType, AnimationState, CharacterAnchors, EquipmentRenderer,
    Point2D, SpriteDirection,
};

/// 장착된 장비 정보
#[derive(Component, Default, Clone)]
pub struct EquippedItems {
    /// 무기 스프라이트 경로
    pub weapon: Option<String>,
    /// 방패 스프라이트 경로
    pub shield: Option<String>,
    /// 투구 스프라이트 경로
    pub helmet: Option<String>,
    /// 망토 스프라이트 경로
    pub cape: Option<String>,
}

impl EquippedItems {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_weapon(mut self, path: &str) -> Self {
        self.weapon = Some(path.to_string());
        self
    }

    pub fn with_shield(mut self, path: &str) -> Self {
        self.shield = Some(path.to_string());
        self
    }
}

/// 캐릭터 애니메이션 상태 컴포넌트
#[derive(Component)]
pub struct CharacterAnimState {
    pub state: AnimationState,
    pub direction: SpriteDirection,
    pub current_frame: usize,
    pub character_type: String, // "warrior_male", etc.
}

impl Default for CharacterAnimState {
    fn default() -> Self {
        Self {
            state: AnimationState::Idle,
            direction: SpriteDirection::Down,
            current_frame: 0,
            character_type: "warrior_male".to_string(),
        }
    }
}

/// 장비 스프라이트 마커
#[derive(Component)]
pub struct EquipmentSprite {
    pub anchor_type: AnchorType,
    pub parent_entity: Entity,
}

/// 장비 렌더링 리소스
#[derive(Resource)]
pub struct EquipmentRenderingSystem {
    pub renderer: EquipmentRenderer,
    /// 로드된 장비 텍스처 핸들
    pub weapon_textures: HashMap<String, Handle<Image>>,
    pub shield_textures: HashMap<String, Handle<Image>>,
}

impl Default for EquipmentRenderingSystem {
    fn default() -> Self {
        let mut renderer = EquipmentRenderer::new();
        
        // 기본 캐릭터 앵커 등록
        renderer.register_character(CharacterAnchors::default_character("warrior_male"));
        renderer.register_character(CharacterAnchors::default_character("warrior_female"));
        renderer.register_character(CharacterAnchors::default_character("rogue_male"));
        renderer.register_character(CharacterAnchors::default_character("rogue_female"));
        renderer.register_character(CharacterAnchors::default_character("mage_male"));
        renderer.register_character(CharacterAnchors::default_character("mage_female"));
        
        Self {
            renderer,
            weapon_textures: HashMap::new(),
            shield_textures: HashMap::new(),
        }
    }
}

/// 장비 위치 업데이트 시스템
pub fn update_equipment_positions(
    equipment_system: Res<EquipmentRenderingSystem>,
    character_query: Query<(Entity, &Transform, &CharacterAnimState, &EquippedItems)>,
    mut equipment_query: Query<(&EquipmentSprite, &mut Transform, &mut Sprite), Without<CharacterAnimState>>,
) {
    for (char_entity, char_transform, anim_state, _equipped) in character_query.iter() {
        // 이 캐릭터에 연결된 장비 스프라이트들 업데이트
        for (equip_sprite, mut equip_transform, mut sprite) in equipment_query.iter_mut() {
            if equip_sprite.parent_entity != char_entity {
                continue;
            }

            // 앵커 위치 계산
            if let Some(render_info) = equipment_system.renderer.calculate_equipment_position(
                &anim_state.character_type,
                Point2D::new(char_transform.translation.x, char_transform.translation.y),
                anim_state.state,
                anim_state.direction,
                anim_state.current_frame,
                equip_sprite.anchor_type,
            ) {
                // 위치 업데이트
                equip_transform.translation.x = render_info.position.x;
                equip_transform.translation.y = render_info.position.y;
                equip_transform.translation.z = char_transform.translation.z + 0.1 * render_info.z_order as f32;
                
                // 회전 업데이트
                equip_transform.rotation = Quat::from_rotation_z(render_info.rotation.to_radians());
                
                // 좌우 반전
                sprite.flip_x = render_info.flip_x;
            }
        }
    }
}

/// 장비 스폰 헬퍼
pub fn spawn_equipment_sprite(
    commands: &mut Commands,
    parent_entity: Entity,
    anchor_type: AnchorType,
    texture: Handle<Image>,
) -> Entity {
    commands.spawn((
        Sprite {
            image: texture,
            ..default()
        },
        Transform::default(),
        EquipmentSprite {
            anchor_type,
            parent_entity,
        },
    )).id()
}

/// 장비 렌더링 플러그인
pub struct EquipmentRenderingPlugin;

impl Plugin for EquipmentRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EquipmentRenderingSystem>()
            .add_systems(Update, update_equipment_positions);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equipped_items() {
        let items = EquippedItems::new()
            .with_weapon("/assets/equipment/weapons/iron_sword.png")
            .with_shield("/assets/equipment/shields/wooden_shield.png");
        
        assert!(items.weapon.is_some());
        assert!(items.shield.is_some());
        assert!(items.helmet.is_none());
    }
}
