//! Paper Doll (Layering) System
//!
//! 어둠의전설의 종이인형 시스템을 구현합니다.
//! 캐릭터를 여러 레이어로 분리하여 장비 교체 시 해당 레이어의 스프라이트만 교체합니다.
//!
//! ## 레이어 구조
//! - **Body**: 기본 몸체 (성별/종족에 따라 다름)
//! - **Hair**: 머리카락 스타일
//! - **Armor/Clothing**: 갑옷/의상
//! - **Weapon**: 무기 (손에 들고 있는 것)
//! - **Accessory**: 악세서리 (망토, 날개 등)
//! - **Effect**: 이펙트 오버레이 (버프, 오라 등)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 레이어 타입 (렌더링 순서 = 레이어 값)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum LayerType {
    /// 그림자 (가장 아래)
    Shadow = 0,
    /// 기본 몸체
    Body = 1,
    /// 하의 (바지, 치마)
    Pants = 2,
    /// 상의 (갑옷, 로브)
    Armor = 3,
    /// 머리카락
    Hair = 4,
    /// 투구/모자
    Helmet = 5,
    /// 무기
    Weapon = 6,
    /// 방패 (왼손)
    Shield = 7,
    /// 망토/날개
    Cape = 8,
    /// 이펙트 (버프 오라 등, 가장 위)
    Effect = 9,
}

impl LayerType {
    /// 렌더링 순서 값 (낮을수록 먼저 렌더링)
    pub fn z_order(&self) -> i32 {
        *self as i32
    }

    /// 모든 레이어 타입 (렌더링 순서대로)
    pub fn all_ordered() -> Vec<LayerType> {
        vec![
            LayerType::Shadow,
            LayerType::Body,
            LayerType::Pants,
            LayerType::Armor,
            LayerType::Hair,
            LayerType::Helmet,
            LayerType::Weapon,
            LayerType::Shield,
            LayerType::Cape,
            LayerType::Effect,
        ]
    }

    /// 필수 레이어 여부
    pub fn is_required(&self) -> bool {
        matches!(self, LayerType::Body)
    }
}

/// 개별 레이어 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerInfo {
    /// 레이어 타입
    pub layer_type: LayerType,
    
    /// 스프라이트 매니페스트 ID (참조)
    pub manifest_id: String,
    
    /// 레이어 오프셋 (body 기준 상대 위치)
    #[serde(default)]
    pub offset: (f32, f32),
    
    /// 레이어 스케일 (1.0 = 원본 크기)
    #[serde(default = "default_scale")]
    pub scale: f32,
    
    /// 색상 틴트 (RGBA hex, 예: "#FFFFFFFF")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint: Option<String>,
    
    /// 이 레이어가 활성화 되어 있는지
    #[serde(default = "default_true")]
    pub visible: bool,
}

fn default_scale() -> f32 { 1.0 }
fn default_true() -> bool { true }

impl LayerInfo {
    pub fn new(layer_type: LayerType, manifest_id: &str) -> Self {
        Self {
            layer_type,
            manifest_id: manifest_id.to_string(),
            offset: (0.0, 0.0),
            scale: 1.0,
            tint: None,
            visible: true,
        }
    }

    pub fn with_offset(mut self, x: f32, y: f32) -> Self {
        self.offset = (x, y);
        self
    }

    pub fn with_tint(mut self, tint: &str) -> Self {
        self.tint = Some(tint.to_string());
        self
    }
}

/// 캐릭터 외형 정의 (Paper Doll 조합)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAppearance {
    /// 캐릭터 고유 ID
    pub character_id: String,
    
    /// 기본 몸체 타입 (성별/종족에 따라 다름)
    pub body_type: String,
    
    /// 머리카락 스타일 ID
    pub hair_style: String,
    
    /// 머리카락 색상 (팔레트 이름 또는 hex)
    #[serde(default)]
    pub hair_color: String,
    
    /// 장착 레이어들
    pub layers: Vec<LayerInfo>,
}

impl CharacterAppearance {
    /// 새 캐릭터 외형 생성 (기본 body만)
    pub fn new(character_id: &str, body_type: &str) -> Self {
        let body_layer = LayerInfo::new(
            LayerType::Body,
            &format!("body_{}", body_type),
        );

        Self {
            character_id: character_id.to_string(),
            body_type: body_type.to_string(),
            hair_style: "default".to_string(),
            hair_color: "#8B4513".to_string(), // 기본 갈색
            layers: vec![body_layer],
        }
    }

    /// 레이어 추가 또는 교체
    pub fn set_layer(&mut self, layer: LayerInfo) {
        // 같은 타입의 기존 레이어 제거
        self.layers.retain(|l| l.layer_type != layer.layer_type);
        self.layers.push(layer);
        // 렌더링 순서대로 정렬
        self.layers.sort_by_key(|l| l.layer_type.z_order());
    }

