//! Inventory System
//!
//! Player inventory management with equipment slots and item stacking.

use bevy::prelude::*;
use crate::shared::data::items::{ItemDef, ItemCategory, get_item_by_id};

/// Maximum inventory slots
pub const INVENTORY_SIZE: usize = 24;

/// Equipment slot types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipSlot {
    Weapon,
    Shield,
    Helmet,
    Armor,
    Pants,
    Boots,
    Gloves,
    Ring1,
    Ring2,
    Necklace,
}

impl EquipSlot {
    pub fn all() -> &'static [EquipSlot] {
        &[
            EquipSlot::Weapon,
            EquipSlot::Shield,
            EquipSlot::Helmet,
            EquipSlot::Armor,
            EquipSlot::Pants,
            EquipSlot::Boots,
            EquipSlot::Gloves,
            EquipSlot::Ring1,
            EquipSlot::Ring2,
            EquipSlot::Necklace,
        ]
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            EquipSlot::Weapon => "Weapon",
            EquipSlot::Shield => "Shield",
            EquipSlot::Helmet => "Helmet",
            EquipSlot::Armor => "Armor",
            EquipSlot::Pants => "Pants",
            EquipSlot::Boots => "Boots",
            EquipSlot::Gloves => "Gloves",
            EquipSlot::Ring1 => "Ring 1",
            EquipSlot::Ring2 => "Ring 2",
            EquipSlot::Necklace => "Necklace",
        }
    }
}

/// An item stack in inventory
#[derive(Debug, Clone)]
pub struct ItemStack {
    /// Item definition ID
    pub item_id: i32,
    /// Quantity (for stackable items)
    pub quantity: i32,
    /// Enhancement level (for equipment)
    pub enhancement: i32,
}

impl ItemStack {
    pub fn new(item_id: i32, quantity: i32) -> Self {
        Self {
            item_id,
            quantity,
            enhancement: 0,
        }
    }

    pub fn single(item_id: i32) -> Self {
        Self::new(item_id, 1)
    }

    pub fn get_def(&self) -> Option<&'static ItemDef> {
        get_item_by_id(self.item_id)
    }

    pub fn can_stack_with(&self, other: &ItemStack) -> bool {
        if self.item_id != other.item_id {
            return false;
        }
        if let Some(def) = self.get_def() {
            def.stackable && self.enhancement == other.enhancement
        } else {
            false
        }
    }

    pub fn max_stack(&self) -> i32 {
        self.get_def().map(|d| d.max_stack).unwrap_or(1)
    }
}

