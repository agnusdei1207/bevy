# 📦 Asset & Technical Specification

> **Target:** Diablo-like 3D Top-down RPG

---

## 🎯 Core Principles

### 3D Top-down Perspective
Unlike 2D isometric games, this project uses a **True 3D Perspective**.

- **Camera Type:** 3D Perspective Camera
- **Angle:** ~60 degrees pitch (looking down)
- **Field of View:** optimized for Action RPG visibility

### 3D Assets & Rendering
- **Environment:** 3D Meshes (Terrain, Buildings, Props).
- **Characters:** 3D Models or High-Quality 2D Billboards in 3D space.
- **Lighting:** Dynamic Point/Directional lights with shadows.

---

## 📐 Asset Standards

### 3D Models (Preferred)
- **Format:** `.glb` or `.gltf` (Binary glTF)
- **Scale:** 1 Unit = 1 Meter
- **Orientation:** Y-Up

### 2D Sprites (Billboards)
If using sprites in the 3D world:
- **Format:** PNG (Transparent background)
- **Orientation:** Vertical Billboards (always facing camera or fixed Y-axis rotation)
- **Resolution:** High definition (supports zoom)

---

## 📂 Directory Structure

```
public/assets/
├── models/                   # 3D GLB/GLTF Models
│   ├── characters/
│   ├── environment/
│   └── items/
│
├── textures/                 # Materials & Textures
│   ├── terrain/
│   └── effects/
│
├── ui/                       # User Interface
│   ├── hud/
│   └── icons/
│
└── audio/                    # SFX & BGM
```

---

## 📌 Technical Constants

### Camera Settings
```rust
pub const CAMERA_HEIGHT: f32 = 15.0;
pub const CAMERA_ANGLE: f32 = 60.0; // Degrees
pub const FIELD_OF_VIEW: f32 = 45.0;
```

### World Scale
```rust
pub const UNIT_SCALE: f32 = 1.0; // 1.0 = 1 meter
```
