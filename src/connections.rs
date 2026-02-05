use std::vec;

use crate::{BaseRegion, BaseConnection, JumpConnection, StateChange, ReventureState, SimpleBitset, States};
use crate::locations::{locations::*, events::*, regions::*};

// Rule helper functions - these replace the lambda functions from Python
pub mod rules {
    use super::*;

    pub fn always(_state: &ReventureState) -> bool {
        true
    }

    pub fn no_princess(state: &ReventureState) -> bool {
        !state.event_bool(States::HasPrincess as u8)
    }

    pub fn princess(state: &ReventureState) -> bool {
        state.event_bool(States::HasPrincess as u8)
    }

    pub fn shovel(state: &ReventureState) -> bool {
        state.event_bool(States::HasShovel as u8)
    }

    pub fn anysword(state: &ReventureState) -> bool {
        state.event_bool(States::HasSword as u8) || state.event_bool(States::HasSwordElder as u8)
    }

    pub fn mrhugs(state: &ReventureState) -> bool {
        state.event_bool(States::HasMrHugs as u8)
    }

    pub fn shield(state: &ReventureState) -> bool {
        state.event_bool(States::HasShield as u8)
    }

    pub fn lavatrinket(state: &ReventureState) -> bool {
        state.event_bool(States::HasLavaTrinket as u8)
    }

    pub fn hook(state: &ReventureState) -> bool {
        state.event_bool(States::HasHook as u8)
    }

    pub fn bomb(state: &ReventureState) -> bool {
        state.event_bool(States::HasBomb as u8)
    }

    pub fn bomb_hook(state: &ReventureState) -> bool {
        bomb(state) || hook(state)
    }

    pub fn nuke(state: &ReventureState) -> bool {
        state.event_bool(States::HasNuke as u8)
    }

    pub fn chicken(state: &ReventureState) -> bool {
        state.event_bool(States::HasChicken as u8)
    }

    pub fn whistle(state: &ReventureState) -> bool {
        state.event_bool(States::HasWhistle as u8)
    }

    pub fn darkstone(state: &ReventureState) -> bool {
        state.event_bool(States::HasDarkStone as u8)
    }

    pub fn burger(state: &ReventureState) -> bool {
        state.event_bool(States::HasBurger as u8)
    }

    pub fn castle_bridge_down(state: &ReventureState) -> bool {
        state.event_bool(States::CastleBridgeDown as u8)
    }

    pub fn no_princess_castle_bridge_up(state: &ReventureState) -> bool {
        no_princess(state) && !castle_bridge_down(state)
    }

    pub fn fortress_bridge_down(state: &ReventureState) -> bool {
        state.event_bool(States::FortressBridgeDown as u8)
    }

    pub fn no_burger_no_princess(state: &ReventureState) -> bool {
        !burger(state) && no_princess(state)
    }

    pub fn no_burger_no_princess_castle_bridge(state: &ReventureState) -> bool {
        no_burger_no_princess(state) && castle_bridge_down(state)
    }

    pub fn no_burger_no_princess_sword(state: &ReventureState) -> bool {
        no_burger_no_princess(state)&& anysword(state)
    }

    pub fn no_burger_no_princess_mrhugs(state: &ReventureState) -> bool {
        no_burger_no_princess(state) && mrhugs(state)
    }

    pub fn no_burger_no_princess_bomb(state: &ReventureState) -> bool {
        no_burger_no_princess(state) && bomb(state)
    }

    pub fn no_princess_has_burger(state: &ReventureState) -> bool {
        no_princess(state) && burger(state)
    }

    pub fn no_burger_has_princess(state: &ReventureState) -> bool {
        !burger(state) && princess(state)
    }

    pub fn sword_or_hook(state: &ReventureState) -> bool {
        anysword(state) || hook(state)
    }

    pub fn shield_no_lavatrinket(state: &ReventureState) -> bool {
        shield(state) && !lavatrinket(state)
    }

    pub fn no_shield_has_lava(state: &ReventureState) -> bool {
        !shield(state) && lavatrinket(state)
    }

    pub fn has_shield_and_lava(state: &ReventureState) -> bool {
        shield(state) && lavatrinket(state)
    }

    pub fn no_princess_no_shield_no_lavatrinket(state: &ReventureState) -> bool {
        no_princess(state) && !shield(state) && !lavatrinket(state)
    }

    pub fn no_princess_shield_no_lavatrinket(state: &ReventureState) -> bool {
        no_princess(state) && shield(state) && !lavatrinket(state)
    }

    pub fn no_princess_no_shield_lavatrinket(state: &ReventureState) -> bool {
        no_princess(state) && !shield(state) && lavatrinket(state)
    }

    pub fn no_princess_shield_lavatrinket(state: &ReventureState) -> bool {
        no_princess(state) && shield(state) && lavatrinket(state)
    }

    pub fn hook_or_shovel_or_bomb_or_chicken(state: &ReventureState) -> bool {
        hook(state) || shovel(state) || bomb(state) || chicken(state)
    }

    pub fn chicken_or_shovel_no_princess(state: &ReventureState) -> bool {
        chicken(state) || (no_princess(state) && shovel(state))
    }

    pub fn no_princess_no_nuke(state: &ReventureState) -> bool {
        no_princess(state) && !nuke(state)
    }

    pub fn no_princess_hook_or_fortress_bridge(state: &ReventureState) -> bool {
        no_princess(state) && 
        (hook(state) || fortress_bridge_down(state))
    }

    pub fn fortress_bridge_up(state: &ReventureState) -> bool {
        !fortress_bridge_down(state)
    }

    pub fn fortress_bridge_up_hook(state: &ReventureState) -> bool {
        fortress_bridge_up(state) && hook(state)
    }

    pub fn fortress_bridge_down_no_princess(state: &ReventureState) -> bool {
        fortress_bridge_down(state) && no_princess(state)
    }

    pub fn anysword_princess(state: &ReventureState) -> bool {
        anysword(state) && princess(state)
    }

    pub fn mrhugs_princess(state: &ReventureState) -> bool {
        mrhugs(state) && princess(state)
    }

    pub fn darkstone_princess(state: &ReventureState) -> bool {
        darkstone(state) && princess(state)
    }

    pub fn sword_or_mrhugs(state: &ReventureState) -> bool {
        anysword(state) || mrhugs(state)
    }

    pub fn no_chicken_princess(state: &ReventureState) -> bool {
        !chicken(state) && princess(state)
    }

    pub fn no_chicken_no_princess(state: &ReventureState) -> bool {
        !chicken(state) && no_princess(state)
    }

    pub fn no_princess_burger(state: &ReventureState) -> bool {
        no_princess(state) && burger(state)
    }

    pub fn no_princess_and_hook(state: &ReventureState) -> bool {
        no_princess(state) && hook(state)
    }

    pub fn no_princess_sword(state: &ReventureState) -> bool {
        no_princess(state) && anysword(state)
    }

    pub fn nuke_no_princess(state: &ReventureState) -> bool {
        nuke(state) && no_princess(state)
    }

    // State change rules
    pub fn can_pickup_sword(state: &ReventureState) -> bool {
        no_princess(state) && 
        !state.event_bool(States::HasSword as u8) && 
        !state.event_bool(States::HasSwordElder as u8) && 
        !mrhugs(state)
    }

    pub fn can_pickup_shovel(state: &ReventureState) -> bool {
        no_princess(state) && !shovel(state)
    }

    pub fn can_pickup_bomb(state: &ReventureState) -> bool {
        no_princess(state) && !bomb(state)
    }

    pub fn can_pickup_shield(state: &ReventureState) -> bool {
        no_princess(state) && !shield(state)
    }

    pub fn can_pickup_mrhugs(state: &ReventureState) -> bool {
        no_princess(state) && !mrhugs(state)
    }

    pub fn can_pickup_lavatrinket(state: &ReventureState) -> bool {
        no_princess(state) && !lavatrinket(state)
    }

    pub fn can_pickup_hook(state: &ReventureState) -> bool {
        no_princess(state) && !hook(state)
    }

    pub fn can_pickup_nuke(state: &ReventureState) -> bool {
        no_princess(state) && !nuke(state)
    }

    pub fn can_pickup_whistle(state: &ReventureState) -> bool {
        no_princess(state) && !whistle(state)
    }

    pub fn can_pickup_chicken(state: &ReventureState) -> bool {
        no_princess(state) && !chicken(state)
    }

    pub fn can_pickup_princess(state: &ReventureState) -> bool {
        no_princess(state)
    }

