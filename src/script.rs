use bevy::{prelude::*, utils::HashMap};

use crate::item::Item;



pub struct ScriptBuilder {
    pub commands: Vec<Command>
}

pub fn build_script(builder: ScriptBuilder, building_bindings: HashMap<u32, Entity>, item_bindings: HashMap<u32, Item>) -> RobotScript {
    RobotScript {
        commands: builder.commands.clone(),
        buildings: building_bindings,
        items: item_bindings,
        step: 0
    }
}

pub struct RobotScript {
    pub commands: Vec<Command>,
    pub buildings: HashMap<u32, Entity>,
    pub items: HashMap<u32, Item>,
    pub step: usize,
}

#[derive(Clone)]
pub enum Command {
    Goto(u32), // Building ID
    Give(u32, u32), // Item ID, amount
    Take(u32, u32), // Item ID, amount
    PrintInventory
}