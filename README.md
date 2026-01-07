# Diablo-like 3D RPG

**Rust + Bevy 0.15 + Axum 0.8**

A High-Performance 3D Top-down Action RPG inspired by Diablo and Eternium.

---

## 🎮 Game Features

### 3D Top-down View
Immersive 3D action RPG experience with a classic top-down perspective.
- **Perspective Camera**: High-angle 3D view (approx. 60°) for tactical combat visibility.
- **3D Environment**: Full 3D rendering with dynamic lighting and shadows.
- **Zoom & Pan**: Smooth camera controls to inspect the battlefield.

### Core Gameplay: Build & Survive
Defense-style RPG inspired by Warcraft 3 custom maps.
- **Hack & Slash Combat**: Fast-paced action against hordes of enemies.
- **Base Building**: Construct defenses to survive waves of attacks.
- **Survival Elements**: Gather resources and upgrade your gear to last longer.

### Progression
- **Hero Classes**: Warrior, Rogue, Mage, Cleric, Monk.
- **Loot System**: Diablo-style random equipment generation.
- **Skill Trees**: Deep customization for each class.

---

## 🚀 Quick Start

```bash
# Setup Environment
cp .env.example .env

# Run Game
docker compose up -d
```

| Service | URL |
|--------|-----|
| 🎮 Web Game | http://localhost:8080 |
| 🔌 API | http://localhost:3000 |
| 🗄️ DB Admin | http://localhost:8081 |

---

## 📁 Project Structure

```
src/
├── game_main.rs          # Bevy Game Client Entry
├── client/               # Game Client Logic
│   ├── camera.rs         # 3D Top-down Camera
│   ├── controls.rs       # Click-to-move & Input
│   ├── player.rs         # Player Logic
│   ├── enemy.rs          # Enemy AI
│   └── graphics.rs       # 3D Rendering & Effects
├── server/               # API Server
└── shared/               # Shared Logic (Net Protocol)
```

---

## 🛠 Tech Stack

| Category | Technology | Version |
|----------|------------|---------|
| Engine | Bevy | 0.15 |
| Backend | Axum | 0.8 |
| Language | Rust | 1.85+ |
| Build Tool | Trunk | Latest |

---

## 🎯 Controls

| Key | Action |
|-----|--------|
| **Left Click** | Move / Interact |
| **Right Click** | Primary Skill |
| **1 - 4** | Active Skills |
| **Space** | Dodge / Ultimate |
| **I** | Inventory |
| **C** | Character |

---

**Made with 🦀 Rust + 🎮 Bevy**
