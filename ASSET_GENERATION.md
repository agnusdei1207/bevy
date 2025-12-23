# Asset Generation Guide

This document tracks asset generation status and provides prompts for creating missing assets.

## 📊 Asset Status Overview

### Monsters (11 types)

| ID | Name | Sprite Type | Size | Level | Status | Notes |
|----|------|-------------|------|-------|--------|-------|
| 1 | Giant Rat | `rat` | Small (32x32) | 1 | ✅ Original | - |
| 2 | Vampire Bat | `bat` | Small (32x32) | 3 | ⚠️ Original | - |
| 3 | Slime | `slime` | Small (32x32) | 5 | ✅ Original | - |
| 4 | Corrupted Fox | `fox` | Medium (48x48) | 10 | ⚠️ Placeholder | Copied from wolf |
| 5 | Wolf | `wolf` | Medium (48x48) | 15 | ✅ Original | - |
| 6 | Skeleton | `skeleton` | Medium (48x48) | 20 | ⚠️ Original | - |
| 7 | Goblin | `goblin` | Medium (48x48) | 30 | ⚠️ Original | - |
| 8 | Ghost | `ghost` | Large (64x64) | 40 | ⚠️ Original | - |
| 9 | Dark Knight | `dark_knight` | Large (64x64) | 60 | ⚠️ Placeholder | Copied from skeleton |
| 10 | Lich | `lich` | Large (64x64) | 80 | ⚠️ Placeholder | Copied from ghost |
| 11 | Red Dragon | `dragon` | Boss (128x128) | 99 | ✅ Original | - |

### Characters (5 classes × 2 genders = 10 sprites)

| Class | Male | Female | Notes |
|-------|------|--------|-------|
| Warrior | ✅ Original | ✅ Original | Base template |
| Rogue | ✅ Original | ✅ Original | - |
| Mage | ⚠️ Placeholder | ⚠️ Placeholder | Copied from warrior |
| Cleric | ⚠️ Placeholder | ⚠️ Placeholder | Copied from warrior |
| Martial Artist | ⚠️ Placeholder | ⚠️ Placeholder | Copied from warrior |

### Other Assets

| Category | Status | Count |
|----------|--------|-------|
| Tiles (ground) | ✅ | 1 tileset |
| Tiles (buildings) | ✅ | 1 spritesheet |
| Decorations | ✅ | 1 (torch) |
| Skill Icons | ❌ Missing | 0 |
| Item Icons | ❌ Missing | 0 |
| UI Elements | ❌ Missing | 0 |
| Audio | ❌ Missing | 0 |

---

## 🎨 Generation Prompts

### Monsters - Missing/Placeholder

#### Corrupted Fox (fox)
```
Dark fantasy pixel art sprite sheet of a corrupted fox monster. 
4x4 grid layout (192x192 total, 48x48 per frame). 
Isometric 2.5D perspective. 
Row 1: Idle animation facing down (4 frames).
Row 2: Walk animation (4 frames). 
Row 3: Attack animation with bite (4 frames). 
Row 4: Death animation (4 frames).
Dark purple and black fur with glowing red eyes, evil corrupted appearance.
Transparent background. 16-bit retro dark fantasy RPG style.
```

#### Dark Knight (dark_knight)
```
Dark fantasy pixel art sprite sheet of an armored dark knight monster.
4x4 grid layout (256x256 total, 64x64 per frame).
Isometric 2.5D perspective.
Row 1: Idle animation standing guard (4 frames).
Row 2: Walk animation in heavy armor (4 frames).
Row 3: Attack animation with greatsword (4 frames).
Row 4: Death animation falling (4 frames).
Black plate armor with red glowing eyes in helmet, menacing undead warrior.
Transparent background. 16-bit retro dark fantasy RPG style.
```

#### Lich (lich)
```
Dark fantasy pixel art sprite sheet of a lich undead sorcerer boss.
4x4 grid layout (256x256 total, 64x64 per frame).
Isometric 2.5D perspective.
Row 1: Idle floating animation with magic aura (4 frames).
Row 2: Glide/move animation hovering (4 frames).
Row 3: Magic attack casting dark spell (4 frames).
Row 4: Death/disintegrate animation (4 frames).
Skeletal mage in tattered purple robes, glowing blue soul fire, floating skull with crown.
Transparent background. 16-bit retro dark fantasy RPG style.
```