    /// 레이어 제거
    pub fn remove_layer(&mut self, layer_type: LayerType) {
        if !layer_type.is_required() {
            self.layers.retain(|l| l.layer_type != layer_type);
        }
    }

    /// 특정 레이어 가져오기
    pub fn get_layer(&self, layer_type: LayerType) -> Option<&LayerInfo> {
        self.layers.iter().find(|l| l.layer_type == layer_type)
    }

    /// 모든 활성 레이어 (렌더링 순서대로)
    pub fn visible_layers(&self) -> Vec<&LayerInfo> {
        self.layers.iter().filter(|l| l.visible).collect()
    }

    /// 머리카락 스타일 변경
    pub fn set_hair_style(&mut self, style: &str, color: &str) {
        self.hair_style = style.to_string();
        self.hair_color = color.to_string();
        
        // 머리카락 레이어 업데이트
        let hair_layer = LayerInfo::new(
            LayerType::Hair,
            &format!("hair_{}", style),
        ).with_tint(color);
        
        self.set_layer(hair_layer);
    }

    /// 무기 장착
    pub fn equip_weapon(&mut self, weapon_id: &str) {
        let weapon_layer = LayerInfo::new(
            LayerType::Weapon,
            &format!("weapon_{}", weapon_id),
        );
        self.set_layer(weapon_layer);
    }

    /// 갑옷 장착
    pub fn equip_armor(&mut self, armor_id: &str) {
        let armor_layer = LayerInfo::new(
            LayerType::Armor,
            &format!("armor_{}", armor_id),
        );
        self.set_layer(armor_layer);
    }
}

/// 장비 슬롯 -> 레이어 타입 매핑
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipmentSlot {
    Weapon,
    Shield,
    Helmet,
    Armor,
    Pants,
    Cape,
}

impl EquipmentSlot {
    /// 장비 슬롯에 해당하는 레이어 타입
    pub fn to_layer_type(&self) -> LayerType {
        match self {
            EquipmentSlot::Weapon => LayerType::Weapon,
            EquipmentSlot::Shield => LayerType::Shield,
            EquipmentSlot::Helmet => LayerType::Helmet,
            EquipmentSlot::Armor => LayerType::Armor,
            EquipmentSlot::Pants => LayerType::Pants,
            EquipmentSlot::Cape => LayerType::Cape,
        }
    }
}

/// Paper Doll 렌더러 설정
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperDollConfig {
    /// 기본 캐릭터 크기 (픽셀)
    pub base_size: (u32, u32),
    
    /// 그림자 오프셋
    pub shadow_offset: (f32, f32),
    
    /// 그림자 투명도 (0.0 - 1.0)
    pub shadow_alpha: f32,
    
    /// 레이어 간 Z 간격
    pub z_spacing: f32,
}

impl Default for PaperDollConfig {
    fn default() -> Self {
        Self {
            base_size: (48, 64),
            shadow_offset: (0.0, -4.0),
            shadow_alpha: 0.3,
            z_spacing: 0.01,
        }
    }
}

/// 스프라이트 레이어 캐시 (최적화용)
#[derive(Debug, Default)]
pub struct LayerCache {
    /// 매니페스트 ID -> 로드된 이미지 핸들 (실제 구현에서는 Handle<Image>)
    pub loaded_manifests: HashMap<String, bool>,
    
    /// 캐릭터 ID -> 조합된 외형
    pub character_appearances: HashMap<String, CharacterAppearance>,
}

impl LayerCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// 캐릭터 외형 등록
    pub fn register_appearance(&mut self, appearance: CharacterAppearance) {
        let id = appearance.character_id.clone();
        self.character_appearances.insert(id, appearance);
    }

    /// 캐릭터 외형 가져오기
    pub fn get_appearance(&self, character_id: &str) -> Option<&CharacterAppearance> {
        self.character_appearances.get(character_id)
    }

    /// 캐릭터 외형 수정
    pub fn get_appearance_mut(&mut self, character_id: &str) -> Option<&mut CharacterAppearance> {
        self.character_appearances.get_mut(character_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_ordering() {
        let layers = LayerType::all_ordered();
        assert_eq!(layers[0], LayerType::Shadow);
        assert_eq!(layers[layers.len() - 1], LayerType::Effect);
    }

    #[test]
    fn test_character_appearance() {
        let mut appearance = CharacterAppearance::new("player1", "male_human");
        
        // 무기 장착
        appearance.equip_weapon("iron_sword");
        assert!(appearance.get_layer(LayerType::Weapon).is_some());
        
        // 갑옷 장착
        appearance.equip_armor("leather_armor");
        assert!(appearance.get_layer(LayerType::Armor).is_some());
        
        // 레이어 순서 확인
        let visible = appearance.visible_layers();
        assert!(visible.len() >= 2);
    }
}
