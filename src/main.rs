use std::{collections::{HashMap, HashSet}};

use crate::{locations::regions::{MENU}};

mod plantuml;

mod locations;
mod connections;

const TOTAL_JUMP_INCREASE: i32 = 0;
const START_JUMP: f32 = 4.0;

// APItems - stores a set of advancement progression items
#[derive(Clone, Debug)]
struct APItems {
    apitems: HashSet<String>,
}

impl APItems {
    fn new() -> Self {
        APItems {
            apitems: HashSet::new(),
        }
    }

    fn add_apitem(&mut self, item: String) -> bool {
        self.apitems.insert(item)
    }

    fn add_apitems(&mut self, items: Vec<String>) -> Vec<String> {
        let mut added = Vec::new();
        for item in items {
            if self.add_apitem(item.clone()) {
                added.push(item);
            }
        }
        added
    }

    fn remove_apitems(&mut self, items: &[String]) {
        for item in items {
            self.apitems.remove(item);
        }
    }

    fn is_subset(&self, other: &APItems) -> bool {
        self.apitems.is_subset(&other.apitems)
    }

    fn to_string(&self) -> String {
        let mut items: Vec<&String> = self.apitems.iter().collect();
        items.sort();
        items.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
    }
}

// APState - manages potential AP item states
#[derive(Clone, Debug)]
struct APState {
    potapitems: Vec<APItems>,
    reducedstates: HashSet<String>,
}

impl APState {
    fn new() -> Self {
        APState {
            potapitems: Vec::new(),
            reducedstates: HashSet::new(),
        }
    }

    fn is_rejected(&self, apitems: &APItems) -> bool {
        self.reducedstates.contains(&apitems.to_string())
    }

    fn reduce_all(&mut self) -> bool {
        let old_len = self.potapitems.len();
        let mut new_potapitems = Vec::new();
        
        // Sort by length
        self.potapitems.sort_by_key(|x| x.apitems.len());
        
        for potapitems in &self.potapitems {
            if new_potapitems.iter().any(|used: &APItems| potapitems.apitems.is_superset(&used.apitems)) {
                self.reducedstates.insert(potapitems.to_string());
            } else {
                new_potapitems.push(potapitems.clone());
            }
        }
        
        self.potapitems = new_potapitems;
        self.potapitems.len() != old_len
    }
}

// ReventureState - stores game state
#[derive(Clone, Debug)]
struct ReventureState {
    state: HashMap<String, StateValue>,
}

#[derive(Clone, Debug)]
enum StateValue {
    Bool(bool),
    // Int(i32), // Used for sacrificecount if needed
}

impl ReventureState {
    fn new() -> Self {
        let mut state = HashMap::new();
        
        // Items
        state.insert("has_chicken".to_string(), StateValue::Bool(false));
        state.insert("has_shovel".to_string(), StateValue::Bool(false));
        state.insert("has_sword".to_string(), StateValue::Bool(false));
        state.insert("has_swordelder".to_string(), StateValue::Bool(false));
        state.insert("has_shield".to_string(), StateValue::Bool(false));
        state.insert("has_map".to_string(), StateValue::Bool(false));
        state.insert("has_compass".to_string(), StateValue::Bool(false));
        state.insert("has_mrhugs".to_string(), StateValue::Bool(false));
        state.insert("has_lavaTrinket".to_string(), StateValue::Bool(false));
        state.insert("has_hook".to_string(), StateValue::Bool(false));
        state.insert("has_princess".to_string(), StateValue::Bool(false));
        state.insert("has_bomb".to_string(), StateValue::Bool(false));
        state.insert("has_nuke".to_string(), StateValue::Bool(false));
        state.insert("has_whistle".to_string(), StateValue::Bool(false));
        state.insert("has_darkstone".to_string(), StateValue::Bool(false));
        state.insert("has_burger".to_string(), StateValue::Bool(false));
        state.insert("has_shotgun".to_string(), StateValue::Bool(false));
        // state.insert("sacrificecount".to_string(), StateValue::Int(0));
        
        // Events
        state.insert("castleBridgeDown".to_string(), StateValue::Bool(false));
        state.insert("fortressBridgeDown".to_string(), StateValue::Bool(false));
        
        ReventureState { state }
    }

    pub fn event_bool(&self, event: &str) -> bool {
        match self.state.get(event) {
            Some(StateValue::Bool(b)) => *b,
            _ => false,
        }
    }