### Characters - Missing/Placeholder

#### Mage Male
```
Dark fantasy pixel art sprite sheet of a male mage character.
4x4 grid layout (192x256 total, 48x64 per frame).
Isometric 2.5D perspective.
Row 1: Idle animation with staff (4 frames, direction: Down, Left, Right, Up).
Row 2: Walk animation frame 1 (4 frames, direction: Down, Left, Right, Up).
Row 3: Walk animation frame 2 (4 frames, direction: Down, Left, Right, Up).
Row 4: Cast spell animation (4 frames, direction: Down, Left, Right, Up).
Blue robes, pointed wizard hat, wooden staff with glowing crystal.
Transparent background. 16-bit retro dark fantasy RPG style.
```

#### Mage Female
```
Dark fantasy pixel art sprite sheet of a female mage character.
4x4 grid layout (192x256 total, 48x64 per frame).
Isometric 2.5D perspective.
Row 1: Idle animation with staff (4 frames, direction: Down, Left, Right, Up).
Row 2: Walk animation frame 1 (4 frames, direction: Down, Left, Right, Up).
Row 3: Walk animation frame 2 (4 frames, direction: Down, Left, Right, Up).
Row 4: Cast spell animation (4 frames, direction: Down, Left, Right, Up).
Purple robes, elegant wizard hat, crystal staff.
Transparent background. 16-bit retro dark fantasy RPG style.
```

#### Cleric Male
```
Dark fantasy pixel art sprite sheet of a male cleric/priest character.
4x4 grid layout (192x256 total, 48x64 per frame).
Isometric 2.5D perspective.
Row 1: Idle animation with holy mace (4 frames, direction: Down, Left, Right, Up).
Row 2: Walk animation frame 1 (4 frames, direction: Down, Left, Right, Up).
Row 3: Walk animation frame 2 (4 frames, direction: Down, Left, Right, Up).
Row 4: Heal/bless animation (4 frames, direction: Down, Left, Right, Up).
White and gold robes, hood, holy symbol pendant, sturdy mace.
Transparent background. 16-bit retro dark fantasy RPG style.
```

#### Cleric Female
```
Dark fantasy pixel art sprite sheet of a female cleric/priestess character.
4x4 grid layout (192x256 total, 48x64 per frame).
Isometric 2.5D perspective.
Row 1: Idle animation with holy staff (4 frames, direction: Down, Left, Right, Up).
Row 2: Walk animation frame 1 (4 frames, direction: Down, Left, Right, Up).
Row 3: Walk animation frame 2 (4 frames, direction: Down, Left, Right, Up).
Row 4: Heal/bless animation (4 frames, direction: Down, Left, Right, Up).
White and silver robes, religious headpiece, healing staff.
Transparent background. 16-bit retro dark fantasy RPG style.
```

#### Martial Artist Male
```
Dark fantasy pixel art sprite sheet of a male martial artist/monk character.
4x4 grid layout (192x256 total, 48x64 per frame).
Isometric 2.5D perspective.
Row 1: Idle fighting stance (4 frames, direction: Down, Left, Right, Up).
Row 2: Walk animation frame 1 (4 frames, direction: Down, Left, Right, Up).
Row 3: Walk animation frame 2 (4 frames, direction: Down, Left, Right, Up).
Row 4: Punch/kick attack (4 frames, direction: Down, Left, Right, Up).
Sleeveless martial arts gi, bandaged hands, muscular build.
Transparent background. 16-bit retro dark fantasy RPG style.
```

#### Martial Artist Female
```
Dark fantasy pixel art sprite sheet of a female martial artist/monk character.
4x4 grid layout (192x256 total, 48x64 per frame).
Isometric 2.5D perspective.
Row 1: Idle fighting stance (4 frames, direction: Down, Left, Right, Up).
Row 2: Walk animation frame 1 (4 frames, direction: Down, Left, Right, Up).
Row 3: Walk animation frame 2 (4 frames, direction: Down, Left, Right, Up).
Row 4: Kick attack (4 frames, direction: Down, Left, Right, Up).
Martial arts outfit, athletic build, confident stance.
Transparent background. 16-bit retro dark fantasy RPG style.
```

---

## 🛒 Skill Icon Prompts

