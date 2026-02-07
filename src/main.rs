use std::collections::{HashMap, HashSet, VecDeque};
use rand::seq::IndexedRandom;

use crate::locations::regions::{MENU};
use espresso_logic::{BoolExpr, Minimizable};

mod plantuml;

mod locations;
mod connections;
mod items;

const TOTAL_JUMP_INCREASE: i32 = 0;
const START_JUMP: f32 = 3.0;

// APItems - stores a set of advancement progression items
#[derive(Clone, Debug)]
struct SimpleBitset {
    contents: u64,
}

const ITEMID_TO_ITEMNAME: [&str; 64] = [
    "Nothing",
    "ProgressiveSword",
    "SwordPedestal",
    "Shovel",
    "Boomerang",
    "Map",
    "Compass",
    "Whistle",
    "Burger",
    "DarkStone",
    "Hook",
    "FishingRod",
    "LavaTrinket",
    "MisterHugs",
    "Bombs",
    "Shield",
    "Nuke",
    "Princess",
    "Anvil",
    "Strawberry",
    "ShopCannon",
    "CastleToShopCannon",
    "DarkFortressCannon",
    "CastleToDarkFortressCannon",
    "DesertGeyserEast",
    "DesertGeyserWest",
    "VolcanoGeyser",  
    "WaterfallGeyser",
    "ElevatorButton",
    "CallElevatorButtons",
    "MirrorPortal",
    "FairyPortal",
    "Vine",
    "OpenCastleFloor",
    "FaceplantStone",
    "SewerPipe",
    "DarkStoneLeverLeft",
    "DarkStoneLeverMiddle",
    "DarkStoneLeverRight",
    "Dragon",
    "Shopkeeper",
    "Mimic",
    "King",
    "Chicken",
    "Elder",
    "Boulder",
    "Closet",
    "Princess Statue",
    "PC",
    "Dolphins",
    "Mimic Pet",
    "Gem",
    "ChangeHeroName",
    "ChangePrincessName",
    "ChangeDarkLordName",
    "JumpIncrease",
    "SwordChest",
    "Filler",
    "Filler",
    "EventKillJuan",
    "EventKillMiguel",
    "EventKillJavi",
    "EventKillAlberto",
    "EventKillDaniel",
];

impl SimpleBitset {
    fn new(items: Vec<u8>) -> Self {
        let mut apitems = SimpleBitset::new_empty();
        for item in items {
            apitems.add_apitem(item);
        }
        apitems
    }

    fn new_empty() -> Self {
        SimpleBitset {
            contents: 0,
        }
    }

    fn contains(&self, item: u8) -> bool {
        self.contents & (1u64 << item) != 0
    }

    fn add_apitem(&mut self, item: u8) -> bool {
        if self.contains(item) {
            return false;
        }
        self.contents |= 1u64 << item;
        true
    }

    fn add_apitems(&mut self, items: SimpleBitset) {
        self.contents |= items.contents;
    }

    fn is_subset(&self, other: &SimpleBitset) -> bool {
        (self.contents & other.contents) == other.contents
    }

    fn to_string(&self) -> String {
        let mut output = String::new();
        for item in 0..64 {
            if self.contains(item) {
                output.push_str(ITEMID_TO_ITEMNAME[item as usize]);
                output.push_str(" & ");
            }
        }
        output.pop(); // remove trailing " "
        output.pop(); // remove trailing &
        output
    }
}

// APState - manages potential AP item states
#[derive(Clone, Debug)]
struct APState {
    potapitems: Vec<SimpleBitset>,
}

impl APState {
    fn new() -> Self {
        APState {
            potapitems: Vec::new(),
        }
    }

    fn add_potapitems(&mut self, new_potapitems: SimpleBitset) -> bool {
        let mut to_remove = Vec::new();

        for potapitems in &self.potapitems {
            if new_potapitems.is_subset(potapitems) {
                return false;
            }
            if potapitems.is_subset(&new_potapitems) {
                to_remove.push(potapitems.contents);
            }
        }
        self.potapitems.retain(|potapitems| !to_remove.contains(&potapitems.contents));
        self.potapitems.push(new_potapitems);
        true
    }
}

