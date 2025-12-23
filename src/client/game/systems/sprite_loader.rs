//! 스프라이트 로더 - 스프라이트시트 기반 캐릭터/몬스터 관리
//!
//! 90년대 RPG 스타일의 스프라이트시트 시스템
//! 
//! ## 스프라이트시트 레이아웃
//! ```text
//! Row 0: IDLE   (4 프레임)
//! Row 1: MOVE   (4 프레임)
//! Row 2: ATTACK (6 프레임)
//! Row 3: DEATH  (4 프레임)
//! ```

#[cfg(feature = "csr")]
use web_sys::HtmlImageElement;
#[cfg(feature = "csr")]
use wasm_bindgen::prelude::*;

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

/// 애니메이션 상태 (스프라이트시트 행에 대응)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationState {
    Idle = 0,   // Row 0
    Move = 1,   // Row 1
    Attack = 2, // Row 2
    Death = 3,  // Row 3
}

impl AnimationState {
    pub fn row_index(&self) -> usize {
        *self as usize
    }

    pub fn frame_count(&self) -> usize {
        match self {
            AnimationState::Idle => 4,
            AnimationState::Move => 4,
            AnimationState::Attack => 6,
            AnimationState::Death => 4,
        }
    }

    pub fn frame_duration_ms(&self) -> f64 {
        match self {
            AnimationState::Idle => 150.0,
            AnimationState::Move => 100.0,
            AnimationState::Attack => 83.0,
            AnimationState::Death => 150.0,
        }
    }
}

/// 스프라이트시트 정보
#[derive(Debug, Clone)]
pub struct SpriteSheetInfo {
    pub frame_width: f64,
    pub frame_height: f64,
    pub columns: usize,
    pub rows: usize,
}

impl SpriteSheetInfo {
    /// 캐릭터용 (64x64)
    pub fn character() -> Self {
        Self {
            frame_width: 64.0,
            frame_height: 64.0,
            columns: 6,
            rows: 4,
        }
    }

    /// 소형 몬스터용 (32x32) - 레벨 1-10
    pub fn small_monster() -> Self {
        Self {
            frame_width: 32.0,
            frame_height: 32.0,
            columns: 6,
            rows: 4,
        }
    }

    /// 중형 몬스터용 (48x48) - 레벨 11-50
    pub fn medium_monster() -> Self {
        Self {
            frame_width: 48.0,
            frame_height: 48.0,
            columns: 6,
            rows: 4,
        }
    }

    /// 대형 몬스터용 (64x64) - 레벨 51+
    pub fn large_monster() -> Self {
        Self {
            frame_width: 64.0,
            frame_height: 64.0,
            columns: 6,
            rows: 4,
        }
    }

    /// 보스용 (128x128)
    #[allow(dead_code)]
    pub fn boss() -> Self {
        Self {
            frame_width: 128.0,
            frame_height: 128.0,
            columns: 6,
            rows: 4,
        }
    }

    /// 스킬 이펙트용 (단일 행)
    #[allow(dead_code)]
    pub fn skill_fx() -> Self {
        Self {
            frame_width: 64.0,
            frame_height: 64.0,
            columns: 6,
            rows: 1,
        }
    }

    /// 스프라이트시트에서 특정 프레임의 소스 좌표 계산
    pub fn get_source_rect(&self, state: AnimationState, frame: usize) -> (f64, f64, f64, f64) {
        let col = frame % self.columns;
        let row = state.row_index();
        
        let sx = col as f64 * self.frame_width;
        let sy = row as f64 * self.frame_height;
        
        (sx, sy, self.frame_width, self.frame_height)
    }
}

/// 캐싱 된 스프라이트시트 (hydrate only)
#[cfg(feature = "csr")]
pub struct CachedSpriteSheet {
    pub image: HtmlImageElement,
    pub info: SpriteSheetInfo,
    pub loaded: bool,
}

/// 스프라이트 로더 - 스프라이트시트 캐싱 및 관리
#[cfg(feature = "csr")]
pub struct SpriteLoader {
    cache: Rc<RefCell<HashMap<String, CachedSpriteSheet>>>,
}