    fn get_weight(&self) -> f32 {
        let mut weight = 0.0;
        if self.event_bool("has_shovel") { weight += 0.5; }
        if self.event_bool("has_sword") { weight += 0.5; }
        if self.event_bool("has_swordelder") { weight += 0.5; }
        if self.event_bool("has_chicken") { weight += 0.5; }
        if self.event_bool("has_shield") { weight += 0.5; }
        if self.event_bool("has_lavaTrinket") { weight += 0.5; }
        if self.event_bool("has_hook") { weight += 0.5; }
        if self.event_bool("has_bomb") { weight += 0.5; }
        if self.event_bool("has_nuke") { weight += 0.5; }
        if self.event_bool("has_whistle") { weight += 0.5; }
        if self.event_bool("has_darkstone") { weight += 0.5; }
        if self.event_bool("has_burger") { weight += 0.5; }
        weight
    }
}

type CollectionRule = fn(&ReventureState) -> bool;

// BaseRegion - template for regions
#[derive(Clone)]
pub struct BaseRegion {
    pub name: String,
    pub connections: Vec<BaseConnection>,
    pub jumpconnections: Vec<JumpConnection>,
    pub statechange: Vec<StateChange>,
    pub locations: Vec<BaseConnection>,
}

impl BaseRegion {
    pub fn new(name: &str) -> Self {
        BaseRegion {
            name: name.to_string(),
            connections: Vec::new(),
            jumpconnections: Vec::new(),
            statechange: Vec::new(),
            locations: Vec::new(),
        }
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
    pub apitems: Vec<String>,
}

impl BaseConnection {
    fn new(goal_region: usize, rule: CollectionRule, apitems: Vec<String>) -> Self {
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
    fn new(goal_region: usize, rule: CollectionRule, apitems: Vec<String>, jump_req: f32) -> Self {
        JumpConnection {
            base: BaseConnection::new(goal_region, rule, apitems),
            jump_req,
        }
    }

    fn get_jumpitems_req(&self, state: &ReventureState) -> i32 {
        let weight = state.get_weight();
        ((self.jump_req + weight - START_JUMP) * 2.0) as i32
    }
}

// StateChange
#[derive(Clone)]
pub struct StateChange {
    rule: CollectionRule,
    pub apitems: Vec<String>,
    pub states: Vec<String>,
    pub values: Vec<bool>,
}

impl StateChange {
    fn new(states: Vec<String>, values: Vec<bool>, rule: CollectionRule, apitems: Vec<String>) -> Self {
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
    apitems: APItems,
}

impl Connection {
    fn new(goal_region_idx: usize, apitems: Vec<String>) -> Self {
        let mut ap = APItems::new();
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
        let name = get_region_name(&[base_region_idx], &state, base_regions);
                
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
}

fn get_region_name(base_region_idxs: &[usize], state: &ReventureState, base_regions: &[BaseRegion]) -> String {
    let mut name = base_region_idxs
        .iter()
        .map(|&idx| base_regions[idx].name.as_str())
        .collect::<Vec<_>>()
        .join("_");
    
    let mut events: Vec<_> = state.state.iter().collect();
    events.sort_by_key(|(k, _)| k.as_str());
    
    for (event, value) in events {
        if let StateValue::Bool(true) = value {
            name.push_str("__");
            name.push_str(event);
        }
    }
    
    name
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
            if con.goal_region_idx == new_connection.goal_region_idx && con.apitems.is_subset(&new_connection.apitems) {
                return;
            }
        }

        let new_connection_region_idx = new_connection.goal_region_idx;
        self.regions[parent_region_idx].connections.push(new_connection);
        self.regions[new_connection_region_idx].parents.push(parent_region_idx);
    }

