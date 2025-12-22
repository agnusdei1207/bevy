//! 오디오 시스템 - BGM 및 효과음 관리
//! 
//! hydrate feature가 활성화된 경우에만 실제 오디오를 재생합니다.

#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "hydrate")]
use web_sys::HtmlAudioElement;

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

/// 오디오 관리자 - BGM과 SFX를 관리
pub struct AudioManager {
    #[cfg(feature = "hydrate")]
    bgm_player: Option<HtmlAudioElement>,
    #[cfg(not(feature = "hydrate"))]
    bgm_player: Option<()>,
    
    #[cfg(feature = "hydrate")]
    sfx_cache: Rc<RefCell<HashMap<String, HtmlAudioElement>>>,
    #[cfg(not(feature = "hydrate"))]
    sfx_cache: Rc<RefCell<HashMap<String, ()>>>,
    
    bgm_volume: f64,
    sfx_volume: f64,
    is_muted: bool,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            bgm_player: None,
            sfx_cache: Rc::new(RefCell::new(HashMap::new())),
            bgm_volume: 0.5,
            sfx_volume: 0.7,
            is_muted: false,
        }
    }

    /// BGM 재생
    #[cfg(feature = "hydrate")]
    pub fn play_bgm(&mut self, path: &str) {
        // 이전 BGM 정지
        if let Some(ref player) = self.bgm_player {
            let _ = player.pause();
        }

        if let Ok(audio) = HtmlAudioElement::new_with_src(path) {
            audio.set_loop(true);
            let _ = audio.set_volume(if self.is_muted { 0.0 } else { self.bgm_volume });
            let _ = audio.play();
            self.bgm_player = Some(audio);
        }
    }

    #[cfg(not(feature = "hydrate"))]
    pub fn play_bgm(&mut self, _path: &str) {
        // SSR에서는 아무것도 하지 않음
    }

    /// BGM 정지
    #[cfg(feature = "hydrate")]
    pub fn stop_bgm(&mut self) {
        if let Some(ref player) = self.bgm_player {
            let _ = player.pause();
            player.set_current_time(0.0);
        }
    }

    #[cfg(not(feature = "hydrate"))]
    pub fn stop_bgm(&mut self) {}

    /// BGM 일시정지
    #[cfg(feature = "hydrate")]
    pub fn pause_bgm(&self) {
        if let Some(ref player) = self.bgm_player {
            let _ = player.pause();
        }
    }

    #[cfg(not(feature = "hydrate"))]
    pub fn pause_bgm(&self) {}

    /// BGM 재개
    #[cfg(feature = "hydrate")]
    pub fn resume_bgm(&self) {
        if let Some(ref player) = self.bgm_player {
            let _ = player.play();
        }
    }

    #[cfg(not(feature = "hydrate"))]
    pub fn resume_bgm(&self) {}

    /// 효과음 재생
    #[cfg(feature = "hydrate")]
    pub fn play_sfx(&self, path: &str) {
        if self.is_muted {
            return;
        }

        let mut cache = self.sfx_cache.borrow_mut();
        
        if let Some(audio) = cache.get(path) {
            // 이미 재생 중이면 복제본 사용
            if let Ok(cloned) = audio.clone_node() {
                if let Ok(audio_clone) = cloned.dyn_into::<HtmlAudioElement>() {
                    let _ = audio_clone.set_volume(self.sfx_volume);
                    let _ = audio_clone.play();
                }
            }
        } else {
            // 새로 로드
            if let Ok(audio) = HtmlAudioElement::new_with_src(path) {
                let _ = audio.set_volume(self.sfx_volume);
                let _ = audio.play();
                cache.insert(path.to_string(), audio);
            }
        }
    }

    #[cfg(not(feature = "hydrate"))]
    pub fn play_sfx(&self, _path: &str) {}

    /// BGM 볼륨 설정 (0.0 - 1.0)
    pub fn set_bgm_volume(&mut self, volume: f64) {
        self.bgm_volume = volume.clamp(0.0, 1.0);
        #[cfg(feature = "hydrate")]
        if let Some(ref player) = self.bgm_player {
            let _ = player.set_volume(if self.is_muted { 0.0 } else { self.bgm_volume });
        }
    }

    /// SFX 볼륨 설정 (0.0 - 1.0)
    pub fn set_sfx_volume(&mut self, volume: f64) {
        self.sfx_volume = volume.clamp(0.0, 1.0);
    }

    /// 음소거 토글
    pub fn toggle_mute(&mut self) -> bool {
        self.is_muted = !self.is_muted;
        #[cfg(feature = "hydrate")]
        if let Some(ref player) = self.bgm_player {
            let _ = player.set_volume(if self.is_muted { 0.0 } else { self.bgm_volume });
        }
        self.is_muted
    }

    /// 음소거 상태 확인
    pub fn is_muted(&self) -> bool {
        self.is_muted
    }
}

/// 오디오 경로 상수
#[allow(dead_code)]
pub mod audio_paths {
    // BGM
    pub const BGM_MAIN_THEME: &str = "/assets/audio/bgm/main_theme.mp3";
    pub const BGM_VILLAGE: &str = "/assets/audio/bgm/village.mp3";
    pub const BGM_DUNGEON: &str = "/assets/audio/bgm/dungeon.mp3";
    pub const BGM_BOSS_BATTLE: &str = "/assets/audio/bgm/boss_battle.mp3";

    // 공격 효과음
    pub const SFX_SWORD_SWING: &str = "/assets/audio/sfx/attack/sword_swing.mp3";
    pub const SFX_HIT_FLESH: &str = "/assets/audio/sfx/attack/hit_flesh.mp3";
    pub const SFX_HIT_METAL: &str = "/assets/audio/sfx/attack/hit_metal.mp3";

    // 스킬 효과음
    pub const SFX_FIRE_CAST: &str = "/assets/audio/sfx/skill/fire_cast.mp3";
    pub const SFX_ICE_CAST: &str = "/assets/audio/sfx/skill/ice_cast.mp3";
    pub const SFX_HEAL_CAST: &str = "/assets/audio/sfx/skill/heal_cast.mp3";

    // UI 효과음
    pub const SFX_CLICK: &str = "/assets/audio/sfx/ui/click.mp3";
    pub const SFX_OPEN_MENU: &str = "/assets/audio/sfx/ui/open_menu.mp3";
    pub const SFX_LEVEL_UP: &str = "/assets/audio/sfx/ui/level_up.mp3";
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new()
    }
}
