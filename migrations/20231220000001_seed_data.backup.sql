-- 1서클 (레벨 1-11) 게임 데이터
-- 어둠의전설 벤치마크

-- 1서클 직업 설명 업데이트
UPDATE classes SET description = '검과 방패로 전장을 누비는 용맹한 전사. 높은 HP와 방어력으로 파티의 방패 역할' WHERE name = '전사';
UPDATE classes SET description = '활과 화살로 원거리에서 적을 제압. 민첩성과 치명타로 빠른 사냥' WHERE name = '궁수';
UPDATE classes SET description = '마나를 이용해 강력한 마법을 시전. 광역 공격과 속성 마법의 달인' WHERE name = '마법사';
UPDATE classes SET description = '그림자 속에서 치명적인 일격을 가하는 암살자. 크리티컬과 회피의 달인' WHERE name = '도적';

-- 1서클 몬스터 (레벨 1-11)
DELETE FROM monsters;
INSERT INTO monsters (name, description, sprite_path, level, hp, attack_min, attack_max, defense, magic_defense, exp_reward, gold_min, gold_max, ai_type, detection_range, attack_range, move_speed, spawn_map) VALUES
-- 초보자 마을 외곽 (Lv.1-3)
('슬라임', '투명하고 말랑말랑한 젤리 몬스터. 초보 모험가의 첫 상대', '/assets/monsters/slime.png', 1, 25, 2, 4, 1, 1, 3, 1, 3, 'passive', 100, 30, 60, 'village_outskirts'),
('아기 슬라임', '작고 귀여운 슬라임. 매우 약함', '/assets/monsters/baby_slime.png', 1, 15, 1, 2, 0, 0, 2, 1, 2, 'passive', 80, 25, 50, 'village_outskirts'),
('파란 슬라임', '파란색 슬라임. 일반 슬라임보다 약간 강함', '/assets/monsters/blue_slime.png', 2, 35, 3, 6, 2, 1, 5, 2, 5, 'passive', 120, 35, 70, 'village_outskirts'),
('달팽이', '느리지만 단단한 껍질을 가진 달팽이', '/assets/monsters/snail.png', 2, 40, 2, 5, 4, 2, 6, 2, 4, 'passive', 80, 25, 40, 'village_outskirts'),
('주황버섯', '주황색 독버섯. 독 포자를 뿌림', '/assets/monsters/orange_mushroom.png', 3, 50, 5, 8, 3, 2, 8, 3, 7, 'passive', 130, 40, 65, 'village_outskirts'),

-- 숲 입구 (Lv.3-5)
('초록버섯', '숲에 사는 평범한 버섯 몬스터', '/assets/monsters/green_mushroom.png', 3, 55, 5, 9, 4, 3, 10, 4, 8, 'passive', 140, 40, 70, 'forest_entrance'),
('스텀프', '움직이는 나무 그루터기. 느리지만 강함', '/assets/monsters/stump.png', 4, 80, 8, 12, 8, 4, 15, 5, 12, 'passive', 100, 45, 50, 'forest_entrance'),
('돼지', '야생 돼지. 가끔 돌진 공격을 함', '/assets/monsters/pig.png', 4, 70, 7, 11, 5, 3, 12, 6, 10, 'aggressive', 180, 50, 90, 'forest_entrance'),
('리본돼지', '리본을 단 귀여운 돼지. 하지만 화나면 무섭다', '/assets/monsters/ribbon_pig.png', 5, 90, 10, 15, 6, 4, 18, 8, 15, 'aggressive', 200, 55, 95, 'forest_entrance'),

