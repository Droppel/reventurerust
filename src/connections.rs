use crate::{BaseRegion, BaseConnection, JumpConnection, StateChange, ReventureState};
use crate::locations::{locations::*, events::*, regions::*};

// Rule helper functions - these replace the lambda functions from Python
mod rules {
    use super::*;

    pub fn always(_state: &ReventureState) -> bool {
        true
    }

    pub fn no_princess(state: &ReventureState) -> bool {
        !state.event_bool("has_princess")
    }

    pub fn no_princess_and_shovel(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && state.event_bool("has_shovel")
    }

    pub fn no_princess_and_sword_or_elder(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && 
        (state.event_bool("has_sword") || state.event_bool("has_swordelder"))
    }

    pub fn no_princess_and_mrhugs(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && state.event_bool("has_mrhugs")
    }

    pub fn has_princess(state: &ReventureState) -> bool {
        state.event_bool("has_princess")
    }

    pub fn has_shovel(state: &ReventureState) -> bool {
        state.event_bool("has_shovel")
    }

    pub fn has_sword(state: &ReventureState) -> bool {
        state.event_bool("has_sword")
    }

    pub fn has_sword_or_elder(state: &ReventureState) -> bool {
        state.event_bool("has_sword") || state.event_bool("has_swordelder")
    }

    pub fn has_mrhugs(state: &ReventureState) -> bool {
        state.event_bool("has_mrhugs")
    }

    pub fn has_shield(state: &ReventureState) -> bool {
        state.event_bool("has_shield")
    }

    pub fn has_lava_trinket(state: &ReventureState) -> bool {
        state.event_bool("has_lavaTrinket")
    }

    pub fn has_hook(state: &ReventureState) -> bool {
        state.event_bool("has_hook")
    }

    pub fn has_bomb(state: &ReventureState) -> bool {
        state.event_bool("has_bomb")
    }

    pub fn has_nuke(state: &ReventureState) -> bool {
        state.event_bool("has_nuke")
    }

    pub fn has_chicken(state: &ReventureState) -> bool {
        state.event_bool("has_chicken")
    }

    pub fn has_whistle(state: &ReventureState) -> bool {
        state.event_bool("has_whistle")
    }

    pub fn has_darkstone(state: &ReventureState) -> bool {
        state.event_bool("has_darkstone")
    }

    pub fn has_burger(state: &ReventureState) -> bool {
        state.event_bool("has_burger")
    }

    pub fn castle_bridge_down(state: &ReventureState) -> bool {
        state.event_bool("castleBridgeDown")
    }

    pub fn fortress_bridge_down(state: &ReventureState) -> bool {
        state.event_bool("fortressBridgeDown")
    }

    pub fn no_burger_no_princess(state: &ReventureState) -> bool {
        !state.event_bool("has_burger") && !state.event_bool("has_princess")
    }

    pub fn no_burger_no_princess_castle_bridge(state: &ReventureState) -> bool {
        !state.event_bool("has_burger") && !state.event_bool("has_princess") && state.event_bool("castleBridgeDown")
    }

    pub fn no_burger_no_princess_sword(state: &ReventureState) -> bool {
        !state.event_bool("has_burger") && !state.event_bool("has_princess") && state.event_bool("has_sword")
    }

    pub fn no_burger_no_princess_mrhugs(state: &ReventureState) -> bool {
        !state.event_bool("has_burger") && !state.event_bool("has_princess") && state.event_bool("has_mrhugs")
    }

    pub fn no_burger_no_princess_bomb(state: &ReventureState) -> bool {
        !state.event_bool("has_burger") && !state.event_bool("has_princess") && state.event_bool("has_bomb")
    }

    pub fn no_princess_has_burger(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && state.event_bool("has_burger")
    }

    pub fn no_burger_has_princess(state: &ReventureState) -> bool {
        !state.event_bool("has_burger") && state.event_bool("has_princess")
    }

    pub fn no_chicken_sword_or_elder(state: &ReventureState) -> bool {
        !state.event_bool("has_chicken") && 
        (state.event_bool("has_sword") || state.event_bool("has_swordelder"))
    }

    pub fn no_chicken_mrhugs(state: &ReventureState) -> bool {
        !state.event_bool("has_chicken") && state.event_bool("has_mrhugs")
    }

    pub fn no_princess_no_chicken(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && !state.event_bool("has_chicken")
    }

    pub fn sword_or_hook(state: &ReventureState) -> bool {
        state.event_bool("has_sword") || state.event_bool("has_hook")
    }

    pub fn shovel_and_lava_trinket(state: &ReventureState) -> bool {
        state.event_bool("has_shovel") && state.event_bool("has_lavaTrinket")
    }

    pub fn has_sword_dragon(state: &ReventureState) -> bool {
        state.event_bool("has_sword")
    }

    pub fn has_shield_no_lava(state: &ReventureState) -> bool {
        state.event_bool("has_shield") && !state.event_bool("has_lavaTrinket")
    }

    pub fn no_shield_has_lava(state: &ReventureState) -> bool {
        !state.event_bool("has_shield") && state.event_bool("has_lavaTrinket")
    }

    pub fn has_shield_and_lava(state: &ReventureState) -> bool {
        state.event_bool("has_shield") && state.event_bool("has_lavaTrinket")
    }

    pub fn no_princess_no_shield_no_lava(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && !state.event_bool("has_shield") && !state.event_bool("has_lavaTrinket")
    }

    pub fn no_princess_has_shield_no_lava(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && state.event_bool("has_shield") && !state.event_bool("has_lavaTrinket")
    }

    pub fn no_princess_no_shield_has_lava(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && !state.event_bool("has_shield") && state.event_bool("has_lavaTrinket")
    }

    pub fn no_princess_has_shield_has_lava(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && state.event_bool("has_shield") && state.event_bool("has_lavaTrinket")
    }

    pub fn hook_or_shovel_or_bomb(state: &ReventureState) -> bool {
        state.event_bool("has_hook") || state.event_bool("has_shovel") || state.event_bool("has_bomb")
    }

    pub fn chicken_or_shovel_no_princess(state: &ReventureState) -> bool {
        state.event_bool("has_chicken") || (!state.event_bool("has_princess") && state.event_bool("has_shovel"))
    }

    pub fn no_princess_no_nuke(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && !state.event_bool("has_nuke")
    }

    pub fn hook_or_fortress_bridge(state: &ReventureState) -> bool {
        state.event_bool("has_hook") || state.event_bool("fortressBridgeDown")
    }

    pub fn no_princess_hook_or_fortress_bridge(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && 
        (state.event_bool("has_hook") || state.event_bool("fortressBridgeDown"))
    }

    pub fn no_fortress_bridge(state: &ReventureState) -> bool {
        !state.event_bool("fortressBridgeDown")
    }

    pub fn no_fortress_bridge_has_hook(state: &ReventureState) -> bool {
        !state.event_bool("fortressBridgeDown") && state.event_bool("has_hook")
    }

    pub fn fortress_bridge_down_no_princess(state: &ReventureState) -> bool {
        state.event_bool("fortressBridgeDown") && !state.event_bool("has_princess")
    }

    pub fn has_sword_princess(state: &ReventureState) -> bool {
        state.event_bool("has_sword") && state.event_bool("has_princess")
    }

    pub fn has_mrhugs_princess(state: &ReventureState) -> bool {
        state.event_bool("has_mrhugs") && state.event_bool("has_princess")
    }

