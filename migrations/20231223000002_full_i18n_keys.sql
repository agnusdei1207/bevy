-- Legend of Darkness M - I18n Data Update
-- This migration updates static game data with i18n keys instead of hardcoded strings

-- 1. Classes
UPDATE classes SET name = 'classes.warrior', description = 'classes.warrior_desc' WHERE id = 1;
UPDATE classes SET name = 'classes.rogue', description = 'classes.rogue_desc' WHERE id = 2;
UPDATE classes SET name = 'classes.mage', description = 'classes.mage_desc' WHERE id = 3;
UPDATE classes SET name = 'classes.cleric', description = 'classes.cleric_desc' WHERE id = 4;
UPDATE classes SET name = 'classes.martial_artist', description = 'classes.martial_artist_desc' WHERE id = 5;

-- 2. Monsters
UPDATE monster_definitions SET name = 'monsters.rat' WHERE name = 'Giant Rat';
UPDATE monster_definitions SET name = 'monsters.bat' WHERE name = 'Vampire Bat';
UPDATE monster_definitions SET name = 'monsters.slime' WHERE name = 'Slime';
UPDATE monster_definitions SET name = 'monsters.fox' WHERE name = 'Corrupted Fox';
UPDATE monster_definitions SET name = 'monsters.wolf' WHERE name = 'Wolf';
UPDATE monster_definitions SET name = 'monsters.skeleton' WHERE name = 'Skeleton';
UPDATE monster_definitions SET name = 'monsters.goblin' WHERE name = 'Goblin';
UPDATE monster_definitions SET name = 'monsters.ghost' WHERE name = 'Ghost';
UPDATE monster_definitions SET name = 'monsters.dark_knight' WHERE name = 'Dark Knight';
UPDATE monster_definitions SET name = 'monsters.lich' WHERE name = 'Lich';
UPDATE monster_definitions SET name = 'monsters.dragon' WHERE name = 'Red Dragon';

-- 3. Items
UPDATE item_definitions SET name = 'items.red_potion' WHERE name = 'Red Potion';
UPDATE item_definitions SET name = 'items.blue_potion' WHERE name = 'Blue Potion';
UPDATE item_definitions SET name = 'items.wooden_sword' WHERE name = 'Wooden Sword';
UPDATE item_definitions SET name = 'items.iron_sword' WHERE name = 'Iron Sword';
UPDATE item_definitions SET name = 'items.steel_sword' WHERE name = 'Steel Sword';
UPDATE item_definitions SET name = 'items.rusty_dagger' WHERE name = 'Rusty Dagger';
UPDATE item_definitions SET name = 'items.iron_dagger' WHERE name = 'Iron Dagger';
UPDATE item_definitions SET name = 'items.wooden_staff' WHERE name = 'Wooden Staff';
UPDATE item_definitions SET name = 'items.magic_staff' WHERE name = 'Magic Staff';

-- 4. Dungeons/Areas
UPDATE dungeon_definitions SET name = 'areas.beginner_hunting_ground' WHERE name = 'Beginner Hunting Ground';
UPDATE dungeon_definitions SET name = 'areas.slime_cave' WHERE name = 'Slime Cave';
UPDATE dungeon_definitions SET name = 'areas.dark_forest' WHERE name = 'Dark Forest';
UPDATE dungeon_definitions SET name = 'areas.haunted_graveyard' WHERE name = 'Haunted Graveyard';
UPDATE dungeon_definitions SET name = 'areas.knights_castle' WHERE name = 'Knights Castle';
UPDATE dungeon_definitions SET name = 'areas.dragon_lair' WHERE name = 'Dragon Lair';