-- 깊은 숲 (Lv.5-7)
('악어', '늪지대의 악어. 강력한 턱 공격', '/assets/monsters/crocodile.png', 5, 100, 12, 18, 10, 5, 22, 10, 20, 'aggressive', 180, 60, 80, 'deep_forest'),
('야생 멧돼지', '거대한 멧돼지. 돌진 공격이 위험', '/assets/monsters/wild_boar.png', 6, 130, 15, 22, 12, 6, 30, 12, 25, 'aggressive', 220, 65, 100, 'deep_forest'),
('고블린', '작은 녹색 고블린. 무리 지어 다님', '/assets/monsters/goblin.png', 6, 110, 13, 20, 8, 5, 25, 15, 30, 'aggressive', 250, 55, 110, 'deep_forest'),
('고블린 전사', '갑옷을 입은 고블린. 일반 고블린보다 강함', '/assets/monsters/goblin_warrior.png', 7, 150, 18, 28, 15, 8, 40, 20, 40, 'aggressive', 280, 60, 100, 'deep_forest'),

-- 던전 입구 (Lv.7-9)
('스켈레톤', '부활한 해골 병사. 검을 들고 있음', '/assets/monsters/skeleton.png', 7, 140, 16, 25, 12, 10, 35, 18, 35, 'aggressive', 200, 70, 90, 'dungeon_entrance'),
('좀비', '느리지만 강한 언데드. HP가 높음', '/assets/monsters/zombie.png', 8, 200, 20, 30, 10, 8, 45, 20, 45, 'passive', 150, 50, 60, 'dungeon_entrance'),
('스켈레톤 궁수', '활을 쏘는 해골. 원거리 공격', '/assets/monsters/skeleton_archer.png', 8, 120, 22, 32, 8, 10, 50, 25, 50, 'aggressive', 350, 250, 85, 'dungeon_entrance'),
('버블링', '던전 물웅덩이의 젤리 몬스터', '/assets/monsters/bubbling.png', 9, 180, 18, 28, 15, 12, 55, 22, 48, 'passive', 180, 45, 70, 'dungeon_entrance'),

-- 던전 깊은 곳 (Lv.9-11)
('리자드맨', '도마뱀 인간. 창을 사용함', '/assets/monsters/lizardman.png', 9, 220, 25, 38, 18, 15, 70, 30, 60, 'aggressive', 250, 80, 95, 'dungeon_deep'),
('다크 스켈레톤', '어둠으로 강화된 해골', '/assets/monsters/dark_skeleton.png', 10, 250, 30, 45, 20, 18, 85, 35, 70, 'aggressive', 280, 75, 100, 'dungeon_deep'),
('고스트', '유령. 물리 공격에 저항성', '/assets/monsters/ghost.png', 10, 180, 35, 50, 5, 25, 90, 40, 80, 'aggressive', 300, 100, 120, 'dungeon_deep'),
('뮤미', '고대 미이라. 저주 공격', '/assets/monsters/mummy.png', 11, 300, 40, 55, 25, 20, 110, 50, 100, 'aggressive', 220, 70, 80, 'dungeon_deep'),

-- 1서클 보스
('슬라임킹', '거대한 슬라임들의 왕. 분열 능력', '/assets/monsters/slime_king.png', 10, 800, 35, 50, 15, 15, 500, 200, 400, 'aggressive', 400, 100, 60, 'village_outskirts'),
('고블린 대장', '고블린 부족의 우두머리. 부하들을 소환', '/assets/monsters/goblin_chief.png', 12, 1200, 50, 70, 25, 20, 800, 400, 800, 'aggressive', 450, 80, 80, 'deep_forest'),
('해골 기사', '던전을 지키는 강력한 언데드 기사', '/assets/monsters/skeleton_knight.png', 15, 2000, 65, 90, 35, 30, 1500, 700, 1500, 'aggressive', 350, 90, 90, 'dungeon_deep');