// ReventureState - stores game state
#[derive(Clone, Debug)]
struct ReventureState {
    state: SimpleBitset,
}

enum States {
    HasSword,
    HasSwordElder,
    HasChicken,
    HasShovel,
    HasShield,
    HasMap,
    HasCompass,
    HasMrHugs,
    HasLavaTrinket,
    HasHook,
    HasPrincess,
    HasBombs,
    HasNuke,
    HasWhistle,
    HasDarkStone,
    HasBurger,
    HasShotgun,
    DestroyedDarkstone,
    // SacSword,
    // SacSwordElder,
    // SacChicken,
    // SacShovel,
    // SacShield,
    // // SacMap,
    // // SacCompass,
    // SacMrHugs,
    // SacLavaTrinket,
    // SacHook,
    // SacBomb,
    // SacNuke,
    // SacWhistle,
    // SacDarkStone,
    // SacBurger,
    CastleBridgeDown,
    FortressBridgeDown,
}


impl ReventureState {
    fn new() -> Self {
        let state = SimpleBitset::new_empty();
        
        ReventureState { state }
    }

    pub fn event_bool(&self, event: u8) -> bool {
        self.state.contains(event)
    }

    fn get_weight(&self) -> f32 {
        let mut weight = 0.0;
        if self.event_bool(States::HasShovel as u8) { weight += 0.5; }
        if self.event_bool(States::HasSword as u8) { weight += 0.5; }
        if self.event_bool(States::HasSwordElder as u8) { weight += 0.5; }
        if self.event_bool(States::HasChicken as u8) { weight += 0.5; }
        if self.event_bool(States::HasShield as u8) { weight += 0.5; }
        if self.event_bool(States::HasLavaTrinket as u8) { weight += 0.5; }
        if self.event_bool(States::HasHook as u8) { weight += 0.5; }
        if self.event_bool(States::HasBombs as u8) { weight += 0.5; }
        if self.event_bool(States::HasNuke as u8) { weight += 0.5; }
        if self.event_bool(States::HasWhistle as u8) { weight += 0.5; }
        if self.event_bool(States::HasDarkStone as u8) { weight += 0.5; }
        if self.event_bool(States::HasBurger as u8) { weight += 0.5; }
        weight
    }
}

type CollectionRule = fn(&ReventureState) -> bool;

// BaseRegion - template for regions
#[derive(Clone)]
pub struct BaseRegion {
    pub name: String,
    pub forcedstatechange: Vec<StateChange>,
    pub connections: Vec<BaseConnection>,
    pub jumpconnections: Vec<JumpConnection>,
    pub statechange: Vec<StateChange>,
    pub locations: Vec<BaseConnection>,
}

impl BaseRegion {
    pub fn new(name: &str) -> Self {
        BaseRegion {
            name: name.to_string(),
            forcedstatechange: Vec::new(),
            connections: Vec::new(),
            jumpconnections: Vec::new(),
            statechange: Vec::new(),
            locations: Vec::new(),
        }
    }

    fn add_forcedstatechange(&mut self, statechange: StateChange) {
        self.forcedstatechange.push(statechange);
    }

    fn add_jumpconnection(&mut self, jumpconnection: JumpConnection) {
        self.jumpconnections.push(jumpconnection);
    }

    fn add_connection(&mut self, connection: BaseConnection) {
        self.connections.push(connection);
    }

    fn add_statechange(&mut self, statechange: StateChange) {
        self.statechange.push(statechange);
    }

    fn add_location(&mut self, location: BaseConnection) {
        self.locations.push(location);
    }
}

// BaseConnection
#[derive(Clone)]
pub struct BaseConnection {
    pub goal_region: usize, // index into region array
    rule: CollectionRule,
    apitems: SimpleBitset,
}

impl BaseConnection {
    fn new(goal_region: usize, rule: CollectionRule, apitems: SimpleBitset) -> Self {
        BaseConnection {
            goal_region,
            rule,
            apitems,
        }
    }

    fn can_use(&self, state: &ReventureState) -> bool {
        (self.rule)(state)
    }
}

