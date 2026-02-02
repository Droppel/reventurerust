use std::vec;

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

    pub fn sword_fortress_bridge_up(state: &ReventureState) -> bool {
        anysword(state) && fortress_bridge_up(state)
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

    // Item 3: Bomb
    base_regions[item_locations[3]].add_statechange(StateChange::new(
        vec!["has_bomb".to_string()],
        vec![true],
        rules::can_pickup_bomb,
        vec!["Bomb".to_string()],
    ));

    // Item 4: Shield
    base_regions[item_locations[4]].add_statechange(StateChange::new(
        vec!["has_shield".to_string()],
        vec![true],
        rules::can_pickup_shield,
        vec!["Shield".to_string()],
    ));

    // Item 5: Mister Hugs
    base_regions[item_locations[5]].add_statechange(StateChange::new(
        vec!["has_mrhugs".to_string()],
        vec![true],
        rules::can_pickup_mrhugs,
        vec!["Mister Hugs".to_string()],
    ));

    // Item 6: Lava Trinket
    base_regions[item_locations[6]].add_statechange(StateChange::new(
        vec!["has_lavaTrinket".to_string()],
        vec![true],
        rules::can_pickup_lavatrinket,
        vec!["Lava Trinket".to_string()],
    ));

    // Item 7: Hook
    base_regions[item_locations[7]].add_statechange(StateChange::new(
        vec!["has_hook".to_string()],
        vec![true],
        rules::can_pickup_hook,
        vec!["Hook".to_string()],
    ));

    // Item 8: Nuke
    base_regions[item_locations[8]].add_statechange(StateChange::new(
        vec!["has_nuke".to_string()],
        vec![true],
        rules::can_pickup_nuke,
        vec!["Nuke".to_string()],
    ));

    // Item 9: Whistle
    base_regions[item_locations[9]].add_statechange(StateChange::new(
        vec!["has_whistle".to_string()],
        vec![true],
        rules::can_pickup_whistle,
        vec!["Whistle".to_string()],
    ));
}