-- 1서클 아이템 (레벨 1-11용)
DELETE FROM items;
INSERT INTO items (name, description, item_type, sub_type, rarity, level_requirement, stat_bonuses, buy_price, sell_price, max_stack, can_be_enhanced, max_enhancement) VALUES
-- 전사 무기
('나무 검', '초보자용 나무 검', 'weapon', 'sword', 'common', 1, '{"attack_min": 2, "attack_max": 4}', 50, 12, 1, true, 5),
('낡은 검', '녹이 슨 오래된 검', 'weapon', 'sword', 'common', 3, '{"attack_min": 4, "attack_max": 8, "str": 1}', 200, 50, 1, true, 7),
('철 검', '단단한 철로 만든 검', 'weapon', 'sword', 'uncommon', 5, '{"attack_min": 8, "attack_max": 14, "str": 2}', 800, 200, 1, true, 7),
('기사의 검', '기사단에서 사용하던 검', 'weapon', 'sword', 'uncommon', 7, '{"attack_min": 12, "attack_max": 20, "str": 3}', 2000, 500, 1, true, 7),
('미스릴 검', '미스릴로 제작된 가벼운 검', 'weapon', 'sword', 'rare', 10, '{"attack_min": 18, "attack_max": 28, "str": 5, "hit_rate": 5}', 8000, 2000, 1, true, 10),

-- 궁수 무기
('단궁', '초보자용 작은 활', 'weapon', 'bow', 'common', 1, '{"attack_min": 2, "attack_max": 5}', 50, 12, 1, true, 5),
('사냥꾼의 활', '사냥꾼들이 사용하는 활', 'weapon', 'bow', 'common', 3, '{"attack_min": 4, "attack_max": 9, "dex": 1}', 200, 50, 1, true, 7),
('복합 활', '여러 재료로 만든 강한 활', 'weapon', 'bow', 'uncommon', 5, '{"attack_min": 8, "attack_max": 15, "dex": 2}', 800, 200, 1, true, 7),
('전투 활', '전쟁에서 사용되는 활', 'weapon', 'bow', 'uncommon', 7, '{"attack_min": 12, "attack_max": 22, "dex": 3}', 2000, 500, 1, true, 7),
('엘프 활', '엘프가 만든 정교한 활', 'weapon', 'bow', 'rare', 10, '{"attack_min": 18, "attack_max": 30, "dex": 5, "critical_rate": 5}', 8000, 2000, 1, true, 10),

-- 마법사 무기
('나무 지팡이', '초보 마법사의 지팡이', 'weapon', 'staff', 'common', 1, '{"magic_attack": 3, "max_mp": 10}', 50, 12, 1, true, 5),
('수련자의 지팡이', '수련 중인 마법사의 지팡이', 'weapon', 'staff', 'common', 3, '{"magic_attack": 6, "int": 1, "max_mp": 20}', 200, 50, 1, true, 7),
('오크 스태프', '오크 나무로 만든 스태프', 'weapon', 'staff', 'uncommon', 5, '{"magic_attack": 12, "int": 2, "max_mp": 40}', 800, 200, 1, true, 7),
('마법사의 스태프', '정식 마법사의 지팡이', 'weapon', 'staff', 'uncommon', 7, '{"magic_attack": 18, "int": 3, "max_mp": 60}', 2000, 500, 1, true, 7),
('크리스탈 스태프', '수정이 박힌 고급 스태프', 'weapon', 'staff', 'rare', 10, '{"magic_attack": 28, "int": 5, "max_mp": 100, "magic_defense": 5}', 8000, 2000, 1, true, 10),

-- 도적 무기
('녹슨 단검', '녹이 슨 단검', 'weapon', 'dagger', 'common', 1, '{"attack_min": 1, "attack_max": 3, "critical_rate": 5}', 50, 12, 1, true, 5),
('단검', '일반적인 단검', 'weapon', 'dagger', 'common', 3, '{"attack_min": 3, "attack_max": 6, "critical_rate": 8, "dex": 1}', 200, 50, 1, true, 7),
('강철 단검', '강철로 만든 날카로운 단검', 'weapon', 'dagger', 'uncommon', 5, '{"attack_min": 6, "attack_max": 11, "critical_rate": 12, "dex": 2}', 800, 200, 1, true, 7),
('암살자의 단검', '암살자들이 사용하는 단검', 'weapon', 'dagger', 'uncommon', 7, '{"attack_min": 10, "attack_max": 17, "critical_rate": 18, "dex": 3}', 2000, 500, 1, true, 7),
('그림자 단검', '그림자 속에서 빛나는 단검', 'weapon', 'dagger', 'rare', 10, '{"attack_min": 15, "attack_max": 25, "critical_rate": 25, "dex": 5, "avoid_rate": 5}', 8000, 2000, 1, true, 10),

