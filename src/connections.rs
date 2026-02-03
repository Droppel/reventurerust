use std::vec;

use crate::{BaseRegion, BaseConnection, JumpConnection, StateChange, ReventureState, APItems};
use crate::locations::{locations::*, events::*, regions::*};

// Rule helper functions - these replace the lambda functions from Python
pub mod rules {
    use super::*;

    pub fn always(_state: &ReventureState) -> bool {
        true
    }

    pub fn no_princess(state: &ReventureState) -> bool {
        !state.event_bool("has_princess")
    }

    pub fn princess(state: &ReventureState) -> bool {
        state.event_bool("has_princess")
    }

    pub fn shovel(state: &ReventureState) -> bool {
        state.event_bool("has_shovel")
    }

    pub fn anysword(state: &ReventureState) -> bool {
        state.event_bool("has_sword") || state.event_bool("has_swordelder")
    }

    pub fn mrhugs(state: &ReventureState) -> bool {
        state.event_bool("has_mrhugs")
    }

    pub fn shield(state: &ReventureState) -> bool {
        state.event_bool("has_shield")
    }

    pub fn lavatrinket(state: &ReventureState) -> bool {
        state.event_bool("has_lavaTrinket")
    }

    pub fn hook(state: &ReventureState) -> bool {
        state.event_bool("has_hook")
    }

    pub fn bomb(state: &ReventureState) -> bool {
        state.event_bool("has_bomb")
    }

    pub fn bomb_hook(state: &ReventureState) -> bool {
        bomb(state) || hook(state)
    }

    pub fn nuke(state: &ReventureState) -> bool {
        state.event_bool("has_nuke")
    }

    pub fn chicken(state: &ReventureState) -> bool {
        state.event_bool("has_chicken")
    }

    pub fn whistle(state: &ReventureState) -> bool {
        state.event_bool("has_whistle")
    }

    pub fn darkstone(state: &ReventureState) -> bool {
        state.event_bool("has_darkstone")
    }

    pub fn burger(state: &ReventureState) -> bool {
        state.event_bool("has_burger")
    }

    pub fn castle_bridge_down(state: &ReventureState) -> bool {
        state.event_bool("castleBridgeDown")
    }

    pub fn castle_bridge_up(state: &ReventureState) -> bool {
        !state.event_bool("castleBridgeDown")
    }

    pub fn fortress_bridge_down(state: &ReventureState) -> bool {
        state.event_bool("fortressBridgeDown")
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

    pub fn hook_or_shovel_or_bomb(state: &ReventureState) -> bool {
        hook(state) || shovel(state) || bomb(state)
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
        !state.event_bool("has_sword") && 
        !state.event_bool("has_swordelder") && 
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
        vec!["has_sword".to_string()],
        vec![true],
        rules::can_pickup_sword,
        APItems::new(vec![56]),
    ));

    // Item 1: Sword Pedestal (Elder)
    base_regions[item_locations[1]].add_statechange(StateChange::new(
        vec!["has_swordelder".to_string()],
        vec![true],
        rules::can_pickup_sword,
        APItems::new(vec![2]),
    ));

    // Item 2: Shovel
    base_regions[item_locations[2]].add_statechange(StateChange::new(
        vec!["has_shovel".to_string()],
        vec![true],
        rules::can_pickup_shovel,
        APItems::new(vec![3]),
    ));

    // Item 3: Bomb
    base_regions[item_locations[3]].add_statechange(StateChange::new(
        vec!["has_bomb".to_string()],
        vec![true],
        rules::can_pickup_bomb,
        APItems::new(vec![14]),
    ));

    // Item 4: Shield
    base_regions[item_locations[4]].add_statechange(StateChange::new(
        vec!["has_shield".to_string()],
        vec![true],
        rules::can_pickup_shield,
        APItems::new(vec![15]),
    ));

    // Item 5: Mister Hugs
    base_regions[item_locations[5]].add_statechange(StateChange::new(
        vec!["has_mrhugs".to_string()],
        vec![true],
        rules::can_pickup_mrhugs,
        APItems::new(vec![13]),
    ));

    // Item 6: Lava Trinket
    base_regions[item_locations[6]].add_statechange(StateChange::new(
        vec!["has_lavaTrinket".to_string()],
        vec![true],
        rules::can_pickup_lavatrinket,
        APItems::new(vec![12]),
    ));

    // Item 7: Hook
    base_regions[item_locations[7]].add_statechange(StateChange::new(
        vec!["has_hook".to_string()],
        vec![true],
        rules::can_pickup_hook,
        APItems::new(vec![10]),
    ));

    // Item 8: Nuke
    base_regions[item_locations[8]].add_statechange(StateChange::new(
        vec!["has_nuke".to_string()],
        vec![true],
        rules::can_pickup_nuke,
        APItems::new(vec![16]),
    ));

    // Item 9: Whistle
    base_regions[item_locations[9]].add_statechange(StateChange::new(
        vec!["has_whistle".to_string()],
        vec![true],
        rules::can_pickup_whistle,
        APItems::new(vec![7]),
    ));
}

