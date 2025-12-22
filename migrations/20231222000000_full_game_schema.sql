-- Full Game Schema for Legend of Darkness (Dark Fantasy RPG)

-- 1. Users & Authentication
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_login_at TIMESTAMP WITH TIME ZONE
);

-- 2. Static Game Data: Classes
CREATE TABLE classes (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL, -- Warrior, Rogue, Mage, Cleric, MartialArtist
    description TEXT
);

INSERT INTO classes (name, description) VALUES
('Warrior', 'Strong melee fighter using swords and high defense.'),
('Rogue', 'Agile fighter using daggers and stealth.'),
('Mage', 'Spellcaster using elemental magic.'),
('Cleric', 'Support class using holy magic and maces.'),
('MartialArtist', 'Melee fighter using knuckles and physical techniques.');

-- 3. Characters
CREATE TABLE characters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(50) UNIQUE NOT NULL,
    class_id INT REFERENCES classes(id),
    gender VARCHAR(10) NOT NULL CHECK (gender IN ('male', 'female')),
    level INT DEFAULT 1,
    exp BIGINT DEFAULT 0,
    hp INT DEFAULT 100,
    max_hp INT DEFAULT 100,
    mp INT DEFAULT 50,
    max_mp INT DEFAULT 50,
    str INT DEFAULT 10,
    dex INT DEFAULT 10,
    int_stat INT DEFAULT 10,
    wis INT DEFAULT 10,
    con INT DEFAULT 10,
    stat_points INT DEFAULT 0,
    gold BIGINT DEFAULT 0,
    map_id VARCHAR(50) DEFAULT 'village_milles', -- 'village_milles' or dungeon ID
    x INT DEFAULT 400,
    y INT DEFAULT 300,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 4. Static Data: Items
CREATE TABLE item_definitions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    item_type VARCHAR(20) NOT NULL, -- weapon, armor, consumable, etc
    sub_type VARCHAR(20), -- sword, staff, potion...
    grade INT DEFAULT 1,
    req_level INT DEFAULT 1,
    req_class INT REFERENCES classes(id), -- NULL = All
    stats JSONB DEFAULT '{}', -- { "attack": 10, "defense": 5 }
    price_buy BIGINT,
    price_sell BIGINT,
    icon_path VARCHAR(255)
);

-- 5. Inventory
CREATE TABLE character_inventory (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    character_id UUID REFERENCES characters(id) ON DELETE CASCADE,
    item_def_id INT REFERENCES item_definitions(id),
    quantity INT DEFAULT 1,
    slot_index INT NOT NULL,
    is_equipped BOOLEAN DEFAULT FALSE,
    equipped_slot VARCHAR(20), -- 'weapon', 'armor', etc.
    UNIQUE(character_id, slot_index)
);

-- 6. Static Data: Monsters
CREATE TABLE monster_definitions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    level INT NOT NULL,
    hp_max INT NOT NULL,
    mp_max INT NOT NULL,
    attack_min INT NOT NULL,
    attack_max INT NOT NULL,
    defense INT NOT NULL,
    exp_reward INT NOT NULL,
    gold_min INT NOT NULL,
    gold_max INT NOT NULL,
    sprite_path VARCHAR(255),
    ai_type VARCHAR(20) DEFAULT 'passive' -- passive, aggressive, defensive
);

-- 7. Static Data: Skills
CREATE TABLE skill_definitions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    class_id INT REFERENCES classes(id),
    req_level INT DEFAULT 1,
    mp_cost INT DEFAULT 0,
    cooldown_ms INT DEFAULT 0,
    description TEXT,
    effect_type VARCHAR(50), -- 'damage', 'heal', 'buff'
    base_value INT DEFAULT 0, -- base damage or heal amount
    icon_path VARCHAR(255)
);

CREATE TABLE character_skills (
    character_id UUID REFERENCES characters(id) ON DELETE CASCADE,
    skill_id INT REFERENCES skill_definitions(id),
    learned_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (character_id, skill_id)
);

-- 8. Static Data: Dungeons
CREATE TABLE dungeon_definitions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    level_req INT DEFAULT 1,
    required_clears_for_next INT DEFAULT 1,
    description TEXT
);

-- 9. Dungeon Progress
CREATE TABLE character_dungeon_progress (
    character_id UUID REFERENCES characters(id) ON DELETE CASCADE,
    dungeon_id INT REFERENCES dungeon_definitions(id),
    clear_count INT DEFAULT 0,
    last_entered_at TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (character_id, dungeon_id)
);

-- SEED DATA -----------------------------------------------------------