-- 투구
('가죽 모자', '가죽으로 만든 간단한 모자', 'armor', 'helmet', 'common', 1, '{"defense": 1}', 40, 10, 1, true, 5),
('천 두건', '천으로 만든 두건', 'armor', 'helmet', 'common', 3, '{"defense": 2, "max_hp": 10}', 150, 38, 1, true, 7),
('가죽 헬멧', '가죽으로 만든 헬멧', 'armor', 'helmet', 'uncommon', 5, '{"defense": 5, "max_hp": 30}', 600, 150, 1, true, 7),
('철 투구', '철로 만든 투구', 'armor', 'helmet', 'uncommon', 7, '{"defense": 8, "max_hp": 50}', 1500, 375, 1, true, 7),
('미스릴 투구', '미스릴로 만든 가벼운 투구', 'armor', 'helmet', 'rare', 10, '{"defense": 12, "max_hp": 80, "magic_defense": 5}', 6000, 1500, 1, true, 10),

-- 갑옷
('천 옷', '평범한 천 옷', 'armor', 'armor', 'common', 1, '{"defense": 2, "max_hp": 10}', 60, 15, 1, true, 5),
('가죽 조끼', '가죽으로 만든 조끼', 'armor', 'armor', 'common', 3, '{"defense": 4, "max_hp": 25}', 250, 63, 1, true, 7),
('가죽 갑옷', '단단한 가죽 갑옷', 'armor', 'armor', 'uncommon', 5, '{"defense": 8, "max_hp": 50}', 1000, 250, 1, true, 7),
('사슬 갑옷', '쇠사슬로 엮은 갑옷', 'armor', 'armor', 'uncommon', 7, '{"defense": 14, "max_hp": 80}', 2500, 625, 1, true, 7),
('판금 갑옷', '무거운 판금 갑옷', 'armor', 'armor', 'rare', 10, '{"defense": 22, "max_hp": 120, "vit": 3}', 10000, 2500, 1, true, 10),

-- 신발
('천 신발', '평범한 천 신발', 'armor', 'boots', 'common', 1, '{"defense": 1, "avoid_rate": 1}', 30, 8, 1, true, 5),
('가죽 신발', '가죽으로 만든 신발', 'armor', 'boots', 'common', 3, '{"defense": 2, "avoid_rate": 3}', 120, 30, 1, true, 7),
('경보', '가벼운 발을 위한 부츠', 'armor', 'boots', 'uncommon', 5, '{"defense": 4, "avoid_rate": 5, "dex": 1}', 500, 125, 1, true, 7),
('전투화', '전투용 부츠', 'armor', 'boots', 'uncommon', 7, '{"defense": 6, "avoid_rate": 8}', 1200, 300, 1, true, 7),
('민첩 부츠', '민첩성을 높이는 부츠', 'armor', 'boots', 'rare', 10, '{"defense": 10, "avoid_rate": 12, "dex": 3}', 5000, 1250, 1, true, 10),

-- 장갑
('천 장갑', '평범한 천 장갑', 'armor', 'gloves', 'common', 1, '{"defense": 1}', 25, 6, 1, true, 5),
('가죽 장갑', '가죽 장갑', 'armor', 'gloves', 'common', 3, '{"defense": 2, "str": 1}', 100, 25, 1, true, 7),
('전투 장갑', '전투용 장갑', 'armor', 'gloves', 'uncommon', 5, '{"defense": 4, "attack_min": 1, "attack_max": 2}', 400, 100, 1, true, 7),
('강철 건틀렛', '강철 건틀렛', 'armor', 'gloves', 'uncommon', 7, '{"defense": 6, "str": 2}', 1000, 250, 1, true, 7),
('미스릴 건틀렛', '미스릴 건틀렛', 'armor', 'gloves', 'rare', 10, '{"defense": 10, "str": 4, "hit_rate": 3}', 4000, 1000, 1, true, 10),

