-- Legend of Darkness M - Skill Data Update
-- This migration updates the skill definitions with appropriate names and values

-- Clean up existing skills to avoid duplicates with seed data
DELETE FROM character_skills;
DELETE FROM skill_definitions;

-- Warrior (Class ID: 1)
INSERT INTO skill_definitions (name, class_id, req_level, mp_cost, cooldown_ms, description, effect_type, base_value) VALUES
('skills.bash_name', 1, 1, 10, 1000, 'skills.bash_desc', 'damage', 20),
('skills.crash_name', 1, 10, 20, 3000, 'skills.crash_desc', 'damage', 50),
('skills.whirlwind_name', 1, 20, 35, 5000, 'skills.whirlwind_desc', 'damage', 40),
('skills.iron_skin_name', 1, 30, 50, 60000, 'skills.iron_skin_desc', 'buff', 10);

-- Rogue (Class ID: 2)
INSERT INTO skill_definitions (name, class_id, req_level, mp_cost, cooldown_ms, description, effect_type, base_value) VALUES
('skills.ambush_name', 2, 1, 15, 2000, 'skills.ambush_desc', 'damage', 30),
('skills.double_attack_name', 2, 10, 20, 1000, 'skills.double_attack_desc', 'damage', 40);

-- Mage (Class ID: 3)
INSERT INTO skill_definitions (name, class_id, req_level, mp_cost, cooldown_ms, description, effect_type, base_value) VALUES
('skills.magic_arrow_name', 3, 1, 5, 500, 'skills.magic_arrow_desc', 'damage', 15),
('skills.fireball_name', 3, 10, 25, 2000, 'skills.fireball_desc', 'damage', 60),
('skills.ice_shield_name', 3, 20, 40, 45000, 'skills.ice_shield_desc', 'buff', 50);

-- Cleric (Class ID: 4)
INSERT INTO skill_definitions (name, class_id, req_level, mp_cost, cooldown_ms, description, effect_type, base_value) VALUES
('skills.heal_name', 4, 1, 20, 2000, 'skills.heal_desc', 'heal', 30),
('skills.holy_light_name', 4, 5, 25, 3000, 'skills.holy_light_desc', 'damage', 40),
('skills.mass_heal_name', 4, 25, 60, 10000, 'skills.mass_heal_desc', 'heal', 80);

-- Martial Artist (Class ID: 5)
INSERT INTO skill_definitions (name, class_id, req_level, mp_cost, cooldown_ms, description, effect_type, base_value) VALUES
('skills.punch_name', 5, 1, 5, 500, 'skills.punch_desc', 'damage', 25),
('skills.spin_kick_name', 5, 12, 25, 4000, 'skills.spin_kick_desc', 'damage', 55);

-- General/Universal Skills (Class ID: NULL)
INSERT INTO skill_definitions (name, class_id, req_level, mp_cost, cooldown_ms, description, effect_type, base_value) VALUES
('skills.basic_attack_name', NULL, 1, 0, 800, 'skills.basic_attack_desc', 'damage', 5);