    fn propagate_apstates(&mut self) {
        // Create a todo list with all regions
        let mut parent_todo_regions: Vec<usize> = (0..self.regions.len()).collect();
        let mut parent_todo_regions_set: HashSet<usize> = parent_todo_regions.iter().copied().collect();
        
        while !parent_todo_regions.is_empty() {
            let region_idx = parent_todo_regions.remove(0);
            parent_todo_regions_set.remove(&region_idx);
            
            // Get connections for this region (need to clone to avoid borrow checker issues)
            let connections: Vec<Connection> = self.regions[region_idx].connections.clone();
            let parent_potapitems = self.regions[region_idx].apstate.potapitems.clone();
            
            for connection in connections {
                let child_idx = connection.goal_region_idx;
                
                // Store previous state lengths for change detection
                let prev_state_len = self.regions[child_idx].apstate.potapitems.len();
                let prev_state_lengths: Vec<usize> = self.regions[child_idx].apstate.potapitems
                    .iter()
                    .map(|p| p.apitems.len())
                    .collect();
                
                let mut added = false;
                
                if !connection.apitems.apitems.is_empty() {
                    // Connection requires AP items
                    for potapitems in &parent_potapitems {
                        let mut new_potapitems = potapitems.clone();
                        let apitems_vec: Vec<String> = connection.apitems.apitems.iter().cloned().collect();
                        let added_items = new_potapitems.add_apitems(apitems_vec.clone());
                        
                        if self.regions[child_idx].apstate.is_rejected(&new_potapitems) {
                            // Remove the items we just added
                            new_potapitems.remove_apitems(&added_items);
                            continue;
                        }
                        
                        added = true;
                        self.regions[child_idx].apstate.potapitems.push(new_potapitems);
                    }
                } else {
                    // No AP items required for this connection
                    for potapitems in &parent_potapitems {
                        if self.regions[child_idx].apstate.is_rejected(potapitems) {
                            continue;
                        }
                        added = true;
                        self.regions[child_idx].apstate.potapitems.push(potapitems.clone());
                    }
                }
                
                if !added {
                    continue;
                }
                
                // Reduce the child's AP state
                self.regions[child_idx].apstate.reduce_all();
                
                // Skip if already in todo list
                if parent_todo_regions_set.contains(&child_idx) {
                    continue;
                }
                
                // Check if state changed - if length changed
                if prev_state_len != self.regions[child_idx].apstate.potapitems.len() {
                    parent_todo_regions.push(child_idx);
                    parent_todo_regions_set.insert(child_idx);
                    continue;
                }
                
                // Check if any individual state lengths changed
                let change = self.regions[child_idx].apstate.potapitems
                    .iter()
                    .enumerate()
                    .any(|(i, potapitems)| {
                        i < prev_state_lengths.len() && potapitems.apitems.len() != prev_state_lengths[i]
                    });
                
                if change {
                    parent_todo_regions.push(child_idx);
                    parent_todo_regions_set.insert(child_idx);
                }
            }
        }
    }
}

fn build_graph(item_locs: &Vec<usize>, base_regions: &Vec<BaseRegion>, start_region: usize) {
    // Build the Reventure graph
    println!("Building Reventure graph...");

    let mut graph = ReventureGraph::new();
    graph.item_locations = item_locs.clone();
    let empty_state = ReventureState::new();
    let mut todo_regions: Vec<usize> = Vec::new();
    let mut menuregion = Region::new(MENU, empty_state.clone(), false, &base_regions);
    menuregion.apstate.potapitems.push(APItems::new());

    let menu_idx = graph.add_region(menuregion);
    todo_regions.push(menu_idx);

    while todo_regions.len() > 0 {
        let region_idx = todo_regions.pop().unwrap();
        let region = graph.regions[region_idx].clone();
        let base_region = &base_regions[region.base_region_idx];
        for jump_connection in &base_region.jumpconnections {
            // Process jump connections
            let req_jump_increases = jump_connection.get_jumpitems_req(&region.state);
            if req_jump_increases > TOTAL_JUMP_INCREASE {
                continue;
            }
            let name = get_region_name(&vec![jump_connection.base.goal_region], &region.state, &base_regions);
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
            let name = get_region_name(&vec![base_connection.goal_region], &region.state, &base_regions);
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
            let name = get_region_name(&vec![location.goal_region], &empty_state.clone(), &base_regions);
            let mut new_region_idx = graph.get_region(&name);
            if new_region_idx.is_none() {
                let new_region = Region::new(
                    location.goal_region,
                    empty_state.clone(),
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
                new_state.state.insert(state.clone(), StateValue::Bool(statechange.values[i]));
            }
            if !region.state.event_bool("has_sword") && new_state.event_bool("has_sword") {  // This state can do the Harakiri ending
                let harakiri_region_name = get_region_name(&vec![locations::locations::LOC47], &empty_state.clone(), &base_regions);
                let mut harakiri_region_idx = graph.get_region(&harakiri_region_name);
                if harakiri_region_idx.is_none() {
                    let harakiri_region = Region::new(
                        locations::locations::LOC47,
                        empty_state.clone(),
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

            let weight = new_state.get_weight();
            let required_jump_increases = (weight * 2.0 - (START_JUMP * 2.0 - 2.0)) as i32;
            if required_jump_increases > TOTAL_JUMP_INCREASE {
                continue; // There are only 6 increases. If we need more, we cannot reach this statechange
            }
            let name = get_region_name(&vec![region.base_region_idx], &new_state, &base_regions);
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
    output.push_str(&graph.regions[start_region].name);
    for region in graph.regions.iter() {
        if !region.location {
            continue;
        }
        let loc_name = &base_regions[region.base_region_idx].name;
        let apstate = region.apstate.clone();

        output.push_str(&format!("{}:", loc_name));
        for state in apstate.potapitems.iter() {
            let items = Vec::from_iter(state.apitems.iter().map(|x| x.as_str())).join("&");
            output.push_str(&format!("{}|", items));
        }
        // remove trailing |
        if output.ends_with("|") {
            output.pop();
        }
        output.push_str("\n");
    }

    // Write to file
    std::fs::write("location_apstates.txt", output).expect("Unable to write file");

    // // Remove duplicate solutions
    // graph.remove_duplicate_solutions();
    // println!("Duplicate solutions removed!");
    // // plantuml::save_plant_uml(&graph, &format!("graphs/ChangeHistory{}-Level{}", -1, 0));

    // println!("Simplifying Graph");

    // graph.detect_errors("before simplification");
    // let mut changes = 0;
    // let mut level = 0;
    // while level < 7 {
    //     println!("Step {}, Simplification Level {}, Graph Size {}", changes, level, graph.count());
    //     let change = graph.simplify(level);
    //     if change.is_empty() {
    //         level += 1;
    //     } else {
    //         // graph.detect_errors(format!("Step {}, Simplification Level {}", changes, level).as_str());
    //         // plantuml::save_plant_uml(&graph, &format!("graphs/ChangeHistory{}-Level{}", changes, level));
    //         changes += 1;
    //         level = 0;
    //     }
    // }
    
    // println!("Final graph has {} regions!", graph.count());
    // let plantuml_filepath = "reventure_graph_rust.plantuml";
    // println!("Saving PlantUML graph to {}...", plantuml_filepath);
    // plantuml::save_plant_uml(&graph, plantuml_filepath);
    // println!("PlantUML graph saved!");

    // save_region_graph(&graph, "output.reg");
}


fn main() {    
    // Create all base regions
    let mut base_regions = locations::create_all_base_regions();

    let item_locs = locations::get_default_item_locations();
    
    // Set up item placements
    connections::setup_item_placements(&mut base_regions, &item_locs);
    println!();
    
    // Set up region connections
    let start_region = locations::regions::LONKS_HOUSE;
    connections::setup_region_connections(&mut base_regions, start_region);
    println!();

    // Build the Reventure graph
    build_graph(&item_locs, &base_regions, start_region);
    // build_simple_graph(&item_locs, &base_regions, start_region);
}

#[allow(dead_code)]
fn build_simple_graph(item_locs: &Vec<usize>, base_regions: &Vec<BaseRegion>) {
    // Build the Reventure graph
    println!("Building Reventure graph...");

    let mut graph = ReventureGraph::new();
    graph.item_locations = item_locs.clone();
    let empty_state = ReventureState::new();
    let mut todo_regions: Vec<usize> = Vec::new();
    let mut menuregion = Region::new(MENU, empty_state.clone(), false, &base_regions);
    menuregion.apstate.potapitems.push(APItems::new());

    let menu_idx = graph.add_region(menuregion);
    todo_regions.push(menu_idx);

    while todo_regions.len() > 0 {
        let region_idx = todo_regions.pop().unwrap();
        let region = graph.regions[region_idx].clone();
        let base_region = &base_regions[region.base_region_idx];
        for jump_connection in &base_region.jumpconnections {
            // Process jump connections
            let name = get_region_name(&vec![jump_connection.base.goal_region], &region.state, &base_regions);
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
            let name = get_region_name(&vec![base_connection.goal_region], &region.state, &base_regions);
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

        // for location in &base_region.locations {
        //     let name = get_region_name(&vec![location.goal_region], &empty_state.clone(), &base_regions);
        //     let mut new_region_idx = graph.get_region(&name);
        //     if new_region_idx.is_none() {
        //         let new_region = Region::new(
        //             location.goal_region,
        //             empty_state.clone(),
        //             true,
        //             &base_regions,
        //         );
        //         new_region_idx = Some(graph.add_region(new_region));
        //         // No reason to add location regions to todo list
        //     }
        //     let new_connection = Connection::new(
        //         new_region_idx.unwrap(),
        //         location.apitems.clone(),
        //     );
        //     graph.add_connection(region_idx, new_connection);
        // }
    }
    plantuml::save_plant_uml(&graph, &format!("simple_graph.plantuml"));
}