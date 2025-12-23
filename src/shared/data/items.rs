//! Item data constants
//!
//! All item definitions that were previously stored in the database.

/// Item types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemCategory {
    Weapon,
    Armor,
    Accessory,
    Consumable,
    Material,
    Etc,
}

/// Weapon sub-types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponType {
    Sword,
    Dagger,
    Staff,
    Mace,
    Knuckle,
    Bow,
}

/// Armor sub-types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmorType {
    Helmet,
    Chest,
    Pants,
    Boots,
    Gloves,
    Shield,
}

/// Item definition
#[derive(Debug, Clone)]
pub struct ItemDef {
    pub id: i32,
    pub name: &'static str,
    pub name_key: &'static str,
    pub description_key: &'static str,
    pub category: ItemCategory,
    pub sub_type: &'static str,
    pub grade: i32,         // 1-12 rarity grade
    pub req_level: i32,
    pub req_class: Option<i32>,  // None = all classes
    pub stats: ItemStats,
    pub price_buy: i64,
    pub price_sell: i64,
    pub icon_path: &'static str,
    pub stackable: bool,
    pub max_stack: i32,
}

/// Item stat bonuses
#[derive(Debug, Clone, Default)]
pub struct ItemStats {
    pub attack: i32,
    pub defense: i32,
    pub magic_attack: i32,
    pub magic_defense: i32,
    pub hp: i32,
    pub mp: i32,
    pub str_stat: i32,
    pub dex_stat: i32,
    pub int_stat: i32,
    pub con_stat: i32,
    pub wis_stat: i32,
    pub heal_hp: i32,  // For potions
    pub heal_mp: i32,  // For potions
}

// ============ Consumables ============

pub const RED_POTION: ItemDef = ItemDef {
    id: 1,
    name: "Red Potion",
    name_key: "item.red_potion",
    description_key: "item.red_potion.desc",
    category: ItemCategory::Consumable,
    sub_type: "potion",
    grade: 1,
    req_level: 1,
    req_class: None,
    stats: ItemStats { heal_hp: 50, ..ItemStats::ZERO },
    price_buy: 50,
    price_sell: 25,
    icon_path: "/assets/items/red_potion.png",
    stackable: true,
    max_stack: 99,
};

pub const BLUE_POTION: ItemDef = ItemDef {
    id: 2,
    name: "Blue Potion",
    name_key: "item.blue_potion",
    description_key: "item.blue_potion.desc",
    category: ItemCategory::Consumable,
    sub_type: "potion",
    grade: 1,
    req_level: 1,
    req_class: None,
    stats: ItemStats { heal_mp: 50, ..ItemStats::ZERO },
    price_buy: 100,
    price_sell: 50,
    icon_path: "/assets/items/blue_potion.png",
    stackable: true,
    max_stack: 99,
};

pub const LARGE_RED_POTION: ItemDef = ItemDef {
    id: 3,
    name: "Large Red Potion",
    name_key: "item.large_red_potion",
    description_key: "item.large_red_potion.desc",
    category: ItemCategory::Consumable,
    sub_type: "potion",
    grade: 2,
    req_level: 10,
    req_class: None,
    stats: ItemStats { heal_hp: 200, ..ItemStats::ZERO },
    price_buy: 200,
    price_sell: 100,
    icon_path: "/assets/items/large_red_potion.png",
    stackable: true,
    max_stack: 99,
};

// ============ Warrior Weapons ============

pub const WOODEN_SWORD: ItemDef = ItemDef {
    id: 10,
    name: "Wooden Sword",
    name_key: "item.wooden_sword",
    description_key: "item.wooden_sword.desc",
    category: ItemCategory::Weapon,
    sub_type: "sword",
    grade: 1,
    req_level: 1,
    req_class: Some(1),
    stats: ItemStats { attack: 5, ..ItemStats::ZERO },
    price_buy: 100,
    price_sell: 50,
    icon_path: "/assets/items/wooden_sword.png",
    stackable: false,
    max_stack: 1,
};

pub const IRON_SWORD: ItemDef = ItemDef {
    id: 11,
    name: "Iron Sword",
    name_key: "item.iron_sword",
    description_key: "item.iron_sword.desc",
    category: ItemCategory::Weapon,
    sub_type: "sword",
    grade: 2,
    req_level: 5,
    req_class: Some(1),
    stats: ItemStats { attack: 15, ..ItemStats::ZERO },
    price_buy: 500,
    price_sell: 250,
    icon_path: "/assets/items/iron_sword.png",
    stackable: false,
    max_stack: 1,
};