    pub fn has_darkstone_princess(state: &ReventureState) -> bool {
        state.event_bool("has_darkstone") && state.event_bool("has_princess")
    }

    pub fn has_sword_or_mrhugs(state: &ReventureState) -> bool {
        state.event_bool("has_sword") || state.event_bool("has_mrhugs")
    }

    pub fn no_chicken_princess(state: &ReventureState) -> bool {
        !state.event_bool("has_chicken") && state.event_bool("has_princess")
    }

    pub fn no_chicken_no_princess(state: &ReventureState) -> bool {
        !state.event_bool("has_chicken") && !state.event_bool("has_princess")
    }

    pub fn has_nuke_princess(state: &ReventureState) -> bool {
        state.event_bool("has_nuke") && state.event_bool("has_princess")
    }

    pub fn no_princess_has_burger_mimic(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && state.event_bool("has_burger")
    }

    pub fn has_sword_no_fortress_bridge(state: &ReventureState) -> bool {
        state.event_bool("has_sword") && !state.event_bool("fortressBridgeDown")
    }

    pub fn no_princess_and_hook(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && state.event_bool("has_hook")
    }

    pub fn no_princess_and_sword(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && state.event_bool("has_sword")
    }

    pub fn has_nuke_no_princess(state: &ReventureState) -> bool {
        state.event_bool("has_nuke") && !state.event_bool("has_princess")
    }

    // State change rules
    pub fn can_pickup_sword(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && 
        !state.event_bool("has_sword") && 
        !state.event_bool("has_swordelder") && 
        !state.event_bool("has_mrhugs")
    }

    pub fn can_pickup_shovel(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && !state.event_bool("has_shovel")
    }

    pub fn can_pickup_bomb(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && !state.event_bool("has_bomb")
    }

    pub fn can_pickup_shield(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && !state.event_bool("has_shield")
    }

    pub fn can_pickup_mrhugs(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && !state.event_bool("has_mrhugs")
    }

    pub fn can_pickup_lava_trinket(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && !state.event_bool("has_lavaTrinket")
    }

    pub fn can_pickup_hook(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && !state.event_bool("has_hook")
    }

    pub fn can_pickup_nuke(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && !state.event_bool("has_nuke")
    }

    pub fn can_pickup_whistle(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && !state.event_bool("has_whistle")
    }

    pub fn can_pickup_chicken(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && !state.event_bool("has_chicken")
    }

    pub fn can_lower_castle_bridge(state: &ReventureState) -> bool {
        !state.event_bool("has_burger") && 
        !state.event_bool("has_princess") && 
        !state.event_bool("castleBridgeDown") && 
        (state.event_bool("has_sword") || state.event_bool("has_shovel"))
    }

    pub fn can_pickup_princess(state: &ReventureState) -> bool {
        !state.event_bool("has_princess")
    }

    pub fn can_pickup_darkstone(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && !state.event_bool("has_darkstone")
    }

    pub fn can_pickup_burger(state: &ReventureState) -> bool {
        !state.event_bool("has_princess") && !state.event_bool("has_burger")
    }
}

/// Set up all item placement state changes
pub fn setup_item_placements(base_regions: &mut [BaseRegion], item_locations: &[usize]) {
    // Item 0: Sword Chest
    base_regions[item_locations[0]].add_statechange(StateChange::new(
        vec!["has_sword".to_string()],
        vec![true],
        rules::can_pickup_sword,
        vec!["Sword Chest".to_string()],
    ));

    // Item 1: Sword Pedestal (Elder)
    base_regions[item_locations[1]].add_statechange(StateChange::new(
        vec!["has_swordelder".to_string()],
        vec![true],
        rules::can_pickup_sword,
        vec!["Sword Pedestal".to_string()],
    ));

    // Item 2: Shovel
    base_regions[item_locations[2]].add_statechange(StateChange::new(
        vec!["has_shovel".to_string()],
        vec![true],
        rules::can_pickup_shovel,
        vec!["Shovel".to_string()],
    ));

    // // Item 3: Bomb
    // base_regions[item_locations[3]].add_statechange(StateChange::new(
    //     vec!["has_bomb".to_string()],
    //     vec![true],
    //     rules::can_pickup_bomb,
    //     vec!["Bomb".to_string()],
    // ));

    // // Item 4: Shield
    // base_regions[item_locations[4]].add_statechange(StateChange::new(
    //     vec!["has_shield".to_string()],
    //     vec![true],
    //     rules::can_pickup_shield,
    //     vec!["Shield".to_string()],
    // ));

    // // Item 5: Mister Hugs
    // base_regions[item_locations[5]].add_statechange(StateChange::new(
    //     vec!["has_mrhugs".to_string()],
    //     vec![true],
    //     rules::can_pickup_mrhugs,
    //     vec!["Mister Hugs".to_string()],
    // ));

    // // Item 6: Lava Trinket
    // base_regions[item_locations[6]].add_statechange(StateChange::new(
    //     vec!["has_lavaTrinket".to_string()],
    //     vec![true],
    //     rules::can_pickup_lava_trinket,
    //     vec!["Lava Trinket".to_string()],
    // ));

    // // Item 7: Hook
    // base_regions[item_locations[7]].add_statechange(StateChange::new(
    //     vec!["has_hook".to_string()],
    //     vec![true],
    //     rules::can_pickup_hook,
    //     vec!["Hook".to_string()],
    // ));

    // // Item 8: Nuke
    // base_regions[item_locations[8]].add_statechange(StateChange::new(
    //     vec!["has_nuke".to_string()],
    //     vec![true],
    //     rules::can_pickup_nuke,
    //     vec!["Nuke".to_string()],
    // ));

    // // Item 9: Whistle
    // base_regions[item_locations[9]].add_statechange(StateChange::new(
    //     vec!["has_whistle".to_string()],
    //     vec![true],
    //     rules::can_pickup_whistle,
    //     vec!["Whistle".to_string()],
    // ));
}