/// Set up all region connections - this is the main function that creates the game graph
pub fn setup_region_connections(base_regions: &mut [BaseRegion], start_region: usize) {
    // Menu connections
    base_regions[MENU].add_connection(BaseConnection::new(start_region, rules::always, vec![]));
    base_regions[MENU].add_location(BaseConnection::new(LOC59, rules::always, vec![]));

    // LonksHouse connections
    base_regions[LONKS_HOUSE].add_connection(BaseConnection::new(LONKS_FRONTGARDEN, rules::no_princess, vec![]));
    base_regions[LONKS_HOUSE].add_connection(BaseConnection::new(LONKS_BACKGARDEN, rules::no_princess, vec![]));
    base_regions[LONKS_HOUSE].add_jumpconnection(JumpConnection::new(SWORD_CHEST, rules::no_princess, vec![], 2.0));
    base_regions[LONKS_HOUSE].add_location(BaseConnection::new(LOC20, rules::no_princess, vec![]));
    base_regions[LONKS_HOUSE].add_location(BaseConnection::new(LOC94, rules::princess, vec![]));

    // LonksBackGarden connections
    base_regions[LONKS_BACKGARDEN].add_jumpconnection(JumpConnection::new(ELDER, rules::always, vec![], 2.0));
    base_regions[LONKS_BACKGARDEN].add_connection(BaseConnection::new(LONKS_HOUSE, rules::always, vec![]));
    base_regions[LONKS_BACKGARDEN].add_connection(BaseConnection::new(VOLCANO_BRIDGE, rules::shovel, vec![]));
    base_regions[LONKS_BACKGARDEN].add_location(BaseConnection::new(LOC03, rules::always, vec![]));

    // LonksFrontGarden connections
    base_regions[LONKS_FRONTGARDEN].add_connection(BaseConnection::new(LONKS_HOUSE, rules::always, vec![]));
    base_regions[LONKS_FRONTGARDEN].add_connection(BaseConnection::new(CASTLE_FIRST_FLOOR, rules::always, vec![]));
    base_regions[LONKS_FRONTGARDEN].add_connection(BaseConnection::new(FAIRY_FOUNTAIN, rules::always, vec!["Fairy Portal".to_string()]));
    base_regions[LONKS_FRONTGARDEN].add_location(BaseConnection::new(LOC02, rules::always, vec!["Faceplant Stone".to_string()]));
    base_regions[LONKS_FRONTGARDEN].add_location(BaseConnection::new(LOC04, rules::anysword, vec![]));
    base_regions[LONKS_FRONTGARDEN].add_location(BaseConnection::new(LOC19, rules::mrhugs, vec![]));

    // SwordChest connections
    base_regions[SWORD_CHEST].add_connection(BaseConnection::new(LONKS_HOUSE, rules::always, vec![]));

    // Elder connections
    base_regions[ELDER].add_jumpconnection(JumpConnection::new(CHICKEN, rules::always, vec![], 2.0));
    base_regions[ELDER].add_connection(BaseConnection::new(SHOVEL, rules::always, vec![]));
    base_regions[ELDER].add_jumpconnection(JumpConnection::new(LONKS_BACKGARDEN, rules::always, vec![], 2.0));
    base_regions[ELDER].add_jumpconnection(JumpConnection::new(VOLCANO_TOP_EXIT, rules::always, vec![], 2.0));
    base_regions[ELDER].add_location(BaseConnection::new(LOC01, rules::anysword, vec!["Elder".to_string()]));
    base_regions[ELDER].add_location(BaseConnection::new(LOC40, rules::mrhugs, vec!["Elder".to_string()]));

    // Chicken connections
    base_regions[CHICKEN].add_connection(BaseConnection::new(ELDER, rules::always, vec![]));
    base_regions[CHICKEN].add_connection(BaseConnection::new(LONKS_BACKGARDEN, rules::always, vec![]));
    base_regions[CHICKEN].add_statechange(StateChange::new(
        vec!["has_chicken".to_string()],
        vec![true],
        rules::can_pickup_chicken,
        vec!["Chicken".to_string()],
    ));
    fn rule_loc63(state: &ReventureState) -> bool {
        !rules::chicken(state) && rules::anysword(state)
    }
    base_regions[CHICKEN].add_location(BaseConnection::new(LOC63, rule_loc63, vec!["Chicken".to_string()]));
    fn rule_loc79(state: &ReventureState) -> bool {
        !rules::chicken(state) && rules::mrhugs(state)
    }
    base_regions[CHICKEN].add_location(BaseConnection::new(LOC79, rule_loc79, vec!["Chicken".to_string()]));

    // Shovel connections
    base_regions[SHOVEL].add_jumpconnection(JumpConnection::new(ELDER, rules::always, vec![], 3.0));
    base_regions[SHOVEL].add_connection(BaseConnection::new(LONKS_BACKGARDEN, rules::shovel, vec![]));

    // CastleFirstFloor connections
    base_regions[CASTLE_FIRST_FLOOR].add_connection(BaseConnection::new(LONKS_FRONTGARDEN, rules::no_burger_no_princess, vec![]));
    base_regions[CASTLE_FIRST_FLOOR].add_jumpconnection(JumpConnection::new(CASTLE_SHIELD_CHEST, rules::no_burger_no_princess, vec![], 2.0));
    base_regions[CASTLE_FIRST_FLOOR].add_jumpconnection(JumpConnection::new(CASTLE_MAP_CHEST, rules::no_burger_no_princess, vec![], 3.0));
    base_regions[CASTLE_FIRST_FLOOR].add_connection(BaseConnection::new(SEWER, rules::no_burger_no_princess, vec!["Open Castle Floor".to_string()]));
    base_regions[CASTLE_FIRST_FLOOR].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::no_burger_no_princess_castle_bridge, vec![]));
    fn can_lower_castle_bridge(state: &ReventureState) -> bool {
        rules::no_burger_no_princess(state) && 
        !rules::castle_bridge_down(state) && 
        (rules::anysword(state) || rules::shovel(state))
    }
    base_regions[CASTLE_FIRST_FLOOR].add_statechange(StateChange::new(
        vec!["castleBridgeDown".to_string()],
        vec![true],
        can_lower_castle_bridge,
        vec![],
    ));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC04, rules::no_burger_no_princess_sword, vec![]));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC05, rules::no_burger_no_princess_sword, vec!["King".to_string()]));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC18, rules::no_burger_no_princess_mrhugs, vec!["King".to_string()]));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC19, rules::no_burger_no_princess_mrhugs, vec![]));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC51, rules::no_burger_has_princess, vec![]));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC60, rules::no_burger_no_princess_bomb, vec![]));
    base_regions[CASTLE_FIRST_FLOOR].add_location(BaseConnection::new(LOC99, rules::no_princess_has_burger, vec!["King".to_string()]));

    // CastleShieldChest connections
    base_regions[CASTLE_SHIELD_CHEST].add_connection(BaseConnection::new(CASTLE_FIRST_FLOOR, rules::always, vec![]));

    // CastleMapChest connections
    base_regions[CASTLE_MAP_CHEST].add_connection(BaseConnection::new(CASTLE_FIRST_FLOOR, rules::always, vec![]));
    base_regions[CASTLE_MAP_CHEST].add_jumpconnection(JumpConnection::new(CASTLE_ROOF, rules::always, vec![], 3.0));

    // CastleRoof connections
    base_regions[CASTLE_ROOF].add_connection(BaseConnection::new(CASTLE_MAP_CHEST, rules::always, vec![]));
    base_regions[CASTLE_ROOF].add_connection(BaseConnection::new(PRINCESS_ROOM, rules::always, vec![]));
    base_regions[CASTLE_ROOF].add_jumpconnection(JumpConnection::new(CHIMNEY, rules::always, vec![], 3.0));
    // base_regions[CASTLE_ROOF].add_location(BaseConnection::new(LOC17, rules::always, vec!["Castle To Dark Fortress Cannon".to_string()]));

    // Chimney connections
    base_regions[CHIMNEY].add_location(BaseConnection::new(LOC30, rules::always, vec![]));

    // PrincessRoom connections
    base_regions[PRINCESS_ROOM].add_jumpconnection(JumpConnection::new(CASTLE_ROOF, rules::always, vec![], 3.0));
    base_regions[PRINCESS_ROOM].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::always, vec![]));
    base_regions[PRINCESS_ROOM].add_connection(BaseConnection::new(ANVIL, rules::always, vec!["Mirror Portal".to_string()]));
    base_regions[PRINCESS_ROOM].add_location(BaseConnection::new(LOC04, rules::anysword, vec![]));
    base_regions[PRINCESS_ROOM].add_location(BaseConnection::new(LOC11, rules::mrhugs, vec![]));
    base_regions[PRINCESS_ROOM].add_location(BaseConnection::new(LOC19, rules::mrhugs, vec![]));

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
    base_regions[SEWER].add_connection(BaseConnection::new(MUSIC_CLUB, rules::shovel, vec![]));

    // MusicClub connections
    base_regions[MUSIC_CLUB].add_connection(BaseConnection::new(BELOW_VOLCANO_BRIDGE, rules::always, vec![]));
    base_regions[MUSIC_CLUB].add_connection(BaseConnection::new(SEWER_PIPE, rules::shovel, vec![]));
    base_regions[MUSIC_CLUB].add_location(BaseConnection::new(EVENT_KILL_DANIEL, rules::anysword, vec![]));

    // BelowVolcanoBridge connections
    base_regions[BELOW_VOLCANO_BRIDGE].add_connection(BaseConnection::new(LEFT_OF_DRAGON, rules::shovel, vec![]));
    base_regions[BELOW_VOLCANO_BRIDGE].add_jumpconnection(JumpConnection::new(GOLD_ROOM, rules::always, vec![], 2.0));
    base_regions[BELOW_VOLCANO_BRIDGE].add_connection(BaseConnection::new(PARASITE, rules::shovel, vec![]));
    base_regions[BELOW_VOLCANO_BRIDGE].add_location(BaseConnection::new(LOC06, rules::no_princess, vec![]));

    // GoldRoom connections
    base_regions[GOLD_ROOM].add_connection(BaseConnection::new(RIGHT_OF_DRAGON, rules::always, vec![]));
    base_regions[GOLD_ROOM].add_jumpconnection(JumpConnection::new(SEWER_PIPE, rules::always, vec![], 2.0));

    // LeftOfDragon connections
    base_regions[LEFT_OF_DRAGON].add_connection(BaseConnection::new(VOLCANO_GEYSER, rules::shovel, vec![]));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC10, rules::shovel, vec![]));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC14, rules::no_princess_no_shield_no_lavatrinket, vec!["Dragon".to_string()]));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC29, rules::no_princess_shield_no_lavatrinket, vec!["Dragon".to_string()]));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC36, rules::no_princess_no_shield_lavatrinket, vec!["Dragon".to_string()]));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC41, rules::no_princess_shield_lavatrinket, vec!["Dragon".to_string()]));
    base_regions[LEFT_OF_DRAGON].add_location(BaseConnection::new(LOC92, rules::princess, vec!["Dragon".to_string()]));

    // RightOfDragon connections
    base_regions[RIGHT_OF_DRAGON].add_connection(BaseConnection::new(VOLCANO_GEYSER, rules::always, vec![]));
    base_regions[RIGHT_OF_DRAGON].add_jumpconnection(JumpConnection::new(GOLD_ROOM, rules::always, vec![], 4.0));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC14, rules::always, vec!["Dragon".to_string()]));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC16, rules::anysword, vec!["Dragon".to_string()]));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC29, rules::shield_no_lavatrinket, vec!["Dragon".to_string()]));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC36, rules::no_shield_has_lava, vec!["Dragon".to_string()]));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC41, rules::has_shield_and_lava, vec!["Dragon".to_string()]));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC43, rules::mrhugs, vec!["Dragon".to_string()]));
    base_regions[RIGHT_OF_DRAGON].add_location(BaseConnection::new(LOC92, rules::princess, vec!["Dragon".to_string()]));

    // SewerPipe connections
    base_regions[SEWER_PIPE].add_connection(BaseConnection::new(GOLD_ROOM, rules::always, vec![]));
    base_regions[SEWER_PIPE].add_location(BaseConnection::new(LOC35, rules::always, vec!["Sewer Pipe".to_string()]));

    // VolcanoGeyser connections
    base_regions[VOLCANO_GEYSER].add_connection(BaseConnection::new(LEFT_OF_DRAGON, rules::lavatrinket, vec![]));
    base_regions[VOLCANO_GEYSER].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::always, vec!["Volcano Geyser".to_string()]));
    base_regions[VOLCANO_GEYSER].add_jumpconnection(JumpConnection::new(ULTIMATE_DOOR, rules::always, vec![], 2.0));
    base_regions[VOLCANO_GEYSER].add_location(BaseConnection::new(LOC06, rules::no_princess, vec![]));

    // UltimateDoor connections
    base_regions[ULTIMATE_DOOR].add_connection(BaseConnection::new(VOLCANO_GEYSER, rules::always, vec![]));
    base_regions[ULTIMATE_DOOR].add_location(BaseConnection::new(LOC67, rules::always, vec![]));
    base_regions[ULTIMATE_DOOR].add_location(BaseConnection::new(LOC100, rules::always, vec![]));

    // CastleMinions connections
    base_regions[CASTLE_MINIONS].add_connection(BaseConnection::new(CASTLE_FIRST_FLOOR, rules::castle_bridge_down, vec![]));
    base_regions[CASTLE_MINIONS].add_connection(BaseConnection::new(SECRET_PATH_MOAT_WELL, rules::castle_bridge_up, vec![]));
    base_regions[CASTLE_MINIONS].add_connection(BaseConnection::new(HOOK_AREA, rules::always, vec![]));
    base_regions[CASTLE_MINIONS].add_jumpconnection(JumpConnection::new(ABOVE_HOOK, rules::always, vec![], 2.0));
    base_regions[CASTLE_MINIONS].add_connection(BaseConnection::new(ABOVE_HOOK, rules::hook, vec![]));
    base_regions[CASTLE_MINIONS].add_connection(BaseConnection::new(CLOUD, rules::always, vec!["Vine".to_string()]));
    base_regions[CASTLE_MINIONS].add_location(BaseConnection::new(LOC03, rules::always, vec![]));
    base_regions[CASTLE_MINIONS].add_location(BaseConnection::new(LOC13, rules::mrhugs, vec![]));
    base_regions[CASTLE_MINIONS].add_location(BaseConnection::new(LOC25, rules::anysword, vec![]));
    base_regions[CASTLE_MINIONS].add_location(BaseConnection::new(LOC95, rules::always, vec![]));

    // Cloud connections
    base_regions[CLOUD].add_connection(BaseConnection::new(CASTLE_ROOF, rules::always, vec![]));
    // Could also drop to CastleMinions, but that would be redundant
    base_regions[CLOUD].add_connection(BaseConnection::new(CASTLE_CANNON_TO_SHOP, rules::always, vec![]));
    base_regions[CLOUD].add_location(BaseConnection::new(LOC77, rules::always, vec![]));

    // BelowCastleBridge connections
    base_regions[BELOW_CASTLE_BRIDGE].add_jumpconnection(JumpConnection::new(SEWER, rules::always, vec![], 2.5));
    base_regions[BELOW_CASTLE_BRIDGE].add_jumpconnection(JumpConnection::new(SECRET_PATH_MOAT_WELL, rules::always, vec![], 3.0));
    base_regions[BELOW_CASTLE_BRIDGE].add_connection(BaseConnection::new(CASTLE_MOAT, rules::always, vec![]));

    // SecretPathMoatWell connections
    base_regions[SECRET_PATH_MOAT_WELL].add_connection(BaseConnection::new(BELOW_CASTLE_BRIDGE, rules::always, vec![]));
    base_regions[SECRET_PATH_MOAT_WELL].add_jumpconnection(JumpConnection::new(CASTLE_MINIONS, rules::always, vec![], 3.0));
    base_regions[SECRET_PATH_MOAT_WELL].add_jumpconnection(JumpConnection::new(BOMB, rules::always, vec![], 2.0));
    base_regions[SECRET_PATH_MOAT_WELL].add_connection(BaseConnection::new(FISHING_ROD, rules::always, vec![]));

    // CastleMoat connections
    base_regions[CASTLE_MOAT].add_jumpconnection(JumpConnection::new(BELOW_CASTLE_BRIDGE, rules::always, vec![], 2.0));
    base_regions[CASTLE_MOAT].add_connection(BaseConnection::new(ULTIMATE_DOOR, rules::shovel, vec![]));
    base_regions[CASTLE_MOAT].add_connection(BaseConnection::new(BARN, rules::anysword, vec![]));
    base_regions[CASTLE_MOAT].add_jumpconnection(JumpConnection::new(FISHING_BRIDGE, rules::always, vec![], 2.0));
    base_regions[CASTLE_MOAT].add_connection(BaseConnection::new(FISHING_BRIDGE, rules::anysword, vec![]));
    base_regions[CASTLE_MOAT].add_location(BaseConnection::new(LOC95, rules::always, vec![]));
    base_regions[CASTLE_MOAT].add_location(BaseConnection::new(LOC07, rules::no_princess, vec![]));

    // Barn connections
    base_regions[BARN].add_jumpconnection(JumpConnection::new(BARN_SECOND_FLOOR, rules::always, vec![], 2.0));
    base_regions[BARN].add_location(BaseConnection::new(LOC86, rules::princess, vec![]));

    // BarnSecondFloor connections
    base_regions[BARN_SECOND_FLOOR].add_location(BaseConnection::new(LOC31, rules::anysword, vec![]));

    // BehindShopBush connections
    base_regions[BEHIND_SHOP_BUSH].add_connection(BaseConnection::new(VOLCANO_DROP_STONE, rules::always, vec![]));
    base_regions[BEHIND_SHOP_BUSH].add_connection(BaseConnection::new(SHOP_LAKE, rules::anysword, vec![]));

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
    base_regions[SHOP].add_connection(BaseConnection::new(SHOP_LAKE, no_shotgun, vec![]));
    base_regions[SHOP].add_jumpconnection(JumpConnection::new(SHOP_ROOF, rules::always, vec![], 2.0));
    base_regions[SHOP].add_jumpconnection(JumpConnection::new(NUKE_STORAGE, rules::always, vec![], 4.0));
    base_regions[SHOP].add_connection(BaseConnection::new(NUKE_STORAGE, rules::hook, vec![]));
    base_regions[SHOP].add_connection(BaseConnection::new(SHOP_CELLAR, princess_no_shotgun, vec![]));
    base_regions[SHOP].add_statechange(StateChange::new(
        vec!["has_shotgun".to_string()],
        vec![true],
        no_shotgun_anysword,
        vec!["Shopkeeper".to_string()],
    ));
    base_regions[SHOP].add_location(BaseConnection::new(LOC09, no_shotgun_anysword, vec!["Shopkeeper".to_string()]));
    base_regions[SHOP].add_location(BaseConnection::new(LOC37, no_shotgun_mrhugs, vec!["Shopkeeper".to_string()]));
    base_regions[SHOP].add_location(BaseConnection::new(LOC95, no_shotgun, vec![]));

    // ShopRoof connections
    fn no_shotgun_no_princess_no_nuke(state: &ReventureState) -> bool {
        no_shotgun(state) && rules::no_princess_no_nuke(state)
    }
    fn no_shotgun_no_princess_nuke(state: &ReventureState) -> bool {
        no_shotgun(state) && rules::no_princess(state) && rules::nuke(state)
    }
    base_regions[SHOP_ROOF].add_connection(BaseConnection::new(SHOP, no_shotgun, vec![]));
    base_regions[SHOP_ROOF].add_jumpconnection(JumpConnection::new(OCEAN, no_shotgun, vec![], 3.0));
    base_regions[SHOP_ROOF].add_connection(BaseConnection::new(OCEAN, no_shotgun_anysword, vec![]));
    base_regions[SHOP_ROOF].add_connection(BaseConnection::new(FORTRESS_MOAT, no_shotgun_no_princess_no_nuke, vec!["Shop Cannon".to_string()]));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC03, no_shotgun, vec![]));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC13, no_shotgun_mrhugs, vec![]));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC17, no_shotgun_no_princess_no_nuke, vec!["Shop Cannon".to_string()]));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC25, no_shotgun_anysword, vec![]));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC27, no_shotgun_no_princess_nuke, vec!["Shop Cannon".to_string()]));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC74, shotgun, vec!["Shop Cannon".to_string(), "Mimic".to_string(), "Elevator Button".to_string()]));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(LOC74, shotgun, vec!["Shop Cannon".to_string(), "Mimic".to_string(), "Call Elevator Buttons".to_string()]));
    base_regions[SHOP_ROOF].add_location(BaseConnection::new(EVENT_KILL_JUAN, no_shotgun_anysword, vec![]));

    // ShopLake connections
    base_regions[SHOP_LAKE].add_jumpconnection(JumpConnection::new(VOLCANO_TOP_EXIT, rules::always, vec![], 2.0));
    base_regions[SHOP_LAKE].add_connection(BaseConnection::new(BEHIND_SHOP_BUSH, rules::anysword, vec![]));
    base_regions[SHOP_LAKE].add_connection(BaseConnection::new(SHOP, rules::always, vec![]));

    // Ocean connections
    base_regions[OCEAN].add_connection(BaseConnection::new(SHOP_ROOF, rules::always, vec![]));
    base_regions[OCEAN].add_location(BaseConnection::new(LOC95, rules::always, vec![]));
    base_regions[OCEAN].add_location(BaseConnection::new(LOC96, rules::always, vec![]));
    base_regions[OCEAN].add_location(BaseConnection::new(LOC97, rules::always, vec![]));

    // NukeStorage connections
    base_regions[NUKE_STORAGE].add_connection(BaseConnection::new(SHOP, rules::always, vec![]));

    // ShopCellar connections
    base_regions[SHOP_CELLAR].add_connection(BaseConnection::new(SHOP, rules::princess, vec![]));
    base_regions[SHOP_CELLAR].add_connection(BaseConnection::new(PARASITE, rules::always, vec![]));
    base_regions[SHOP_CELLAR].add_location(BaseConnection::new(LOC78, rules::always, vec![]));

    // Parasite connections
    base_regions[PARASITE].add_location(BaseConnection::new(LOC89, rules::always, vec![]));

    // HookArea connections
    base_regions[HOOK_AREA].add_jumpconnection(JumpConnection::new(CASTLE_MINIONS, rules::always, vec![], 3.0));
    base_regions[HOOK_AREA].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::hook, vec![]));

    // AboveHook connections
    base_regions[ABOVE_HOOK].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::always, vec![]));
    base_regions[ABOVE_HOOK].add_jumpconnection(JumpConnection::new(ABOVE_ABOVE_HOOK, rules::always, vec![], 3.0));
    base_regions[ABOVE_HOOK].add_jumpconnection(JumpConnection::new(ABOVE_ABOVE_HOOK, rules::anysword, vec![], 2.0));
    base_regions[ABOVE_HOOK].add_connection(BaseConnection::new(ABOVE_ABOVE_HOOK, rules::hook, vec![]));
    base_regions[ABOVE_HOOK].add_connection(BaseConnection::new(BOMB, rules::always, vec![]));

    // AboveAboveHook connections
    base_regions[ABOVE_ABOVE_HOOK].add_connection(BaseConnection::new(ABOVE_HOOK, rules::always, vec![]));
    base_regions[ABOVE_ABOVE_HOOK].add_jumpconnection(JumpConnection::new(CASTLE_CANNON_TO_SHOP, rules::always, vec![], 3.0));
    base_regions[ABOVE_ABOVE_HOOK].add_connection(BaseConnection::new(CASTLE_CANNON_TO_SHOP, rules::hook, vec![]));
    base_regions[ABOVE_ABOVE_HOOK].add_jumpconnection(JumpConnection::new(ALTAR, rules::always, vec![], 2.0));
    base_regions[ABOVE_ABOVE_HOOK].add_connection(BaseConnection::new(ALTAR, rules::hook, vec![]));

    // CastleCannonToShop connections
    base_regions[CASTLE_CANNON_TO_SHOP].add_connection(BaseConnection::new(ABOVE_ABOVE_HOOK, rules::always, vec![]));
    base_regions[CASTLE_CANNON_TO_SHOP].add_connection(BaseConnection::new(SHOP_LAKE, rules::no_princess_no_nuke, vec!["Castle To Shop Cannon".to_string()]));
    base_regions[CASTLE_CANNON_TO_SHOP].add_location(BaseConnection::new(LOC17, rules::no_princess_no_nuke, vec!["Castle To Shop Cannon".to_string()]));
    base_regions[CASTLE_CANNON_TO_SHOP].add_location(BaseConnection::new(LOC56, rules::nuke_no_princess, vec!["Castle To Shop Cannon".to_string()]));

    // Altar connections
    base_regions[ALTAR].add_connection(BaseConnection::new(ABOVE_ABOVE_HOOK, rules::always, vec![]));
    base_regions[ALTAR].add_jumpconnection(JumpConnection::new(MOUNTAIN_LEFT_OUTCROP, rules::always, vec![], 2.0));
    base_regions[ALTAR].add_jumpconnection(JumpConnection::new(LEVERS, rules::always, vec![], 3.0));
    base_regions[ALTAR].add_connection(BaseConnection::new(LEVERS, rules::hook, vec![]));
    base_regions[ALTAR].add_connection(BaseConnection::new(GREAT_WATERFALL, rules::always, vec![]));
    base_regions[ALTAR].add_location(BaseConnection::new(LOC72, rules::princess, vec![]));

    // Bomb connections
    base_regions[BOMB].add_jumpconnection(JumpConnection::new(ABOVE_HOOK, rules::always, vec![], 3.0));
    base_regions[BOMB].add_connection(BaseConnection::new(ABOVE_HOOK, rules::hook, vec![]));
    base_regions[BOMB].add_connection(BaseConnection::new(FISHING_ROD, rules::always, vec![]));
    base_regions[BOMB].add_connection(BaseConnection::new(SECRET_PATH_MOAT_WELL, rules::always, vec![]));
    base_regions[BOMB].add_jumpconnection(JumpConnection::new(SECRET_ABOVE_BOMB, rules::always, vec![], 3.0));
    base_regions[BOMB].add_jumpconnection(JumpConnection::new(GREAT_WATERFALL, rules::bomb, vec![], 2.0));
    base_regions[BOMB].add_connection(BaseConnection::new(GREAT_WATERFALL, rules::bomb_hook, vec![]));
    base_regions[BOMB].add_location(BaseConnection::new(LOC28, rules::bomb, vec![]));
    base_regions[BOMB].add_location(BaseConnection::new(LOC32, rules::anysword, vec!["Boulder".to_string()]));
    base_regions[BOMB].add_location(BaseConnection::new(LOC54, rules::mrhugs, vec!["Boulder".to_string()]));

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
    base_regions[MOUNTAIN_TOP].add_connection(BaseConnection::new(CLOUD, rules::chicken, vec![]));
    base_regions[MOUNTAIN_TOP].add_jumpconnection(JumpConnection::new(STRAWBERRY, rules::always, vec![], 3.0));
    base_regions[MOUNTAIN_TOP].add_location(BaseConnection::new(EVENT_KILL_MIGUEL, rules::anysword, vec![]));

    // Strawberry connections
    base_regions[STRAWBERRY].add_location(BaseConnection::new(LOC24, rules::always, vec!["Strawberry".to_string()]));

    // MountainTreasure connections
    base_regions[MOUNTAIN_TREASURE].add_connection(BaseConnection::new(BELOW_LEAP_OF_FAITH, rules::always, vec![]));
    base_regions[MOUNTAIN_TREASURE].add_location(BaseConnection::new(LOC33, rules::no_princess, vec![]));
    base_regions[MOUNTAIN_TREASURE].add_location(BaseConnection::new(LOC62, rules::shovel, vec![]));

    // Levers connections
    base_regions[LEVERS].add_jumpconnection(JumpConnection::new(ALTAR, rules::always, vec![], 4.0));
    base_regions[LEVERS].add_connection(BaseConnection::new(ALTAR, rules::hook, vec![]));
    base_regions[LEVERS].add_jumpconnection(JumpConnection::new(BELOW_LEAP_OF_FAITH, rules::always, vec![], 4.0));
    base_regions[LEVERS].add_connection(BaseConnection::new(BELOW_LEAP_OF_FAITH, rules::hook, vec![]));
    base_regions[LEVERS].add_jumpconnection(JumpConnection::new(DARKSTONE, rules::always, vec!["Dark Stone Lever Middle".to_string()], 3.0));
    base_regions[LEVERS].add_connection(BaseConnection::new(DARKSTONE, rules::hook, vec!["Dark Stone Lever Middle".to_string()]));
    base_regions[LEVERS].add_connection(BaseConnection::new(GREAT_WATERFALL, rules::always, vec!["Dark Stone Lever Middle".to_string()]));
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
    base_regions[GREAT_WATERFALL].add_connection(BaseConnection::new(ALTAR, rules::hook, vec![]));
    base_regions[GREAT_WATERFALL].add_connection(BaseConnection::new(BELOW_FISHING_BRIDGE, rules::always, vec![]));
    base_regions[GREAT_WATERFALL].add_connection(BaseConnection::new(BOMB, rules::bomb, vec![]));
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
    base_regions[WATER_FALLS].add_location(BaseConnection::new(LOC08, rules::no_princess, vec![]));
    base_regions[WATER_FALLS].add_location(BaseConnection::new(LOC82, rules::princess, vec![]));
    base_regions[WATER_FALLS].add_location(BaseConnection::new(LOC87, rules::always, vec!["Event Kill Juan".to_string(), "Event Kill Miguel".to_string(), "Event Kill Javi".to_string(), "Event Kill Alberto".to_string(), "Event Kill Daniel".to_string()]));

    // AboveWaterfalls connections
    base_regions[ABOVE_WATERFALLS].add_connection(BaseConnection::new(WATER_FALLS, rules::always, vec![]));
    base_regions[ABOVE_WATERFALLS].add_connection(BaseConnection::new(BELOW_FISHING_BRIDGE, rules::always, vec![]));
    base_regions[ABOVE_WATERFALLS].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::always, vec![]));

    // FortressMoat connections
    base_regions[FORTRESS_MOAT].add_connection(BaseConnection::new(WATER_FALLS, rules::always, vec![]));
    base_regions[FORTRESS_MOAT].add_jumpconnection(JumpConnection::new(ABOVE_WATERFALLS, rules::always, vec![], 2.0));
    base_regions[FORTRESS_MOAT].add_connection(BaseConnection::new(FAIRY_FOUNTAIN, rules::always, vec![]));
    base_regions[FORTRESS_MOAT].add_jumpconnection(JumpConnection::new(FORTRESS_BRIDGE_BUTTON, rules::always, vec![], 3.0));
    base_regions[FORTRESS_MOAT].add_connection(BaseConnection::new(FORTRESS_BRIDGE_BUTTON, rules::hook, vec![]));
    base_regions[FORTRESS_MOAT].add_jumpconnection(JumpConnection::new(RIGHT_OF_FORTRESS, rules::always, vec![], 3.0));
    base_regions[FORTRESS_MOAT].add_connection(BaseConnection::new(RIGHT_OF_FORTRESS, rules::hook_or_shovel_or_bomb, vec![]));
    base_regions[FORTRESS_MOAT].add_location(BaseConnection::new(LOC15, rules::always, vec![]));
    base_regions[FORTRESS_MOAT].add_location(BaseConnection::new(LOC21, rules::always, vec![]));
    base_regions[FORTRESS_MOAT].add_location(BaseConnection::new(LOC48, rules::always, vec![]));
    base_regions[FORTRESS_MOAT].add_location(BaseConnection::new(LOC49, rules::anysword, vec![]));
    base_regions[FORTRESS_MOAT].add_location(BaseConnection::new(LOC61, rules::always, vec![]));

    // FortressBridgeButton connections
    base_regions[FORTRESS_BRIDGE_BUTTON].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::always, vec![]));
    base_regions[FORTRESS_BRIDGE_BUTTON].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::hook, vec![]));
    base_regions[FORTRESS_BRIDGE_BUTTON].add_jumpconnection(JumpConnection::new(WHISTLE_ALTAR, rules::fortress_bridge_down, vec![], 2.0));
    base_regions[FORTRESS_BRIDGE_BUTTON].add_statechange(StateChange::new(
        vec!["fortressBridgeDown".to_string()],
        vec![true],
        rules::fortress_bridge_up,
        vec![],
    ));

    // FairyFountain connections
    base_regions[FAIRY_FOUNTAIN].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::always, vec![]));
    base_regions[FAIRY_FOUNTAIN].add_connection(BaseConnection::new(LONKS_FRONTGARDEN, rules::always, vec!["Fairy Portal".to_string()]));
    base_regions[FAIRY_FOUNTAIN].add_location(BaseConnection::new(LOC65, rules::always, vec![]));
    base_regions[FAIRY_FOUNTAIN].add_location(BaseConnection::new(LOC85, rules::sword_or_mrhugs, vec![]));

    // Whistle connections
    base_regions[WHISTLE].add_jumpconnection(JumpConnection::new(GREAT_WATERFALL, rules::always, vec![], 2.0));
    base_regions[WHISTLE].add_connection(BaseConnection::new(GREAT_WATERFALL, rules::hook, vec![]));
    base_regions[WHISTLE].add_connection(BaseConnection::new(GREAT_WATERFALL_BOTTOM, rules::always, vec![]));
    base_regions[WHISTLE].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::always, vec![]));

    // WhistleAltar connections
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(GREAT_WATERFALL, rules::always, vec![], 2.0));
    base_regions[WHISTLE_ALTAR].add_connection(BaseConnection::new(GREAT_WATERFALL_BOTTOM, rules::always, vec![]));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(BELOW_LEAP_OF_FAITH, rules::always, vec![], 3.0));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(ELEVATOR, rules::no_princess, vec![], 3.0));
    base_regions[WHISTLE_ALTAR].add_connection(BaseConnection::new(ELEVATOR, rules::no_princess_hook_or_fortress_bridge, vec![]));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(FORTRESS_ROOF, rules::fortress_bridge_up, vec![], 3.0));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(FORTRESS_ROOF, rules::fortress_bridge_up_hook, vec![], 2.0));
    base_regions[WHISTLE_ALTAR].add_jumpconnection(JumpConnection::new(WHISTLE, rules::always, vec![], 3.0));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC39, rules::no_princess, vec![]));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC69, rules::anysword_princess, vec![]));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC73, rules::mrhugs_princess, vec![]));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC75, rules::princess, vec![]));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC83, rules::whistle, vec![]));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC90, rules::anysword_princess, vec![]));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(LOC93, rules::darkstone_princess, vec![]));
    base_regions[WHISTLE_ALTAR].add_location(BaseConnection::new(EVENT_KILL_ALBERTO, rules::sword_fortress_bridge_up, vec![]));

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
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC34, rules::princess, vec![]));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC50, rules::princess, vec![]));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC66, rules::darkstone, vec![]));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC76, rules::princess, vec![]));
    base_regions[ELEVATOR].add_location(BaseConnection::new(LOC80, rules::chicken, vec![]));

    // FortressRoof connections
    base_regions[FORTRESS_ROOF].add_jumpconnection(JumpConnection::new(WHISTLE_ALTAR, rules::always, vec![], 4.0));
    base_regions[FORTRESS_ROOF].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::fortress_bridge_down, vec![]));
    base_regions[FORTRESS_ROOF].add_connection(BaseConnection::new(ANVIL, rules::always, vec![]));
    base_regions[FORTRESS_ROOF].add_connection(BaseConnection::new(CASTLE_MINIONS, rules::no_princess_no_nuke, vec!["Dark Fortress Cannon".to_string()]));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC17, rules::no_princess_no_nuke, vec!["Dark Fortress Cannon".to_string()]));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC42, rules::no_princess, vec!["Princess".to_string()]));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC52, rules::princess, vec!["Dark Fortress Cannon".to_string()]));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC55, rules::no_chicken_princess, vec![]));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC58, rules::no_chicken_no_princess, vec![]));
    base_regions[FORTRESS_ROOF].add_location(BaseConnection::new(LOC84, rules::nuke_no_princess, vec!["Dark Fortress Cannon".to_string()]));

    // Anvil connections
    base_regions[ANVIL].add_jumpconnection(JumpConnection::new(FORTRESS_ROOF, rules::always, vec![], 4.0));
    base_regions[ANVIL].add_connection(BaseConnection::new(FORTRESS_ROOF, rules::hook, vec![]));
    base_regions[ANVIL].add_connection(BaseConnection::new(ELEVATOR, rules::always, vec!["Elevator Button".to_string()]));
    base_regions[ANVIL].add_connection(BaseConnection::new(ELEVATOR, rules::always, vec!["Call Elevator Buttons".to_string()]));
    base_regions[ANVIL].add_jumpconnection(JumpConnection::new(PRINCESS, rules::always, vec![], 3.0));
    base_regions[ANVIL].add_connection(BaseConnection::new(PRINCESS, rules::hook, vec![]));
    base_regions[ANVIL].add_connection(BaseConnection::new(FIRE_ESCAPE, rules::princess, vec![]));
    base_regions[ANVIL].add_connection(BaseConnection::new(FORTRESS_TREASURE, rules::princess, vec![]));
    base_regions[ANVIL].add_location(BaseConnection::new(LOC22, rules::always, vec!["Anvil".to_string()]));
    base_regions[ANVIL].add_location(BaseConnection::new(LOC23, rules::always, vec!["Mimic".to_string()]));
    base_regions[ANVIL].add_location(BaseConnection::new(LOC53, rules::princess, vec![]));
    base_regions[ANVIL].add_location(BaseConnection::new(LOC98, rules::no_princess_burger, vec!["Mimic".to_string()]));

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
    base_regions[PRINCESS].add_location(BaseConnection::new(LOC45, rules::princess, vec![]));
    base_regions[PRINCESS].add_location(BaseConnection::new(LOC57, rules::mrhugs_princess, vec![]));
    base_regions[PRINCESS].add_location(BaseConnection::new(LOC64, rules::no_princess_sword, vec![]));

    // SpikeTrap connections
    base_regions[SPIKE_TRAP].add_location(BaseConnection::new(LOC70, rules::always, vec![]));

    // FireEscape connections
    base_regions[FIRE_ESCAPE].add_connection(BaseConnection::new(ELEVATOR, rules::always, vec![]));
    base_regions[FIRE_ESCAPE].add_jumpconnection(JumpConnection::new(FORTRESS_ROOF, rules::always, vec![], 2.0));
    base_regions[FIRE_ESCAPE].add_connection(BaseConnection::new(WHISTLE_ALTAR, rules::fortress_bridge_down, vec![]));

    // FortressTreasure connections
    base_regions[FORTRESS_TREASURE].add_connection(BaseConnection::new(RIGHT_OF_FORTRESS, rules::always, vec![]));
    base_regions[FORTRESS_TREASURE].add_location(BaseConnection::new(LOC68, rules::always, vec![]));
    base_regions[FORTRESS_TREASURE].add_location(BaseConnection::new(EVENT_KILL_JAVI, rules::anysword, vec![]));

    // RightOfFortress connections
    base_regions[RIGHT_OF_FORTRESS].add_jumpconnection(JumpConnection::new(FORTRESS_TREASURE, rules::always, vec![], 3.0));
    base_regions[RIGHT_OF_FORTRESS].add_connection(BaseConnection::new(ELEVATOR, rules::always, vec![]));
    base_regions[RIGHT_OF_FORTRESS].add_connection(BaseConnection::new(FORTRESS_MOAT, rules::hook, vec![]));
    fn can_enter_desert(state: &ReventureState) -> bool {
        state.get_weight() >= 2.0
    }
    base_regions[RIGHT_OF_FORTRESS].add_connection(BaseConnection::new(DESERT, can_enter_desert, vec![]));
    base_regions[RIGHT_OF_FORTRESS].add_location(BaseConnection::new(LOC81, rules::princess, vec![]));

    // Desert connections
    base_regions[DESERT].add_location(BaseConnection::new(LOC91, rules::always, vec![]));

    println!("Region connections setup complete!");
}