/// Set up all region connections - this is the main function that creates the game graph
pub fn setup_region_connections(base_regions: &mut [BaseRegion], start_region: usize) {
    // Menu connections
    base_regions[MENU].add_connection(BaseConnection::new(start_region, rules::always, APItems::new_empty()));
    base_regions[MENU].add_location(BaseConnection::new(LOC59, rules::always, APItems::new_empty()));

    // LonksHouse connections
    base_regions[LONKS_HOUSE].add_connection(BaseConnection::new(LONKS_FRONTGARDEN, rules::no_princess, APItems::new_empty()));
    base_regions[LONKS_HOUSE].add_connection(BaseConnection::new(LONKS_BACKGARDEN, rules::no_princess, APItems::new_empty()));
    base_regions[LONKS_HOUSE].add_jumpconnection(JumpConnection::new(SWORD_CHEST, rules::no_princess, APItems::new_empty(), 2.0));
    base_regions[LONKS_HOUSE].add_location(BaseConnection::new(LOC20, rules::no_princess, APItems::new_empty()));
    base_regions[LONKS_HOUSE].add_location(BaseConnection::new(LOC94, rules::princess, APItems::new_empty()));

    // LonksBackGarden connections
    base_regions[LONKS_BACKGARDEN].add_jumpconnection(JumpConnection::new(ELDER, rules::always, APItems::new_empty(), 2.0));
    base_regions[LONKS_BACKGARDEN].add_connection(BaseConnection::new(LONKS_HOUSE, rules::always, APItems::new_empty()));
    base_regions[LONKS_BACKGARDEN].add_connection(BaseConnection::new(VOLCANO_BRIDGE, rules::shovel, APItems::new_empty()));
    base_regions[LONKS_BACKGARDEN].add_location(BaseConnection::new(LOC03, rules::always, APItems::new_empty()));

    // LonksFrontGarden connections
    base_regions[LONKS_FRONTGARDEN].add_connection(BaseConnection::new(LONKS_HOUSE, rules::always, APItems::new_empty()));
    base_regions[LONKS_FRONTGARDEN].add_connection(BaseConnection::new(CASTLE_FIRST_FLOOR, rules::always, APItems::new_empty()));
    base_regions[LONKS_FRONTGARDEN].add_connection(BaseConnection::new(FAIRY_FOUNTAIN, rules::always,
         APItems::new(vec![31])));
    base_regions[LONKS_FRONTGARDEN].add_location(BaseConnection::new(LOC02, rules::always, 
        APItems::new(vec![34])));
    base_regions[LONKS_FRONTGARDEN].add_location(BaseConnection::new(LOC04, rules::anysword, APItems::new_empty()));
    base_regions[LONKS_FRONTGARDEN].add_location(BaseConnection::new(LOC19, rules::mrhugs, APItems::new_empty()));

    // SwordChest connections
    base_regions[SWORD_CHEST].add_connection(BaseConnection::new(LONKS_HOUSE, rules::always, APItems::new_empty()));

    // Elder connections
    base_regions[ELDER].add_jumpconnection(JumpConnection::new(CHICKEN, rules::always, APItems::new_empty(), 2.0));
    base_regions[ELDER].add_connection(BaseConnection::new(SHOVEL, rules::always, APItems::new_empty()));
    base_regions[ELDER].add_jumpconnection(JumpConnection::new(LONKS_BACKGARDEN, rules::always, APItems::new_empty(), 2.0));
    base_regions[ELDER].add_jumpconnection(JumpConnection::new(VOLCANO_TOP_EXIT, rules::always, APItems::new_empty(), 2.0));
    base_regions[ELDER].add_location(BaseConnection::new(LOC01, rules::anysword, 
        APItems::new(vec![44])));
    base_regions[ELDER].add_location(BaseConnection::new(LOC40, rules::mrhugs, 
        APItems::new(vec![44])));

    // Chicken connections
    base_regions[CHICKEN].add_connection(BaseConnection::new(ELDER, rules::always, APItems::new_empty()));
    base_regions[CHICKEN].add_connection(BaseConnection::new(LONKS_BACKGARDEN, rules::always, APItems::new_empty()));
    base_regions[CHICKEN].add_statechange(StateChange::new(
        vec!["has_chicken".to_string()],
        vec![true],
        rules::can_pickup_chicken,
        APItems::new(vec![43]),
    ));
    fn rule_loc63(state: &ReventureState) -> bool {
        !rules::chicken(state) && rules::anysword(state)
    }
    base_regions[CHICKEN].add_location(BaseConnection::new(LOC63, rule_loc63, APItems::new(vec![43])));
    fn rule_loc79(state: &ReventureState) -> bool {
        !rules::chicken(state) && rules::mrhugs(state)
    }
    base_regions[CHICKEN].add_location(BaseConnection::new(LOC79, rule_loc79, APItems::new(vec![43])));

    // Shovel connections
    base_regions[SHOVEL].add_jumpconnection(JumpConnection::new(ELDER, rules::always, APItems::new_empty(), 3.0));
    base_regions[SHOVEL].add_connection(BaseConnection::new(LONKS_BACKGARDEN, rules::shovel, APItems::new_empty()));

    // CastleFirstFloor connections
    base_regions[CASTLE_FIRST_FLOOR].add_connection(BaseConnection::new(LONKS_FRONTGARDEN, rules::no_burger_no_princess, APItems::new_empty()));
    base_regions[CASTLE_FIRST_FLOOR].add_jumpconnection(JumpConnection::new(CASTLE_SHIELD_CHEST, rules::no_burger_no_princess, APItems::new_empty(), 2.0));
    base_regions[CASTLE_FIRST_FLOOR].add_jumpconnection(JumpConnection::new(CASTLE_MAP_CHEST, rules::no_burger_no_princess, APItems::new_empty(), 3.0));
    base_regions[CASTLE_FIRST_FLOOR].add_connection(BaseConnection::new(SEWER, rules::no_burger_no_princess, APItems::new(vec![33])));
    base_regions[CASTLE_FIRST_FLOOR].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::no_burger_no_princess_castle_bridge, APItems::new_empty()));
    fn can_lower_castle_bridge(state: &ReventureState) -> bool {
        rules::no_burger_no_princess(state) && 
        !rules::castle_bridge_down(state) && 
        (rules::anysword(state) || rules::shovel(state))
    }
    base_regions[CASTLE_FIRST_FLOOR].add_statechange(StateChange::new(
        vec!["castleBridgeDown".to_string()],
        vec![true],
        can_lower_castle_bridge,
        APItems::new_empty(),
    ));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC04, rules::no_burger_no_princess_sword, APItems::new_empty()));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC05, rules::no_burger_no_princess_sword, APItems::new(vec![42])));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC18, rules::no_burger_no_princess_mrhugs, APItems::new(vec![42])));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC19, rules::no_burger_no_princess_mrhugs, APItems::new_empty()));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC51, rules::no_burger_has_princess, APItems::new_empty()));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC60, rules::no_burger_no_princess_bomb, APItems::new_empty()));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC99, rules::no_princess_has_burger, APItems::new(vec![42])));

    // CastleShieldChest connections
    base_regions[CASTLE_SHIELD_CHEST].add_connection(BaseConnection::new(CASTLE_FIRST_FLOOR, rules::always, APItems::new_empty()));

    // CastleMapChest connections
    base_regions[CASTLE_MAP_CHEST].add_connection(BaseConnection::new(CASTLE_FIRST_FLOOR, rules::always, APItems::new_empty()));
    base_regions[CASTLE_MAP_CHEST].add_jumpconnection(JumpConnection::new(CASTLE_ROOF, rules::always, APItems::new_empty(), 3.0));

    // CastleRoof connections
    base_regions[CASTLE_ROOF].add_connection(BaseConnection::new(CASTLE_MAP_CHEST, rules::always, APItems::new_empty()));
    base_regions[CASTLE_ROOF].add_connection(BaseConnection::new(PRINCESS_ROOM, rules::always, APItems::new_empty()));
    base_regions[CASTLE_ROOF].add_jumpconnection(JumpConnection::new(CHIMNEY, rules::always, APItems::new_empty(), 3.0));
    // base_regions[CASTLE_ROOF].add_location(BaseConnection::new(LOC17, rules::always, vec!["Castle To Dark Fortress Cannon".to_string()]));

    // Chimney connections
    base_regions[CHIMNEY].add_location(BaseConnection::new(LOC30, rules::always, APItems::new_empty()));

    // PrincessRoom connections
    base_regions[PRINCESS_ROOM].add_jumpconnection(JumpConnection::new(CASTLE_ROOF, rules::always, APItems::new_empty(), 3.0));
    base_regions[PRINCESS_ROOM].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::always, APItems::new_empty()));
    base_regions[PRINCESS_ROOM].add_connection(BaseConnection::new(ANVIL, rules::always, APItems::new(vec![30])));
    base_regions[PRINCESS_ROOM].add_location(BaseConnection::new(LOC04, rules::anysword, APItems::new_empty()));
    base_regions[PRINCESS_ROOM].add_location(BaseConnection::new(LOC11, rules::mrhugs, APItems::new_empty()));
    base_regions[PRINCESS_ROOM].add_location(BaseConnection::new(LOC19, rules::mrhugs, APItems::new_empty()));

    // VolcanoTopExit connections
    base_regions[VOLCANO_TOP_EXIT].add_connection(BaseConnection::new(ELDER, rules::always, APItems::new_empty()));
    base_regions[VOLCANO_TOP_EXIT].add_connection(BaseConnection::new(LAVA_TRINKET, rules::always, APItems::new_empty()));
    base_regions[VOLCANO_TOP_EXIT].add_connection(BaseConnection::new(SHOP_LAKE, rules::always, APItems::new_empty()));

    // LavaTrinket connections
    base_regions[LAVA_TRINKET].add_jumpconnection(JumpConnection::new(VOLCANO_TOP_EXIT, rules::always, APItems::new_empty(), 2.0));
    base_regions[LAVA_TRINKET].add_connection(BaseConnection::new(VOLCANO_BRIDGE, rules::always, APItems::new_empty()));

    // VolcanoDropStone connections
    base_regions[VOLCANO_DROP_STONE].add_jumpconnection(JumpConnection::new(VOLCANO_BRIDGE, rules::always, APItems::new_empty(), 2.0));
    base_regions[VOLCANO_DROP_STONE].add_jumpconnection(JumpConnection::new(BEHIND_SHOP_BUSH, rules::always, APItems::new_empty(), 2.0));
    base_regions[VOLCANO_DROP_STONE].add_location(BaseConnection::new(LOC06, rules::no_princess, APItems::new_empty()));

    // VolcanoBridge connections
    base_regions[VOLCANO_BRIDGE].add_connection(BaseConnection::new(VOLCANO_DROP_STONE, rules::always, APItems::new_empty()));
    base_regions[VOLCANO_BRIDGE].add_connection(BaseConnection::new(BELOW_VOLCANO_BRIDGE, rules::always, APItems::new_empty()));
    base_regions[VOLCANO_BRIDGE].add_jumpconnection(JumpConnection::new(LAVA_TRINKET, rules::always, APItems::new_empty(), 2.0));
    base_regions[VOLCANO_BRIDGE].add_jumpconnection(JumpConnection::new(SEWER, rules::always, APItems::new_empty(), 3.0));
    base_regions[VOLCANO_BRIDGE].add_connection(BaseConnection::new(SEWER, rules::sword_or_hook, APItems::new_empty()));

    // Sewer connections
    base_regions[SEWER].add_jumpconnection(JumpConnection::new(CASTLE_FIRST_FLOOR, rules::always, APItems::new(vec![33]), 3.0));
    base_regions[SEWER].add_connection(BaseConnection::new(VOLCANO_BRIDGE, rules::always, APItems::new_empty()));
    base_regions[SEWER].add_connection(BaseConnection::new(BELOW_CASTLE_BRIDGE, rules::always, APItems::new_empty()));
    base_regions[SEWER].add_connection(BaseConnection::new(MUSIC_CLUB, rules::shovel, APItems::new_empty()));

    // MusicClub connections
    base_regions[MUSIC_CLUB].add_connection(BaseConnection::new(BELOW_VOLCANO_BRIDGE, rules::always, APItems::new_empty()));
    base_regions[MUSIC_CLUB].add_connection(BaseConnection::new(SEWER_PIPE, rules::shovel, APItems::new_empty()));
    base_regions[MUSIC_CLUB].add_location(BaseConnection::new(EVENT_KILL_DANIEL, rules::anysword, APItems::new_empty()));

    // BelowVolcanoBridge connections
    base_regions[BELOW_VOLCANO_BRIDGE].add_connection(BaseConnection::new(LEFT_OF_DRAGON, rules::shovel, APItems::new_empty()));
    base_regions[BELOW_VOLCANO_BRIDGE].add_jumpconnection(JumpConnection::new(GOLD_ROOM, rules::always, APItems::new_empty(), 2.0));
    base_regions[BELOW_VOLCANO_BRIDGE].add_connection(BaseConnection::new(PARASITE, rules::shovel, APItems::new_empty()));
    base_regions[BELOW_VOLCANO_BRIDGE].add_location(BaseConnection::new(LOC06, rules::no_princess, APItems::new_empty()));

    // GoldRoom connections
    base_regions[GOLD_ROOM].add_connection(BaseConnection::new(RIGHT_OF_DRAGON, rules::always, APItems::new_empty()));
    base_regions[GOLD_ROOM].add_jumpconnection(JumpConnection::new(SEWER_PIPE, rules::always, APItems::new_empty(), 2.0));

    // LeftOfDragon connections
    base_regions[LEFT_OF_DRAGON].add_connection(BaseConnection::new(VOLCANO_GEYSER, rules::shovel, APItems::new_empty()));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC10, rules::shovel, APItems::new_empty()));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC14, rules::no_princess_no_shield_no_lavatrinket, APItems::new(vec![39])));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC29, rules::no_princess_shield_no_lavatrinket, APItems::new(vec![39])));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC36, rules::no_princess_no_shield_lavatrinket, APItems::new(vec![39])));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC41, rules::no_princess_shield_lavatrinket, APItems::new(vec![39])));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC92, rules::princess, APItems::new(vec![39])));

    // RightOfDragon connections
    base_regions[RIGHT_OF_DRAGON].add_connection(BaseConnection::new(VOLCANO_GEYSER, rules::always, APItems::new_empty()));
    base_regions[RIGHT_OF_DRAGON].add_jumpconnection(JumpConnection::new(GOLD_ROOM, rules::always, APItems::new_empty(), 4.0));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC14, rules::always, APItems::new(vec![39])));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC16, rules::anysword, APItems::new(vec![39])));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC29, rules::shield_no_lavatrinket, APItems::new(vec![39])));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC36, rules::no_shield_has_lava, APItems::new(vec![39])));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC41, rules::has_shield_and_lava, APItems::new(vec![39])));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC43, rules::mrhugs, APItems::new(vec![39])));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC92, rules::princess, APItems::new(vec![39])));

    // SewerPipe connections
    base_regions[SEWER_PIPE].add_connection(BaseConnection::new(GOLD_ROOM, rules::always, APItems::new_empty()));
    base_regions[SEWER_PIPE].add_location(BaseConnection::new(LOC35, rules::always, APItems::new(vec![35])));

    // VolcanoGeyser connections
    base_regions[VOLCANO_GEYSER].add_connection(BaseConnection::new(LEFT_OF_DRAGON, rules::lavatrinket, APItems::new_empty()));
    base_regions[VOLCANO_GEYSER].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::always, APItems::new(vec![26])));
    base_regions[VOLCANO_GEYSER].add_jumpconnection(JumpConnection::new(ULTIMATE_DOOR, rules::always, APItems::new_empty(), 2.0));
    base_regions[VOLCANO_GEYSER].add_location(BaseConnection::new(LOC06, rules::no_princess, APItems::new_empty()));

    // UltimateDoor connections
    base_regions[ULTIMATE_DOOR].add_connection(BaseConnection::new(VOLCANO_GEYSER, rules::always, APItems::new_empty()));
    base_regions[ULTIMATE_DOOR].add_location(BaseConnection::new(LOC67, rules::always, APItems::new_empty()));
    base_regions[ULTIMATE_DOOR].add_location(BaseConnection::new(LOC100, rules::always, APItems::new_empty()));

    // CastleMinions connections
    base_regions[CASTLE_MINIONS].add_connection(BaseConnection::new(CASTLE_FIRST_FLOOR, rules::castle_bridge_down, APItems::new_empty()));
    base_regions[CASTLE_MINIONS].add_statechange(StateChange::new(
        vec!["castleBridgeDown".to_string()],
        vec![true],
        rules::princess,
        APItems::new_empty(),
    ));
    base_regions[CASTLE_MINIONS].add_connection(BaseConnection::new(SECRET_PATH_MOAT_WELL, rules::castle_bridge_up, APItems::new_empty()));
    base_regions[CASTLE_MINIONS].add_connection(BaseConnection::new(HOOK_AREA, rules::always, APItems::new_empty()));
    base_regions[CASTLE_MINIONS].add_jumpconnection(JumpConnection::new(ABOVE_HOOK, rules::always, APItems::new_empty(), 2.0));
    base_regions[CASTLE_MINIONS].add_connection(BaseConnection::new(ABOVE_HOOK, rules::hook, APItems::new_empty()));
    base_regions[CASTLE_MINIONS].add_connection(BaseConnection::new(CLOUD, rules::always, APItems::new(vec![32])));
    base_regions[CASTLE_MINIONS].add_location(BaseConnection::new(LOC03, rules::always, APItems::new_empty()));
    base_regions[CASTLE_MINIONS].add_location(BaseConnection::new(LOC13, rules::mrhugs, APItems::new_empty()));
    base_regions[CASTLE_MINIONS].add_location(BaseConnection::new(LOC25, rules::anysword, APItems::new_empty()));
    base_regions[CASTLE_MINIONS].add_location(BaseConnection::new(LOC95, rules::always, APItems::new_empty()));

    // Cloud connections
    base_regions[CLOUD].add_connection(BaseConnection::new(CASTLE_ROOF, rules::always, APItems::new_empty()));
    // Could also drop to CastleMinions, but that would be redundant
    base_regions[CLOUD].add_connection(BaseConnection::new(CASTLE_CANNON_TO_SHOP, rules::always, APItems::new_empty()));
    base_regions[CLOUD].add_location(BaseConnection::new(LOC77, rules::always, APItems::new_empty()));

    // BelowCastleBridge connections
    base_regions[BELOW_CASTLE_BRIDGE].add_jumpconnection(JumpConnection::new(SEWER, rules::always, APItems::new_empty(), 2.5));
    base_regions[BELOW_CASTLE_BRIDGE].add_jumpconnection(JumpConnection::new(SECRET_PATH_MOAT_WELL, rules::always, APItems::new_empty(), 3.0));
    base_regions[BELOW_CASTLE_BRIDGE].add_connection(BaseConnection::new(CASTLE_MOAT, rules::always, APItems::new_empty()));

    // SecretPathMoatWell connections
    base_regions[SECRET_PATH_MOAT_WELL].add_connection(BaseConnection::new(BELOW_CASTLE_BRIDGE, rules::always, APItems::new_empty()));
    base_regions[SECRET_PATH_MOAT_WELL].add_jumpconnection(JumpConnection::new(CASTLE_MINIONS, rules::always, APItems::new_empty(), 3.0));
    base_regions[SECRET_PATH_MOAT_WELL].add_jumpconnection(JumpConnection::new(BOMB, rules::always, APItems::new_empty(), 2.0));
    base_regions[SECRET_PATH_MOAT_WELL].add_connection(BaseConnection::new(FISHING_ROD, rules::always, APItems::new_empty()));

    // CastleMoat connections
    base_regions[CASTLE_MOAT].add_jumpconnection(JumpConnection::new(BELOW_CASTLE_BRIDGE, rules::always, APItems::new_empty(), 2.0));
    base_regions[CASTLE_MOAT].add_connection(BaseConnection::new(ULTIMATE_DOOR, rules::shovel, APItems::new_empty()));
    base_regions[CASTLE_MOAT].add_connection(BaseConnection::new(BARN, rules::anysword, APItems::new_empty()));
    base_regions[CASTLE_MOAT].add_jumpconnection(JumpConnection::new(FISHING_BRIDGE, rules::always, APItems::new_empty(), 2.0));
    base_regions[CASTLE_MOAT].add_connection(BaseConnection::new(FISHING_BRIDGE, rules::anysword, APItems::new_empty()));
    base_regions[CASTLE_MOAT].add_location(BaseConnection::new(LOC95, rules::always, APItems::new_empty()));
    base_regions[CASTLE_MOAT].add_location(BaseConnection::new(LOC07, rules::no_princess, APItems::new_empty()));

    // Barn connections
    base_regions[BARN].add_jumpconnection(JumpConnection::new(BARN_SECOND_FLOOR, rules::always, APItems::new_empty(), 2.0));
    base_regions[BARN].add_location(BaseConnection::new(LOC86, rules::princess, APItems::new_empty()));

    // BarnSecondFloor connections
    base_regions[BARN_SECOND_FLOOR].add_location(BaseConnection::new(LOC31, rules::anysword, APItems::new_empty()));

    // BehindShopBush connections
    base_regions[BEHIND_SHOP_BUSH].add_connection(BaseConnection::new(VOLCANO_DROP_STONE, rules::always, APItems::new_empty()));
    base_regions[BEHIND_SHOP_BUSH].add_connection(BaseConnection::new(SHOP_LAKE, rules::anysword, APItems::new_empty()));

    // Shop connections
    fn shotgun(state: &ReventureState) -> bool {
        state.event_bool("has_shotgun")
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
    base_regions[SHOP].add_connection(BaseConnection::new(SHOP_LAKE, no_shotgun, APItems::new_empty()));
    base_regions[SHOP].add_jumpconnection(JumpConnection::new(SHOP_ROOF, rules::always, APItems::new_empty(), 2.0));
    base_regions[SHOP].add_jumpconnection(JumpConnection::new(NUKE_STORAGE, no_shotgun, APItems::new_empty(), 4.0));
    base_regions[SHOP].add_connection(BaseConnection::new(NUKE_STORAGE, no_shotgun_hook, APItems::new_empty()));
    base_regions[SHOP].add_connection(BaseConnection::new(SHOP_CELLAR, princess_no_shotgun, APItems::new_empty()));
    base_regions[SHOP].add_statechange(StateChange::new(
        vec!["has_shotgun".to_string()],
        vec![true],
        no_shotgun_anysword,
        APItems::new(vec![40]),
    ));
    base_regions[SHOP].add_location(BaseConnection::new(LOC09, no_shotgun_anysword, APItems::new(vec![40])));
    base_regions[SHOP].add_location(BaseConnection::new(LOC37, no_shotgun_mrhugs, APItems::new(vec![40])));
    base_regions[SHOP].add_location(BaseConnection::new(LOC95, no_shotgun, APItems::new_empty()));

    // ShopRoof connections
    fn no_shotgun_no_princess_no_nuke(state: &ReventureState) -> bool {
        no_shotgun(state) && rules::no_princess_no_nuke(state)
    }
    fn no_shotgun_no_princess_nuke(state: &ReventureState) -> bool {
        no_shotgun(state) && rules::no_princess(state) && rules::nuke(state)
    }
    base_regions[SHOP_ROOF].add_connection(BaseConnection::new(SHOP, no_shotgun, APItems::new_empty()));
    base_regions[SHOP_ROOF].add_jumpconnection(JumpConnection::new(OCEAN, no_shotgun, APItems::new_empty(), 3.0));
    base_regions[SHOP_ROOF].add_connection(BaseConnection::new(OCEAN, no_shotgun_anysword, APItems::new_empty()));
    base_regions[SHOP_ROOF].add_connection(BaseConnection::new(FORTRESS_MOAT, no_shotgun_no_princess_no_nuke, APItems::new(vec![20])));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC03, no_shotgun, APItems::new_empty()));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC13, no_shotgun_mrhugs, APItems::new_empty()));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC17, no_shotgun_no_princess_no_nuke, APItems::new(vec![20])));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC25, no_shotgun_anysword, APItems::new_empty()));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC27, no_shotgun_no_princess_nuke, APItems::new(vec![20])));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC74, shotgun, APItems::new(vec![20, 41, 28])));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC74, shotgun, APItems::new(vec![20, 41, 29])));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(EVENT_KILL_JUAN, no_shotgun_anysword, APItems::new_empty()));

    // ShopLake connections
    base_regions[SHOP_LAKE].add_jumpconnection(JumpConnection::new(VOLCANO_TOP_EXIT, rules::always, APItems::new_empty(), 2.0));
    base_regions[SHOP_LAKE].add_connection(BaseConnection::new(BEHIND_SHOP_BUSH, rules::anysword, APItems::new_empty()));
    base_regions[SHOP_LAKE].add_connection(BaseConnection::new(SHOP, rules::always, APItems::new_empty()));

    // Ocean connections
    base_regions[OCEAN].add_connection(BaseConnection::new(SHOP_ROOF, rules::always, APItems::new_empty()));
    base_regions[OCEAN].add_location(BaseConnection::new(LOC95, rules::always, APItems::new_empty()));
    base_regions[OCEAN].add_location(BaseConnection::new(LOC96, rules::always, APItems::new_empty()));
    base_regions[OCEAN].add_location(BaseConnection::new(LOC97, rules::always, APItems::new_empty()));

    // NukeStorage connections
    base_regions[NUKE_STORAGE].add_connection(BaseConnection::new(SHOP, rules::always, APItems::new_empty()));

    // ShopCellar connections
    base_regions[SHOP_CELLAR].add_connection(BaseConnection::new(SHOP, rules::princess, APItems::new_empty()));
    base_regions[SHOP_CELLAR].add_connection(BaseConnection::new(PARASITE, rules::always, APItems::new_empty()));
    base_regions[SHOP_CELLAR].add_location(BaseConnection::new(LOC78, rules::always, APItems::new_empty()));

    // Parasite connections
    base_regions[PARASITE].add_location(BaseConnection::new(LOC89, rules::always, APItems::new_empty()));

    // HookArea connections
    base_regions[HOOK_AREA].add_jumpconnection(JumpConnection::new(CASTLE_MINIONS, rules::always, APItems::new_empty(), 3.0));
    base_regions[HOOK_AREA].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::hook, APItems::new_empty()));

    // AboveHook connections
    base_regions[ABOVE_HOOK].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::always, APItems::new_empty()));
    base_regions[ABOVE_HOOK].add_jumpconnection(JumpConnection::new(ABOVE_ABOVE_HOOK, rules::always, APItems::new_empty(), 3.0));
    base_regions[ABOVE_HOOK].add_jumpconnection(JumpConnection::new(ABOVE_ABOVE_HOOK, rules::anysword, APItems::new_empty(), 2.0));
    base_regions[ABOVE_HOOK].add_connection(BaseConnection::new(ABOVE_ABOVE_HOOK, rules::hook, APItems::new_empty()));
    base_regions[ABOVE_HOOK].add_connection(BaseConnection::new(BOMB, rules::always, APItems::new_empty()));

    // AboveAboveHook connections
    base_regions[ABOVE_ABOVE_HOOK].add_connection(BaseConnection::new(ABOVE_HOOK, rules::always, APItems::new_empty()));
    base_regions[ABOVE_ABOVE_HOOK].add_jumpconnection(JumpConnection::new(CASTLE_CANNON_TO_SHOP, rules::always, APItems::new_empty(), 3.0));
    base_regions[ABOVE_ABOVE_HOOK].add_connection(BaseConnection::new(CASTLE_CANNON_TO_SHOP, rules::hook, APItems::new_empty()));
    base_regions[ABOVE_ABOVE_HOOK].add_jumpconnection(JumpConnection::new(ALTAR, rules::always, APItems::new_empty(), 2.0));
    base_regions[ABOVE_ABOVE_HOOK].add_connection(BaseConnection::new(ALTAR, rules::hook, APItems::new_empty()));

    // CastleCannonToShop connections
    base_regions[CASTLE_CANNON_TO_SHOP].add_connection(BaseConnection::new(ABOVE_ABOVE_HOOK, rules::always, APItems::new_empty()));
    base_regions[CASTLE_CANNON_TO_SHOP].add_connection(BaseConnection::new(SHOP_LAKE, rules::no_princess_no_nuke, APItems::new(vec![21])));
    base_regions[CASTLE_CANNON_TO_SHOP].add_location(BaseConnection::new(LOC17, rules::no_princess_no_nuke, APItems::new(vec![21])));
    base_regions[CASTLE_CANNON_TO_SHOP].add_location(BaseConnection::new(LOC56, rules::nuke_no_princess, APItems::new(vec![21])));

    // Altar connections
    base_regions[ALTAR].add_connection(BaseConnection::new(ABOVE_ABOVE_HOOK, rules::always, APItems::new_empty()));
    base_regions[ALTAR].add_jumpconnection(JumpConnection::new(MOUNTAIN_LEFT_OUTCROP, rules::always, APItems::new_empty(), 2.0));
    base_regions[ALTAR].add_jumpconnection(JumpConnection::new(LEVERS, rules::always, APItems::new_empty(), 3.0));
    base_regions[ALTAR].add_connection(BaseConnection::new(LEVERS, rules::hook, APItems::new_empty()));
    base_regions[ALTAR].add_connection(BaseConnection::new(GREAT_WATERFALL, rules::always, APItems::new_empty()));
    base_regions[ALTAR].add_location(BaseConnection::new(LOC72, rules::princess, APItems::new_empty()));

    // Bomb connections
    base_regions[BOMB].add_jumpconnection(JumpConnection::new(ABOVE_HOOK, rules::always, APItems::new_empty(), 3.0));
    base_regions[BOMB].add_connection(BaseConnection::new(ABOVE_HOOK, rules::hook, APItems::new_empty()));
    base_regions[BOMB].add_connection(BaseConnection::new(FISHING_ROD, rules::always, APItems::new_empty()));
    base_regions[BOMB].add_connection(BaseConnection::new(SECRET_PATH_MOAT_WELL, rules::always, APItems::new_empty()));
    base_regions[BOMB].add_jumpconnection(JumpConnection::new(SECRET_ABOVE_BOMB, rules::always, APItems::new_empty(), 3.0));
    base_regions[BOMB].add_jumpconnection(JumpConnection::new(GREAT_WATERFALL, rules::bomb, APItems::new_empty(), 2.0));
    base_regions[BOMB].add_connection(BaseConnection::new(GREAT_WATERFALL, rules::bomb_hook, APItems::new_empty()));
    base_regions[BOMB].add_location(BaseConnection::new(LOC28, rules::bomb, APItems::new_empty()));
    base_regions[BOMB].add_location(BaseConnection::new(LOC32, rules::anysword, APItems::new(vec![45])));
    base_regions[BOMB].add_location(BaseConnection::new(LOC54, rules::mrhugs, APItems::new(vec![45])));

    // FishingBridge connections
    base_regions[FISHING_BRIDGE].add_connection(BaseConnection::new(CASTLE_MOAT, rules::always, APItems::new_empty()));
    base_regions[FISHING_BRIDGE].add_jumpconnection(JumpConnection::new(FISHING_ROD, rules::always, APItems::new_empty(), 2.0));
    base_regions[FISHING_BRIDGE].add_connection(BaseConnection::new(BELOW_FISHING_BRIDGE, rules::always, APItems::new_empty()));

    // BelowFishingBridge connections
    base_regions[BELOW_FISHING_BRIDGE].add_jumpconnection(JumpConnection::new(FISHING_BRIDGE, rules::always, APItems::new_empty(), 2.0));
    base_regions[BELOW_FISHING_BRIDGE].add_connection(BaseConnection::new(WATER_FALLS, rules::always, APItems::new_empty()));

    // FishingRod connections
    base_regions[FISHING_ROD].add_connection(BaseConnection::new(FISHING_BRIDGE, rules::always, APItems::new_empty()));
    base_regions[FISHING_ROD].add_jumpconnection(JumpConnection::new(BOMB, rules::always, APItems::new_empty(), 2.0));
    base_regions[FISHING_ROD].add_location(BaseConnection::new(LOC12, rules::no_princess, APItems::new(vec![11])));

    // MountainLeftOutcrop connections
    base_regions[MOUNTAIN_LEFT_OUTCROP].add_connection(BaseConnection::new(ALTAR, rules::always, APItems::new_empty()));
    base_regions[MOUNTAIN_LEFT_OUTCROP].add_jumpconnection(JumpConnection::new(MOUNTAIN_TOP, rules::always, APItems::new_empty(), 3.0));
    base_regions[MOUNTAIN_LEFT_OUTCROP].add_connection(BaseConnection::new(MOUNTAIN_TOP, rules::sword_or_hook, APItems::new_empty()));
    base_regions[MOUNTAIN_LEFT_OUTCROP].add_location(BaseConnection::new(LOC46, rules::always, APItems::new_empty()));

    // MountainTop connections
    base_regions[MOUNTAIN_TOP].add_connection(BaseConnection::new(MOUNTAIN_LEFT_OUTCROP, rules::always, APItems::new_empty()));
    base_regions[MOUNTAIN_TOP].add_connection(BaseConnection::new(MOUNTAIN_TREASURE, rules::always, APItems::new_empty()));
    base_regions[MOUNTAIN_TOP].add_connection(BaseConnection::new(CLOUD, rules::chicken, APItems::new_empty()));
    base_regions[MOUNTAIN_TOP].add_jumpconnection(JumpConnection::new(STRAWBERRY, rules::always, APItems::new_empty(), 3.0));
    base_regions[MOUNTAIN_TOP].add_location(BaseConnection::new(EVENT_KILL_MIGUEL, rules::anysword, APItems::new_empty()));

    // Strawberry connections
    base_regions[STRAWBERRY].add_location(BaseConnection::new(LOC24, rules::always, APItems::new(vec![19])));

    // MountainTreasure connections
    base_regions[MOUNTAIN_TREASURE].add_connection(BaseConnection::new(BELOW_LEAP_OF_FAITH, rules::always, APItems::new_empty()));
    base_regions[MOUNTAIN_TREASURE].add_location(BaseConnection::new(LOC33, rules::no_princess, APItems::new_empty()));
    base_regions[MOUNTAIN_TREASURE].add_location(BaseConnection::new(LOC62, rules::shovel, APItems::new_empty()));

    // Levers connections
    base_regions[LEVERS].add_jumpconnection(JumpConnection::new(ALTAR, rules::always, APItems::new_empty(), 4.0));
    base_regions[LEVERS].add_connection(BaseConnection::new(ALTAR, rules::hook, APItems::new_empty()));
    base_regions[LEVERS].add_jumpconnection(JumpConnection::new(BELOW_LEAP_OF_FAITH, rules::always, APItems::new_empty(), 4.0));
    base_regions[LEVERS].add_connection(BaseConnection::new(BELOW_LEAP_OF_FAITH, rules::hook, APItems::new_empty()));
    base_regions[LEVERS].add_jumpconnection(JumpConnection::new(DARKSTONE, rules::always, APItems::new(vec![37]), 3.0));
    base_regions[LEVERS].add_connection(BaseConnection::new(DARKSTONE, rules::hook, APItems::new(vec![37])));
    base_regions[LEVERS].add_connection(BaseConnection::new(GREAT_WATERFALL, rules::always, APItems::new(vec![37])));
    base_regions[LEVERS].add_location(BaseConnection::new(LOC38, rules::no_princess, APItems::new(vec![36])));
    base_regions[LEVERS].add_location(BaseConnection::new(LOC44, rules::no_princess, APItems::new(vec![38])));

    // Darkstone connections
    base_regions[DARKSTONE].add_connection(BaseConnection::new(LEVERS, rules::always, APItems::new_empty()));
    base_regions[DARKSTONE].add_statechange(StateChange::new(
        vec!["has_darkstone".to_string()],
        vec![true],
        rules::can_pickup_darkstone,
        APItems::new(vec![9]),
    ));
    base_regions[DARKSTONE].add_statechange(StateChange::new(
        vec!["has_burger".to_string()],
        vec![true],
        rules::can_pickup_burger,
        APItems::new(vec![8]),
    ));

    // GreatWaterfall connections
    base_regions[GREAT_WATERFALL].add_jumpconnection(JumpConnection::new(ALTAR, rules::always, APItems::new_empty(), 2.0));
    base_regions[GREAT_WATERFALL].add_connection(BaseConnection::new(ALTAR, rules::hook, APItems::new_empty()));
    base_regions[GREAT_WATERFALL].add_connection(BaseConnection::new(BELOW_FISHING_BRIDGE, rules::always, APItems::new_empty()));
    base_regions[GREAT_WATERFALL].add_connection(BaseConnection::new(BOMB, rules::bomb, APItems::new_empty()));
    base_regions[GREAT_WATERFALL].add_connection(BaseConnection::new(GREAT_WATERFALL_BOTTOM, rules::always, APItems::new_empty()));
    base_regions[GREAT_WATERFALL].add_connection(BaseConnection::new(WHISTLE, rules::always, APItems::new_empty()));
    base_regions[GREAT_WATERFALL].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::always, APItems::new_empty()));

    // GreatWaterfallBottom connections
    base_regions[GREAT_WATERFALL_BOTTOM].add_connection(BaseConnection::new(WATER_FALLS, rules::always, APItems::new_empty()));
    base_regions[GREAT_WATERFALL_BOTTOM].add_jumpconnection(JumpConnection::new(ABOVE_WATERFALLS, rules::always, APItems::new_empty(), 2.0));
    base_regions[GREAT_WATERFALL_BOTTOM].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::always, APItems::new_empty()));

    // SecretAboveBomb connections
    base_regions[SECRET_ABOVE_BOMB].add_connection(BaseConnection::new(BOMB, rules::always, APItems::new_empty()));
    base_regions[SECRET_ABOVE_BOMB].add_connection(BaseConnection::new(GREAT_WATERFALL, rules::always, APItems::new_empty()));

    // WaterFalls connections
    base_regions[WATER_FALLS].add_jumpconnection(JumpConnection::new(BELOW_FISHING_BRIDGE, rules::always, APItems::new_empty(), 2.0));
    base_regions[WATER_FALLS].add_connection(BaseConnection::new(MOUNTAIN_TOP, rules::chicken_or_shovel_no_princess, APItems::new(vec![27])));
    base_regions[WATER_FALLS].add_jumpconnection(JumpConnection::new(ABOVE_WATERFALLS, rules::always, APItems::new_empty(), 2.0));
    base_regions[WATER_FALLS].add_location(BaseConnection::new(LOC08, rules::no_princess, APItems::new_empty()));
    base_regions[WATER_FALLS].add_location(BaseConnection::new(LOC82, rules::princess, APItems::new_empty()));
    base_regions[WATER_FALLS].add_location(BaseConnection::new(LOC87, rules::always, APItems::new(vec![59,60,61,62,63])));

    // AboveWaterfalls connections
    base_regions[ABOVE_WATERFALLS].add_connection(BaseConnection::new(WATER_FALLS, rules::always, APItems::new_empty()));
    base_regions[ABOVE_WATERFALLS].add_connection(BaseConnection::new(BELOW_FISHING_BRIDGE, rules::always, APItems::new_empty()));
    base_regions[ABOVE_WATERFALLS].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::always, APItems::new_empty()));

    // FortressMoat connections
    base_regions[FORTRESS_MOAT].add_connection(BaseConnection::new(ALBERTO, rules::hook, APItems::new_empty()));
    base_regions[FORTRESS_MOAT].add_connection(BaseConnection::new(WATER_FALLS, rules::always, APItems::new_empty()));
    base_regions[FORTRESS_MOAT].add_jumpconnection(JumpConnection::new(ABOVE_WATERFALLS, rules::always, APItems::new_empty(), 2.0));
    base_regions[FORTRESS_MOAT].add_connection(BaseConnection::new(FAIRY_FOUNTAIN, rules::always, APItems::new_empty()));
    base_regions[FORTRESS_MOAT].add_jumpconnection(JumpConnection::new(FORTRESS_BRIDGE_BUTTON, rules::always, APItems::new_empty(), 3.0));
    base_regions[FORTRESS_MOAT].add_connection(BaseConnection::new(FORTRESS_BRIDGE_BUTTON, rules::hook, APItems::new_empty()));
    base_regions[FORTRESS_MOAT].add_jumpconnection(JumpConnection::new(RIGHT_OF_FORTRESS, rules::always, APItems::new_empty(), 3.0));
    base_regions[FORTRESS_MOAT].add_connection(BaseConnection::new(RIGHT_OF_FORTRESS, rules::hook_or_shovel_or_bomb, APItems::new_empty()));
    base_regions[FORTRESS_MOAT].add_location(BaseConnection::new(LOC15, rules::always, APItems::new_empty()));
    base_regions[FORTRESS_MOAT].add_location(BaseConnection::new(LOC21, rules::always, APItems::new_empty()));
    base_regions[FORTRESS_MOAT].add_location(BaseConnection::new(LOC48, rules::always, APItems::new_empty()));
    base_regions[FORTRESS_MOAT].add_location(BaseConnection::new(LOC49, rules::anysword, APItems::new_empty()));
    base_regions[FORTRESS_MOAT].add_location(BaseConnection::new(LOC61, rules::always, APItems::new_empty()));

    // FortressBridgeButton connections
    base_regions[FORTRESS_BRIDGE_BUTTON].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::always, APItems::new_empty()));
    base_regions[FORTRESS_BRIDGE_BUTTON].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::hook, APItems::new_empty()));
    base_regions[FORTRESS_BRIDGE_BUTTON].add_jumpconnection(JumpConnection::new(WHISTLE_ALTAR, rules::fortress_bridge_down, APItems::new_empty(), 2.0));
    base_regions[FORTRESS_BRIDGE_BUTTON].add_statechange(StateChange::new(
        vec!["fortressBridgeDown".to_string()],
        vec![true],
        rules::fortress_bridge_up,
        APItems::new_empty(),
    ));

    // FairyFountain connections
    base_regions[FAIRY_FOUNTAIN].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::always, APItems::new_empty()));
    base_regions[FAIRY_FOUNTAIN].add_connection(BaseConnection::new(LONKS_FRONTGARDEN, rules::always, APItems::new(vec![31])));
    base_regions[FAIRY_FOUNTAIN].add_location(BaseConnection::new(LOC65, rules::always, APItems::new_empty()));
    base_regions[FAIRY_FOUNTAIN].add_location(BaseConnection::new(LOC85, rules::sword_or_mrhugs, APItems::new_empty()));

    // Whistle connections
    base_regions[WHISTLE].add_jumpconnection(JumpConnection::new(GREAT_WATERFALL, rules::always, APItems::new_empty(), 2.0));
    base_regions[WHISTLE].add_connection(BaseConnection::new(GREAT_WATERFALL, rules::hook, APItems::new_empty()));
    base_regions[WHISTLE].add_connection(BaseConnection::new(GREAT_WATERFALL_BOTTOM, rules::always, APItems::new_empty()));
    base_regions[WHISTLE].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::always, APItems::new_empty()));

    // WhistleAltar connections
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(ALBERTO, rules::always, APItems::new_empty(), 2.0));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(GREAT_WATERFALL, rules::always, APItems::new_empty(), 2.0));
    base_regions[WHISTLE_ALTAR].add_connection(BaseConnection::new(GREAT_WATERFALL_BOTTOM, rules::always, APItems::new_empty()));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(BELOW_LEAP_OF_FAITH, rules::always, APItems::new_empty(), 3.0));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(ELEVATOR, rules::no_princess, APItems::new_empty(), 3.0));
    base_regions[WHISTLE_ALTAR].add_connection(BaseConnection::new(ELEVATOR, rules::no_princess_hook_or_fortress_bridge, APItems::new_empty()));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(FORTRESS_ROOF, rules::fortress_bridge_up, APItems::new_empty(), 3.0));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(FORTRESS_ROOF, rules::fortress_bridge_up_hook, APItems::new_empty(), 2.0));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(WHISTLE, rules::always, APItems::new_empty(), 3.0));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC39, rules::no_princess, APItems::new_empty()));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC69, rules::anysword_princess, APItems::new_empty()));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC73, rules::mrhugs_princess, APItems::new_empty()));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC75, rules::princess, APItems::new_empty()));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC83, rules::whistle, APItems::new_empty()));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC90, rules::anysword_princess, APItems::new_empty()));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC93, rules::darkstone_princess, APItems::new_empty()));

    // BelowLeapOfFaith connections
    base_regions[BELOW_LEAP_OF_FAITH].add_connection(BaseConnection::new(LEVERS, rules::always, APItems::new_empty()));
    base_regions[BELOW_LEAP_OF_FAITH].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::always, APItems::new_empty()));

    // Elevator connections
    base_regions[ELEVATOR].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::fortress_bridge_down_no_princess, APItems::new_empty()));
    base_regions[ELEVATOR].add_connection(BaseConnection::new(ANVIL, rules::always, APItems::new(vec![28])));
    base_regions[ELEVATOR].add_connection(BaseConnection::new(ANVIL, rules::always, APItems::new(vec![29])));
    base_regions[ELEVATOR].add_jumpconnection(JumpConnection::new(RIGHT_OF_FORTRESS, rules::always, APItems::new_empty(), 4.0));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC34, rules::always, APItems::new(vec![28])));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC34, rules::always, APItems::new(vec![29])));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC34, rules::princess, APItems::new_empty()));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC50, rules::princess, APItems::new_empty()));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC66, rules::darkstone, APItems::new_empty()));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC76, rules::princess, APItems::new_empty()));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC80, rules::chicken, APItems::new_empty()));

    // FortressRoof connections
    base_regions[FORTRESS_ROOF].add_jumpconnection(JumpConnection::new(WHISTLE_ALTAR, rules::always, APItems::new_empty(), 4.0));
    base_regions[FORTRESS_ROOF].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::fortress_bridge_down, APItems::new_empty()));
    base_regions[FORTRESS_ROOF].add_connection(BaseConnection::new(ANVIL, rules::always, APItems::new_empty()));
    base_regions[FORTRESS_ROOF].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::no_princess_no_nuke, APItems::new(vec![22])));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC17, rules::no_princess_no_nuke, APItems::new(vec![22])));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC42, rules::no_princess, APItems::new(vec![17])));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC52, rules::princess, APItems::new(vec![22])));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC55, rules::no_chicken_princess, APItems::new_empty()));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC58, rules::no_chicken_no_princess, APItems::new_empty()));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC84, rules::nuke_no_princess, APItems::new(vec![22])));

    // Anvil connections
    base_regions[ANVIL].add_jumpconnection(JumpConnection::new(FORTRESS_ROOF, rules::always, APItems::new_empty(), 4.0));
    base_regions[ANVIL].add_connection(BaseConnection::new(FORTRESS_ROOF, rules::hook, APItems::new_empty()));
    base_regions[ANVIL].add_connection(BaseConnection::new(ELEVATOR, rules::always, APItems::new(vec![28])));
    base_regions[ANVIL].add_connection(BaseConnection::new(ELEVATOR, rules::always, APItems::new(vec![29])));
    base_regions[ANVIL].add_jumpconnection(JumpConnection::new(PRINCESS, rules::always, APItems::new_empty(), 2.0));
    base_regions[ANVIL].add_connection(BaseConnection::new(PRINCESS, rules::hook, APItems::new_empty()));
    base_regions[ANVIL].add_connection(BaseConnection::new(FIRE_ESCAPE, rules::princess, APItems::new_empty()));
    base_regions[ANVIL].add_connection(BaseConnection::new(FORTRESS_TREASURE, rules::princess, APItems::new_empty()));
    base_regions[ANVIL].add_location(BaseConnection::new(LOC22, rules::always, APItems::new(vec![18])));
    base_regions[ANVIL].add_location(BaseConnection::new(LOC23, rules::always, APItems::new(vec![41])));
    base_regions[ANVIL].add_location(BaseConnection::new(LOC53, rules::princess, APItems::new_empty()));
    base_regions[ANVIL].add_location(BaseConnection::new(LOC98, rules::no_princess_burger, APItems::new(vec![41])));

    // Princess connections
    base_regions[PRINCESS].add_connection(BaseConnection::new(ANVIL, rules::always, APItems::new_empty()));
    base_regions[PRINCESS].add_jumpconnection(JumpConnection::new(SPIKE_TRAP, rules::no_princess, APItems::new_empty(), 2.0));
    base_regions[PRINCESS].add_connection(BaseConnection::new(SPIKE_TRAP, rules::no_princess_and_hook, APItems::new_empty()));
    base_regions[PRINCESS].add_statechange(StateChange::new(
        vec!["has_princess".to_string(), "fortressBridgeDown".to_string()],
        vec![true, true],
        rules::can_pickup_princess,
        APItems::new(vec![17]),
    ));
    base_regions[PRINCESS].add_location(BaseConnection::new(LOC45, rules::princess, APItems::new_empty()));
    base_regions[PRINCESS].add_location(BaseConnection::new(LOC57, rules::mrhugs_princess, APItems::new_empty()));
    base_regions[PRINCESS].add_location(BaseConnection::new(LOC64, rules::no_princess_sword, APItems::new_empty()));

    // SpikeTrap connections
    base_regions[SPIKE_TRAP].add_location(BaseConnection::new(LOC70, rules::always, APItems::new_empty()));

    // FireEscape connections
    base_regions[FIRE_ESCAPE].add_connection(BaseConnection::new(ELEVATOR, rules::always, APItems::new_empty()));
    base_regions[FIRE_ESCAPE].add_jumpconnection(JumpConnection::new(FORTRESS_ROOF, rules::always, APItems::new_empty(), 2.0));
    base_regions[FIRE_ESCAPE].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::fortress_bridge_down, APItems::new_empty()));

    // FortressTreasure connections
    base_regions[FORTRESS_TREASURE].add_connection(BaseConnection::new(RIGHT_OF_FORTRESS, rules::always, APItems::new_empty()));
    base_regions[FORTRESS_TREASURE].add_location(BaseConnection::new(LOC68, rules::always, APItems::new_empty()));
    base_regions[FORTRESS_TREASURE].add_location(BaseConnection::new(EVENT_KILL_JAVI, rules::anysword, APItems::new_empty()));

    // RightOfFortress connections
    base_regions[RIGHT_OF_FORTRESS].add_jumpconnection(JumpConnection::new(FORTRESS_TREASURE, rules::always, APItems::new_empty(), 3.0));
    base_regions[RIGHT_OF_FORTRESS].add_connection(BaseConnection::new(ELEVATOR, rules::always, APItems::new_empty()));
    base_regions[RIGHT_OF_FORTRESS].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::hook, APItems::new_empty()));
    // fn can_enter_desert(state: &ReventureState) -> bool {
    //     state.get_weight() >= 2.0
    // }
    // base_regions[RIGHT_OF_FORTRESS].add_connection(BaseConnection::new(DESERT, can_enter_desert, APItems::new_empty()));
    base_regions[RIGHT_OF_FORTRESS].add_location(BaseConnection::new(LOC81, rules::princess, APItems::new_empty()));

    // Desert connections
    base_regions[DESERT].add_location(BaseConnection::new(LOC91, rules::always, APItems::new_empty()));

    // Alberto connections
    base_regions[ALBERTO].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::always, APItems::new_empty()));
    base_regions[ALBERTO].add_connection(BaseConnection::new(GREAT_WATERFALL_BOTTOM, rules::always, APItems::new_empty()));
    base_regions[ALBERTO].add_location(BaseConnection::new(EVENT_KILL_ALBERTO, rules::anysword, APItems::new_empty()));

    println!("Region connections setup complete!");
}