    pub fn can_pickup_darkstone(state: &ReventureState) -> bool {
        no_princess(state) && !darkstone(state)
    }

    pub fn can_pickup_burger(state: &ReventureState) -> bool {
        no_princess(state) && !burger(state)
    }
}

/// Set up all item placement state changes
pub fn setup_item_placements(base_regions: &mut [BaseRegion], item_locations: &[usize]) {
    // Item 0: Sword Chest
    base_regions[item_locations[0]].add_statechange(StateChange::new(
        vec![States::HasSword as u8],
        vec![true],
        rules::can_pickup_sword,
        SimpleBitset::new(vec![56]),
    ));

    // Item 1: Sword Pedestal (Elder)
    base_regions[item_locations[1]].add_statechange(StateChange::new(
        vec![States::HasSwordElder as u8],
        vec![true],
        rules::can_pickup_sword,
        SimpleBitset::new(vec![2]),
    ));

    // Item 2: Shovel
    base_regions[item_locations[2]].add_statechange(StateChange::new(
        vec![States::HasShovel as u8],
        vec![true],
        rules::can_pickup_shovel,
        SimpleBitset::new(vec![3]),
    ));

    // Item 3: Bomb
    base_regions[item_locations[3]].add_statechange(StateChange::new(
        vec![States::HasBomb as u8],
        vec![true],
        rules::can_pickup_bomb,
        SimpleBitset::new(vec![14]),
    ));

    // Item 4: Shield
    base_regions[item_locations[4]].add_statechange(StateChange::new(
        vec![States::HasShield as u8],
        vec![true],
        rules::can_pickup_shield,
        SimpleBitset::new(vec![15]),
    ));

    // Item 5: Mister Hugs
    base_regions[item_locations[5]].add_statechange(StateChange::new(
        vec![States::HasMrHugs as u8],
        vec![true],
        rules::can_pickup_mrhugs,
        SimpleBitset::new(vec![13]),
    ));

    // Item 6: Lava Trinket
    base_regions[item_locations[6]].add_statechange(StateChange::new(
        vec![States::HasLavaTrinket as u8],
        vec![true],
        rules::can_pickup_lavatrinket,
        SimpleBitset::new(vec![12]),
    ));

    // Item 7: Hook
    base_regions[item_locations[7]].add_statechange(StateChange::new(
        vec![States::HasHook as u8],
        vec![true],
        rules::can_pickup_hook,
        SimpleBitset::new(vec![10]),
    ));

    // Item 8: Nuke
    base_regions[item_locations[8]].add_statechange(StateChange::new(
        vec![States::HasNuke as u8],
        vec![true],
        rules::can_pickup_nuke,
        SimpleBitset::new(vec![16]),
    ));

    // Item 9: Whistle
    base_regions[item_locations[9]].add_statechange(StateChange::new(
        vec![States::HasWhistle as u8],
        vec![true],
        rules::can_pickup_whistle,
        SimpleBitset::new(vec![7]),
    ));
}