// JumpConnection
#[derive(Clone)]
pub struct JumpConnection {
    pub base: BaseConnection,
    pub jump_req: f32,
}

impl JumpConnection {
    fn new(goal_region: usize, rule: CollectionRule, apitems: SimpleBitset, jump_req: f32) -> Self {
        JumpConnection {
            base: BaseConnection::new(goal_region, rule, apitems),
            jump_req,
        }
    }

    fn get_jumpitems_req(&self, state: &ReventureState) -> i32 {
        let weight = state.get_weight();
        ((self.jump_req + weight - START_JUMP) * 2.0) as i32
    }

    fn can_use(&self, state: &ReventureState) -> bool {
        (self.base.rule)(state)
    }
}

// StateChange
#[derive(Clone)]
pub struct StateChange {
    rule: CollectionRule,
    apitems: SimpleBitset,
    pub states: Vec<u8>,
    pub values: Vec<bool>,
}

impl StateChange {
    fn new(states: Vec<u8>, values: Vec<bool>, rule: CollectionRule, apitems: SimpleBitset) -> Self {
        StateChange {
            rule,
            apitems,
            states,
            values,
        }
    }

    fn can_use(&self, state: &ReventureState) -> bool {
        (self.rule)(state)
    }
}

// Connection - runtime connection between regions
#[derive(Clone)]
struct Connection {
    goal_region_idx: usize,
    apitems: SimpleBitset,
}

impl Connection {
    fn new(goal_region_idx: usize, apitems: SimpleBitset) -> Self {
        let mut ap = SimpleBitset::new_empty();
        ap.add_apitems(apitems);
        Connection {
            goal_region_idx,
            apitems: ap,
        }
    }
}

// Region - runtime region
#[derive(Clone)]
struct Region {
    name: String,
    base_region_idx: usize,
    state: ReventureState,
    location: bool,
    connections: Vec<Connection>,
    parents: Vec<usize>,
    apstate: APState,
}

impl Region {
    fn new(base_region_idx: usize, state: ReventureState, location: bool, base_regions: &[BaseRegion]) -> Self {
        let _base_region = &base_regions[base_region_idx];
        let name = get_region_identifier(base_region_idx, &state, base_regions);
                
        Region {
            name,
            base_region_idx,
            state,
            location,
            connections: Vec::new(),
            parents: Vec::new(),
            apstate: APState::new(),
        }
    }

    #[allow(dead_code)]
    fn get_reachable_regions(&self, graph: &ReventureGraph, apitems: SimpleBitset) -> HashSet<usize> {
        let mut reachable_regions = HashSet::new();
        let mut todo_regions: Vec<usize> = vec![graph.region_map[&self.name]];
        
        while let Some(current_idx) = todo_regions.pop() {
            if reachable_regions.contains(&current_idx) {
                continue;
            }
            reachable_regions.insert(current_idx);
            
            for conn in &graph.regions[current_idx].connections {
                if !apitems.is_subset(&conn.apitems) {
                    continue;
                }
                
                if !reachable_regions.contains(&conn.goal_region_idx) {
                    todo_regions.push(conn.goal_region_idx);
                }
            }
        }
        
        reachable_regions
    }
}

fn get_region_identifier(base_region_idx: usize, state: &ReventureState, base_regions: &[BaseRegion]) -> String {
    let mut identifier = base_regions[base_region_idx].name.clone();
    identifier.push_str("__");
    identifier.push_str(state.state.contents.to_string().as_str());
    identifier
}

// ReventureGraph
struct ReventureGraph {
    regions: Vec<Region>,
    region_map: HashMap<String, usize>,
    item_locations: Vec<usize>,
}

impl ReventureGraph {
    fn new() -> Self {
        ReventureGraph {
            regions: Vec::new(),
            region_map: HashMap::new(),
            item_locations: Vec::new(),
        }
    }

    fn add_region(&mut self, region: Region) -> usize {
        let name = region.name.clone();
        let idx = self.regions.len();
        self.regions.push(region);
        self.region_map.insert(name, idx);
        idx
    }