-- Items (Consolidated Weapons & Potions)
INSERT INTO item_definitions (name, item_type, sub_type, grade, req_class, stats, price_buy) VALUES
-- Potions
('Red Potion', 'consumable', 'potion', 1, NULL, '{"heal_hp": 50}', 50),
('Blue Potion', 'consumable', 'potion', 1, NULL, '{"heal_mp": 50}', 100),
-- Warrior Weapons
('Wooden Sword', 'weapon', 'sword', 1, 1, '{"attack": 5}', 100),
('Iron Sword', 'weapon', 'sword', 2, 1, '{"attack": 15}', 500),
('Steel Sword', 'weapon', 'sword', 3, 1, '{"attack": 30}', 1500),
-- Rogue Daggers
('Rusty Dagger', 'weapon', 'dagger', 1, 2, '{"attack": 3}', 100),
('Iron Dagger', 'weapon', 'dagger', 2, 2, '{"attack": 10}', 500),
-- Mage Staffs
('Wooden Staff', 'weapon', 'staff', 1, 3, '{"magic_attack": 5}', 100),
('Magic Staff', 'weapon', 'staff', 2, 3, '{"magic_attack": 15}', 500);

-- Monsters (1-99)
INSERT INTO monster_definitions (name, level, hp_max, mp_max, attack_min, attack_max, defense, exp_reward, gold_min, gold_max, sprite_path, ai_type) VALUES
('Giant Rat', 1, 20, 0, 2, 4, 0, 5, 1, 3, 'assets/monsters/rat.png', 'aggressive'),
('Vampire Bat', 3, 35, 0, 4, 6, 1, 10, 2, 5, 'assets/monsters/bat.png', 'aggressive'),
('Slime', 5, 50, 0, 6, 9, 2, 18, 5, 10, 'assets/monsters/slime.png', 'aggressive'),
('Corrupted Fox', 10, 120, 20, 15, 20, 5, 45, 15, 30, 'assets/monsters/fox.png', 'aggressive'),
('Wolf', 15, 200, 30, 25, 35, 10, 80, 25, 50, 'assets/monsters/wolf.png', 'aggressive'),
('Skeleton', 20, 350, 0, 40, 55, 15, 150, 40, 80, 'assets/monsters/skeleton.png', 'aggressive'),
('Goblin', 30, 600, 50, 60, 80, 25, 300, 80, 150, 'assets/monsters/goblin.png', 'aggressive'),
('Ghost', 40, 1000, 200, 90, 120, 10, 600, 120, 250, 'assets/monsters/ghost.png', 'aggressive'),
('Dark Knight', 60, 3000, 500, 200, 300, 100, 2000, 500, 1000, 'assets/monsters/dark_knight.png', 'aggressive'),
('Lich', 80, 8000, 5000, 500, 800, 200, 10000, 2000, 5000, 'assets/monsters/lich.png', 'defensive'),
('Red Dragon', 99, 50000, 10000, 1000, 2000, 500, 100000, 10000, 50000, 'assets/monsters/dragon.png', 'defensive');

-- Skills (Legend of Darkness inspired)
INSERT INTO skill_definitions (name, class_id, req_level, mp_cost, cooldown_ms, description, effect_type, base_value) VALUES
-- Warrior
('Bash', 1, 1, 10, 1000, 'Strong physical attack', 'damage', 50),
('Crash', 1, 10, 30, 3000, 'Powerful charging attack', 'damage', 150),
('Iron Will', 1, 20, 50, 60000, 'Increase defense temporarily', 'buff', 20),
-- Rogue
('Double Stab', 2, 1, 10, 1000, 'Stab twice quickly', 'damage', 40),
('Ambush', 2, 10, 30, 5000, 'Attack from shadows', 'damage', 180),
-- Mage
('Fireball', 3, 1, 15, 1500, 'Shoot a ball of fire', 'damage', 60),
('Thunder Bolt', 3, 10, 40, 4000, 'Strike with lightning', 'damage', 160),
('Ice Shield', 3, 20, 60, 45000, 'Absorb damage', 'buff', 50),
-- Cleric
('Heal', 4, 1, 20, 2000, 'Restore HP', 'heal', 40),
('Holy Bolt', 4, 5, 25, 2000, 'Holy damage attack', 'damage', 50),
('Great Heal', 4, 20, 60, 5000, 'Restore a lot of HP', 'heal', 150),
-- Martial Artist
('Punch', 5, 1, 5, 500, 'Quick punch', 'damage', 30),
('Power Kick', 5, 10, 20, 2000, 'Strong kick', 'damage', 80);

-- Dungeons
INSERT INTO dungeon_definitions (name, level_req, required_clears_for_next) VALUES
('Beginner Hunting Ground', 1, 10), -- Rabbit, Squirrel
('Slime Cave', 10, 10),            -- Slime, Fox
('Dark Forest', 20, 15),          -- Wolf, Skeleton
('Haunted Graveyard', 40, 20),    -- Ghost, Goblin
('Knights Castle', 60, 25),       -- Dark Knight
('Dragon Lair', 90, 1);           -- Dragon