pub const STEEL_SWORD: ItemDef = ItemDef {
    id: 12,
    name: "Steel Sword",
    name_key: "item.steel_sword",
    description_key: "item.steel_sword.desc",
    category: ItemCategory::Weapon,
    sub_type: "sword",
    grade: 3,
    req_level: 15,
    req_class: Some(1),
    stats: ItemStats { attack: 30, ..ItemStats::ZERO },
    price_buy: 1500,
    price_sell: 750,
    icon_path: "/assets/items/steel_sword.png",
    stackable: false,
    max_stack: 1,
};

// ============ Rogue Weapons ============

pub const RUSTY_DAGGER: ItemDef = ItemDef {
    id: 20,
    name: "Rusty Dagger",
    name_key: "item.rusty_dagger",
    description_key: "item.rusty_dagger.desc",
    category: ItemCategory::Weapon,
    sub_type: "dagger",
    grade: 1,
    req_level: 1,
    req_class: Some(2),
    stats: ItemStats { attack: 3, dex_stat: 1, ..ItemStats::ZERO },
    price_buy: 100,
    price_sell: 50,
    icon_path: "/assets/items/rusty_dagger.png",
    stackable: false,
    max_stack: 1,
};

pub const IRON_DAGGER: ItemDef = ItemDef {
    id: 21,
    name: "Iron Dagger",
    name_key: "item.iron_dagger",
    description_key: "item.iron_dagger.desc",
    category: ItemCategory::Weapon,
    sub_type: "dagger",
    grade: 2,
    req_level: 5,
    req_class: Some(2),
    stats: ItemStats { attack: 10, dex_stat: 2, ..ItemStats::ZERO },
    price_buy: 500,
    price_sell: 250,
    icon_path: "/assets/items/iron_dagger.png",
    stackable: false,
    max_stack: 1,
};

// ============ Mage Weapons ============

pub const WOODEN_STAFF: ItemDef = ItemDef {
    id: 30,
    name: "Wooden Staff",
    name_key: "item.wooden_staff",
    description_key: "item.wooden_staff.desc",
    category: ItemCategory::Weapon,
    sub_type: "staff",
    grade: 1,
    req_level: 1,
    req_class: Some(3),
    stats: ItemStats { magic_attack: 5, ..ItemStats::ZERO },
    price_buy: 100,
    price_sell: 50,
    icon_path: "/assets/items/wooden_staff.png",
    stackable: false,
    max_stack: 1,
};

pub const MAGIC_STAFF: ItemDef = ItemDef {
    id: 31,
    name: "Magic Staff",
    name_key: "item.magic_staff",
    description_key: "item.magic_staff.desc",
    category: ItemCategory::Weapon,
    sub_type: "staff",
    grade: 2,
    req_level: 10,
    req_class: Some(3),
    stats: ItemStats { magic_attack: 15, int_stat: 2, ..ItemStats::ZERO },
    price_buy: 500,
    price_sell: 250,
    icon_path: "/assets/items/magic_staff.png",
    stackable: false,
    max_stack: 1,
};

// Helper constant for cleaner initialization
impl ItemStats {
    pub const ZERO: ItemStats = ItemStats {
        attack: 0,
        defense: 0,
        magic_attack: 0,
        magic_defense: 0,
        hp: 0,
        mp: 0,
        str_stat: 0,
        dex_stat: 0,
        int_stat: 0,
        con_stat: 0,
        wis_stat: 0,
        heal_hp: 0,
        heal_mp: 0,
    };
}

/// All item definitions
pub const ALL_ITEMS: &[&ItemDef] = &[
    // Consumables
    &RED_POTION,
    &BLUE_POTION,
    &LARGE_RED_POTION,
    // Warrior weapons
    &WOODEN_SWORD,
    &IRON_SWORD,
    &STEEL_SWORD,
    // Rogue weapons
    &RUSTY_DAGGER,
    &IRON_DAGGER,
    // Mage weapons
    &WOODEN_STAFF,
    &MAGIC_STAFF,
];

/// Get item by ID
pub fn get_item_by_id(id: i32) -> Option<&'static ItemDef> {
    ALL_ITEMS.iter().find(|item| item.id == id).copied()
}

/// Get items by category
pub fn get_items_by_category(category: ItemCategory) -> Vec<&'static ItemDef> {
    ALL_ITEMS.iter()
        .filter(|item| item.category == category)
        .copied()
        .collect()
}

/// Get weapons for a specific class
pub fn get_weapons_for_class(class_id: i32) -> Vec<&'static ItemDef> {
    ALL_ITEMS.iter()
        .filter(|item| {
            item.category == ItemCategory::Weapon &&
            (item.req_class.is_none() || item.req_class == Some(class_id))
        })
        .copied()
        .collect()
}