    fn get_region(&self, name: &str) -> Option<usize> {
        self.region_map.get(name).copied()
    }

    fn count(&self) -> usize {
        self.region_map.len()
    }

    fn add_connection(&mut self, parent_region_idx: usize, new_connection: Connection) {
        // Avoid self-connections and duplicate connections
        if new_connection.goal_region_idx == parent_region_idx {
            return;
        }

        let parent_region = &self.regions[parent_region_idx];
        for con in &parent_region.connections {
            if con.goal_region_idx == new_connection.goal_region_idx && new_connection.apitems.is_subset(&con.apitems) {
                return;
            }
        }

        let new_connection_region_idx = new_connection.goal_region_idx;
        self.regions[parent_region_idx].connections.push(new_connection);
        self.regions[new_connection_region_idx].parents.push(parent_region_idx);
    }

    fn propagate_apstates(&mut self) {
        // Create a todo list with all regions
        let mut parent_todo_regions: VecDeque<usize> = (0..self.regions.len()).collect();
        let mut parent_todo_regions_set: HashSet<usize> = (0..self.regions.len()).collect();
        
        while !parent_todo_regions.is_empty() {
            let region_idx = parent_todo_regions.pop_front().unwrap();
            parent_todo_regions_set.remove(&region_idx);
            
            // Get connections for this region (need to clone to avoid borrow checker issues)
            let connections: Vec<Connection> = self.regions[region_idx].connections.clone();
            let parent_potapitems = self.regions[region_idx].apstate.potapitems.clone();
            
            for connection in connections {
                let child_idx: usize = connection.goal_region_idx;
                
                // Store previous state lengths for change detection
                let prev_state_len = self.regions[child_idx].apstate.potapitems.len();
                let prev_state_lengths: Vec<u64> = self.regions[child_idx].apstate.potapitems
                    .iter()
                    .map(|p| p.contents)
                    .collect();
                
                let mut added = false;
                
                if connection.apitems.contents != 0 {
                    // Connection requires AP items
                    for potapitems in &parent_potapitems {
                        let mut new_potapitems = potapitems.clone();
                        new_potapitems.add_apitems(connection.apitems.clone());
                        
                        if self.regions[child_idx].apstate.add_potapitems(new_potapitems) {
                            added = true;
                        }
                    }
                } else {
                    // No AP items required for this connection
                    for potapitems in &parent_potapitems {
                        if self.regions[child_idx].apstate.add_potapitems(potapitems.clone()) {
                            added = true;
                        }
                    }
                }
                
                if !added {
                    continue;
                }
                
                // Reduce the child's AP state
                // self.regions[child_idx].apstate.reduce_all();
                
                // Skip if already in todo list
                if parent_todo_regions_set.contains(&child_idx) {
                    continue;
                }
                
                // Check if state changed - if length changed
                if prev_state_len != self.regions[child_idx].apstate.potapitems.len() {
                    parent_todo_regions.push_back(child_idx);
                    parent_todo_regions_set.insert(child_idx);
                    continue;
                }
                
                // Check if any individual state lengths changed
                let change = self.regions[child_idx].apstate.potapitems
                    .iter()
                    .enumerate()
                    .any(|(i, potapitems)| {
                        potapitems.contents != prev_state_lengths[i]
                    });
                
                if change {
                    parent_todo_regions.push_back(child_idx);
                    parent_todo_regions_set.insert(child_idx);
                }
            }
        }
    }
}