-- 악세서리 - 반지
('나무 반지', '나무로 만든 반지', 'accessory', 'ring', 'common', 1, '{"max_hp": 5}', 50, 12, 1, false, 0),
('힘의 반지', '힘이 깃든 반지', 'accessory', 'ring', 'uncommon', 5, '{"str": 2}', 500, 125, 1, false, 0),
('민첩의 반지', '민첩함이 깃든 반지', 'accessory', 'ring', 'uncommon', 5, '{"dex": 2}', 500, 125, 1, false, 0),
('지혜의 반지', '지혜가 깃든 반지', 'accessory', 'ring', 'uncommon', 5, '{"int": 2}', 500, 125, 1, false, 0),
('생명의 반지', '생명력이 깃든 반지', 'accessory', 'ring', 'rare', 8, '{"max_hp": 50, "vit": 2}', 2000, 500, 1, false, 0),

-- 악세서리 - 목걸이
('구슬 목걸이', '구슬로 만든 목걸이', 'accessory', 'necklace', 'common', 1, '{"max_mp": 10}', 50, 12, 1, false, 0),
('은 목걸이', '은으로 만든 목걸이', 'accessory', 'necklace', 'uncommon', 5, '{"magic_defense": 3}', 500, 125, 1, false, 0),
('마력의 목걸이', '마력이 깃든 목걸이', 'accessory', 'necklace', 'rare', 8, '{"max_mp": 50, "magic_attack": 5}', 2000, 500, 1, false, 0),

-- 소비 아이템 - 포션
('빨간 포션(소)', 'HP를 30 회복', 'consumable', 'potion', 'common', 1, '{"heal_hp": 30}', 25, 5, 100, false, 0),
('빨간 포션(중)', 'HP를 80 회복', 'consumable', 'potion', 'common', 5, '{"heal_hp": 80}', 80, 16, 100, false, 0),
('빨간 포션(대)', 'HP를 200 회복', 'consumable', 'potion', 'uncommon', 10, '{"heal_hp": 200}', 250, 50, 100, false, 0),
('파란 포션(소)', 'MP를 20 회복', 'consumable', 'potion', 'common', 1, '{"heal_mp": 20}', 30, 6, 100, false, 0),
('파란 포션(중)', 'MP를 50 회복', 'consumable', 'potion', 'common', 5, '{"heal_mp": 50}', 100, 20, 100, false, 0),
('파란 포션(대)', 'MP를 120 회복', 'consumable', 'potion', 'uncommon', 10, '{"heal_mp": 120}', 300, 60, 100, false, 0),
('만능약', 'HP와 MP를 100씩 회복', 'consumable', 'potion', 'rare', 8, '{"heal_hp": 100, "heal_mp": 100}', 500, 100, 100, false, 0),
('귀환 주문서', '마을로 즉시 귀환', 'consumable', 'scroll', 'common', 1, '{"teleport": "village"}', 100, 20, 20, false, 0),