### Common Skills
```
Pixel art RPG skill icon set, 32x32 pixels each. Dark fantasy theme.
Grid of 4 icons:
1. Basic Attack - Sword slash effect
2. Defend - Shield with glow
3. Escape - Running feet with dust
4. Use Item - Potion bottle
Transparent background. 16-bit retro style.
```

### Warrior Skills
```
Pixel art RPG warrior skill icons, 32x32 pixels each. Dark fantasy theme.
Grid of 5 icons:
1. Power Strike - Glowing red sword swing
2. Shield Bash - Shield impact effect
3. War Cry - Shouting warrior face
4. Whirlwind - Spinning blade tornado
5. Berserker Rage - Fiery aura
Transparent background. 16-bit retro style.
```

### Rogue Skills
```
Pixel art RPG rogue skill icons, 32x32 pixels each. Dark fantasy theme.
Grid of 5 icons:
1. Backstab - Dagger stabbing shadow
2. Poison Blade - Dripping green dagger
3. Shadow Step - Ghostly footprints
4. Steal - Reaching hand
5. Assassinate - Skull with crossed daggers
Transparent background. 16-bit retro style.
```

### Mage Skills
```
Pixel art RPG mage skill icons, 32x32 pixels each. Dark fantasy theme.
Grid of 5 icons:
1. Fireball - Flaming orb
2. Ice Lance - Blue ice spike
3. Lightning Bolt - Electric bolt
4. Teleport - Portal swirl
5. Meteor - Falling fire rock
Transparent background. 16-bit retro style.
```

### Cleric Skills
```
Pixel art RPG cleric skill icons, 32x32 pixels each. Dark fantasy theme.
Grid of 5 icons:
1. Heal - Green cross with sparkles
2. Holy Light - Radiant sunburst
3. Blessing - Angel wings glow
4. Resurrection - Rising spirit
5. Divine Shield - Holy barrier
Transparent background. 16-bit retro style.
```

### Martial Artist Skills
```
Pixel art RPG martial artist skill icons, 32x32 pixels each. Dark fantasy theme.
Grid of 5 icons:
1. Flying Kick - Leg with motion blur
2. Chi Strike - Fist with energy
3. Iron Body - Flexing muscle
4. Combo Attack - Multiple fist impacts
5. Dragon Fist - Dragon-shaped punch
Transparent background. 16-bit retro style.
```

---

## 📦 Item Icon Prompts

### Consumables
```
Pixel art RPG consumable item icons, 32x32 pixels each. Dark fantasy theme.
Grid of 6 icons:
1. Red Potion - Healing potion bottle
2. Blue Potion - Mana potion bottle
3. Antidote - Green potion bottle
4. Scroll - Rolled parchment
5. Food - Bread loaf
6. Phoenix Down - Golden feather
Transparent background. 16-bit retro style.
```

### Weapons
```
Pixel art RPG weapon icons, 32x32 pixels each. Dark fantasy theme.
Grid of 6 icons:
1. Iron Sword - Basic sword
2. Steel Dagger - Sleek dagger
3. Wooden Staff - Wizard staff
4. Holy Mace - Golden mace
5. Knuckles - Fighting knuckles
6. Shield - Wooden shield
Transparent background. 16-bit retro style.
```

---

## 📂 File Placement

After generating assets, place them in:

```
public/assets/
├── monsters/{type}/spritesheet.png
├── characters/{class}/{gender}/spritesheet.png
├── skills/{class}/{skill_id}.png
├── items/{category}/{item_id}.png
└── ui/...
```

---

## 🔗 Free Asset Resources

If you prefer downloading free assets:

| Resource | License | Best For |
|----------|---------|----------|
| [Ninja Adventure](https://pixel-boy.itch.io/ninja-adventure-asset-pack) | CC0 | Characters, Monsters, Tiles |
| [Dungeon Tileset II](https://0x72.itch.io/dungeontileset-ii) | CC0 | Dungeon tiles |
| [Shikashi's Fantasy Icons](https://cheekyinkling.itch.io/shikashis-fantasy-icons-pack) | Free | Item icons |
| [RPG Icon Pack](https://kenney.nl/assets/game-icons) | CC0 | UI icons |
| [OpenGameArt](https://opengameart.org/) | Various | All categories |

---

## 🎵 BGM & Audio Resources

### Required BGM Tracks

| Track | Usage | Style | Status |
|-------|-------|-------|--------|
| `bgm_village.mp3` | Milles Village | Peaceful medieval | ❌ Missing |
| `bgm_dungeon.mp3` | Dungeon/Cave | Dark ambient, eerie | ❌ Missing |
| `bgm_horror_castle.mp3` | Horror Castle | 어둠의전설 스타일, creepy | ❌ Missing |
| `bgm_battle.mp3` | Combat | Intense, fast-paced | ❌ Missing |
| `bgm_boss.mp3` | Boss Fight | Epic, dramatic | ❌ Missing |

### 🆓 Free BGM Download Sources

#### 🏆 Recommended (CC0 / No Attribution)

| Source | Description | Link |
|--------|-------------|------|
| **juanjo_sound** | Dark Dungeon Ambient Vol.1 (16 tracks, Elder Scrolls style) | [itch.io](https://juanjo-sound.itch.io/free-dark-dungeon-ambient-music-vol-1) |
| **AlkaKrab** | Medieval Fantasy Ambient (10 tracks) | [itch.io](https://alkakrab.itch.io/) |
| **OpenGameArt CC0** | Fantasy Music & Sounds Collection | [opengameart.org](https://opengameart.org/content/cc0-fantasy-music-sounds) |
| **StockTune** | Dark Fantasy Stock Music (No Attribution) | [stocktune.com](https://stocktune.com/free-music/dark-fantasy) |

#### ⚠️ Attribution Required (CC-BY)

| Source | Description | Link |
|--------|-------------|------|
| **Freesound** | Creepy Dungeon Ambience (loopable) | [freesound.org](https://freesound.org/) |
| **Scott Arc** | 10 Horror Soundscapes/Ambient Loops | [Reddit thread](https://www.reddit.com/r/gamedev/comments/) |

### Horror Castle Style BGM (어둠의전설 스타일)

For the authentic "Legend of Darkness" Horror Castle atmosphere, look for:

**Keywords to search:**
- "Dark dungeon ambient"
- "Horror castle BGM"
- "Gothic RPG music"
- "Creepy medieval ambient"
- "Dark fantasy loop"

**Recommended characteristics:**
- 🎹 Low drone/pad sounds
- 🔔 Occasional bell tolls or chimes
- 💀 Eerie whispers or distant moans
- 🏰 Echo/reverb for castle atmosphere
- ⏱️ Slow tempo (60-80 BPM)
- 🔁 Seamless loop capability

### SFX (Sound Effects)

| Effect | Filename | Notes |
|--------|----------|-------|
| Attack | `sfx_attack.wav` | Sword swing |
| Hit | `sfx_hit.wav` | Impact sound |
| Death | `sfx_death.wav` | Monster death |
| Level Up | `sfx_levelup.wav` | Fanfare/chime |
| Potion | `sfx_potion.wav` | Liquid/heal |
| Door Open | `sfx_door.wav` | Creaking wood |
| Magic Cast | `sfx_magic.wav` | Spell activation |

### File Placement

```
public/assets/audio/
├── bgm/
│   ├── bgm_village.mp3
│   ├── bgm_dungeon.mp3
│   ├── bgm_horror_castle.mp3
│   ├── bgm_battle.mp3
│   └── bgm_boss.mp3
└── sfx/
    ├── sfx_attack.wav
    ├── sfx_hit.wav
    ├── sfx_death.wav
    └── ...
```

### Audio Format Specs

| Type | Format | Sample Rate | Notes |
|------|--------|-------------|-------|
| BGM | MP3 / OGG | 44.1kHz | Loopable preferred |
| SFX | WAV / OGG | 44.1kHz | Short duration (<3s) |

---

## 🎯 Priority Order

1. **High Priority** (Visible immediately):
   - [ ] Mage character (male/female)
   - [ ] Cleric character (male/female)
   - [ ] Martial Artist character (male/female)
   - [ ] BGM: Village, Dungeon, Horror Castle

2. **Medium Priority** (Early game):
   - [ ] Corrupted Fox monster
   - [ ] Skill icons (common + class-specific)
   - [ ] Item icons (potions first)
   - [ ] SFX: Attack, Hit, Death

3. **Low Priority** (Late game):
   - [ ] Dark Knight monster
   - [ ] Lich monster
   - [ ] Advanced weapon icons
   - [ ] UI elements
   - [ ] BGM: Battle, Boss