fn build_graph(item_locs: &Vec<usize>, base_regions: &Vec<BaseRegion>, start_region: usize) -> ReventureGraph{
    // Build the Reventure graph
    println!("Building Reventure graph...");

    let mut graph = ReventureGraph::new();
    graph.item_locations = item_locs.clone();
    let mut todo_regions: Vec<usize> = Vec::new();
    let mut menuregion = Region::new(MENU, ReventureState::new(), false, &base_regions);
    menuregion.apstate.potapitems.push(SimpleBitset::new_empty());

    let menu_idx = graph.add_region(menuregion);
    todo_regions.push(menu_idx);

    while todo_regions.len() > 0 {
        let region_idx = todo_regions.pop().unwrap();
        let region = graph.regions[region_idx].clone();
        let base_region = &base_regions[region.base_region_idx];
        let mut forced_change_applied = false;
        let mut new_forced_state = region.state.clone();
        for forced_statechange in &base_region.forcedstatechange {
            if !forced_statechange.can_use(&region.state) {
                continue;
            }
            forced_change_applied = true;
            for (i, state) in forced_statechange.states.iter().enumerate() {
                if forced_statechange.values[i] {
                    new_forced_state.state.add_apitem(*state);
                }
            }
        }
        if forced_change_applied {
            let name = get_region_identifier(region.base_region_idx, &new_forced_state, &base_regions);
            let mut new_region_idx = graph.get_region(&name);
            if new_region_idx.is_none() {
                let new_region = Region::new(
                    region.base_region_idx,
                    new_forced_state,
                    region.location,
                    &base_regions,
                );
                new_region_idx = Some(graph.add_region(new_region));
                todo_regions.push(new_region_idx.unwrap());
            }
            let new_connection = Connection::new(
                new_region_idx.unwrap(),
                SimpleBitset::new_empty(),
            );
            graph.add_connection(region_idx, new_connection);
            continue;
        }

        for jump_connection in &base_region.jumpconnections {
            // Process jump connections
            let req_jump_increases = jump_connection.get_jumpitems_req(&region.state);
            if req_jump_increases > TOTAL_JUMP_INCREASE {
                continue;
            }
            if !jump_connection.can_use(&region.state) {
                continue;
            }
            let name = get_region_identifier(jump_connection.base.goal_region, &region.state, &base_regions);
            let mut new_region_idx = graph.get_region(&name);
            if new_region_idx.is_none() {
                let new_region = Region::new(
                    jump_connection.base.goal_region,
                    region.state.clone(),
                    false,
                    &base_regions,
                );
                new_region_idx = Some(graph.add_region(new_region));
                todo_regions.push(new_region_idx.unwrap());
            }
            let new_connection = Connection::new(
                new_region_idx.unwrap(),
                jump_connection.base.apitems.clone(),
            );
            graph.add_connection(region_idx, new_connection);
        }

        for base_connection in &base_region.connections {
            if !base_connection.can_use(&region.state) {
                continue;
            }
            let name = get_region_identifier(base_connection.goal_region, &region.state, &base_regions);
            let mut new_region_idx = graph.get_region(&name);
            if new_region_idx.is_none() {
                let new_region = Region::new(
                    base_connection.goal_region,
                    region.state.clone(),
                    false,
                    &base_regions,
                );
                new_region_idx = Some(graph.add_region(new_region));
                todo_regions.push(new_region_idx.unwrap());
            }
            let new_connection = Connection::new(
                new_region_idx.unwrap(),
                base_connection.apitems.clone(),
            );
            graph.add_connection(region_idx, new_connection);
        }

        for location in &base_region.locations {
            if !location.can_use(&region.state) {
                continue;
            }
            let name = get_region_identifier(location.goal_region, &ReventureState::new(), &base_regions);
            let mut new_region_idx = graph.get_region(&name);
            if new_region_idx.is_none() {
                let new_region = Region::new(
                    location.goal_region,
                    ReventureState::new(),
                    true,
                    &base_regions,
                );
                new_region_idx = Some(graph.add_region(new_region));
                // No reason to add location regions to todo list
            }
            let new_connection = Connection::new(
                new_region_idx.unwrap(),
                location.apitems.clone(),
            );
            graph.add_connection(region_idx, new_connection);
        }

        for statechange in &base_region.statechange {
            if !statechange.can_use(&region.state) {
                continue;
            }
            let mut new_state = region.state.clone();
            for (i, state) in statechange.states.iter().enumerate() {
                if statechange.values[i] {
                    new_state.state.add_apitem(*state);
                }
            }

            // Check for Harakiri ending unlock
            if !(region.state.event_bool(States::HasSword as u8) || region.state.event_bool(States::HasSwordElder as u8))
             && (new_state.event_bool(States::HasSword as u8) || new_state.event_bool(States::HasSwordElder as u8)) {  // This state can do the Harakiri ending
                let harakiri_region_name = get_region_identifier(locations::locations::LOC47, &ReventureState::new(), &base_regions);
                let mut harakiri_region_idx = graph.get_region(&harakiri_region_name);
                if harakiri_region_idx.is_none() {
                    let harakiri_region = Region::new(
                        locations::locations::LOC47,
                        ReventureState::new(),
                        true,
                        &base_regions,
                    );
                    harakiri_region_idx = Some(graph.add_region(harakiri_region));
                }
                let harakiri_connection = Connection::new(
                    harakiri_region_idx.unwrap(),
                    statechange.apitems.clone(),
                );
                graph.add_connection(region_idx, harakiri_connection);
            }

            // Check for greedy bastard ending 
            let weight = new_state.get_weight();
            if weight > 2.5 {
                // Disabled for now because it creates a MASSIVE rule in the end
                let greedy_region_name = get_region_identifier(locations::locations::LOC26, &ReventureState::new(), &base_regions);
                let mut greedy_region_idx = graph.get_region(&greedy_region_name);
                if greedy_region_idx.is_none() {
                    let greedy_region = Region::new(
                        locations::locations::LOC26,
                        ReventureState::new(),
                        true,
                        &base_regions,
                    );
                    greedy_region_idx = Some(graph.add_region(greedy_region));
                }
                let greedy_connection = Connection::new(
                    greedy_region_idx.unwrap(),
                    statechange.apitems.clone(),
                );
                graph.add_connection(region_idx, greedy_connection);
                continue; // This statechange leads to greedy bastard ending, no further progress is possible
            }

            // let required_jump_increases = (weight * 2.0 - (START_JUMP * 2.0 - 2.0)) as i32;
            // if required_jump_increases > TOTAL_JUMP_INCREASE {
            //     continue;
            // }
            let name = get_region_identifier(region.base_region_idx, &new_state, &base_regions);
            let mut new_region_idx = graph.get_region(&name);
            if new_region_idx.is_none() {
                let new_region = Region::new(
                    region.base_region_idx,
                    new_state,
                    false,
                    &base_regions,
                );
                new_region_idx = Some(graph.add_region(new_region));
                todo_regions.push(new_region_idx.unwrap());
            }
            let new_connection = Connection::new(
                new_region_idx.unwrap(),
                statechange.apitems.clone(),
            );
            graph.add_connection(region_idx, new_connection);
        }
    }

    println!("Reventure graph built with {} regions!", graph.count());

    // Propagate AP states
    println!("Propagating AP states...");
    graph.propagate_apstates();
    println!("AP states propagated!");

    // std::fs::remove_dir_all("graphs".to_string()).expect("Deletion error");
    // std::fs::create_dir("graphs".to_string()).expect("Creation error");
    // plantuml::save_plant_uml(&graph, &format!("graphs/ChangeHistory{}-Level{}", -2, 0));

    let mut output = String::new();
    output.push_str(base_regions[start_region].name.as_str());
    output.push_str("\n");
    for item_loc in item_locs.iter() {
        output.push_str(&format!("{},", base_regions[*item_loc].name));
    }
    // remove trailing |
    output.pop();
    output.push_str("\n");

    for region in graph.regions.iter() {
        if !region.location {
            continue;
        }
        let loc_name = &base_regions[region.base_region_idx].name;
        let apstate = region.apstate.clone();

        let mut logic_string = String::new();


        for apitems in apstate.potapitems.iter() {
            logic_string.push_str(&format!("{} | ", apitems.to_string()));
        }
        // remove trailing " | "
        logic_string.pop();
        logic_string.pop();
        logic_string.pop();

        if logic_string.is_empty() {
            logic_string = "true".to_string();
        }

        let logic_expression = BoolExpr::parse(&logic_string).expect(format!("Failed to parse logic expression for rules '{}'", logic_string).as_str());
        let minimized_expression = logic_expression.minimize().expect("Failed to minimize logic expression");

        output.push_str(&format!("{}=", loc_name));
        output.push_str(&format!("{}", minimized_expression));
        output.push_str("\n");
    }

    // Write to file
    std::fs::write("location_apstates.txt", output).expect("Unable to write file");

    // let encoded = bincode::serialize(&graph).expect("Serialization failed");
    // std::fs::write("graph.bin", encoded).expect("Unable to write file");

    graph
}