#[cfg(feature = "csr")]
impl SpriteLoader {
    pub fn new() -> Self {
        Self {
            cache: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    /// 스프라이트시트 로드 (비동기)
    pub fn load(&self, path: &str, info: SpriteSheetInfo) {
        let mut cache = self.cache.borrow_mut();
        
        if cache.contains_key(path) {
            return;
        }

        if let Ok(img) = HtmlImageElement::new() {
            img.set_src(path);
            
            let path_clone = path.to_string();
            let cache_clone = self.cache.clone();
            
            let onload = Closure::wrap(Box::new(move || {
                let mut cache = cache_clone.borrow_mut();
                if let Some(sprite) = cache.get_mut(&path_clone) {
                    sprite.loaded = true;
                }
            }) as Box<dyn FnMut()>);
            
            img.set_onload(Some(onload.as_ref().unchecked_ref()));
            onload.forget();
            
            cache.insert(path.to_string(), CachedSpriteSheet {
                image: img,
                info,
                loaded: false,
            });
        }
    }

    /// 로드된 스프라이트시트 가져오기
    pub fn get(&self, path: &str) -> Option<(HtmlImageElement, SpriteSheetInfo)> {
        let cache = self.cache.borrow();
        cache.get(path).and_then(|sprite| {
            if sprite.loaded {
                Some((sprite.image.clone(), sprite.info.clone()))
            } else {
                None
            }
        })
    }

    /// 로드 여부 확인
    #[allow(dead_code)]
    pub fn is_loaded(&self, path: &str) -> bool {
        let cache = self.cache.borrow();
        cache.get(path).map(|s| s.loaded).unwrap_or(false)
    }
}

#[cfg(feature = "csr")]
impl Default for SpriteLoader {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================
// 경로 생성 헬퍼 (SSR에서도 사용 가능)
// =========================================

/// 캐릭터 스프라이트시트 경로
pub fn character_path(class: &str, gender: &str) -> String {
    format!(
        "/assets/characters/{}/{}_spritesheet.png",
        class.to_lowercase(),
        gender.to_lowercase()
    )
}

/// 몬스터 스프라이트시트 경로
pub fn monster_path(monster_type: &str) -> String {
    format!(
        "/assets/monsters/{}/spritesheet.png",
        monster_type.to_lowercase()
    )
}

/// 스킬 이펙트 경로
#[allow(dead_code)]
pub fn skill_fx_path(class: &str, skill_name: &str) -> String {
    format!(
        "/assets/skills/{}/{}_fx.png",
        class.to_lowercase(),
        skill_name.to_lowercase()
    )
}

/// 타일셋 경로
pub fn tileset_path() -> String {
    "/assets/tiles/ground/tileset.png".to_string()
}

/// 건물 시트 경로
pub fn buildings_path() -> String {
    "/assets/tiles/buildings/buildings.png".to_string()
}

/// 장식물 경로
pub fn decorations_path(name: &str) -> String {
    format!("/assets/tiles/decorations/{}.png", name.to_lowercase())
}

// =========================================
// 애니메이션 계산 (SSR에서도 사용 가능)
// =========================================

/// 애니메이션 프레임 계산기
pub struct AnimationCalculator;

impl AnimationCalculator {
    /// 루프 애니메이션 프레임 계산 (Idle, Move)
    pub fn get_loop_frame(current_time: f64, state: AnimationState) -> usize {
        let frame_count = state.frame_count();
        let frame_duration = state.frame_duration_ms();
        ((current_time / frame_duration) as usize) % frame_count
    }

    /// 원샷 애니메이션 프레임 계산 (Attack, Death)
    pub fn get_oneshot_frame(
        current_time: f64,
        start_time: f64,
        state: AnimationState,
    ) -> usize {
        let elapsed = current_time - start_time;
        let frame_count = state.frame_count();
        let total_duration = state.frame_duration_ms() * frame_count as f64;
        
        if elapsed >= total_duration {
            frame_count - 1
        } else {
            let frame_duration = state.frame_duration_ms();
            (elapsed / frame_duration) as usize
        }
    }

    /// 공격 애니메이션이 완료되었는지 확인
    pub fn is_attack_finished(current_time: f64, attack_start_time: f64) -> bool {
        let attack_duration = AnimationState::Attack.frame_duration_ms() 
            * AnimationState::Attack.frame_count() as f64;
        current_time - attack_start_time >= attack_duration
    }

    /// 사망 애니메이션이 완료되었는지 확인
    #[allow(dead_code)]
    pub fn is_death_finished(current_time: f64, death_start_time: f64) -> bool {
        let death_duration = AnimationState::Death.frame_duration_ms() 
            * AnimationState::Death.frame_count() as f64;
        current_time - death_start_time >= death_duration
    }
}

// =========================================
// 1타일 규칙 관련 (SSR에서도 사용 가능)
// =========================================

/// 1타일 렌더링 헬퍼
pub struct TileRenderer;

impl TileRenderer {
    /// 타일 크기 (아이소메트릭)
    pub const TILE_WIDTH: f64 = 64.0;
    pub const TILE_HEIGHT: f64 = 32.0;

    /// 스프라이트를 타일 중심에 맞춰 그릴 위치 계산
    pub fn get_draw_position(
        screen_x: f64,
        screen_y: f64,
        sprite_width: f64,
        sprite_height: f64,
    ) -> (f64, f64) {
        let draw_x = screen_x - sprite_width / 2.0;
        let draw_y = screen_y - sprite_height + Self::TILE_HEIGHT / 2.0;
        (draw_x, draw_y)
    }

    /// 그리드 좌표를 화면 좌표로 변환 (아이소메트릭)
    #[allow(dead_code)]
    pub fn grid_to_screen(grid_x: f64, grid_y: f64, origin_x: f64, origin_y: f64) -> (f64, f64) {
        let screen_x = origin_x + (grid_x - grid_y) * (Self::TILE_WIDTH / 2.0);
        let screen_y = origin_y + (grid_x + grid_y) * (Self::TILE_HEIGHT / 2.0);
        (screen_x, screen_y)
    }

    /// 화면 좌표를 그리드 좌표로 변환 (클릭 처리용)
    #[allow(dead_code)]
    pub fn screen_to_grid(screen_x: f64, screen_y: f64, origin_x: f64, origin_y: f64) -> (f64, f64) {
        let rel_x = screen_x - origin_x;
        let rel_y = screen_y - origin_y;
        
        let grid_x = (rel_x / (Self::TILE_WIDTH / 2.0) + rel_y / (Self::TILE_HEIGHT / 2.0)) / 2.0;
        let grid_y = (rel_y / (Self::TILE_HEIGHT / 2.0) - rel_x / (Self::TILE_WIDTH / 2.0)) / 2.0;
        
        (grid_x, grid_y)
    }
}