/// Set up all region connections - this is the main function that creates the game graph
pub fn setup_region_connections(base_regions: &mut [BaseRegion], start_region: usize) {
    // Menu connections
    base_regions[MENU].add_connection(BaseConnection::new(start_region, rules::always, vec![]));
    base_regions[MENU].add_location(BaseConnection::new(LOC59, rules::always, vec![]));

    // LonksHouse connections
    base_regions[LONKS_HOUSE].add_jumpconnection(JumpConnection::new(ELDER, rules::no_princess, vec![], 2.0));
    base_regions[LONKS_HOUSE].add_connection(BaseConnection::new(CASTLE_FIRST_FLOOR, rules::no_princess, vec![]));
    base_regions[LONKS_HOUSE].add_connection(BaseConnection::new(VOLCANO_BRIDGE, rules::no_princess_and_shovel, vec![]));
    base_regions[LONKS_HOUSE].add_connection(BaseConnection::new(FAIRY_FOUNTAIN, rules::no_princess, vec!["Fairy Portal".to_string()]));
    base_regions[LONKS_HOUSE].add_jumpconnection(JumpConnection::new(SWORD_CHEST, rules::no_princess, vec![], 2.0));
    base_regions[LONKS_HOUSE].add_location(BaseConnection::new(LOC02, rules::no_princess, vec!["Faceplant Stone".to_string()]));
    base_regions[LONKS_HOUSE].add_location(BaseConnection::new(LOC03, rules::no_princess, vec![]));
    base_regions[LONKS_HOUSE].add_location(BaseConnection::new(LOC04, rules::no_princess_and_sword_or_elder, vec![]));
    base_regions[LONKS_HOUSE].add_location(BaseConnection::new(LOC19, rules::no_princess_and_mrhugs, vec![]));
    base_regions[LONKS_HOUSE].add_location(BaseConnection::new(LOC20, rules::no_princess, vec![]));
    base_regions[LONKS_HOUSE].add_location(BaseConnection::new(LOC94, rules::has_princess, vec![]));

    // SwordChest connections
    base_regions[SWORD_CHEST].add_connection(BaseConnection::new(LONKS_HOUSE, rules::always, vec![]));

    // Elder connections
    base_regions[ELDER].add_jumpconnection(JumpConnection::new(CHICKEN, rules::always, vec![], 2.0));
    base_regions[ELDER].add_connection(BaseConnection::new(SHOVEL, rules::always, vec![]));
    base_regions[ELDER].add_jumpconnection(JumpConnection::new(LONKS_HOUSE, rules::always, vec![], 2.0));
    base_regions[ELDER].add_jumpconnection(JumpConnection::new(VOLCANO_TOP_EXIT, rules::always, vec![], 2.0));
    base_regions[ELDER].add_location(BaseConnection::new(LOC01, rules::has_sword_or_elder, vec!["Elder".to_string()]));
    base_regions[ELDER].add_location(BaseConnection::new(LOC40, rules::has_mrhugs, vec!["Elder".to_string()]));

    // Chicken connections
    base_regions[CHICKEN].add_connection(BaseConnection::new(ELDER, rules::always, vec![]));
    base_regions[CHICKEN].add_connection(BaseConnection::new(LONKS_HOUSE, rules::always, vec![]));
    base_regions[CHICKEN].add_statechange(StateChange::new(
        vec!["has_chicken".to_string()],
        vec![true],
        rules::can_pickup_chicken,
        vec!["Chicken".to_string()],
    ));
    base_regions[CHICKEN].add_location(BaseConnection::new(LOC63, rules::no_chicken_sword_or_elder, vec!["Chicken".to_string()]));
    base_regions[CHICKEN].add_location(BaseConnection::new(LOC79, rules::no_chicken_mrhugs, vec!["Chicken".to_string()]));

    // Shovel connections
    base_regions[SHOVEL].add_jumpconnection(JumpConnection::new(ELDER, rules::always, vec![], 3.0));
    base_regions[SHOVEL].add_connection(BaseConnection::new(LONKS_HOUSE, rules::has_shovel, vec![]));

    // CastleFirstFloor connections
    base_regions[CASTLE_FIRST_FLOOR].add_connection(BaseConnection::new(LONKS_HOUSE, rules::no_burger_no_princess, vec![]));
    base_regions[CASTLE_FIRST_FLOOR].add_jumpconnection(JumpConnection::new(CASTLE_SHIELD_CHEST, rules::no_burger_no_princess, vec![], 2.0));
    base_regions[CASTLE_FIRST_FLOOR].add_jumpconnection(JumpConnection::new(CASTLE_MAP_CHEST, rules::no_burger_no_princess, vec![], 3.0));
    base_regions[CASTLE_FIRST_FLOOR].add_connection(BaseConnection::new(SEWER, rules::no_burger_no_princess, vec!["Open Castle Floor".to_string()]));
    base_regions[CASTLE_FIRST_FLOOR].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::no_burger_no_princess_castle_bridge, vec![]));
    base_regions[CASTLE_FIRST_FLOOR].add_statechange(StateChange::new(
        vec!["castleBridgeDown".to_string()],
        vec![true],
        rules::can_lower_castle_bridge,
        vec![],
    ));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC04, rules::no_burger_no_princess_sword, vec![]));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC05, rules::no_burger_no_princess_sword, vec!["King".to_string()]));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC18, rules::no_burger_no_princess_mrhugs, vec!["King".to_string()]));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC19, rules::no_burger_no_princess_mrhugs, vec![]));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC51, rules::no_burger_has_princess, vec![]));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC60, rules::no_burger_no_princess_bomb, vec![]));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC99, rules::no_princess_has_burger, vec![]));

    // CastleShieldChest connections
    base_regions[CASTLE_SHIELD_CHEST].add_connection(BaseConnection::new(CASTLE_FIRST_FLOOR, rules::always, vec![]));

    // CastleMapChest connections
    base_regions[CASTLE_MAP_CHEST].add_connection(BaseConnection::new(CASTLE_FIRST_FLOOR, rules::always, vec![]));
    base_regions[CASTLE_MAP_CHEST].add_jumpconnection(JumpConnection::new(CASTLE_ROOF, rules::always, vec![], 3.0));

    // CastleRoof connections
    base_regions[CASTLE_ROOF].add_connection(BaseConnection::new(CASTLE_MAP_CHEST, rules::always, vec![]));
    base_regions[CASTLE_ROOF].add_connection(BaseConnection::new(PRINCESS_ROOM, rules::always, vec![]));
    base_regions[CASTLE_ROOF].add_jumpconnection(JumpConnection::new(CHIMNEY, rules::always, vec![], 3.0));

    // Chimney connections
    base_regions[CHIMNEY].add_location(BaseConnection::new(LOC30, rules::always, vec![]));

    // PrincessRoom connections
    base_regions[PRINCESS_ROOM].add_jumpconnection(JumpConnection::new(CASTLE_ROOF, rules::always, vec![], 3.0));
    base_regions[PRINCESS_ROOM].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::always, vec![]));
    base_regions[PRINCESS_ROOM].add_connection(BaseConnection::new(ANVIL, rules::always, vec!["Mirror Portal".to_string()]));
    base_regions[PRINCESS_ROOM].add_location(BaseConnection::new(LOC04, rules::has_sword, vec![]));
    base_regions[PRINCESS_ROOM].add_location(BaseConnection::new(LOC11, rules::has_mrhugs, vec![]));
    base_regions[PRINCESS_ROOM].add_location(BaseConnection::new(LOC19, rules::has_mrhugs, vec![]));

    // VolcanoTopExit connections
    base_regions[VOLCANO_TOP_EXIT].add_connection(BaseConnection::new(ELDER, rules::always, vec![]));
    base_regions[VOLCANO_TOP_EXIT].add_connection(BaseConnection::new(LAVA_TRINKET, rules::always, vec![]));
    base_regions[VOLCANO_TOP_EXIT].add_connection(BaseConnection::new(SHOP_LAKE, rules::always, vec![]));

    // LavaTrinket connections
    base_regions[LAVA_TRINKET].add_jumpconnection(JumpConnection::new(VOLCANO_TOP_EXIT, rules::always, vec![], 2.0));
    base_regions[LAVA_TRINKET].add_connection(BaseConnection::new(VOLCANO_BRIDGE, rules::always, vec![]));

    // VolcanoDropStone connections
    base_regions[VOLCANO_DROP_STONE].add_jumpconnection(JumpConnection::new(VOLCANO_BRIDGE, rules::always, vec![], 2.0));
    base_regions[VOLCANO_DROP_STONE].add_jumpconnection(JumpConnection::new(BEHIND_SHOP_BUSH, rules::always, vec![], 2.0));
    base_regions[VOLCANO_DROP_STONE].add_location(BaseConnection::new(LOC06, rules::no_princess, vec![]));

    // VolcanoBridge connections
    base_regions[VOLCANO_BRIDGE].add_connection(BaseConnection::new(VOLCANO_DROP_STONE, rules::always, vec![]));
    base_regions[VOLCANO_BRIDGE].add_connection(BaseConnection::new(BELOW_VOLCANO_BRIDGE, rules::always, vec![]));
    base_regions[VOLCANO_BRIDGE].add_jumpconnection(JumpConnection::new(LAVA_TRINKET, rules::always, vec![], 2.0));
    base_regions[VOLCANO_BRIDGE].add_jumpconnection(JumpConnection::new(SEWER, rules::always, vec![], 3.0));
    base_regions[VOLCANO_BRIDGE].add_connection(BaseConnection::new(SEWER, rules::sword_or_hook, vec![]));

    // Sewer connections
    base_regions[SEWER].add_jumpconnection(JumpConnection::new(CASTLE_FIRST_FLOOR, rules::always, vec!["Open Castle Floor".to_string()], 3.0));
    base_regions[SEWER].add_connection(BaseConnection::new(VOLCANO_BRIDGE, rules::always, vec![]));
    base_regions[SEWER].add_connection(BaseConnection::new(BELOW_CASTLE_BRIDGE, rules::always, vec![]));
    base_regions[SEWER].add_connection(BaseConnection::new(MUSIC_CLUB, rules::has_shovel, vec![]));

    // MusicClub connections
    base_regions[MUSIC_CLUB].add_connection(BaseConnection::new(BELOW_VOLCANO_BRIDGE, rules::always, vec![]));
    base_regions[MUSIC_CLUB].add_connection(BaseConnection::new(SEWER_PIPE, rules::has_shovel, vec![]));
    base_regions[MUSIC_CLUB].add_location(BaseConnection::new(EVENT_KILL_DANIEL, rules::has_sword, vec![]));

    // BelowVolcanoBridge connections
    base_regions[BELOW_VOLCANO_BRIDGE].add_connection(BaseConnection::new(LEFT_OF_DRAGON, rules::has_shovel, vec![]));
    base_regions[BELOW_VOLCANO_BRIDGE].add_connection(BaseConnection::new(GOLD_ROOM, rules::always, vec![]));
    base_regions[BELOW_VOLCANO_BRIDGE].add_connection(BaseConnection::new(PARASITE, rules::shovel_and_lava_trinket, vec![]));
    base_regions[BELOW_VOLCANO_BRIDGE].add_location(BaseConnection::new(LOC06, rules::no_princess, vec![]));

    // GoldRoom connections
    base_regions[GOLD_ROOM].add_connection(BaseConnection::new(RIGHT_OF_DRAGON, rules::always, vec![]));
    base_regions[GOLD_ROOM].add_jumpconnection(JumpConnection::new(SEWER_PIPE, rules::always, vec![], 2.0));

    // LeftOfDragon connections
    base_regions[LEFT_OF_DRAGON].add_connection(BaseConnection::new(VOLCANO_GEYSER, rules::has_shovel, vec![]));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC10, rules::has_shovel, vec![]));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC14, rules::no_princess_no_shield_no_lava, vec!["Dragon".to_string()]));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC29, rules::no_princess_has_shield_no_lava, vec!["Dragon".to_string()]));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC36, rules::no_princess_no_shield_has_lava, vec!["Dragon".to_string()]));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC41, rules::no_princess_has_shield_has_lava, vec!["Dragon".to_string()]));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC92, rules::has_princess, vec!["Dragon".to_string()]));

    // RightOfDragon connections
    base_regions[RIGHT_OF_DRAGON].add_connection(BaseConnection::new(VOLCANO_GEYSER, rules::always, vec![]));
    base_regions[RIGHT_OF_DRAGON].add_jumpconnection(JumpConnection::new(GOLD_ROOM, rules::always, vec![], 4.0));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC14, rules::always, vec!["Dragon".to_string()]));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC16, rules::has_sword, vec!["Dragon".to_string()]));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC29, rules::has_shield_no_lava, vec!["Dragon".to_string()]));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC36, rules::no_shield_has_lava, vec!["Dragon".to_string()]));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC41, rules::has_shield_and_lava, vec!["Dragon".to_string()]));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC43, rules::has_mrhugs, vec!["Dragon".to_string()]));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC92, rules::has_princess, vec!["Dragon".to_string()]));

    // SewerPipe connections
    base_regions[SEWER_PIPE].add_connection(BaseConnection::new(GOLD_ROOM, rules::always, vec![]));
    base_regions[SEWER_PIPE].add_location(BaseConnection::new(LOC35, rules::always, vec!["Sewer Pipe".to_string()]));

    // VolcanoGeyser connections
    base_regions[VOLCANO_GEYSER].add_connection(BaseConnection::new(LEFT_OF_DRAGON, rules::has_lava_trinket, vec![]));
    base_regions[VOLCANO_GEYSER].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::always, vec!["Volcano Geyser".to_string()]));
    base_regions[VOLCANO_GEYSER].add_jumpconnection(JumpConnection::new(ULTIMATE_DOOR, rules::always, vec![], 2.0));
    base_regions[VOLCANO_GEYSER].add_location(BaseConnection::new(LOC06, rules::no_princess, vec![]));

    // UltimateDoor connections
    base_regions[ULTIMATE_DOOR].add_connection(BaseConnection::new(VOLCANO_GEYSER, rules::always, vec![]));
    base_regions[ULTIMATE_DOOR].add_location(BaseConnection::new(LOC67, rules::always, vec![]));
    base_regions[ULTIMATE_DOOR].add_location(BaseConnection::new(LOC100, rules::always, vec![]));

    // CastleMinions connections
    base_regions[CASTLE_MINIONS].add_connection(BaseConnection::new(CASTLE_FIRST_FLOOR, rules::castle_bridge_down, vec![]));
    base_regions[CASTLE_MINIONS].add_connection(BaseConnection::new(SECRET_PATH_MOAT_WELL, rules::no_fortress_bridge, vec![]));
    base_regions[CASTLE_MINIONS].add_connection(BaseConnection::new(HOOK_AREA, rules::always, vec![]));
    base_regions[CASTLE_MINIONS].add_jumpconnection(JumpConnection::new(ABOVE_HOOK, rules::always, vec![], 2.0));
    base_regions[CASTLE_MINIONS].add_connection(BaseConnection::new(ABOVE_HOOK, rules::has_hook, vec![]));
    base_regions[CASTLE_MINIONS].add_connection(BaseConnection::new(CLOUD, rules::always, vec!["Vine".to_string()]));
    base_regions[CASTLE_MINIONS].add_location(BaseConnection::new(LOC03, rules::always, vec![]));
    base_regions[CASTLE_MINIONS].add_location(BaseConnection::new(LOC13, rules::has_mrhugs, vec![]));
    base_regions[CASTLE_MINIONS].add_location(BaseConnection::new(LOC25, rules::has_sword, vec![]));
    base_regions[CASTLE_MINIONS].add_location(BaseConnection::new(LOC95, rules::always, vec![]));

    // Cloud connections
    base_regions[CLOUD].add_connection(BaseConnection::new(CASTLE_ROOF, rules::always, vec![]));
    base_regions[CLOUD].add_location(BaseConnection::new(LOC77, rules::always, vec![]));

    // BelowCastleBridge connections
    base_regions[BELOW_CASTLE_BRIDGE].add_jumpconnection(JumpConnection::new(SEWER, rules::always, vec![], 2.5));
    base_regions[BELOW_CASTLE_BRIDGE].add_jumpconnection(JumpConnection::new(SECRET_PATH_MOAT_WELL, rules::always, vec![], 3.0));
    base_regions[BELOW_CASTLE_BRIDGE].add_connection(BaseConnection::new(CASTLE_MOAT, rules::always, vec![]));

    // SecretPathMoatWell connections
    base_regions[SECRET_PATH_MOAT_WELL].add_connection(BaseConnection::new(BELOW_CASTLE_BRIDGE, rules::always, vec![]));
    base_regions[SECRET_PATH_MOAT_WELL].add_jumpconnection(JumpConnection::new(CASTLE_MINIONS, rules::always, vec![], 3.0));
    base_regions[SECRET_PATH_MOAT_WELL].add_jumpconnection(JumpConnection::new(BOMB, rules::always, vec![], 2.0));

    // CastleMoat connections
    base_regions[CASTLE_MOAT].add_jumpconnection(JumpConnection::new(BELOW_CASTLE_BRIDGE, rules::always, vec![], 2.0));
    base_regions[CASTLE_MOAT].add_connection(BaseConnection::new(ULTIMATE_DOOR, rules::has_shovel, vec![]));
    base_regions[CASTLE_MOAT].add_connection(BaseConnection::new(BARN, rules::has_sword, vec![]));
    base_regions[CASTLE_MOAT].add_jumpconnection(JumpConnection::new(FISHING_BRIDGE, rules::always, vec![], 2.0));
    base_regions[CASTLE_MOAT].add_connection(BaseConnection::new(FISHING_BRIDGE, rules::has_sword, vec![]));
    base_regions[CASTLE_MOAT].add_location(BaseConnection::new(LOC95, rules::always, vec![]));
    base_regions[CASTLE_MOAT].add_location(BaseConnection::new(LOC07, rules::no_princess, vec![]));

    // Barn connections
    base_regions[BARN].add_jumpconnection(JumpConnection::new(BARN_SECOND_FLOOR, rules::always, vec![], 2.0));
    base_regions[BARN].add_location(BaseConnection::new(LOC86, rules::has_princess, vec![]));

    // BarnSecondFloor connections
    base_regions[BARN_SECOND_FLOOR].add_location(BaseConnection::new(LOC31, rules::has_sword, vec![]));

    // BehindShopBush connections
    base_regions[BEHIND_SHOP_BUSH].add_connection(BaseConnection::new(VOLCANO_DROP_STONE, rules::always, vec![]));
    base_regions[BEHIND_SHOP_BUSH].add_connection(BaseConnection::new(SHOP_LAKE, rules::has_sword, vec![]));

    // Shop connections
    base_regions[SHOP].add_connection(BaseConnection::new(SHOP_LAKE, rules::always, vec![]));
    base_regions[SHOP].add_jumpconnection(JumpConnection::new(SHOP_ROOF, rules::always, vec![], 2.0));
    base_regions[SHOP].add_jumpconnection(JumpConnection::new(NUKE_STORAGE, rules::always, vec![], 4.0));
    base_regions[SHOP].add_connection(BaseConnection::new(NUKE_STORAGE, rules::has_hook, vec![]));
    base_regions[SHOP].add_connection(BaseConnection::new(SHOP_CELLAR, rules::has_princess, vec![]));
    base_regions[SHOP].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::no_princess_no_nuke, vec!["Shop Cannon".to_string()]));
    base_regions[SHOP].add_location(BaseConnection::new(LOC09, rules::has_sword, vec!["Shopkeeper".to_string()]));
    base_regions[SHOP].add_location(BaseConnection::new(LOC17, rules::no_princess_no_nuke, vec!["Shop Cannon".to_string()]));
    base_regions[SHOP].add_location(BaseConnection::new(LOC27, rules::has_nuke, vec!["Shop Cannon".to_string()]));
    base_regions[SHOP].add_location(BaseConnection::new(LOC37, rules::has_mrhugs, vec!["Shopkeeper".to_string()]));
    base_regions[SHOP].add_location(BaseConnection::new(LOC74, rules::has_sword, vec!["Shopkeeper".to_string(), "Shop Cannon".to_string(), "Mimic".to_string(), "Elevator Button".to_string()]));
    base_regions[SHOP].add_location(BaseConnection::new(LOC74, rules::has_sword, vec!["Shopkeeper".to_string(), "Shop Cannon".to_string(), "Mimic".to_string(), "Call Elevator Buttons".to_string()]));
    base_regions[SHOP].add_location(BaseConnection::new(LOC95, rules::always, vec![]));

    // ShopRoof connections
    base_regions[SHOP_ROOF].add_connection(BaseConnection::new(SHOP, rules::always, vec![]));
    base_regions[SHOP_ROOF].add_jumpconnection(JumpConnection::new(OCEAN, rules::always, vec![], 3.0));
    base_regions[SHOP_ROOF].add_connection(BaseConnection::new(OCEAN, rules::has_sword, vec![]));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC03, rules::always, vec![]));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC13, rules::has_mrhugs, vec![]));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC25, rules::has_sword, vec![]));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(EVENT_KILL_JUAN, rules::has_sword, vec![]));

    // ShopLake connections
    base_regions[SHOP_LAKE].add_jumpconnection(JumpConnection::new(VOLCANO_TOP_EXIT, rules::always, vec![], 2.0));
    base_regions[SHOP_LAKE].add_connection(BaseConnection::new(BEHIND_SHOP_BUSH, rules::has_sword, vec![]));
    base_regions[SHOP_LAKE].add_connection(BaseConnection::new(SHOP, rules::always, vec![]));

    // Ocean connections
    base_regions[OCEAN].add_connection(BaseConnection::new(SHOP_ROOF, rules::always, vec![]));
    base_regions[OCEAN].add_location(BaseConnection::new(LOC95, rules::always, vec![]));
    base_regions[OCEAN].add_location(BaseConnection::new(LOC96, rules::always, vec![]));
    base_regions[OCEAN].add_location(BaseConnection::new(LOC97, rules::always, vec![]));

    // NukeStorage connections
    base_regions[NUKE_STORAGE].add_connection(BaseConnection::new(SHOP, rules::always, vec![]));

    // ShopCellar connections
    base_regions[SHOP_CELLAR].add_connection(BaseConnection::new(SHOP, rules::has_princess, vec![]));
    base_regions[SHOP_CELLAR].add_connection(BaseConnection::new(PARASITE, rules::always, vec![]));
    base_regions[SHOP_CELLAR].add_location(BaseConnection::new(LOC78, rules::always, vec![]));

    // Parasite connections
    base_regions[PARASITE].add_location(BaseConnection::new(LOC89, rules::always, vec![]));

    // HookArea connections
    base_regions[HOOK_AREA].add_jumpconnection(JumpConnection::new(CASTLE_MINIONS, rules::always, vec![], 3.0));
    base_regions[HOOK_AREA].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::has_hook, vec![]));

    // AboveHook connections
    base_regions[ABOVE_HOOK].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::always, vec![]));
    base_regions[ABOVE_HOOK].add_jumpconnection(JumpConnection::new(ABOVE_ABOVE_HOOK, rules::always, vec![], 3.0));
    base_regions[ABOVE_HOOK].add_connection(BaseConnection::new(ABOVE_ABOVE_HOOK, rules::has_hook, vec![]));
    base_regions[ABOVE_HOOK].add_connection(BaseConnection::new(BOMB, rules::always, vec![]));

    // AboveAboveHook connections
    base_regions[ABOVE_ABOVE_HOOK].add_connection(BaseConnection::new(ABOVE_HOOK, rules::always, vec![]));
    base_regions[ABOVE_ABOVE_HOOK].add_jumpconnection(JumpConnection::new(CASTLE_CANNON_TO_SHOP, rules::always, vec![], 3.0));
    base_regions[ABOVE_ABOVE_HOOK].add_connection(BaseConnection::new(CASTLE_CANNON_TO_SHOP, rules::has_hook, vec![]));
    base_regions[ABOVE_ABOVE_HOOK].add_jumpconnection(JumpConnection::new(ALTAR, rules::always, vec![], 2.0));
    base_regions[ABOVE_ABOVE_HOOK].add_connection(BaseConnection::new(ALTAR, rules::has_hook, vec![]));

    // CastleCannonToShop connections
    base_regions[CASTLE_CANNON_TO_SHOP].add_connection(BaseConnection::new(ABOVE_ABOVE_HOOK, rules::always, vec![]));
    base_regions[CASTLE_CANNON_TO_SHOP].add_connection(BaseConnection::new(SHOP_LAKE, rules::no_princess_no_nuke, vec!["Castle To Shop Cannon".to_string()]));
    base_regions[CASTLE_CANNON_TO_SHOP].add_location(BaseConnection::new(LOC17, rules::no_princess_no_nuke, vec!["Castle To Shop Cannon".to_string()]));
    base_regions[CASTLE_CANNON_TO_SHOP].add_location(BaseConnection::new(LOC56, rules::has_nuke_princess, vec!["Castle To Shop Cannon".to_string()]));

    // Altar connections
    base_regions[ALTAR].add_connection(BaseConnection::new(ABOVE_ABOVE_HOOK, rules::always, vec![]));
    base_regions[ALTAR].add_jumpconnection(JumpConnection::new(MOUNTAIN_LEFT_OUTCROP, rules::always, vec![], 2.0));
    base_regions[ALTAR].add_jumpconnection(JumpConnection::new(LEVERS, rules::always, vec![], 3.0));
    base_regions[ALTAR].add_connection(BaseConnection::new(LEVERS, rules::has_hook, vec![]));
    base_regions[ALTAR].add_connection(BaseConnection::new(GREAT_WATERFALL, rules::always, vec![]));
    base_regions[ALTAR].add_location(BaseConnection::new(LOC72, rules::has_princess, vec![]));

    // Bomb connections
    base_regions[BOMB].add_jumpconnection(JumpConnection::new(ABOVE_HOOK, rules::always, vec![], 3.0));
    base_regions[BOMB].add_connection(BaseConnection::new(ABOVE_HOOK, rules::has_hook, vec![]));
    base_regions[BOMB].add_connection(BaseConnection::new(FISHING_BRIDGE, rules::always, vec![]));
    base_regions[BOMB].add_connection(BaseConnection::new(SECRET_PATH_MOAT_WELL, rules::always, vec![]));
    base_regions[BOMB].add_jumpconnection(JumpConnection::new(SECRET_ABOVE_BOMB, rules::always, vec![], 3.0));
    base_regions[BOMB].add_jumpconnection(JumpConnection::new(GREAT_WATERFALL, rules::has_bomb, vec![], 2.0));
    base_regions[BOMB].add_location(BaseConnection::new(LOC28, rules::has_bomb, vec![]));
    base_regions[BOMB].add_location(BaseConnection::new(LOC32, rules::has_sword, vec!["Boulder".to_string()]));
    base_regions[BOMB].add_location(BaseConnection::new(LOC54, rules::has_mrhugs, vec!["Boulder".to_string()]));

    // FishingBridge connections
    base_regions[FISHING_BRIDGE].add_connection(BaseConnection::new(CASTLE_MOAT, rules::always, vec![]));
    base_regions[FISHING_BRIDGE].add_jumpconnection(JumpConnection::new(FISHING_ROD, rules::always, vec![], 2.0));
    base_regions[FISHING_BRIDGE].add_connection(BaseConnection::new(BELOW_FISHING_BRIDGE, rules::always, vec![]));

    // BelowFishingBridge connections
    base_regions[BELOW_FISHING_BRIDGE].add_jumpconnection(JumpConnection::new(FISHING_BRIDGE, rules::always, vec![], 2.0));
    base_regions[BELOW_FISHING_BRIDGE].add_connection(BaseConnection::new(WATER_FALLS, rules::always, vec![]));

    // FishingRod connections
    base_regions[FISHING_ROD].add_connection(BaseConnection::new(FISHING_BRIDGE, rules::always, vec![]));
    base_regions[FISHING_ROD].add_jumpconnection(JumpConnection::new(BOMB, rules::always, vec![], 2.0));
    base_regions[FISHING_ROD].add_location(BaseConnection::new(LOC12, rules::no_princess, vec!["Fishing Rod".to_string()]));

    // MountainLeftOutcrop connections
    base_regions[MOUNTAIN_LEFT_OUTCROP].add_connection(BaseConnection::new(ALTAR, rules::always, vec![]));
    base_regions[MOUNTAIN_LEFT_OUTCROP].add_jumpconnection(JumpConnection::new(MOUNTAIN_TOP, rules::always, vec![], 3.0));
    base_regions[MOUNTAIN_LEFT_OUTCROP].add_connection(BaseConnection::new(MOUNTAIN_TOP, rules::sword_or_hook, vec![]));
    base_regions[MOUNTAIN_LEFT_OUTCROP].add_location(BaseConnection::new(LOC46, rules::always, vec![]));

    // MountainTop connections
    base_regions[MOUNTAIN_TOP].add_connection(BaseConnection::new(MOUNTAIN_LEFT_OUTCROP, rules::always, vec![]));
    base_regions[MOUNTAIN_TOP].add_connection(BaseConnection::new(MOUNTAIN_TREASURE, rules::always, vec![]));
    base_regions[MOUNTAIN_TOP].add_connection(BaseConnection::new(CLOUD, rules::has_chicken, vec![]));
    base_regions[MOUNTAIN_TOP].add_jumpconnection(JumpConnection::new(STRAWBERRY, rules::always, vec![], 3.0));
    base_regions[MOUNTAIN_TOP].add_location(BaseConnection::new(EVENT_KILL_MIGUEL, rules::has_sword, vec![]));

    // Strawberry connections
    base_regions[STRAWBERRY].add_location(BaseConnection::new(LOC24, rules::always, vec![]));

    // MountainTreasure connections
    base_regions[MOUNTAIN_TREASURE].add_connection(BaseConnection::new(BELOW_LEAP_OF_FAITH, rules::always, vec![]));
    base_regions[MOUNTAIN_TREASURE].add_location(BaseConnection::new(LOC33, rules::no_princess, vec![]));
    base_regions[MOUNTAIN_TREASURE].add_location(BaseConnection::new(LOC62, rules::has_shovel, vec![]));

    // Levers connections
    base_regions[LEVERS].add_jumpconnection(JumpConnection::new(ALTAR, rules::always, vec![], 4.0));
    base_regions[LEVERS].add_connection(BaseConnection::new(ALTAR, rules::has_hook, vec![]));
    base_regions[LEVERS].add_jumpconnection(JumpConnection::new(BELOW_LEAP_OF_FAITH, rules::always, vec![], 4.0));
    base_regions[LEVERS].add_connection(BaseConnection::new(BELOW_LEAP_OF_FAITH, rules::has_hook, vec![]));
    base_regions[LEVERS].add_jumpconnection(JumpConnection::new(DARKSTONE, rules::always, vec!["Dark Stone Lever Middle".to_string()], 3.0));
    base_regions[LEVERS].add_connection(BaseConnection::new(DARKSTONE, rules::has_hook, vec!["Dark Stone Lever Middle".to_string()]));
    base_regions[LEVERS].add_connection(BaseConnection::new(GREAT_WATERFALL, rules::always, vec![]));
    base_regions[LEVERS].add_location(BaseConnection::new(LOC38, rules::no_princess, vec!["Dark Stone Lever Left".to_string()]));
    base_regions[LEVERS].add_location(BaseConnection::new(LOC44, rules::no_princess, vec!["Dark Stone Lever Right".to_string()]));

    // Darkstone connections
    base_regions[DARKSTONE].add_connection(BaseConnection::new(LEVERS, rules::always, vec![]));
    base_regions[DARKSTONE].add_statechange(StateChange::new(
        vec!["has_darkstone".to_string()],
        vec![true],
        rules::can_pickup_darkstone,
        vec!["Dark Stone".to_string()],
    ));
    base_regions[DARKSTONE].add_statechange(StateChange::new(
        vec!["has_burger".to_string()],
        vec![true],
        rules::can_pickup_burger,
        vec!["Burger".to_string()],
    ));

    // GreatWaterfall connections
    base_regions[GREAT_WATERFALL].add_jumpconnection(JumpConnection::new(ALTAR, rules::always, vec![], 2.0));
    base_regions[GREAT_WATERFALL].add_connection(BaseConnection::new(BELOW_FISHING_BRIDGE, rules::always, vec![]));
    base_regions[GREAT_WATERFALL].add_connection(BaseConnection::new(BOMB, rules::has_bomb, vec![]));
    base_regions[GREAT_WATERFALL].add_connection(BaseConnection::new(GREAT_WATERFALL_BOTTOM, rules::always, vec![]));
    base_regions[GREAT_WATERFALL].add_connection(BaseConnection::new(WHISTLE, rules::always, vec![]));
    base_regions[GREAT_WATERFALL].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::always, vec![]));

    // GreatWaterfallBottom connections
    base_regions[GREAT_WATERFALL_BOTTOM].add_connection(BaseConnection::new(WATER_FALLS, rules::always, vec![]));
    base_regions[GREAT_WATERFALL_BOTTOM].add_jumpconnection(JumpConnection::new(ABOVE_WATERFALLS, rules::always, vec![], 2.0));
    base_regions[GREAT_WATERFALL_BOTTOM].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::always, vec![]));

    // SecretAboveBomb connections
    base_regions[SECRET_ABOVE_BOMB].add_connection(BaseConnection::new(BOMB, rules::always, vec![]));
    base_regions[SECRET_ABOVE_BOMB].add_connection(BaseConnection::new(GREAT_WATERFALL, rules::always, vec![]));

    // WaterFalls connections
    base_regions[WATER_FALLS].add_jumpconnection(JumpConnection::new(BELOW_FISHING_BRIDGE, rules::always, vec![], 2.0));
    base_regions[WATER_FALLS].add_connection(BaseConnection::new(MOUNTAIN_TOP, rules::chicken_or_shovel_no_princess, vec!["Waterfall Geyser".to_string()]));
    base_regions[WATER_FALLS].add_jumpconnection(JumpConnection::new(ABOVE_WATERFALLS, rules::always, vec![], 2.0));
    base_regions[WATER_FALLS].add_location(BaseConnection::new(LOC08, rules::always, vec![]));
    base_regions[WATER_FALLS].add_location(BaseConnection::new(LOC82, rules::has_princess, vec![]));
    base_regions[WATER_FALLS].add_location(BaseConnection::new(LOC87, rules::always, vec!["Event Kill Juan".to_string(), "Event Kill Miguel".to_string(), "Event Kill Javi".to_string(), "Event Kill Alberto".to_string(), "Event Kill Daniel".to_string()]));

    // AboveWaterfalls connections
    base_regions[ABOVE_WATERFALLS].add_connection(BaseConnection::new(WATER_FALLS, rules::always, vec![]));
    base_regions[ABOVE_WATERFALLS].add_connection(BaseConnection::new(BELOW_FISHING_BRIDGE, rules::always, vec![]));
    base_regions[ABOVE_WATERFALLS].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::always, vec![]));

    // FortressMoat connections
    base_regions[FORTRESS_MOAT].add_connection(BaseConnection::new(WATER_FALLS, rules::always, vec![]));
    base_regions[FORTRESS_MOAT].add_jumpconnection(JumpConnection::new(ABOVE_WATERFALLS, rules::always, vec![], 2.0));
    base_regions[FORTRESS_MOAT].add_connection(BaseConnection::new(FAIRY_FOUNTAIN, rules::always, vec![]));
    base_regions[FORTRESS_MOAT].add_jumpconnection(JumpConnection::new(FORTRESS_BRIDGE_BUTTON, rules::always, vec![], 2.0));
    base_regions[FORTRESS_MOAT].add_jumpconnection(JumpConnection::new(RIGHT_OF_FORTRESS, rules::always, vec![], 3.0));
    base_regions[FORTRESS_MOAT].add_connection(BaseConnection::new(RIGHT_OF_FORTRESS, rules::hook_or_shovel_or_bomb, vec![]));
    base_regions[FORTRESS_MOAT].add_location(BaseConnection::new(LOC15, rules::always, vec![]));
    base_regions[FORTRESS_MOAT].add_location(BaseConnection::new(LOC21, rules::always, vec![]));
    base_regions[FORTRESS_MOAT].add_location(BaseConnection::new(LOC48, rules::always, vec![]));
    base_regions[FORTRESS_MOAT].add_location(BaseConnection::new(LOC49, rules::has_sword, vec![]));
    base_regions[FORTRESS_MOAT].add_location(BaseConnection::new(LOC61, rules::always, vec![]));

    // FortressBridgeButton connections
    base_regions[FORTRESS_BRIDGE_BUTTON].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::always, vec![]));
    base_regions[FORTRESS_BRIDGE_BUTTON].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::fortress_bridge_down, vec![]));
    base_regions[FORTRESS_BRIDGE_BUTTON].add_statechange(StateChange::new(
        vec!["fortressBridgeDown".to_string()],
        vec![true],
        rules::no_fortress_bridge,
        vec![],
    ));

    // FairyFountain connections
    base_regions[FAIRY_FOUNTAIN].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::always, vec![]));
    base_regions[FAIRY_FOUNTAIN].add_connection(BaseConnection::new(LONKS_HOUSE, rules::always, vec!["Fairy Portal".to_string()]));
    base_regions[FAIRY_FOUNTAIN].add_location(BaseConnection::new(LOC65, rules::always, vec![]));
    base_regions[FAIRY_FOUNTAIN].add_location(BaseConnection::new(LOC85, rules::has_sword_or_mrhugs, vec![]));

    // Whistle connections
    base_regions[WHISTLE].add_jumpconnection(JumpConnection::new(GREAT_WATERFALL, rules::always, vec![], 2.0));
    base_regions[WHISTLE].add_connection(BaseConnection::new(GREAT_WATERFALL_BOTTOM, rules::always, vec![]));
    base_regions[WHISTLE].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::always, vec![]));

    // WhistleAltar connections
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(GREAT_WATERFALL, rules::always, vec![], 2.0));
    base_regions[WHISTLE_ALTAR].add_connection(BaseConnection::new(GREAT_WATERFALL_BOTTOM, rules::always, vec![]));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(BELOW_LEAP_OF_FAITH, rules::always, vec![], 3.0));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(ELEVATOR, rules::no_princess, vec![], 3.0));
    base_regions[WHISTLE_ALTAR].add_connection(BaseConnection::new(ELEVATOR, rules::no_princess_hook_or_fortress_bridge, vec![]));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(FORTRESS_ROOF, rules::no_fortress_bridge, vec![], 3.0));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(FORTRESS_ROOF, rules::no_fortress_bridge_has_hook, vec![], 2.0));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(WHISTLE, rules::always, vec![], 3.0));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC39, rules::no_princess, vec![]));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC69, rules::has_sword_princess, vec![]));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC73, rules::has_mrhugs_princess, vec![]));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC75, rules::has_princess, vec![]));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC83, rules::has_whistle, vec![]));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC90, rules::has_sword_princess, vec![]));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC93, rules::has_darkstone_princess, vec![]));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(EVENT_KILL_ALBERTO, rules::has_sword_no_fortress_bridge, vec![]));

    // BelowLeapOfFaith connections
    base_regions[BELOW_LEAP_OF_FAITH].add_connection(BaseConnection::new(LEVERS, rules::always, vec![]));
    base_regions[BELOW_LEAP_OF_FAITH].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::always, vec![]));

    // Elevator connections
    base_regions[ELEVATOR].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::fortress_bridge_down_no_princess, vec![]));
    base_regions[ELEVATOR].add_connection(BaseConnection::new(ANVIL, rules::always, vec!["Elevator Button".to_string()]));
    base_regions[ELEVATOR].add_connection(BaseConnection::new(ANVIL, rules::always, vec!["Call Elevator Buttons".to_string()]));
    base_regions[ELEVATOR].add_jumpconnection(JumpConnection::new(RIGHT_OF_FORTRESS, rules::always, vec![], 4.0));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC34, rules::always, vec!["Elevator Button".to_string()]));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC34, rules::always, vec!["Call Elevator Buttons".to_string()]));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC34, rules::has_princess, vec![]));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC50, rules::has_princess, vec![]));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC66, rules::has_darkstone, vec![]));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC76, rules::has_princess, vec![]));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC80, rules::has_chicken, vec![]));

    // FortressRoof connections
    base_regions[FORTRESS_ROOF].add_jumpconnection(JumpConnection::new(WHISTLE_ALTAR, rules::always, vec![], 4.0));
    base_regions[FORTRESS_ROOF].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::fortress_bridge_down, vec![]));
    base_regions[FORTRESS_ROOF].add_connection(BaseConnection::new(ANVIL, rules::always, vec![]));
    base_regions[FORTRESS_ROOF].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::no_princess_no_nuke, vec!["Dark Fortress Cannon".to_string()]));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC17, rules::no_princess_no_nuke, vec!["Dark Fortress Cannon".to_string()]));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC42, rules::no_princess, vec!["Princess".to_string()]));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC52, rules::has_princess, vec!["Dark Fortress Cannon".to_string()]));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC55, rules::no_chicken_princess, vec![]));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC58, rules::no_chicken_no_princess, vec![]));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC84, rules::has_nuke_no_princess, vec!["Dark Fortress Cannon".to_string()]));

    // Anvil connections
    base_regions[ANVIL].add_jumpconnection(JumpConnection::new(FORTRESS_ROOF, rules::always, vec![], 4.0));
    base_regions[ANVIL].add_connection(BaseConnection::new(FORTRESS_ROOF, rules::has_hook, vec![]));
    base_regions[ANVIL].add_connection(BaseConnection::new(ELEVATOR, rules::always, vec!["Elevator Button".to_string()]));
    base_regions[ANVIL].add_connection(BaseConnection::new(ELEVATOR, rules::always, vec!["Call Elevator Buttons".to_string()]));
    base_regions[ANVIL].add_jumpconnection(JumpConnection::new(PRINCESS, rules::always, vec![], 3.0));
    base_regions[ANVIL].add_connection(BaseConnection::new(PRINCESS, rules::has_hook, vec![]));
    base_regions[ANVIL].add_connection(BaseConnection::new(FIRE_ESCAPE, rules::has_princess, vec![]));
    base_regions[ANVIL].add_connection(BaseConnection::new(FORTRESS_TREASURE, rules::has_princess, vec![]));
    base_regions[ANVIL].add_location(BaseConnection::new(LOC22, rules::always, vec!["Anvil".to_string()]));
    base_regions[ANVIL].add_location(BaseConnection::new(LOC23, rules::always, vec!["Mimic".to_string()]));
    base_regions[ANVIL].add_location(BaseConnection::new(LOC53, rules::has_princess, vec![]));
    base_regions[ANVIL].add_location(BaseConnection::new(LOC98, rules::no_princess_has_burger_mimic, vec!["Mimic".to_string()]));

    // Princess connections
    base_regions[PRINCESS].add_connection(BaseConnection::new(ANVIL, rules::always, vec![]));
    base_regions[PRINCESS].add_jumpconnection(JumpConnection::new(SPIKE_TRAP, rules::no_princess, vec![], 2.0));
    base_regions[PRINCESS].add_connection(BaseConnection::new(SPIKE_TRAP, rules::no_princess_and_hook, vec![]));
    base_regions[PRINCESS].add_statechange(StateChange::new(
        vec!["has_princess".to_string(), "fortressBridgeDown".to_string()],
        vec![true, true],
        rules::can_pickup_princess,
        vec!["Princess".to_string()],
    ));
    base_regions[PRINCESS].add_location(BaseConnection::new(LOC45, rules::has_princess, vec![]));
    base_regions[PRINCESS].add_location(BaseConnection::new(LOC57, rules::has_mrhugs_princess, vec![]));
    base_regions[PRINCESS].add_location(BaseConnection::new(LOC64, rules::no_princess_and_sword, vec![]));

    // SpikeTrap connections
    base_regions[SPIKE_TRAP].add_location(BaseConnection::new(LOC70, rules::always, vec![]));

    // FireEscape connections
    base_regions[FIRE_ESCAPE].add_connection(BaseConnection::new(ELEVATOR, rules::always, vec![]));
    base_regions[FIRE_ESCAPE].add_jumpconnection(JumpConnection::new(FORTRESS_ROOF, rules::always, vec![], 2.0));
    base_regions[FIRE_ESCAPE].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::fortress_bridge_down, vec![]));

    // FortressTreasure connections
    base_regions[FORTRESS_TREASURE].add_connection(BaseConnection::new(RIGHT_OF_FORTRESS, rules::always, vec![]));
    base_regions[FORTRESS_TREASURE].add_location(BaseConnection::new(LOC68, rules::always, vec![]));
    base_regions[FORTRESS_TREASURE].add_location(BaseConnection::new(EVENT_KILL_JAVI, rules::has_sword, vec![]));

    // RightOfFortress connections
    base_regions[RIGHT_OF_FORTRESS].add_jumpconnection(JumpConnection::new(FORTRESS_TREASURE, rules::always, vec![], 3.0));
    base_regions[RIGHT_OF_FORTRESS].add_connection(BaseConnection::new(ELEVATOR, rules::always, vec![]));
    base_regions[RIGHT_OF_FORTRESS].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::has_hook, vec![]));
    base_regions[RIGHT_OF_FORTRESS].add_location(BaseConnection::new(LOC81, rules::has_princess, vec![]));

    // Desert connections
    base_regions[DESERT].add_location(BaseConnection::new(LOC91, rules::always, vec![]));

    println!("Region connections setup complete!");
}