fn main() {    
    // Create all base regions
    let mut base_regions = locations::create_all_base_regions();

    let valid_regions = locations::get_all_game_regions();
    let rng = &mut rand::rng();

    // Get random item_locs
    let item_locs = valid_regions.choose_multiple(rng, 10).cloned().collect::<Vec<_>>();
    let item_locs = locations::get_default_item_locations(); // For testing purposes

    // Set up item placements
    connections::setup_item_placements(&mut base_regions, &item_locs);
    println!();
    
    // Select random start_region from valid_regions
    let start_region = *valid_regions.choose(rng).unwrap();
    let start_region = locations::regions::LONKS_HOUSE; // For testing purposes
    println!("Selected start region: {}", base_regions[start_region].name); 

    // Set up region connections
    connections::setup_region_connections(&mut base_regions, start_region);
    println!();

    // Build the Reventure graph
    build_graph(&item_locs, &base_regions, start_region);

    // Benchmark buildgraph
    // let iterations = 10;
    // let mut total_duration = 0;0;
    // for _ in 0..iterations {
    //     let start_time = std::time::Instant::now();
    //     let _graph = build_graph(&item_locs, &base_regions, start_region);
    //     let duration = start_time.elapsed().as_millis();
    //     total_duration += duration;
    // }
    // let average_duration = total_duration as f64 / iterations as f64;
    // println!("Average graph build time over {} iterations: {:.2} ms", iterations, average_duration);

    // build_simple_graph(&item_locs, &base_regions, start_region);

    // plantuml::save_plant_uml(&graph, "test.plantuml");
}