-- 재료 아이템
('슬라임 젤리', '슬라임에서 얻은 젤리', 'material', 'monster_drop', 'common', 1, '{}', 5, 1, 999, false, 0),
('달팽이 껍질', '단단한 달팽이 껍질', 'material', 'monster_drop', 'common', 2, '{}', 8, 2, 999, false, 0),
('버섯 포자', '버섯에서 얻은 포자', 'material', 'monster_drop', 'common', 3, '{}', 10, 2, 999, false, 0),
('돼지 가죽', '돼지의 질긴 가죽', 'material', 'monster_drop', 'common', 4, '{}', 15, 3, 999, false, 0),
('고블린 귀', '고블린의 뾰족한 귀', 'material', 'monster_drop', 'common', 6, '{}', 25, 5, 999, false, 0),
('해골 뼈', '스켈레톤의 뼈', 'material', 'monster_drop', 'common', 7, '{}', 30, 6, 999, false, 0),
('좀비 손톱', '좀비의 손톱', 'material', 'monster_drop', 'uncommon', 8, '{}', 40, 8, 999, false, 0),
('유령의 조각', '고스트에서 얻은 조각', 'material', 'monster_drop', 'uncommon', 10, '{}', 60, 12, 999, false, 0),
('슬라임킹 왕관', '슬라임킹이 쓰던 왕관', 'material', 'monster_drop', 'rare', 10, '{}', 500, 100, 99, false, 0),
('고블린 대장의 도끼 날', '고블린 대장의 도끼 파편', 'material', 'monster_drop', 'rare', 12, '{}', 800, 160, 99, false, 0);

-- 몬스터 드롭 테이블
DELETE FROM monster_drops;
INSERT INTO monster_drops (monster_id, item_id, drop_rate, min_quantity, max_quantity) VALUES
-- 슬라임 드롭
(1, (SELECT id FROM items WHERE name = '슬라임 젤리'), 70.00, 1, 2),
(1, (SELECT id FROM items WHERE name = '빨간 포션(소)'), 10.00, 1, 1),
(1, (SELECT id FROM items WHERE name = '나무 반지'), 1.00, 1, 1),

-- 아기 슬라임 드롭
(2, (SELECT id FROM items WHERE name = '슬라임 젤리'), 50.00, 1, 1),

-- 파란 슬라임 드롭
(3, (SELECT id FROM items WHERE name = '슬라임 젤리'), 80.00, 1, 3),
(3, (SELECT id FROM items WHERE name = '파란 포션(소)'), 15.00, 1, 1),

-- 달팽이 드롭
(4, (SELECT id FROM items WHERE name = '달팽이 껍질'), 60.00, 1, 2),
(4, (SELECT id FROM items WHERE name = '가죽 모자'), 2.00, 1, 1),

-- 주황버섯 드롭
(5, (SELECT id FROM items WHERE name = '버섯 포자'), 65.00, 1, 2),
(5, (SELECT id FROM items WHERE name = '빨간 포션(소)'), 20.00, 1, 2),
(5, (SELECT id FROM items WHERE name = '천 옷'), 3.00, 1, 1),

-- 초록버섯 드롭
(6, (SELECT id FROM items WHERE name = '버섯 포자'), 70.00, 1, 3),
(6, (SELECT id FROM items WHERE name = '파란 포션(소)'), 15.00, 1, 1),

-- 스텀프 드롭
(7, (SELECT id FROM items WHERE name = '빨간 포션(중)'), 20.00, 1, 1),
(7, (SELECT id FROM items WHERE name = '가죽 조끼'), 5.00, 1, 1),

-- 돼지 드롭
(8, (SELECT id FROM items WHERE name = '돼지 가죽'), 60.00, 1, 2),
(8, (SELECT id FROM items WHERE name = '가죽 신발'), 4.00, 1, 1),

-- 리본돼지 드롭
(9, (SELECT id FROM items WHERE name = '돼지 가죽'), 70.00, 1, 3),
(9, (SELECT id FROM items WHERE name = '낡은 검'), 3.00, 1, 1),
(9, (SELECT id FROM items WHERE name = '사냥꾼의 활'), 3.00, 1, 1),

-- 고블린 드롭
(11, (SELECT id FROM items WHERE name = '고블린 귀'), 55.00, 1, 2),
(11, (SELECT id FROM items WHERE name = '빨간 포션(중)'), 25.00, 1, 2),
(11, (SELECT id FROM items WHERE name = '철 검'), 2.00, 1, 1),

