-- =====================================================
-- Legend of Darkness M - Base Schema (Simplified)
-- =====================================================
-- 
-- 정적 데이터는 모두 Rust const로 관리됩니다:
--   - 몬스터: src/shared/data/monsters.rs
--   - 스킬: src/shared/data/skills.rs  
--   - 아이템: src/shared/data/items.rs
--   - 맵/던전: src/shared/data/maps.rs
--   - 경험치 테이블: src/shared/data/characters.rs
--
-- DB에는 동적 사용자 데이터만 저장됩니다.
-- =====================================================

-- 1. Users (계정)
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_login_at TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN DEFAULT TRUE
);

-- 2. Classes (클래스 참조 테이블)
-- 실제 스탯/스킬은 const로 관리, 여기는 FK 참조용
CREATE TABLE IF NOT EXISTS classes (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL
);

INSERT INTO classes (id, name) VALUES
(1, 'Warrior'),
(2, 'Rogue'),
(3, 'Mage'),
(4, 'Cleric'),
(5, 'MartialArtist')
ON CONFLICT (id) DO NOTHING;

-- 3. Characters (캐릭터)
CREATE TABLE IF NOT EXISTS characters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(50) UNIQUE NOT NULL,
    class_id INT REFERENCES classes(id),
    gender VARCHAR(10) NOT NULL CHECK (gender IN ('male', 'female')),
    
    -- 레벨/경험치 (만렙: 99)
    level INT DEFAULT 1 CHECK (level >= 1 AND level <= 99),
    exp BIGINT DEFAULT 0,  -- 현재 레벨에서 획득한 경험치
    total_exp BIGINT DEFAULT 0,  -- 누적 총 경험치
    
    -- HP/MP (현재값)
    hp INT DEFAULT 100,
    mp INT DEFAULT 50,
    
    -- 추가 스탯 (기본 스탯은 클래스별 const에서 가져옴)
    bonus_str_stat INT DEFAULT 0,
    bonus_dex_stat INT DEFAULT 0,
    bonus_int_stat INT DEFAULT 0,
    bonus_wis_stat INT DEFAULT 0,
    bonus_con_stat INT DEFAULT 0,
    stat_points INT DEFAULT 0,  -- 미배분 스탯 포인트
    
    -- 재화
    gold BIGINT DEFAULT 100,
    
    -- 위치 (맵 ID는 const의 MapDef.id 참조)
    current_map VARCHAR(50) DEFAULT 'village_milles',
    pos_x FLOAT DEFAULT 400,
    pos_y FLOAT DEFAULT 300,
    
    -- 통계
    monsters_killed INT DEFAULT 0,
    deaths INT DEFAULT 0,
    play_time_seconds BIGINT DEFAULT 0,
    
    -- 타임스탬프
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_played_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 4. Character Inventory (인벤토리)
-- item_id는 const의 ItemDef.id를 참조
CREATE TABLE IF NOT EXISTS character_inventory (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    character_id UUID REFERENCES characters(id) ON DELETE CASCADE,
    item_id INT NOT NULL,  -- const ItemDef.id 참조
    quantity INT DEFAULT 1 CHECK (quantity >= 1),
    slot_index INT,  -- NULL이면 가방, 숫자면 슬롯 위치
    is_equipped BOOLEAN DEFAULT FALSE,
    equipped_slot VARCHAR(20),  -- 'weapon', 'armor', 'helmet', etc.
    enhancement_level INT DEFAULT 0 CHECK (enhancement_level >= 0),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 5. Character Skills (습득한 스킬)
-- skill_id는 const의 SkillDef.id를 참조
CREATE TABLE IF NOT EXISTS character_skills (
    character_id UUID REFERENCES characters(id) ON DELETE CASCADE,
    skill_id INT NOT NULL,  -- const SkillDef.id 참조
    skill_level INT DEFAULT 1,
    slot_index INT,  -- 스킬바 슬롯 (1-5)
    learned_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (character_id, skill_id)
);

-- 6. Character Dungeon Progress (던전 진행도)
-- dungeon_id는 const의 DungeonDef.id를 참조
CREATE TABLE IF NOT EXISTS character_dungeon_progress (
    character_id UUID REFERENCES characters(id) ON DELETE CASCADE,
    dungeon_id INT NOT NULL,  -- const DungeonDef.id 참조
    clear_count INT DEFAULT 0,
    best_clear_time_ms BIGINT,
    last_entered_at TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (character_id, dungeon_id)
);

-- 인덱스
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_characters_user_id ON characters(user_id);
CREATE INDEX IF NOT EXISTS idx_characters_name ON characters(name);
CREATE INDEX IF NOT EXISTS idx_character_inventory_character ON character_inventory(character_id);
CREATE INDEX IF NOT EXISTS idx_character_inventory_equipped ON character_inventory(character_id, is_equipped) WHERE is_equipped = TRUE;
