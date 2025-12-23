# Asset Directory Structure

This document describes the expected asset folder structure for Legend of Darkness M.

> **📝 Note**: For asset generation prompts and status tracking, see [ASSET_GENERATION.md](./ASSET_GENERATION.md)

## Directory Tree

```
public/assets/
├── characters/                    # Character sprites by class
│   ├── warrior/
│   │   ├── male/
│   │   │   └── spritesheet.png   # 4-direction walk/idle/attack animations
│   │   └── female/
│   │       └── spritesheet.png
│   ├── rogue/
│   │   ├── male/
│   │   └── female/
│   ├── mage/
│   │   ├── male/
│   │   └── female/
│   ├── cleric/
│   │   ├── male/
│   │   └── female/
│   └── martial_artist/
│       ├── male/
│       └── female/
│
├── monsters/                      # Monster sprites by type
│   ├── rat/                      # Lv1 (Small)
│   ├── bat/                      # Lv3 (Small)
│   ├── slime/                    # Lv5 (Small)
│   ├── fox/                      # Lv10 (Medium)
│   ├── wolf/                     # Lv15 (Medium)
│   ├── skeleton/                 # Lv20 (Medium)
│   ├── goblin/                   # Lv30 (Medium)
│   ├── ghost/                    # Lv40 (Large)
│   ├── dark_knight/              # Lv60 (Large)
│   ├── lich/                     # Lv80 (Large)
│   └── dragon/                   # Lv99 Boss (Boss)
│
├── items/                         # Item icons
│   ├── weapons/                  # Weapon icons (32x32)
│   ├── armor/                    # Armor icons (32x32)
│   ├── consumables/              # Potion/scroll icons (32x32)
│   └── materials/                # Crafting materials (32x32)
│
├── skills/                        # Skill icons and effects
│   ├── common/                   # Shared skills (attack, defend)
│   ├── warrior/
│   ├── rogue/
│   ├── mage/
│   ├── cleric/
│   └── martial_artist/
│
├── tiles/                         # Map tiles
│   ├── ground/
│   │   └── tileset.png           # Isometric ground tiles (64x32)
│   ├── buildings/
│   │   └── buildings.png
│   └── decorations/
│       ├── torch.png
│       ├── tree.png
│       └── ...
│
├── ui/                            # UI elements
│   ├── buttons/
│   ├── panels/
│   ├── icons/
│   └── bars/
│
├── fonts/                         # Game fonts
│   └── NanumGothic.ttf
│
└── audio/                         # Sound files
    ├── bgm/
    └── sfx/
```

## Sprite Sheet Specifications

### Characters (4-direction, animated)

| Property | Value |
|----------|-------|
| Total Size | 192 × 256 pixels |
| Grid | 4 columns × 4 rows |
| Frame Size | 48 × 64 pixels |
| Animation FPS | 8 |

**Row Layout**:
| Row | Direction Order | Animation |
|-----|-----------------|-----------|
| 0 | Down, Left, Right, Up | Idle |
| 1 | Down, Left, Right, Up | Walk Frame 1 |
| 2 | Down, Left, Right, Up | Walk Frame 2 |
| 3 | Down, Left, Right, Up | Attack |

### Monsters (animated)

| Size Class | Frame Size | Total Size | Levels |
|------------|------------|------------|--------|
| Small | 32 × 32 | 128 × 128 | 1-10 |
| Medium | 48 × 48 | 192 × 192 | 11-50 |
| Large | 64 × 64 | 256 × 256 | 51-98 |
| Boss | 128 × 128 | 512 × 512 | 99+ |

**Row Layout** (same for all sizes):
| Row | Animation | Frames |
|-----|-----------|--------|
| 0 | Idle | 4 |
| 1 | Walk | 4 |
| 2 | Attack | 4 |
| 3 | Death | 4 |

### Items & Skills

| Type | Size | Format |
|------|------|--------|
| Item Icons | 32 × 32 | PNG with transparency |
| Skill Icons | 32 × 32 | PNG with transparency |
| Skill Effects | Variable | PNG sequence or spritesheet |

### Tiles

| Type | Size | Notes |
|------|------|-------|
| Ground | 64 × 32 | Isometric diamond (2:1 ratio) |
| Buildings | Variable | Isometric perspective |
| Decorations | Variable | Must match isometric angle |

## Color Palette (Dark Fantasy Theme)

| Purpose | Color | Hex |
|---------|-------|-----|
| Blood Red (Primary) | 🔴 | `#8b0000` |
| Magic Purple (Secondary) | 🟣 | `#4a0080` |
| Gold (Accent) | 🟡 | `#daa520` |
| Dark Background | ⬛ | `#0a0a0a` |
| Panel Background | 🔵 | `#1a1a2e` |
| Text White | ⬜ | `#e5e5e5` |

## Naming Conventions

```
characters/{class}/{gender}/spritesheet.png
monsters/{sprite_type}/spritesheet.png
items/{category}/{item_id}.png
skills/{class}/{skill_id}.png
```

## Related Files

- **[ASSET_GENERATION.md](./ASSET_GENERATION.md)** - Generation prompts & status
- **[DESIGN_GUIDELINES.md](./DESIGN_GUIDELINES.md)** - Visual style guide
- **[src/shared/data/assets.rs](./src/shared/data/assets.rs)** - Asset path constants