-- 고블린 전사 드롭
(12, (SELECT id FROM items WHERE name = '고블린 귀'), 70.00, 2, 3),
(12, (SELECT id FROM items WHERE name = '가죽 갑옷'), 5.00, 1, 1),
(12, (SELECT id FROM items WHERE name = '힘의 반지'), 2.00, 1, 1),

-- 스켈레톤 드롭
(13, (SELECT id FROM items WHERE name = '해골 뼈'), 65.00, 1, 2),
(13, (SELECT id FROM items WHERE name = '기사의 검'), 3.00, 1, 1),
(13, (SELECT id FROM items WHERE name = '철 투구'), 4.00, 1, 1),

-- 좀비 드롭
(14, (SELECT id FROM items WHERE name = '좀비 손톱'), 60.00, 1, 2),
(14, (SELECT id FROM items WHERE name = '빨간 포션(대)'), 15.00, 1, 1),

-- 고스트 드롭
(19, (SELECT id FROM items WHERE name = '유령의 조각'), 55.00, 1, 2),
(19, (SELECT id FROM items WHERE name = '마력의 목걸이'), 3.00, 1, 1),
(19, (SELECT id FROM items WHERE name = '크리스탈 스태프'), 1.00, 1, 1),

-- 슬라임킹 드롭 (보스)
(21, (SELECT id FROM items WHERE name = '슬라임킹 왕관'), 100.00, 1, 1),
(21, (SELECT id FROM items WHERE name = '미스릴 검'), 10.00, 1, 1),
(21, (SELECT id FROM items WHERE name = '생명의 반지'), 5.00, 1, 1),

-- 고블린 대장 드롭 (보스)
(22, (SELECT id FROM items WHERE name = '고블린 대장의 도끼 날'), 100.00, 1, 1),
(22, (SELECT id FROM items WHERE name = '판금 갑옷'), 8.00, 1, 1),
(22, (SELECT id FROM items WHERE name = '미스릴 건틀렛'), 8.00, 1, 1);

-- 1서클 맵
DELETE FROM maps;
INSERT INTO maps (name, display_name, description, width, height, tile_size, tile_data, min_level, pvp_enabled, background_music) VALUES
('village', '리젠 마을', '모험이 시작되는 평화로운 마을. 상점과 창고가 있다', 40, 30, 32, '{}', 1, false, '/assets/bgm/village.mp3'),
('village_outskirts', '마을 외곽', '마을 주변의 들판. 약한 몬스터들이 서식', 50, 40, 32, '{}', 1, false, '/assets/bgm/field.mp3'),
('forest_entrance', '숲 입구', '깊은 숲으로 가는 길목. 야생 동물 주의', 60, 45, 32, '{}', 3, false, '/assets/bgm/forest.mp3'),
('deep_forest', '깊은 숲', '어두운 깊은 숲. 고블린들이 출현', 70, 50, 32, '{}', 5, false, '/assets/bgm/deep_forest.mp3'),
('dungeon_entrance', '던전 입구', '버려진 던전의 입구. 언데드 주의', 50, 50, 32, '{}', 7, false, '/assets/bgm/dungeon.mp3'),
('dungeon_deep', '던전 깊은 곳', '던전의 깊숙한 곳. 강력한 언데드 서식', 60, 60, 32, '{}', 9, false, '/assets/bgm/dungeon_deep.mp3');

-- 1서클 맵 포털
DELETE FROM map_portals;
INSERT INTO map_portals (from_map_id, to_map_id, from_x, from_y, to_x, to_y, portal_type) VALUES
-- 마을 <-> 마을 외곽
((SELECT id FROM maps WHERE name = 'village'), (SELECT id FROM maps WHERE name = 'village_outskirts'), 38, 15, 2, 20, 'normal'),
((SELECT id FROM maps WHERE name = 'village_outskirts'), (SELECT id FROM maps WHERE name = 'village'), 2, 20, 38, 15, 'normal'),