/// Set up all region connections - this is the main function that creates the game graph
pub fn setup_region_connections(base_regions: &mut [BaseRegion], start_region: usize) {
    // Menu connections
    base_regions[MENU].add_connection(BaseConnection::new(start_region, rules::always, SimpleBitset::new_empty()));
    base_regions[MENU].add_location(BaseConnection::new(LOC59, rules::always, SimpleBitset::new_empty()));

    // LonksHouse connections
    base_regions[LONKS_HOUSE].add_connection(BaseConnection::new(LONKS_FRONTGARDEN, rules::no_princess, SimpleBitset::new_empty()));
    base_regions[LONKS_HOUSE].add_connection(BaseConnection::new(LONKS_BACKGARDEN, rules::no_princess, SimpleBitset::new_empty()));
    base_regions[LONKS_HOUSE].add_jumpconnection(JumpConnection::new(SWORD_CHEST, rules::no_princess, SimpleBitset::new_empty(), 2.0));
    base_regions[LONKS_HOUSE].add_location(BaseConnection::new(LOC20, rules::no_princess, SimpleBitset::new_empty()));
    base_regions[LONKS_HOUSE].add_location(BaseConnection::new(LOC94, rules::princess, SimpleBitset::new_empty()));
    base_regions[LONKS_HOUSE].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));
    
    // LonksBackGarden connections
    base_regions[LONKS_BACKGARDEN].add_jumpconnection(JumpConnection::new(ELDER, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[LONKS_BACKGARDEN].add_connection(BaseConnection::new(LONKS_HOUSE, rules::always, SimpleBitset::new_empty()));
    base_regions[LONKS_BACKGARDEN].add_jumpconnection(JumpConnection::new(LONKS_FRONTGARDEN, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[LONKS_BACKGARDEN].add_connection(BaseConnection::new(VOLCANO_BRIDGE, rules::shovel, SimpleBitset::new_empty()));
    base_regions[LONKS_BACKGARDEN].add_location(BaseConnection::new(LOC03, rules::always, SimpleBitset::new_empty()));
    base_regions[LONKS_BACKGARDEN].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // LonksFrontGarden connections
    base_regions[LONKS_FRONTGARDEN].add_connection(BaseConnection::new(LONKS_HOUSE, rules::always, SimpleBitset::new_empty()));
    base_regions[LONKS_FRONTGARDEN].add_jumpconnection(JumpConnection::new(LONKS_BACKGARDEN, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[LONKS_FRONTGARDEN].add_connection(BaseConnection::new(CASTLE_FIRST_FLOOR, rules::always, SimpleBitset::new_empty()));
    base_regions[LONKS_FRONTGARDEN].add_connection(BaseConnection::new(FAIRY_FOUNTAIN, rules::always,
         SimpleBitset::new(vec![31])));
    base_regions[LONKS_FRONTGARDEN].add_location(BaseConnection::new(LOC02, rules::always, 
        SimpleBitset::new(vec![34])));
    base_regions[LONKS_FRONTGARDEN].add_location(BaseConnection::new(LOC04, rules::anysword, SimpleBitset::new_empty()));
    base_regions[LONKS_FRONTGARDEN].add_location(BaseConnection::new(LOC19, rules::mrhugs, SimpleBitset::new_empty()));
    base_regions[LONKS_FRONTGARDEN].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // SwordChest connections
    base_regions[SWORD_CHEST].add_connection(BaseConnection::new(LONKS_HOUSE, rules::always, SimpleBitset::new_empty()));
    base_regions[SWORD_CHEST].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // Elder connections
    base_regions[ELDER].add_jumpconnection(JumpConnection::new(CHICKEN, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[ELDER].add_connection(BaseConnection::new(SHOVEL, rules::always, SimpleBitset::new_empty()));
    base_regions[ELDER].add_jumpconnection(JumpConnection::new(LONKS_BACKGARDEN, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[ELDER].add_jumpconnection(JumpConnection::new(VOLCANO_TOP_EXIT, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[ELDER].add_location(BaseConnection::new(LOC01, rules::anysword, 
        SimpleBitset::new(vec![44])));
    base_regions[ELDER].add_location(BaseConnection::new(LOC40, rules::mrhugs, 
        SimpleBitset::new(vec![44])));

    // Chicken connections
    base_regions[CHICKEN].add_connection(BaseConnection::new(ELDER, rules::always, SimpleBitset::new_empty()));
    base_regions[CHICKEN].add_connection(BaseConnection::new(LONKS_BACKGARDEN, rules::always, SimpleBitset::new_empty()));
    base_regions[CHICKEN].add_statechange(StateChange::new(
        vec![States::HasChicken as u8],
        vec![true],
        rules::can_pickup_chicken,
        SimpleBitset::new(vec![43]),
    ));
    fn rule_loc63(state: &ReventureState) -> bool {
        !rules::chicken(state) && rules::anysword(state)
    }
    base_regions[CHICKEN].add_location(BaseConnection::new(LOC63, rule_loc63, SimpleBitset::new(vec![43])));
    fn rule_loc79(state: &ReventureState) -> bool {
        !rules::chicken(state) && rules::mrhugs(state)
    }
    base_regions[CHICKEN].add_location(BaseConnection::new(LOC79, rule_loc79, SimpleBitset::new(vec![43])));
    base_regions[CHICKEN].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // Shovel connections
    base_regions[SHOVEL].add_jumpconnection(JumpConnection::new(ELDER, rules::always, SimpleBitset::new_empty(), 3.0));
    base_regions[SHOVEL].add_connection(BaseConnection::new(LONKS_BACKGARDEN, rules::shovel, SimpleBitset::new_empty()));

    // CastleFirstFloor connections
    base_regions[CASTLE_FIRST_FLOOR].add_connection(BaseConnection::new(LONKS_FRONTGARDEN, rules::no_burger_no_princess, SimpleBitset::new_empty()));
    base_regions[CASTLE_FIRST_FLOOR].add_jumpconnection(JumpConnection::new(CASTLE_SHIELD_CHEST, rules::no_burger_no_princess, SimpleBitset::new_empty(), 2.0));
    base_regions[CASTLE_FIRST_FLOOR].add_jumpconnection(JumpConnection::new(CASTLE_MAP_CHEST, rules::no_burger_no_princess, SimpleBitset::new_empty(), 3.0));
    base_regions[CASTLE_FIRST_FLOOR].add_connection(BaseConnection::new(SEWER, rules::no_burger_no_princess, SimpleBitset::new(vec![33])));
    base_regions[CASTLE_FIRST_FLOOR].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::no_burger_no_princess_castle_bridge, SimpleBitset::new_empty()));
    fn can_lower_castle_bridge(state: &ReventureState) -> bool {
        rules::no_burger_no_princess(state) && 
        !rules::castle_bridge_down(state) && 
        (rules::anysword(state) || rules::shovel(state))
    }
    base_regions[CASTLE_FIRST_FLOOR].add_statechange(StateChange::new(
        vec![States::CastleBridgeDown as u8],
        vec![true],
        can_lower_castle_bridge,
        SimpleBitset::new_empty(),
    ));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC04, rules::no_burger_no_princess_sword, SimpleBitset::new_empty()));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC05, rules::no_burger_no_princess_sword, SimpleBitset::new(vec![42])));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC18, rules::no_burger_no_princess_mrhugs, SimpleBitset::new(vec![42])));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC19, rules::no_burger_no_princess_mrhugs, SimpleBitset::new_empty()));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC51, rules::no_burger_has_princess, SimpleBitset::new_empty()));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC60, rules::no_burger_no_princess_bomb, SimpleBitset::new_empty()));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC99, rules::no_princess_has_burger, SimpleBitset::new(vec![42])));
    base_regions[CASTLE_FIRST_FLOOR].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // CastleShieldChest connections
    base_regions[CASTLE_SHIELD_CHEST].add_connection(BaseConnection::new(CASTLE_FIRST_FLOOR, rules::always, SimpleBitset::new_empty()));
    base_regions[CASTLE_SHIELD_CHEST].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // CastleMapChest connections
    base_regions[CASTLE_MAP_CHEST].add_connection(BaseConnection::new(CASTLE_FIRST_FLOOR, rules::always, SimpleBitset::new_empty()));
    base_regions[CASTLE_MAP_CHEST].add_jumpconnection(JumpConnection::new(CASTLE_ROOF, rules::always, SimpleBitset::new_empty(), 3.0));
    base_regions[CASTLE_MAP_CHEST].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // CastleRoof connections
    base_regions[CASTLE_ROOF].add_connection(BaseConnection::new(CASTLE_MAP_CHEST, rules::always, SimpleBitset::new_empty()));
    base_regions[CASTLE_ROOF].add_connection(BaseConnection::new(PRINCESS_ROOM, rules::always, SimpleBitset::new_empty()));
    base_regions[CASTLE_ROOF].add_jumpconnection(JumpConnection::new(CHIMNEY, rules::always, SimpleBitset::new_empty(), 3.0));
    // base_regions[CASTLE_ROOF].add_location(BaseConnection::new(LOC17, rules::always, vec!["Castle To Dark Fortress Cannon".to_string()]));
    base_regions[CASTLE_ROOF].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // Chimney connections
    base_regions[CHIMNEY].add_location(BaseConnection::new(LOC30, rules::always, SimpleBitset::new_empty()));
    base_regions[CHIMNEY].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // PrincessRoom connections
    base_regions[PRINCESS_ROOM].add_jumpconnection(JumpConnection::new(CASTLE_ROOF, rules::always, SimpleBitset::new_empty(), 3.0));
    base_regions[PRINCESS_ROOM].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::always, SimpleBitset::new_empty()));
    base_regions[PRINCESS_ROOM].add_connection(BaseConnection::new(ANVIL, rules::always, SimpleBitset::new(vec![30])));
    base_regions[PRINCESS_ROOM].add_location(BaseConnection::new(LOC04, rules::anysword, SimpleBitset::new_empty()));
    base_regions[PRINCESS_ROOM].add_location(BaseConnection::new(LOC11, rules::mrhugs, SimpleBitset::new_empty()));
    base_regions[PRINCESS_ROOM].add_location(BaseConnection::new(LOC19, rules::mrhugs, SimpleBitset::new_empty()));
    base_regions[PRINCESS_ROOM].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // VolcanoTopExit connections
    base_regions[VOLCANO_TOP_EXIT].add_connection(BaseConnection::new(ELDER, rules::always, SimpleBitset::new_empty()));
    base_regions[VOLCANO_TOP_EXIT].add_connection(BaseConnection::new(LAVA_TRINKET, rules::always, SimpleBitset::new_empty()));
    base_regions[VOLCANO_TOP_EXIT].add_connection(BaseConnection::new(SHOP_LAKE, rules::always, SimpleBitset::new_empty()));
    base_regions[VOLCANO_TOP_EXIT].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // LavaTrinket connections
    base_regions[LAVA_TRINKET].add_jumpconnection(JumpConnection::new(VOLCANO_TOP_EXIT, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[LAVA_TRINKET].add_connection(BaseConnection::new(VOLCANO_BRIDGE, rules::always, SimpleBitset::new_empty()));

    // VolcanoDropStone connections
    base_regions[VOLCANO_DROP_STONE].add_jumpconnection(JumpConnection::new(VOLCANO_BRIDGE, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[VOLCANO_DROP_STONE].add_jumpconnection(JumpConnection::new(BEHIND_SHOP_BUSH, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[VOLCANO_DROP_STONE].add_location(BaseConnection::new(LOC06, rules::no_princess, SimpleBitset::new_empty()));

    // VolcanoBridge connections
    base_regions[VOLCANO_BRIDGE].add_connection(BaseConnection::new(VOLCANO_DROP_STONE, rules::always, SimpleBitset::new_empty()));
    base_regions[VOLCANO_BRIDGE].add_connection(BaseConnection::new(BELOW_VOLCANO_BRIDGE, rules::always, SimpleBitset::new_empty()));
    base_regions[VOLCANO_BRIDGE].add_jumpconnection(JumpConnection::new(LAVA_TRINKET, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[VOLCANO_BRIDGE].add_jumpconnection(JumpConnection::new(SEWER, rules::always, SimpleBitset::new_empty(), 3.0));
    base_regions[VOLCANO_BRIDGE].add_connection(BaseConnection::new(SEWER, rules::sword_or_hook, SimpleBitset::new_empty()));

    // Sewer connections
    base_regions[SEWER].add_jumpconnection(JumpConnection::new(CASTLE_FIRST_FLOOR, rules::always, SimpleBitset::new(vec![33]), 3.0));
    base_regions[SEWER].add_connection(BaseConnection::new(VOLCANO_BRIDGE, rules::always, SimpleBitset::new_empty()));
    base_regions[SEWER].add_connection(BaseConnection::new(BELOW_CASTLE_BRIDGE, rules::always, SimpleBitset::new_empty()));
    base_regions[SEWER].add_connection(BaseConnection::new(MUSIC_CLUB, rules::shovel, SimpleBitset::new_empty()));

    // MusicClub connections
    base_regions[MUSIC_CLUB].add_connection(BaseConnection::new(BELOW_VOLCANO_BRIDGE, rules::always, SimpleBitset::new_empty()));
    base_regions[MUSIC_CLUB].add_connection(BaseConnection::new(SEWER_PIPE, rules::shovel, SimpleBitset::new_empty()));
    base_regions[MUSIC_CLUB].add_location(BaseConnection::new(EVENT_KILL_DANIEL, rules::anysword, SimpleBitset::new_empty()));

    // BelowVolcanoBridge connections
    base_regions[BELOW_VOLCANO_BRIDGE].add_connection(BaseConnection::new(LEFT_OF_DRAGON, rules::shovel, SimpleBitset::new_empty()));
    base_regions[BELOW_VOLCANO_BRIDGE].add_jumpconnection(JumpConnection::new(GOLD_ROOM, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[BELOW_VOLCANO_BRIDGE].add_connection(BaseConnection::new(PARASITE, rules::shovel, SimpleBitset::new_empty()));
    base_regions[BELOW_VOLCANO_BRIDGE].add_location(BaseConnection::new(LOC06, rules::no_princess, SimpleBitset::new_empty()));

    // GoldRoom connections
    base_regions[GOLD_ROOM].add_connection(BaseConnection::new(RIGHT_OF_DRAGON, rules::always, SimpleBitset::new_empty()));
    base_regions[GOLD_ROOM].add_jumpconnection(JumpConnection::new(SEWER_PIPE, rules::always, SimpleBitset::new_empty(), 2.0));

    // LeftOfDragon connections
    base_regions[LEFT_OF_DRAGON].add_connection(BaseConnection::new(VOLCANO_GEYSER, rules::shovel, SimpleBitset::new_empty()));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC10, rules::shovel, SimpleBitset::new_empty()));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC14, rules::no_princess_no_shield_no_lavatrinket, SimpleBitset::new(vec![39])));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC29, rules::no_princess_shield_no_lavatrinket, SimpleBitset::new(vec![39])));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC36, rules::no_princess_no_shield_lavatrinket, SimpleBitset::new(vec![39])));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC41, rules::no_princess_shield_lavatrinket, SimpleBitset::new(vec![39])));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC92, rules::princess, SimpleBitset::new(vec![39])));

    // RightOfDragon connections
    base_regions[RIGHT_OF_DRAGON].add_connection(BaseConnection::new(VOLCANO_GEYSER, rules::always, SimpleBitset::new_empty()));
    base_regions[RIGHT_OF_DRAGON].add_jumpconnection(JumpConnection::new(GOLD_ROOM, rules::always, SimpleBitset::new_empty(), 4.0));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC14, rules::always, SimpleBitset::new(vec![39])));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC16, rules::anysword, SimpleBitset::new(vec![39])));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC29, rules::shield_no_lavatrinket, SimpleBitset::new(vec![39])));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC36, rules::no_shield_has_lava, SimpleBitset::new(vec![39])));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC41, rules::has_shield_and_lava, SimpleBitset::new(vec![39])));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC43, rules::mrhugs, SimpleBitset::new(vec![39])));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC92, rules::princess, SimpleBitset::new(vec![39])));

    // SewerPipe connections
    base_regions[SEWER_PIPE].add_connection(BaseConnection::new(GOLD_ROOM, rules::always, SimpleBitset::new_empty()));
    base_regions[SEWER_PIPE].add_location(BaseConnection::new(LOC35, rules::always, SimpleBitset::new(vec![35])));

    // VolcanoGeyser connections
    base_regions[VOLCANO_GEYSER].add_connection(BaseConnection::new(LEFT_OF_DRAGON, rules::lavatrinket, SimpleBitset::new_empty()));
    base_regions[VOLCANO_GEYSER].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::always, SimpleBitset::new(vec![26])));
    base_regions[VOLCANO_GEYSER].add_jumpconnection(JumpConnection::new(ULTIMATE_DOOR, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[VOLCANO_GEYSER].add_location(BaseConnection::new(LOC06, rules::no_princess, SimpleBitset::new_empty()));

    // UltimateDoor connections
    base_regions[ULTIMATE_DOOR].add_connection(BaseConnection::new(VOLCANO_GEYSER, rules::always, SimpleBitset::new_empty()));
    base_regions[ULTIMATE_DOOR].add_location(BaseConnection::new(LOC67, rules::always, SimpleBitset::new_empty()));
    base_regions[ULTIMATE_DOOR].add_location(BaseConnection::new(LOC100, rules::always, SimpleBitset::new_empty()));

    // CastleMinions connections
    base_regions[CASTLE_MINIONS].add_statechange(StateChange::new(
        vec![States::CastleBridgeDown as u8],
        vec![true],
        rules::princess,
        SimpleBitset::new_empty(),
    ));
    base_regions[CASTLE_MINIONS].add_connection(BaseConnection::new(CASTLE_FIRST_FLOOR, rules::castle_bridge_down, SimpleBitset::new_empty()));
    base_regions[CASTLE_MINIONS].add_connection(BaseConnection::new(SECRET_PATH_MOAT_WELL, rules::no_princess_castle_bridge_up, SimpleBitset::new_empty()));
    base_regions[CASTLE_MINIONS].add_connection(BaseConnection::new(HOOK_AREA, rules::always, SimpleBitset::new_empty()));
    base_regions[CASTLE_MINIONS].add_jumpconnection(JumpConnection::new(ABOVE_HOOK, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[CASTLE_MINIONS].add_connection(BaseConnection::new(ABOVE_HOOK, rules::hook, SimpleBitset::new_empty()));
    base_regions[CASTLE_MINIONS].add_connection(BaseConnection::new(CLOUD, rules::always, SimpleBitset::new(vec![32])));
    base_regions[CASTLE_MINIONS].add_location(BaseConnection::new(LOC03, rules::always, SimpleBitset::new_empty()));
    base_regions[CASTLE_MINIONS].add_location(BaseConnection::new(LOC13, rules::mrhugs, SimpleBitset::new_empty()));
    base_regions[CASTLE_MINIONS].add_location(BaseConnection::new(LOC25, rules::anysword, SimpleBitset::new_empty()));
    base_regions[CASTLE_MINIONS].add_location(BaseConnection::new(LOC95, rules::always, SimpleBitset::new_empty()));
    base_regions[CASTLE_MINIONS].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // Cloud connections
    base_regions[CLOUD].add_connection(BaseConnection::new(CASTLE_ROOF, rules::always, SimpleBitset::new_empty()));
    // Could also drop to CastleMinions, but that would be redundant
    base_regions[CLOUD].add_connection(BaseConnection::new(CASTLE_CANNON_TO_SHOP, rules::always, SimpleBitset::new_empty()));
    base_regions[CLOUD].add_location(BaseConnection::new(LOC77, rules::always, SimpleBitset::new_empty()));
    base_regions[CLOUD].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // BelowCastleBridge connections
    base_regions[BELOW_CASTLE_BRIDGE].add_jumpconnection(JumpConnection::new(SEWER, rules::always, SimpleBitset::new_empty(), 2.5));
    base_regions[BELOW_CASTLE_BRIDGE].add_jumpconnection(JumpConnection::new(SECRET_PATH_MOAT_WELL, rules::always, SimpleBitset::new_empty(), 3.0));
    base_regions[BELOW_CASTLE_BRIDGE].add_connection(BaseConnection::new(CASTLE_MOAT, rules::always, SimpleBitset::new_empty()));

    // SecretPathMoatWell connections
    base_regions[SECRET_PATH_MOAT_WELL].add_connection(BaseConnection::new(BELOW_CASTLE_BRIDGE, rules::always, SimpleBitset::new_empty()));
    base_regions[SECRET_PATH_MOAT_WELL].add_jumpconnection(JumpConnection::new(CASTLE_MINIONS, rules::always, SimpleBitset::new_empty(), 3.0));
    base_regions[SECRET_PATH_MOAT_WELL].add_jumpconnection(JumpConnection::new(BOMB, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[SECRET_PATH_MOAT_WELL].add_connection(BaseConnection::new(FISHING_ROD, rules::always, SimpleBitset::new_empty()));

    // CastleMoat connections
    base_regions[CASTLE_MOAT].add_jumpconnection(JumpConnection::new(BELOW_CASTLE_BRIDGE, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[CASTLE_MOAT].add_connection(BaseConnection::new(ULTIMATE_DOOR, rules::shovel, SimpleBitset::new_empty()));
    base_regions[CASTLE_MOAT].add_connection(BaseConnection::new(BARN, rules::anysword, SimpleBitset::new_empty()));
    base_regions[CASTLE_MOAT].add_jumpconnection(JumpConnection::new(FISHING_BRIDGE, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[CASTLE_MOAT].add_connection(BaseConnection::new(FISHING_BRIDGE, rules::anysword, SimpleBitset::new_empty()));
    base_regions[CASTLE_MOAT].add_location(BaseConnection::new(LOC95, rules::always, SimpleBitset::new_empty()));
    base_regions[CASTLE_MOAT].add_location(BaseConnection::new(LOC07, rules::no_princess, SimpleBitset::new_empty()));

    // Barn connections
    base_regions[BARN].add_jumpconnection(JumpConnection::new(BARN_SECOND_FLOOR, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[BARN].add_location(BaseConnection::new(LOC86, rules::princess, SimpleBitset::new_empty()));

    // BarnSecondFloor connections
    base_regions[BARN_SECOND_FLOOR].add_location(BaseConnection::new(LOC31, rules::anysword, SimpleBitset::new_empty()));

    // BehindShopBush connections
    base_regions[BEHIND_SHOP_BUSH].add_connection(BaseConnection::new(VOLCANO_DROP_STONE, rules::always, SimpleBitset::new_empty()));
    base_regions[BEHIND_SHOP_BUSH].add_connection(BaseConnection::new(SHOP_LAKE, rules::anysword, SimpleBitset::new_empty()));

    // Shop connections
    fn shotgun(state: &ReventureState) -> bool {
        state.event_bool(States::HasShotgun as u8)
    }
    fn no_shotgun(state: &ReventureState) -> bool {
        !shotgun(state)
    }
    fn princess_no_shotgun(state: &ReventureState) -> bool {
        rules::princess(state) && no_shotgun(state)
    }
    fn no_shotgun_anysword(state: &ReventureState) -> bool {
        no_shotgun(state) && rules::anysword(state)
    }
    fn no_shotgun_mrhugs(state: &ReventureState) -> bool {
        no_shotgun(state) && rules::mrhugs(state)
    }
    fn no_shotgun_hook(state: &ReventureState) -> bool {
        no_shotgun(state) && rules::hook(state)
    }
    base_regions[SHOP].add_connection(BaseConnection::new(SHOP_LAKE, no_shotgun, SimpleBitset::new_empty()));
    base_regions[SHOP].add_jumpconnection(JumpConnection::new(SHOP_ROOF, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[SHOP].add_jumpconnection(JumpConnection::new(NUKE_STORAGE, no_shotgun, SimpleBitset::new_empty(), 4.0));
    base_regions[SHOP].add_connection(BaseConnection::new(NUKE_STORAGE, no_shotgun_hook, SimpleBitset::new_empty()));
    base_regions[SHOP].add_connection(BaseConnection::new(SHOP_CELLAR, princess_no_shotgun, SimpleBitset::new_empty()));
    base_regions[SHOP].add_statechange(StateChange::new(
        vec![States::HasShotgun as u8],
        vec![true],
        no_shotgun_anysword,
        SimpleBitset::new(vec![40]),
    ));
    base_regions[SHOP].add_location(BaseConnection::new(LOC09, no_shotgun_anysword, SimpleBitset::new(vec![40])));
    base_regions[SHOP].add_location(BaseConnection::new(LOC37, no_shotgun_mrhugs, SimpleBitset::new(vec![40])));
    base_regions[SHOP].add_location(BaseConnection::new(LOC95, no_shotgun, SimpleBitset::new_empty()));
    base_regions[SHOP].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // ShopRoof connections
    fn no_shotgun_no_princess_no_nuke(state: &ReventureState) -> bool {
        no_shotgun(state) && rules::no_princess_no_nuke(state)
    }
    fn no_shotgun_no_princess_nuke(state: &ReventureState) -> bool {
        no_shotgun(state) && rules::no_princess(state) && rules::nuke(state)
    }
    base_regions[SHOP_ROOF].add_connection(BaseConnection::new(SHOP, no_shotgun, SimpleBitset::new_empty()));
    base_regions[SHOP_ROOF].add_jumpconnection(JumpConnection::new(OCEAN, no_shotgun, SimpleBitset::new_empty(), 3.0));
    base_regions[SHOP_ROOF].add_connection(BaseConnection::new(OCEAN, no_shotgun_anysword, SimpleBitset::new_empty()));
    base_regions[SHOP_ROOF].add_connection(BaseConnection::new(FORTRESS_MOAT, no_shotgun_no_princess_no_nuke, SimpleBitset::new(vec![20])));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC03, no_shotgun, SimpleBitset::new_empty()));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC13, no_shotgun_mrhugs, SimpleBitset::new_empty()));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC17, no_shotgun_no_princess_no_nuke, SimpleBitset::new(vec![20])));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC25, no_shotgun_anysword, SimpleBitset::new_empty()));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC27, no_shotgun_no_princess_nuke, SimpleBitset::new(vec![20])));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC74, shotgun, SimpleBitset::new(vec![20, 41, 28])));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC74, shotgun, SimpleBitset::new(vec![20, 41, 29])));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(EVENT_KILL_JUAN, no_shotgun_anysword, SimpleBitset::new_empty()));
    base_regions[SHOP_ROOF].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // ShopLake connections
    base_regions[SHOP_LAKE].add_jumpconnection(JumpConnection::new(VOLCANO_TOP_EXIT, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[SHOP_LAKE].add_connection(BaseConnection::new(BEHIND_SHOP_BUSH, rules::anysword, SimpleBitset::new_empty()));
    base_regions[SHOP_LAKE].add_connection(BaseConnection::new(SHOP, rules::always, SimpleBitset::new_empty()));
    base_regions[SHOP_LAKE].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // Ocean connections
    base_regions[OCEAN].add_connection(BaseConnection::new(SHOP_ROOF, rules::always, SimpleBitset::new_empty()));
    base_regions[OCEAN].add_location(BaseConnection::new(LOC95, rules::always, SimpleBitset::new_empty()));
    base_regions[OCEAN].add_location(BaseConnection::new(LOC96, rules::always, SimpleBitset::new_empty()));
    base_regions[OCEAN].add_location(BaseConnection::new(LOC97, rules::always, SimpleBitset::new_empty()));
    base_regions[OCEAN].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // NukeStorage connections
    base_regions[NUKE_STORAGE].add_connection(BaseConnection::new(SHOP, rules::always, SimpleBitset::new_empty()));
    base_regions[NUKE_STORAGE].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // ShopCellar connections
    base_regions[SHOP_CELLAR].add_connection(BaseConnection::new(SHOP, rules::princess, SimpleBitset::new_empty()));
    base_regions[SHOP_CELLAR].add_connection(BaseConnection::new(PARASITE, rules::always, SimpleBitset::new_empty()));
    base_regions[SHOP_CELLAR].add_location(BaseConnection::new(LOC78, rules::always, SimpleBitset::new_empty()));
    base_regions[SHOP_CELLAR].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // Parasite connections
    base_regions[PARASITE].add_location(BaseConnection::new(LOC89, rules::always, SimpleBitset::new_empty()));

    // HookArea connections
    base_regions[HOOK_AREA].add_jumpconnection(JumpConnection::new(CASTLE_MINIONS, rules::always, SimpleBitset::new_empty(), 3.0));
    base_regions[HOOK_AREA].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::hook, SimpleBitset::new_empty()));
    base_regions[HOOK_AREA].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // AboveHook connections
    base_regions[ABOVE_HOOK].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::always, SimpleBitset::new_empty()));
    base_regions[ABOVE_HOOK].add_jumpconnection(JumpConnection::new(ABOVE_ABOVE_HOOK, rules::always, SimpleBitset::new_empty(), 3.0));
    base_regions[ABOVE_HOOK].add_jumpconnection(JumpConnection::new(ABOVE_ABOVE_HOOK, rules::anysword, SimpleBitset::new_empty(), 2.0));
    base_regions[ABOVE_HOOK].add_connection(BaseConnection::new(ABOVE_ABOVE_HOOK, rules::hook, SimpleBitset::new_empty()));
    base_regions[ABOVE_HOOK].add_connection(BaseConnection::new(BOMB, rules::always, SimpleBitset::new_empty()));
    base_regions[ABOVE_HOOK].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // AboveAboveHook connections
    base_regions[ABOVE_ABOVE_HOOK].add_connection(BaseConnection::new(ABOVE_HOOK, rules::always, SimpleBitset::new_empty()));
    base_regions[ABOVE_ABOVE_HOOK].add_jumpconnection(JumpConnection::new(CASTLE_CANNON_TO_SHOP, rules::always, SimpleBitset::new_empty(), 3.0));
    base_regions[ABOVE_ABOVE_HOOK].add_connection(BaseConnection::new(CASTLE_CANNON_TO_SHOP, rules::hook, SimpleBitset::new_empty()));
    base_regions[ABOVE_ABOVE_HOOK].add_jumpconnection(JumpConnection::new(ALTAR, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[ABOVE_ABOVE_HOOK].add_connection(BaseConnection::new(ALTAR, rules::hook, SimpleBitset::new_empty()));
    base_regions[ABOVE_ABOVE_HOOK].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // CastleCannonToShop connections
    base_regions[CASTLE_CANNON_TO_SHOP].add_connection(BaseConnection::new(ABOVE_ABOVE_HOOK, rules::always, SimpleBitset::new_empty()));
    base_regions[CASTLE_CANNON_TO_SHOP].add_connection(BaseConnection::new(SHOP_LAKE, rules::no_princess_no_nuke, SimpleBitset::new(vec![21])));
    base_regions[CASTLE_CANNON_TO_SHOP].add_location(BaseConnection::new(LOC17, rules::no_princess_no_nuke, SimpleBitset::new(vec![21])));
    base_regions[CASTLE_CANNON_TO_SHOP].add_location(BaseConnection::new(LOC56, rules::nuke_no_princess, SimpleBitset::new(vec![21])));
    base_regions[CASTLE_CANNON_TO_SHOP].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // Altar connections
    base_regions[ALTAR].add_connection(BaseConnection::new(ABOVE_ABOVE_HOOK, rules::always, SimpleBitset::new_empty()));
    base_regions[ALTAR].add_jumpconnection(JumpConnection::new(MOUNTAIN_LEFT_OUTCROP, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[ALTAR].add_jumpconnection(JumpConnection::new(LEVERS, rules::always, SimpleBitset::new_empty(), 3.0));
    base_regions[ALTAR].add_connection(BaseConnection::new(LEVERS, rules::hook, SimpleBitset::new_empty()));
    base_regions[ALTAR].add_connection(BaseConnection::new(GREAT_WATERFALL, rules::always, SimpleBitset::new_empty()));
    base_regions[ALTAR].add_location(BaseConnection::new(LOC72, rules::princess, SimpleBitset::new_empty()));
    base_regions[ALTAR].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // Bomb connections
    base_regions[BOMB].add_jumpconnection(JumpConnection::new(ABOVE_HOOK, rules::always, SimpleBitset::new_empty(), 3.0));
    base_regions[BOMB].add_connection(BaseConnection::new(ABOVE_HOOK, rules::hook, SimpleBitset::new_empty()));
    base_regions[BOMB].add_connection(BaseConnection::new(FISHING_ROD, rules::always, SimpleBitset::new_empty()));
    base_regions[BOMB].add_connection(BaseConnection::new(SECRET_PATH_MOAT_WELL, rules::always, SimpleBitset::new_empty()));
    base_regions[BOMB].add_jumpconnection(JumpConnection::new(SECRET_ABOVE_BOMB, rules::always, SimpleBitset::new_empty(), 3.0));
    base_regions[BOMB].add_jumpconnection(JumpConnection::new(GREAT_WATERFALL, rules::bomb, SimpleBitset::new_empty(), 2.0));
    base_regions[BOMB].add_connection(BaseConnection::new(GREAT_WATERFALL, rules::bomb_hook, SimpleBitset::new_empty()));
    base_regions[BOMB].add_location(BaseConnection::new(LOC28, rules::bomb, SimpleBitset::new_empty()));
    base_regions[BOMB].add_location(BaseConnection::new(LOC32, rules::anysword, SimpleBitset::new(vec![45])));
    base_regions[BOMB].add_location(BaseConnection::new(LOC54, rules::mrhugs, SimpleBitset::new(vec![45])));
    base_regions[BOMB].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // FishingBridge connections
    base_regions[FISHING_BRIDGE].add_connection(BaseConnection::new(CASTLE_MOAT, rules::always, SimpleBitset::new_empty()));
    base_regions[FISHING_BRIDGE].add_jumpconnection(JumpConnection::new(FISHING_ROD, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[FISHING_BRIDGE].add_connection(BaseConnection::new(BELOW_FISHING_BRIDGE, rules::always, SimpleBitset::new_empty()));
    base_regions[FISHING_BRIDGE].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // BelowFishingBridge connections
    base_regions[BELOW_FISHING_BRIDGE].add_jumpconnection(JumpConnection::new(FISHING_BRIDGE, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[BELOW_FISHING_BRIDGE].add_connection(BaseConnection::new(WATER_FALLS, rules::always, SimpleBitset::new_empty()));
    base_regions[BELOW_FISHING_BRIDGE].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // FishingRod connections
    base_regions[FISHING_ROD].add_connection(BaseConnection::new(FISHING_BRIDGE, rules::always, SimpleBitset::new_empty()));
    base_regions[FISHING_ROD].add_jumpconnection(JumpConnection::new(BOMB, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[FISHING_ROD].add_location(BaseConnection::new(LOC12, rules::no_princess, SimpleBitset::new(vec![11])));
    base_regions[FISHING_ROD].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // MountainLeftOutcrop connections
    base_regions[MOUNTAIN_LEFT_OUTCROP].add_connection(BaseConnection::new(ALTAR, rules::always, SimpleBitset::new_empty()));
    base_regions[MOUNTAIN_LEFT_OUTCROP].add_jumpconnection(JumpConnection::new(MOUNTAIN_TOP, rules::always, SimpleBitset::new_empty(), 3.0));
    base_regions[MOUNTAIN_LEFT_OUTCROP].add_connection(BaseConnection::new(MOUNTAIN_TOP, rules::sword_or_hook, SimpleBitset::new_empty()));
    base_regions[MOUNTAIN_LEFT_OUTCROP].add_location(BaseConnection::new(LOC46, rules::always, SimpleBitset::new_empty()));
    base_regions[MOUNTAIN_LEFT_OUTCROP].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // MountainTop connections
    base_regions[MOUNTAIN_TOP].add_connection(BaseConnection::new(MOUNTAIN_LEFT_OUTCROP, rules::always, SimpleBitset::new_empty()));
    base_regions[MOUNTAIN_TOP].add_connection(BaseConnection::new(MOUNTAIN_TREASURE, rules::always, SimpleBitset::new_empty()));
    base_regions[MOUNTAIN_TOP].add_connection(BaseConnection::new(CLOUD, rules::chicken, SimpleBitset::new_empty()));
    base_regions[MOUNTAIN_TOP].add_jumpconnection(JumpConnection::new(STRAWBERRY, rules::always, SimpleBitset::new_empty(), 3.0));
    base_regions[MOUNTAIN_TOP].add_location(BaseConnection::new(EVENT_KILL_MIGUEL, rules::anysword, SimpleBitset::new_empty()));
    base_regions[MOUNTAIN_TOP].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // Strawberry connections
    base_regions[STRAWBERRY].add_location(BaseConnection::new(LOC24, rules::always, SimpleBitset::new(vec![19])));
    base_regions[STRAWBERRY].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // MountainTreasure connections
    base_regions[MOUNTAIN_TREASURE].add_connection(BaseConnection::new(BELOW_LEAP_OF_FAITH, rules::always, SimpleBitset::new_empty()));
    base_regions[MOUNTAIN_TREASURE].add_location(BaseConnection::new(LOC33, rules::no_princess, SimpleBitset::new_empty()));
    base_regions[MOUNTAIN_TREASURE].add_location(BaseConnection::new(LOC62, rules::shovel, SimpleBitset::new_empty()));
    base_regions[MOUNTAIN_TREASURE].add_forcedstatechange(StateChange::new(
        vec![States::HasDarkStone as u8, States::DestroyedDarkstone as u8],
        vec![false, true],
        rules::darkstone,
        SimpleBitset::new(vec![]),
    ));

    // Levers connections
    base_regions[LEVERS].add_jumpconnection(JumpConnection::new(ALTAR, rules::always, SimpleBitset::new_empty(), 4.0));
    base_regions[LEVERS].add_connection(BaseConnection::new(ALTAR, rules::hook, SimpleBitset::new_empty()));
    base_regions[LEVERS].add_jumpconnection(JumpConnection::new(BELOW_LEAP_OF_FAITH, rules::always, SimpleBitset::new_empty(), 4.0));
    base_regions[LEVERS].add_connection(BaseConnection::new(BELOW_LEAP_OF_FAITH, rules::hook, SimpleBitset::new_empty()));
    base_regions[LEVERS].add_jumpconnection(JumpConnection::new(DARKSTONE, rules::always, SimpleBitset::new(vec![37]), 3.0));
    base_regions[LEVERS].add_connection(BaseConnection::new(DARKSTONE, rules::hook, SimpleBitset::new(vec![37])));
    base_regions[LEVERS].add_connection(BaseConnection::new(GREAT_WATERFALL, rules::always, SimpleBitset::new(vec![37])));
    base_regions[LEVERS].add_location(BaseConnection::new(LOC38, rules::no_princess, SimpleBitset::new(vec![36])));
    base_regions[LEVERS].add_location(BaseConnection::new(LOC44, rules::no_princess, SimpleBitset::new(vec![38])));

    // Darkstone connections
    base_regions[DARKSTONE].add_connection(BaseConnection::new(LEVERS, rules::always, SimpleBitset::new_empty()));
    base_regions[DARKSTONE].add_statechange(StateChange::new(
        vec![States::HasDarkStone as u8],
        vec![true],
        rules::can_pickup_darkstone,
        SimpleBitset::new(vec![9]),
    ));
    base_regions[DARKSTONE].add_statechange(StateChange::new(
        vec![States::HasBurger as u8],
        vec![true],
        rules::can_pickup_burger,
        SimpleBitset::new(vec![8]),
    ));

    // GreatWaterfall connections
    base_regions[GREAT_WATERFALL].add_jumpconnection(JumpConnection::new(ALTAR, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[GREAT_WATERFALL].add_connection(BaseConnection::new(ALTAR, rules::hook, SimpleBitset::new_empty()));
    base_regions[GREAT_WATERFALL].add_connection(BaseConnection::new(BELOW_FISHING_BRIDGE, rules::always, SimpleBitset::new_empty()));
    base_regions[GREAT_WATERFALL].add_connection(BaseConnection::new(BOMB, rules::bomb, SimpleBitset::new_empty()));
    base_regions[GREAT_WATERFALL].add_connection(BaseConnection::new(GREAT_WATERFALL_BOTTOM, rules::always, SimpleBitset::new_empty()));
    base_regions[GREAT_WATERFALL].add_connection(BaseConnection::new(WHISTLE, rules::always, SimpleBitset::new_empty()));
    base_regions[GREAT_WATERFALL].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::always, SimpleBitset::new_empty()));

    // GreatWaterfallBottom connections
    base_regions[GREAT_WATERFALL_BOTTOM].add_connection(BaseConnection::new(WATER_FALLS, rules::always, SimpleBitset::new_empty()));
    base_regions[GREAT_WATERFALL_BOTTOM].add_jumpconnection(JumpConnection::new(ABOVE_WATERFALLS, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[GREAT_WATERFALL_BOTTOM].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::always, SimpleBitset::new_empty()));

    // SecretAboveBomb connections
    base_regions[SECRET_ABOVE_BOMB].add_connection(BaseConnection::new(BOMB, rules::always, SimpleBitset::new_empty()));
    base_regions[SECRET_ABOVE_BOMB].add_connection(BaseConnection::new(GREAT_WATERFALL, rules::always, SimpleBitset::new_empty()));

    // WaterFalls connections
    base_regions[WATER_FALLS].add_jumpconnection(JumpConnection::new(BELOW_FISHING_BRIDGE, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[WATER_FALLS].add_connection(BaseConnection::new(MOUNTAIN_TOP, rules::chicken_or_shovel_no_princess, SimpleBitset::new(vec![27])));
    base_regions[WATER_FALLS].add_jumpconnection(JumpConnection::new(ABOVE_WATERFALLS, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[WATER_FALLS].add_location(BaseConnection::new(LOC08, rules::no_princess, SimpleBitset::new_empty()));
    base_regions[WATER_FALLS].add_location(BaseConnection::new(LOC82, rules::princess, SimpleBitset::new_empty()));
    base_regions[WATER_FALLS].add_location(BaseConnection::new(LOC87, rules::always, SimpleBitset::new(vec![59,60,61,62,63])));

    // AboveWaterfalls connections
    base_regions[ABOVE_WATERFALLS].add_connection(BaseConnection::new(WATER_FALLS, rules::always, SimpleBitset::new_empty()));
    base_regions[ABOVE_WATERFALLS].add_connection(BaseConnection::new(BELOW_FISHING_BRIDGE, rules::always, SimpleBitset::new_empty()));
    base_regions[ABOVE_WATERFALLS].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::always, SimpleBitset::new_empty()));

    // FortressMoat connections
    base_regions[FORTRESS_MOAT].add_connection(BaseConnection::new(ALBERTO, rules::hook, SimpleBitset::new_empty()));
    base_regions[FORTRESS_MOAT].add_connection(BaseConnection::new(WATER_FALLS, rules::always, SimpleBitset::new_empty()));
    base_regions[FORTRESS_MOAT].add_jumpconnection(JumpConnection::new(ABOVE_WATERFALLS, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[FORTRESS_MOAT].add_connection(BaseConnection::new(FAIRY_FOUNTAIN, rules::always, SimpleBitset::new_empty()));
    base_regions[FORTRESS_MOAT].add_jumpconnection(JumpConnection::new(FORTRESS_BRIDGE_BUTTON, rules::always, SimpleBitset::new_empty(), 3.0));
    base_regions[FORTRESS_MOAT].add_connection(BaseConnection::new(FORTRESS_BRIDGE_BUTTON, rules::hook, SimpleBitset::new_empty()));
    base_regions[FORTRESS_MOAT].add_jumpconnection(JumpConnection::new(RIGHT_OF_FORTRESS, rules::always, SimpleBitset::new_empty(), 3.0));
    base_regions[FORTRESS_MOAT].add_connection(BaseConnection::new(RIGHT_OF_FORTRESS, rules::hook_or_shovel_or_bomb_or_chicken, SimpleBitset::new_empty()));
    base_regions[FORTRESS_MOAT].add_location(BaseConnection::new(LOC15, rules::always, SimpleBitset::new_empty()));
    base_regions[FORTRESS_MOAT].add_location(BaseConnection::new(LOC21, rules::always, SimpleBitset::new_empty()));
    base_regions[FORTRESS_MOAT].add_location(BaseConnection::new(LOC48, rules::always, SimpleBitset::new_empty()));
    base_regions[FORTRESS_MOAT].add_location(BaseConnection::new(LOC49, rules::anysword, SimpleBitset::new_empty()));
    base_regions[FORTRESS_MOAT].add_location(BaseConnection::new(LOC61, rules::always, SimpleBitset::new_empty()));

    // FortressBridgeButton connections
    base_regions[FORTRESS_BRIDGE_BUTTON].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::always, SimpleBitset::new_empty()));
    base_regions[FORTRESS_BRIDGE_BUTTON].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::hook, SimpleBitset::new_empty()));
    base_regions[FORTRESS_BRIDGE_BUTTON].add_jumpconnection(JumpConnection::new(WHISTLE_ALTAR, rules::fortress_bridge_down, SimpleBitset::new_empty(), 2.0));
    base_regions[FORTRESS_BRIDGE_BUTTON].add_statechange(StateChange::new(
        vec![States::FortressBridgeDown as u8],
        vec![true],
        rules::fortress_bridge_up,
        SimpleBitset::new_empty(),
    ));

    // FairyFountain connections
    base_regions[FAIRY_FOUNTAIN].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::always, SimpleBitset::new_empty()));
    base_regions[FAIRY_FOUNTAIN].add_connection(BaseConnection::new(LONKS_FRONTGARDEN, rules::always, SimpleBitset::new(vec![31])));
    base_regions[FAIRY_FOUNTAIN].add_location(BaseConnection::new(LOC65, rules::always, SimpleBitset::new_empty()));
    base_regions[FAIRY_FOUNTAIN].add_location(BaseConnection::new(LOC85, rules::sword_or_mrhugs, SimpleBitset::new_empty()));

    // Whistle connections
    base_regions[WHISTLE].add_jumpconnection(JumpConnection::new(GREAT_WATERFALL, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[WHISTLE].add_connection(BaseConnection::new(GREAT_WATERFALL, rules::hook, SimpleBitset::new_empty()));
    base_regions[WHISTLE].add_connection(BaseConnection::new(GREAT_WATERFALL_BOTTOM, rules::always, SimpleBitset::new_empty()));
    base_regions[WHISTLE].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::always, SimpleBitset::new_empty()));

    // WhistleAltar connections
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(ALBERTO, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(GREAT_WATERFALL, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[WHISTLE_ALTAR].add_connection(BaseConnection::new(GREAT_WATERFALL_BOTTOM, rules::always, SimpleBitset::new_empty()));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(BELOW_LEAP_OF_FAITH, rules::always, SimpleBitset::new_empty(), 3.0));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(ELEVATOR, rules::no_princess, SimpleBitset::new_empty(), 3.0));
    base_regions[WHISTLE_ALTAR].add_connection(BaseConnection::new(ELEVATOR, rules::no_princess_hook_or_fortress_bridge, SimpleBitset::new_empty()));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(FORTRESS_ROOF, rules::fortress_bridge_up, SimpleBitset::new_empty(), 3.0));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(FORTRESS_ROOF, rules::fortress_bridge_up_hook, SimpleBitset::new_empty(), 2.0));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(WHISTLE, rules::always, SimpleBitset::new_empty(), 3.0));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC39, rules::no_princess, SimpleBitset::new_empty()));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC69, rules::anysword_princess, SimpleBitset::new_empty()));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC73, rules::mrhugs_princess, SimpleBitset::new_empty()));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC75, rules::princess, SimpleBitset::new_empty()));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC83, rules::whistle, SimpleBitset::new_empty()));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC90, rules::anysword_princess, SimpleBitset::new_empty()));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC93, rules::darkstone_princess, SimpleBitset::new_empty()));

    // BelowLeapOfFaith connections
    base_regions[BELOW_LEAP_OF_FAITH].add_connection(BaseConnection::new(LEVERS, rules::always, SimpleBitset::new_empty()));
    base_regions[BELOW_LEAP_OF_FAITH].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::always, SimpleBitset::new_empty()));

    // Elevator connections
    base_regions[ELEVATOR].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::fortress_bridge_down_no_princess, SimpleBitset::new_empty()));
    base_regions[ELEVATOR].add_connection(BaseConnection::new(ANVIL, rules::always, SimpleBitset::new(vec![28])));
    base_regions[ELEVATOR].add_connection(BaseConnection::new(ANVIL, rules::always, SimpleBitset::new(vec![29])));
    base_regions[ELEVATOR].add_jumpconnection(JumpConnection::new(RIGHT_OF_FORTRESS, rules::always, SimpleBitset::new_empty(), 4.0));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC34, rules::always, SimpleBitset::new(vec![28])));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC34, rules::always, SimpleBitset::new(vec![29])));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC34, rules::princess, SimpleBitset::new_empty()));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC50, rules::princess, SimpleBitset::new_empty()));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC66, rules::darkstone, SimpleBitset::new_empty()));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC76, rules::princess, SimpleBitset::new_empty()));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC80, rules::chicken, SimpleBitset::new_empty()));

    // FortressRoof connections
    base_regions[FORTRESS_ROOF].add_jumpconnection(JumpConnection::new(WHISTLE_ALTAR, rules::always, SimpleBitset::new_empty(), 4.0));
    base_regions[FORTRESS_ROOF].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::fortress_bridge_down, SimpleBitset::new_empty()));
    base_regions[FORTRESS_ROOF].add_connection(BaseConnection::new(ANVIL, rules::always, SimpleBitset::new_empty()));
    base_regions[FORTRESS_ROOF].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::no_princess_no_nuke, SimpleBitset::new(vec![22])));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC17, rules::no_princess_no_nuke, SimpleBitset::new(vec![22])));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC42, rules::no_princess, SimpleBitset::new(vec![17])));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC52, rules::princess, SimpleBitset::new(vec![22])));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC55, rules::no_chicken_princess, SimpleBitset::new_empty()));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC58, rules::no_chicken_no_princess, SimpleBitset::new_empty()));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC84, rules::nuke_no_princess, SimpleBitset::new(vec![22])));

    // Anvil connections
    base_regions[ANVIL].add_jumpconnection(JumpConnection::new(FORTRESS_ROOF, rules::always, SimpleBitset::new_empty(), 4.0));
    base_regions[ANVIL].add_connection(BaseConnection::new(FORTRESS_ROOF, rules::hook, SimpleBitset::new_empty()));
    base_regions[ANVIL].add_connection(BaseConnection::new(ELEVATOR, rules::always, SimpleBitset::new(vec![28])));
    base_regions[ANVIL].add_connection(BaseConnection::new(ELEVATOR, rules::always, SimpleBitset::new(vec![29])));
    base_regions[ANVIL].add_jumpconnection(JumpConnection::new(PRINCESS, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[ANVIL].add_connection(BaseConnection::new(PRINCESS, rules::hook, SimpleBitset::new_empty()));
    base_regions[ANVIL].add_connection(BaseConnection::new(FIRE_ESCAPE, rules::princess, SimpleBitset::new_empty()));
    base_regions[ANVIL].add_connection(BaseConnection::new(FORTRESS_TREASURE, rules::princess, SimpleBitset::new_empty()));
    base_regions[ANVIL].add_location(BaseConnection::new(LOC22, rules::always, SimpleBitset::new(vec![18])));
    base_regions[ANVIL].add_location(BaseConnection::new(LOC23, rules::always, SimpleBitset::new(vec![41])));
    base_regions[ANVIL].add_location(BaseConnection::new(LOC53, rules::princess, SimpleBitset::new_empty()));
    base_regions[ANVIL].add_location(BaseConnection::new(LOC98, rules::no_princess_burger, SimpleBitset::new(vec![41])));

    // Princess connections
    base_regions[PRINCESS].add_connection(BaseConnection::new(ANVIL, rules::always, SimpleBitset::new_empty()));
    base_regions[PRINCESS].add_jumpconnection(JumpConnection::new(SPIKE_TRAP, rules::no_princess, SimpleBitset::new_empty(), 2.0));
    base_regions[PRINCESS].add_connection(BaseConnection::new(SPIKE_TRAP, rules::no_princess_and_hook, SimpleBitset::new_empty()));
    base_regions[PRINCESS].add_statechange(StateChange::new(
        vec![States::HasPrincess as u8, States::FortressBridgeDown as u8],
        vec![true, true],
        rules::can_pickup_princess,
        SimpleBitset::new(vec![17]),
    ));
    base_regions[PRINCESS].add_location(BaseConnection::new(LOC45, rules::princess, SimpleBitset::new_empty()));
    base_regions[PRINCESS].add_location(BaseConnection::new(LOC57, rules::mrhugs_princess, SimpleBitset::new_empty()));
    base_regions[PRINCESS].add_location(BaseConnection::new(LOC64, rules::no_princess_sword, SimpleBitset::new_empty()));

    // SpikeTrap connections
    base_regions[SPIKE_TRAP].add_location(BaseConnection::new(LOC70, rules::always, SimpleBitset::new_empty()));

    // FireEscape connections
    base_regions[FIRE_ESCAPE].add_connection(BaseConnection::new(ELEVATOR, rules::always, SimpleBitset::new_empty()));
    base_regions[FIRE_ESCAPE].add_jumpconnection(JumpConnection::new(FORTRESS_ROOF, rules::always, SimpleBitset::new_empty(), 2.0));
    base_regions[FIRE_ESCAPE].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::fortress_bridge_down, SimpleBitset::new_empty()));

    // FortressTreasure connections
    base_regions[FORTRESS_TREASURE].add_connection(BaseConnection::new(RIGHT_OF_FORTRESS, rules::always, SimpleBitset::new_empty()));
    base_regions[FORTRESS_TREASURE].add_location(BaseConnection::new(LOC68, rules::always, SimpleBitset::new_empty()));
    base_regions[FORTRESS_TREASURE].add_location(BaseConnection::new(EVENT_KILL_JAVI, rules::anysword, SimpleBitset::new_empty()));

    // RightOfFortress connections
    base_regions[RIGHT_OF_FORTRESS].add_jumpconnection(JumpConnection::new(FORTRESS_TREASURE, rules::always, SimpleBitset::new_empty(), 3.0));
    base_regions[RIGHT_OF_FORTRESS].add_connection(BaseConnection::new(ELEVATOR, rules::always, SimpleBitset::new_empty()));
    base_regions[RIGHT_OF_FORTRESS].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::hook, SimpleBitset::new_empty()));
    // Same problem as greedy bastard
    // fn can_enter_desert(state: &ReventureState) -> bool {
    //     state.get_weight() >= 2.0
    // }
    // base_regions[RIGHT_OF_FORTRESS].add_connection(BaseConnection::new(DESERT, can_enter_desert, SimpleBitset::new_empty()));
    base_regions[RIGHT_OF_FORTRESS].add_location(BaseConnection::new(LOC81, rules::princess, SimpleBitset::new_empty()));

    // Desert connections
    base_regions[DESERT].add_location(BaseConnection::new(LOC91, rules::always, SimpleBitset::new_empty()));
    base_regions[DESERT].add_location(BaseConnection::new(LOC88, rules::always, SimpleBitset::new(vec![25])));

    // Alberto connections
    base_regions[ALBERTO].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::always, SimpleBitset::new_empty()));
    base_regions[ALBERTO].add_connection(BaseConnection::new(GREAT_WATERFALL_BOTTOM, rules::always, SimpleBitset::new_empty()));
    base_regions[ALBERTO].add_location(BaseConnection::new(EVENT_KILL_ALBERTO, rules::anysword, SimpleBitset::new_empty()));

    println!("Region connections setup complete!");
}