/// Player inventory component
#[derive(Component, Default)]
pub struct Inventory {
    /// Main inventory slots (24 slots)
    pub slots: [Option<ItemStack>; INVENTORY_SIZE],
    /// Equipment slots
    pub equipment: std::collections::HashMap<EquipSlot, ItemStack>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            slots: Default::default(),
            equipment: std::collections::HashMap::new(),
        }
    }

    /// Add item to inventory. Returns remaining quantity that couldn't fit.
    pub fn add_item(&mut self, item_id: i32, mut quantity: i32) -> i32 {
        let Some(def) = get_item_by_id(item_id) else {
            return quantity;
        };

        // Try to stack with existing items first
        if def.stackable {
            for slot in self.slots.iter_mut() {
                if quantity <= 0 {
                    break;
                }
                if let Some(stack) = slot {
                    if stack.item_id == item_id && stack.quantity < def.max_stack {
                        let space = def.max_stack - stack.quantity;
                        let add = quantity.min(space);
                        stack.quantity += add;
                        quantity -= add;
                    }
                }
            }
        }

        // Add to empty slots
        for slot in self.slots.iter_mut() {
            if quantity <= 0 {
                break;
            }
            if slot.is_none() {
                let add = if def.stackable {
                    quantity.min(def.max_stack)
                } else {
                    1
                };
                *slot = Some(ItemStack::new(item_id, add));
                quantity -= add;
            }
        }

        quantity // Return remaining that couldn't fit
    }

    /// Remove item from inventory. Returns actual amount removed.
    pub fn remove_item(&mut self, item_id: i32, mut quantity: i32) -> i32 {
        let mut removed = 0;

        for slot in self.slots.iter_mut() {
            if quantity <= 0 {
                break;
            }
            if let Some(stack) = slot {
                if stack.item_id == item_id {
                    let take = quantity.min(stack.quantity);
                    stack.quantity -= take;
                    quantity -= take;
                    removed += take;

                    if stack.quantity <= 0 {
                        *slot = None;
                    }
                }
            }
        }

        removed
    }

    /// Check if inventory has at least `quantity` of item
    pub fn has_item(&self, item_id: i32, quantity: i32) -> bool {
        let total: i32 = self.slots.iter()
            .filter_map(|s| s.as_ref())
            .filter(|s| s.item_id == item_id)
            .map(|s| s.quantity)
            .sum();
        total >= quantity
    }

    /// Get total count of an item
    pub fn count_item(&self, item_id: i32) -> i32 {
        self.slots.iter()
            .filter_map(|s| s.as_ref())
            .filter(|s| s.item_id == item_id)
            .map(|s| s.quantity)
            .sum()
    }

    /// Find first slot containing item
    pub fn find_item(&self, item_id: i32) -> Option<usize> {
        self.slots.iter()
            .position(|s| s.as_ref().map(|i| i.item_id == item_id).unwrap_or(false))
    }

    /// Get number of empty slots
    pub fn empty_slots(&self) -> usize {
        self.slots.iter().filter(|s| s.is_none()).count()
    }

    /// Check if inventory is full
    pub fn is_full(&self) -> bool {
        self.empty_slots() == 0
    }

    /// Equip item from inventory slot
    pub fn equip(&mut self, slot_index: usize, equip_slot: EquipSlot) -> Result<(), &'static str> {
        let Some(stack) = self.slots[slot_index].take() else {
            return Err("No item in slot");
        };

        let Some(def) = stack.get_def() else {
            self.slots[slot_index] = Some(stack);
            return Err("Unknown item");
        };

        // Validate item can go in slot
        let valid = match equip_slot {
            EquipSlot::Weapon => def.category == ItemCategory::Weapon,
            EquipSlot::Shield => def.sub_type == "shield",
            EquipSlot::Helmet => def.sub_type == "helmet",
            EquipSlot::Armor => def.sub_type == "chest",
            EquipSlot::Pants => def.sub_type == "pants",
            EquipSlot::Boots => def.sub_type == "boots",
            EquipSlot::Gloves => def.sub_type == "gloves",
            EquipSlot::Ring1 | EquipSlot::Ring2 => def.sub_type == "ring",
            EquipSlot::Necklace => def.sub_type == "necklace",
        };

        if !valid {
            self.slots[slot_index] = Some(stack);
            return Err("Item cannot be equipped in that slot");
        }

        // Swap with current equipment
        let old_equip = self.equipment.insert(equip_slot, stack);
        if let Some(old) = old_equip {
            self.slots[slot_index] = Some(old);
        }

        Ok(())
    }

    /// Unequip item to inventory
    pub fn unequip(&mut self, equip_slot: EquipSlot) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Inventory full");
        }

        let Some(stack) = self.equipment.remove(&equip_slot) else {
            return Err("Nothing equipped in slot");
        };

        // Find empty slot
        for slot in self.slots.iter_mut() {
            if slot.is_none() {
                *slot = Some(stack);
                return Ok(());
            }
        }

        // Shouldn't reach here due to is_full check
        self.equipment.insert(equip_slot, stack);
        Err("No space in inventory")
    }

    /// Calculate total stat bonuses from equipment
    pub fn equipment_stats(&self) -> EquipmentStats {
        let mut stats = EquipmentStats::default();  

        for stack in self.equipment.values() {
            if let Some(def) = stack.get_def() {
                stats.attack += def.stats.attack;
                stats.defense += def.stats.defense;
                stats.magic_attack += def.stats.magic_attack;
                stats.magic_defense += def.stats.magic_defense;
                stats.hp += def.stats.hp;
                stats.mp += def.stats.mp;
                stats.str_stat += def.stats.str_stat;
                stats.dex_stat += def.stats.dex_stat;
                stats.int_stat += def.stats.int_stat;
                stats.con_stat += def.stats.con_stat;
                stats.wis_stat += def.stats.wis_stat;
            }
        }

        stats
    }
}

/// Equipment stat bonuses
#[derive(Debug, Clone, Default)]
pub struct EquipmentStats {
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
}

/// Use a consumable item
pub fn use_consumable(
    inventory: &mut Inventory,
    slot_index: usize,
    player_hp: &mut i32,
    player_mp: &mut i32,
    max_hp: i32,
    max_mp: i32,
) -> Result<String, &'static str> {
    let Some(stack) = inventory.slots[slot_index].as_ref() else {
        return Err("No item in slot");
    };

    let Some(def) = stack.get_def() else {
        return Err("Unknown item");
    };

    if def.category != ItemCategory::Consumable {
        return Err("Item is not consumable");
    }

    // Apply effects
    let mut message = String::new();

    if def.stats.heal_hp > 0 {
        let heal = def.stats.heal_hp;
        *player_hp = (*player_hp + heal).min(max_hp);
        message.push_str(&format!("Restored {} HP. ", heal));
    }

    if def.stats.heal_mp > 0 {
        let heal = def.stats.heal_mp;
        *player_mp = (*player_mp + heal).min(max_mp);
        message.push_str(&format!("Restored {} MP. ", heal));
    }

    // Remove one item
    if let Some(stack) = inventory.slots[slot_index].as_mut() {
        stack.quantity -= 1;
        if stack.quantity <= 0 {
            inventory.slots[slot_index] = None;
        }
    }

    Ok(message)
}