-- 마을 외곽 <-> 숲 입구
((SELECT id FROM maps WHERE name = 'village_outskirts'), (SELECT id FROM maps WHERE name = 'forest_entrance'), 48, 20, 2, 22, 'normal'),
((SELECT id FROM maps WHERE name = 'forest_entrance'), (SELECT id FROM maps WHERE name = 'village_outskirts'), 2, 22, 48, 20, 'normal'),

-- 숲 입구 <-> 깊은 숲
((SELECT id FROM maps WHERE name = 'forest_entrance'), (SELECT id FROM maps WHERE name = 'deep_forest'), 58, 25, 2, 25, 'normal'),
((SELECT id FROM maps WHERE name = 'deep_forest'), (SELECT id FROM maps WHERE name = 'forest_entrance'), 2, 25, 58, 25, 'normal'),

-- 깊은 숲 <-> 던전 입구
((SELECT id FROM maps WHERE name = 'deep_forest'), (SELECT id FROM maps WHERE name = 'dungeon_entrance'), 35, 48, 25, 2, 'normal'),
((SELECT id FROM maps WHERE name = 'dungeon_entrance'), (SELECT id FROM maps WHERE name = 'deep_forest'), 25, 2, 35, 48, 'normal'),

-- 던전 입구 <-> 던전 깊은 곳
((SELECT id FROM maps WHERE name = 'dungeon_entrance'), (SELECT id FROM maps WHERE name = 'dungeon_deep'), 25, 48, 30, 2, 'normal'),
((SELECT id FROM maps WHERE name = 'dungeon_deep'), (SELECT id FROM maps WHERE name = 'dungeon_entrance'), 30, 2, 25, 48, 'normal');

-- 1서클 퀘스트
DELETE FROM quests;
INSERT INTO quests (name, description, required_level, objectives, exp_reward, gold_reward, item_rewards) VALUES
('첫 번째 사냥', '마을 외곽에서 슬라임 5마리를 처치하세요', 1, '{"kill": [{"monster": "슬라임", "count": 5}]}', 50, 100, '[]'),
('슬라임 젤리 수집', '슬라임 젤리 10개를 마을 NPC에게 가져다 주세요', 1, '{"collect": [{"item": "슬라임 젤리", "count": 10}]}', 80, 150, '[]'),
('버섯 퇴치', '주황버섯과 초록버섯을 각각 10마리씩 처치하세요', 3, '{"kill": [{"monster": "주황버섯", "count": 10}, {"monster": "초록버섯", "count": 10}]}', 200, 300, '[]'),
('돼지 사냥', '야생 돼지 15마리를 처치하세요', 4, '{"kill": [{"monster": "돼지", "count": 15}]}', 300, 400, '[]'),
('고블린 토벌', '고블린 20마리를 처치하고 고블린 귀 15개를 가져오세요', 6, '{"kill": [{"monster": "고블린", "count": 20}], "collect": [{"item": "고블린 귀", "count": 15}]}', 600, 800, '[]'),
('언데드 정화', '스켈레톤과 좀비를 각각 15마리씩 처치하세요', 7, '{"kill": [{"monster": "스켈레톤", "count": 15}, {"monster": "좀비", "count": 15}]}', 800, 1000, '[]'),
('고스트 헌터', '던전의 고스트 10마리를 처치하세요', 10, '{"kill": [{"monster": "고스트", "count": 10}]}', 1200, 1500, '[]'),
('슬라임킹 토벌', '마을 외곽의 보스 슬라임킹을 처치하세요', 8, '{"kill": [{"monster": "슬라임킹", "count": 1}]}', 3000, 2000, '[{"item": "미스릴 검", "count": 1}]'),
('고블린 대장 토벌', '깊은 숲의 고블린 대장을 처치하세요', 10, '{"kill": [{"monster": "고블린 대장", "count": 1}]}', 5000, 3000, '[{"item": "판금 갑옷", "count": 1}]');