#[allow(dead_code)]
fn build_simple_graph(item_locs: &Vec<usize>, base_regions: &Vec<BaseRegion>) {
    // Build the Reventure graph
    println!("Building Reventure graph...");

    let mut graph = ReventureGraph::new();
    graph.item_locations = item_locs.clone();
    let mut todo_regions: Vec<usize> = Vec::new();
    let menuregion = Region::new(MENU, ReventureState::new(), false, &base_regions);

    let menu_idx = graph.add_region(menuregion);
    todo_regions.push(menu_idx);

    while todo_regions.len() > 0 {
        let region_idx = todo_regions.pop().unwrap();
        let region = graph.regions[region_idx].clone();
        let base_region = &base_regions[region.base_region_idx];
        for jump_connection in &base_region.jumpconnections {
            // Process jump connections
            let name = get_region_identifier(jump_connection.base.goal_region, &region.state, &base_regions);
            let mut new_region_idx = graph.get_region(&name);
            if new_region_idx.is_none() {
                let new_region = Region::new(
                    jump_connection.base.goal_region,
                    region.state.clone(),
                    false,
                    &base_regions,
                );
                new_region_idx = Some(graph.add_region(new_region));
                todo_regions.push(new_region_idx.unwrap());
            }
            let new_connection = Connection::new(
                new_region_idx.unwrap(),
                jump_connection.base.apitems.clone(),
            );
            graph.add_connection(region_idx, new_connection);
        }

        for base_connection in &base_region.connections {
            let name = get_region_identifier(base_connection.goal_region, &region.state, &base_regions);
            let mut new_region_idx = graph.get_region(&name);
            if new_region_idx.is_none() {
                let new_region = Region::new(
                    base_connection.goal_region,
                    region.state.clone(),
                    false,
                    &base_regions,
                );
                new_region_idx = Some(graph.add_region(new_region));
                todo_regions.push(new_region_idx.unwrap());
            }
            let new_connection = Connection::new(
                new_region_idx.unwrap(),
                base_connection.apitems.clone(),
            );
            graph.add_connection(region_idx, new_connection);
        }
    }
    plantuml::save_plant_uml(&graph, &format!("simple_graph.plantuml"));
}