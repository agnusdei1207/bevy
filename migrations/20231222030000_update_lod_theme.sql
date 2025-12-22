-- 1서클 직업 (레벨 1-11)
-- Update classes: Warrior, Rogue, Mage, Cleric, Martial Artist
DELETE FROM classes;
INSERT INTO classes (name, description, base_stats) VALUES
('전사', '강인한 체력과 파괴력. 근접 전투의 전문가.', '{"str": 10, "con": 10, "dex": 3, "wis": 3, "int": 3}'),
('도적', '빠른 속도와 기습 공격. 적의 배후를 노린다.', '{"str": 7, "con": 5, "dex": 10, "wis": 3, "int": 3}'),
('마법사', '강력한 원소 마법. 낮은 체력이지만 파괴적.', '{"str": 3, "con": 3, "dex": 4, "wis": 5, "int": 10}'),
('성직자', '신성한 힘으로 아군을 치유하고 보호한다.', '{"str": 4, "con": 5, "dex": 3, "wis": 10, "int": 7}'),
('무도가', '육체를 극한으로 단련한 격투가. 화려한 연계기.', '{"str": 8, "con": 8, "dex": 8, "wis": 3, "int": 3}');

-- 1서클 몬스터 (Dark Fantasy / LoD Style)
DELETE FROM monsters;
INSERT INTO monsters (name, description, sprite_path, level, hp, attack_min, attack_max, defense, magic_defense, exp_reward, gold_min, gold_max, ai_type, detection_range, attack_range, move_speed, spawn_map) VALUES
('쥐', '어두운 하수구의 쥐.', 'rat', 1, 10, 1, 3, 0, 0, 1, 1, 2, 'passive', 100, 30, 50, 'village'),
('박쥐', '동굴 천장에 매달린 흡혈 박쥐.', 'bat', 2, 20, 3, 5, 1, 0, 3, 2, 4, 'aggressive', 150, 30, 80, 'dungeon_1'),
('스켈레톤', '죽지 않는 해골 병사.', 'skeleton', 4, 50, 8, 12, 3, 2, 10, 5, 10, 'aggressive', 200, 35, 60, 'dungeon_1'),
('코볼트', '교활한 광산의 약탈자.', 'kobold', 3, 35, 5, 8, 2, 1, 6, 3, 7, 'passive', 120, 35, 70, 'field'),
('거대 거미', '독을 품은 거대한 거미.', 'spider', 5, 70, 10, 15, 4, 3, 15, 8, 15, 'aggressive', 180, 40, 75, 'field'),
('구울', '시체를 먹는 몬스터. 마비 공격.', 'ghoul', 7, 120, 15, 20, 6, 4, 25, 12, 25, 'aggressive', 150, 30, 55, 'dungeon_2'),
('라이칸스로프', '늑대인간. 빠르고 치명적.', 'werewolf', 10, 300, 30, 45, 10, 8, 80, 40, 80, 'aggressive', 300, 50, 110, 'dungeon_deep'),
('서큐버스', '몽마. 마법 공격을 사용.', 'succubus', 9, 200, 20, 30, 5, 15, 50, 30, 60, 'aggressive', 250, 120, 90, 'dungeon_deep');

-- 1서클 아이템 (기본 장비)
DELETE FROM items;
INSERT INTO items (name, description, item_type, sub_type, rarity, level_requirement, stat_bonuses, buy_price, sell_price) VALUES
('목검', '수련용 나무 검', 'weapon', 'sword', 'common', 1, '{"attack_min": 2, "attack_max": 4}', 10, 2),
('숏소드', '짧은 철제 검', 'weapon', 'sword', 'common', 5, '{"attack_min": 6, "attack_max": 10}', 100, 20),
('메이스', '성직자용 둔기', 'weapon', 'mace', 'common', 1, '{"attack_min": 3, "attack_max": 5}', 15, 3),
('우드완드', '초보 마법사용 지팡이', 'weapon', 'staff', 'common', 1, '{"magic_attack": 5}', 20, 4),
('가죽 장갑', '격투가용 장갑', 'weapon', 'knuckle', 'common', 1, '{"attack_min": 2, "attack_max": 3}', 15, 3),
('단검', '예리한 단검', 'weapon', 'dagger', 'common', 1, '{"attack_min": 2, "attack_max": 4}', 15, 3),
('천옷', '허름한 옷', 'armor', 'cloth', 'common', 1, '{"defense": 1}', 10, 2),
('가죽갑옷', '질긴 가죽 갑옷', 'armor', 'leather', 'common', 5, '{"defense": 5}', 100, 20);

-- Maps
DELETE FROM maps;
INSERT INTO maps (name, display_name, description, bgm_path) VALUES
('village', '로톤 마을', '어둠 속에서 살아남은 자들의 마을.', '/assets/bgm/village.mp3'),
('field', '버려진 들판', '마을 밖의 위험한 들판.', '/assets/bgm/field.mp3'),
('dungeon_1', '지하 수로 1층', '습하고 어두운 지하 수로.', '/assets/bgm/dungeon.mp3'),
('dungeon_2', '지하 수로 2층', '더 깊고 위험한 곳.', '/assets/bgm/dungeon.mp3'),
('dungeon_deep', '고대의 무덤', '저주받은 자들의 안식처.', '/assets/bgm/boss.mp3');
