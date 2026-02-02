use std::{collections::{HashMap, HashSet}};

use crate::{locations::regions::{LONKS_HOUSE, MENU}, plantuml::save_region_graph};

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

    fn add_apitems_from_string(&mut self, items: &str) {
        if items.is_empty() {
            return;
        }
        let items_vec: Vec<String> = items.split(", ").map(|s| s.to_string()).collect();
        self.add_apitems(items_vec);
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
    complexity: i32,
}

impl Region {
    fn new(base_region_idx: usize, state: ReventureState, location: bool, base_regions: &[BaseRegion]) -> Self {
        let _base_region = &base_regions[base_region_idx];
        let name = get_region_name(&[base_region_idx], &state, base_regions);
        
        let mut complexity = 0;
        for value in state.state.values() {
            if let StateValue::Bool(true) = value {
                complexity += 1;
            }
        }
        
        Region {
            name,
            base_region_idx,
            state,
            location,
            connections: Vec::new(),
            parents: Vec::new(),
            apstate: APState::new(),
            complexity,
        }
    }

    fn get_connections(&self, goal_region_idx: usize) -> Vec<&Connection> {
        self.connections
            .iter()
            .filter(|conn| conn.goal_region_idx == goal_region_idx)
            .collect()
    }

    fn remove_connections(&mut self, goal_region_idx: usize) {
        self.connections.retain(|conn| conn.goal_region_idx != goal_region_idx);
    }

    fn get_reachable_regions(&self, graph: &ReventureGraph) -> HashSet<usize> {
        let mut reachable_regions = HashSet::new();
        let mut todo_regions: Vec<usize> = vec![graph.region_map[&self.name]];
        
        while let Some(current_idx) = todo_regions.pop() {
            if reachable_regions.contains(&current_idx) {
                continue;
            }
            reachable_regions.insert(current_idx);
            
            for conn in &graph.regions[current_idx].connections {
                if !reachable_regions.contains(&conn.goal_region_idx) {
                    todo_regions.push(conn.goal_region_idx);
                }
            }
        }
        
        reachable_regions
    }

    fn get_reachable_regions_ignore(&self, graph: &ReventureGraph, ignore_idx: Option<usize>) -> HashSet<usize> {
        let mut reachable_regions = HashSet::new();
        let mut todo_regions: Vec<usize> = vec![graph.region_map[&self.name]];
        
        while let Some(current_idx) = todo_regions.pop() {
            if Some(current_idx) == ignore_idx {
                continue;
            }
            if reachable_regions.contains(&current_idx) {
                continue;
            }
            reachable_regions.insert(current_idx);
            
            for conn in &graph.regions[current_idx].connections {
                if !reachable_regions.contains(&conn.goal_region_idx) {
                    todo_regions.push(conn.goal_region_idx);
                }
            }
        }
        
        reachable_regions
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
    start_region: usize,
    item_locations: Vec<usize>,
}

impl ReventureGraph {
    fn new(start_region: usize) -> Self {
        ReventureGraph {
            regions: Vec::new(),
            region_map: HashMap::new(),
            start_region,
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
        self.regions.len()
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

    fn remove_duplicate_solutions(&mut self) {
        println!("Removing duplicate solutions");
        
        for region_idx in 0..self.regions.len() {            
            // Only process location regions
            if !self.regions[region_idx].location {
                continue;
            }

            // Group parents by their apitems string (including connection requirements)
            let mut parent_diffed_by_apitems: HashMap<String, Vec<usize>> = HashMap::new();
            
            let parent_indices: Vec<usize> = self.regions[region_idx].parents.clone();
            let mut mult_connections = false;
            
            for &parent_idx in &parent_indices {
                let connections = self.regions[parent_idx].get_connections(region_idx);
                if connections.is_empty() {
                    continue; // No connections found, skip
                }
                
                if connections.len() > 1 {
                    // I am working of of the assumption that there are only ever 0 or 1 connections here
                    // If there are multiple connections, skip this region.
                    mult_connections = true;
                    break;
                }

                let connection = connections[0]; // No merging has happened yet, so at most one connection
                
                for potapitems in &self.regions[parent_idx].apstate.potapitems {
                    let apitems_string = if !connection.apitems.apitems.is_empty() {
                        let mut loc_apitems = potapitems.clone();
                        let items_vec: Vec<String> = connection.apitems.apitems.iter().cloned().collect();
                        loc_apitems.add_apitems(items_vec);
                        loc_apitems.to_string()
                    } else {
                        potapitems.to_string()
                    };
                    
                    parent_diffed_by_apitems
                        .entry(apitems_string)
                        .or_insert_with(Vec::new)
                        .push(parent_idx);
                }
            }
            
            if mult_connections {
                continue;
            }

            // Find used apitem sets, whilst removing unnecessary ones
            let mut used_apstates: Vec<String> = Vec::new();
            
            for apstate_str in parent_diffed_by_apitems.keys() {
                if used_apstates.is_empty() {
                    used_apstates.push(apstate_str.clone());
                    continue;
                }
                
                let mut apstate = APItems::new();
                apstate.add_apitems_from_string(apstate_str);
                
                let mut new = true;
                let mut remove = Vec::new();
                
                for used_apstate_str in &used_apstates {
                    let mut used_apstate = APItems::new();
                    used_apstate.add_apitems_from_string(used_apstate_str);
                    
                    if apstate.is_subset(&used_apstate) {
                        remove.push(used_apstate_str.clone());
                    } else if used_apstate.is_subset(&apstate) {
                        new = false;
                        break;
                    }
                }
                
                for rem in remove {
                    used_apstates.retain(|s| s != &rem);
                }
                
                if new {
                    used_apstates.push(apstate_str.clone());
                }
            }
            
            // Remove all parents not in used_apstates
            let mut to_remove: Vec<usize> = Vec::new();
            
            for &parent_idx in &parent_indices {
                let connections = self.regions[parent_idx].get_connections(region_idx);
                if connections.is_empty() {
                    continue;
                }
                
                let connection = connections[0];
                let mut is_used = false;
                
                for potapitems in &self.regions[parent_idx].apstate.potapitems {
                    let apitems_string = if !connection.apitems.apitems.is_empty() {
                        let mut loc_apitems = potapitems.clone();
                        let items_vec: Vec<String> = connection.apitems.apitems.iter().cloned().collect();
                        loc_apitems.add_apitems(items_vec);
                        loc_apitems.to_string()
                    } else {
                        potapitems.to_string()
                    };
                    
                    if used_apstates.contains(&apitems_string) {
                        is_used = true;
                        break;
                    }
                }
                
                if !is_used {
                    to_remove.push(parent_idx);
                }
            }
            
            for parent_idx in to_remove {
                self.regions[parent_idx].remove_connections(region_idx);
                self.regions[region_idx].parents.retain(|&p| p != parent_idx);
            }
            
            // For each apstate only keep parent with lowest complexity
            for (apstate, apstate_parents) in &parent_diffed_by_apitems {
                if !used_apstates.contains(apstate) {
                    continue; // Not in used apstates, skip
                }
                
                // In case of a single parent, nothing happens here
                if apstate_parents.len() <= 1 {
                    continue;
                }
                
                // Find best parent (lowest complexity)
                let mut best_parent_idx = apstate_parents[0];
                let mut best_complexity = self.regions[best_parent_idx].complexity;
                
                for &parent_idx in &apstate_parents[1..] {
                    if self.regions[parent_idx].complexity >= best_complexity {
                        // Remove this parent's connection
                        self.regions[parent_idx].remove_connections(region_idx);
                        if let Some(pos) = self.regions[region_idx].parents.iter().position(|&p| p == parent_idx) {
                            self.regions[region_idx].parents.remove(pos);
                        }
                    } else {
                        // Remove previous best parent's connection
                        self.regions[best_parent_idx].remove_connections(region_idx);
                        if let Some(pos) = self.regions[region_idx].parents.iter().position(|&p| p == best_parent_idx) {
                            self.regions[region_idx].parents.remove(pos);
                        }
                        // Update best parent
                        best_parent_idx = parent_idx;
                        best_complexity = self.regions[parent_idx].complexity;
                    }
                }
            }
        }
    }

    fn remove_region(&mut self, region_idx: usize) {
        // Clone data we need before mutating
        let parent_indices = self.regions[region_idx].parents.clone();
        let connections = self.regions[region_idx].connections.clone();
        let region_name = self.regions[region_idx].name.clone();
        
        // Remove connections from parents
        for parent_idx in parent_indices {
            self.regions[parent_idx].remove_connections(region_idx);
        }
        
        // Remove this region from children's parent lists
        let mut removed_children = HashSet::new();
        for conn in &connections {
            if removed_children.contains(&conn.goal_region_idx) {
                continue;
            }
            removed_children.insert(conn.goal_region_idx);
            self.regions[conn.goal_region_idx].parents.retain(|&p| p != region_idx);
        }
        
        // Remove from region map
        self.region_map.remove(&region_name);
        
        // Note: We don't actually remove from the Vec to preserve indices
        // Instead we mark it as removed by clearing its data
        self.regions[region_idx].connections.clear();
        self.regions[region_idx].parents.clear();
        self.regions[region_idx].name = String::new();
    }

    fn detect_errors(&self, codeloc: &str) {        
        #[allow(unreachable_code)]
        for region_idx in 0..self.regions.len() {
            if !self.region_map.contains_key(&self.regions[region_idx].name) {
                continue; // Already removed
            }
            
            if self.regions[region_idx].name == "Menu" {
                continue;
            }
            
            let mut error = String::new();
            
            // Check connections
            for conn in &self.regions[region_idx].connections {
                let child_name = &self.regions[conn.goal_region_idx].name;
                
                // Check for zombie connections (child not in region map)
                if !self.region_map.contains_key(child_name) {
                    error = format!("Zombie connection: {} -> {}", self.regions[region_idx].name, child_name);
                }
                
                // Check for broken connections (region not in child's parents)
                if !self.regions[conn.goal_region_idx].parents.contains(&region_idx) {
                    error = format!("Broken connection: {} -> {}", self.regions[region_idx].name, child_name);
                }
            }
            
            // Check parents
            for &parent_idx in &self.regions[region_idx].parents {
                // Check for self-parent
                if parent_idx == region_idx {
                    error = format!("Self parent: {}", self.regions[region_idx].name);
                }
                
                // Check for zombie parents
                if !self.region_map.contains_key(&self.regions[parent_idx].name) {
                    error = format!("Zombie parent: {} <- {}", 
                        self.regions[region_idx].name, 
                        self.regions[parent_idx].name);
                }
            }
            
            // Check for duplicate parents
            // for i in 0..self.regions[region_idx].parents.len() {
            //     for j in (i + 1)..self.regions[region_idx].parents.len() {
            //         if self.regions[region_idx].parents[i] == self.regions[region_idx].parents[j] {
            //             error = format!("Duplicate parent: {} <- {}", 
            //                 self.regions[region_idx].name, 
            //                 self.regions[self.regions[region_idx].parents[i]].name);
            //         }
            //     }
            // }
            
            if !error.is_empty() {
                plantuml::save_plant_uml(self, "Error.puml");
                eprintln!("ERROR DETECTED in {}:", codeloc);
                eprintln!("{}", error);
                panic!("Graph validation failed: {}", error);
            }
        }
    }

    fn simplify(&mut self, level: i32) -> String {

        match level {
            0 => self.simplify_simpleregions(),
            1 => self.simplify_deadendloops(),
            2 => self.simplify_merge(),
            3 => self.simplify_dupeconnections(),
            4 => self.simplify_subrootnodes(),
            5 => self.simplify_tryremoveregion(),
            6 => self.simplify_tryremoveconnections(),
            7 => self.simplify_4(),
            _ => String::new(),
        }
    }

    fn simplify_simpleregions(&mut self) -> String {
        let mut changed = String::new();
        let mut simple_remove = Vec::new();
        let mut oneways = Vec::new();

        // Check for needless regions
        for region_idx in 0..self.regions.len() {
            if !self.region_map.contains_key(&self.regions[region_idx].name) {
                continue; // Already removed
            }

            if self.regions[region_idx].location {
                continue;
            }
            if self.regions[region_idx].name == "Menu" {
                continue;
            }

            // Remove regions with no parents
            if self.regions[region_idx].parents.is_empty() {
                simple_remove.push(region_idx);
                continue;
            }

            // Remove regions with no connections
            if self.regions[region_idx].connections.is_empty() {
                simple_remove.push(region_idx);
                continue;
            }

            // Remove regions only connected to a single parent
            let first_parent = self.regions[region_idx].parents[0];
            let all_same_parent = self.regions[region_idx].parents.iter().all(|&p| p == first_parent);
            
            if all_same_parent {
                let all_to_parent = self.regions[region_idx].connections.iter().all(|c| c.goal_region_idx == first_parent);
                if all_to_parent {
                    simple_remove.push(region_idx);
                    continue;
                }
            }

            // One way regions
            if self.regions[region_idx].parents.len() == 1 && self.regions[region_idx].connections.len() == 1 {
                let parent_idx = self.regions[region_idx].parents[0];
                let connections_from_parent = self.regions[parent_idx].get_connections(region_idx);
                let conn_apitems_len = self.regions[region_idx].connections[0].apitems.apitems.len();
                let parent_apitems_len = if !connections_from_parent.is_empty() {
                    connections_from_parent[0].apitems.apitems.len()
                } else {
                    0
                };
                
                if connections_from_parent.len() == 1 || conn_apitems_len + parent_apitems_len <= 2 {
                    oneways.push(region_idx);
                    continue;
                }
            }
        }

        // Remove simple regions
        for &idx in &simple_remove {
            self.remove_region(idx);
        }

        // Process oneways
        for &region_idx in &oneways {
            if !self.region_map.contains_key(&self.regions[region_idx].name) {
                continue;
            }

            if self.regions[region_idx].parents.len() != 1 || self.regions[region_idx].connections.len() != 1 {
                continue;
            }

            let parent_idx = self.regions[region_idx].parents[0];
            let child_conn = self.regions[region_idx].connections[0].clone();
            let connections_from_parent: Vec<Connection> = self.regions[parent_idx]
                .get_connections(region_idx)
                .iter()
                .map(|&c| c.clone())
                .collect();

            for parent_conn in connections_from_parent {
                let mut combined_apitems = child_conn.apitems.clone();
                let parent_items: Vec<String> = parent_conn.apitems.apitems.iter().cloned().collect();
                combined_apitems.add_apitems(parent_items);
                
                let new_conn = Connection::new(child_conn.goal_region_idx, combined_apitems.apitems.iter().cloned().collect());
                self.add_connection(parent_idx, new_conn);
            }
            
            self.remove_region(region_idx);
        }
        self.reindex();

        if !simple_remove.is_empty() || !oneways.is_empty() {
            changed.push_str(&format!("Removed {} simple regions and {} oneways\n", simple_remove.len(), oneways.len()));
            self.reindex();
        }
        changed
    }

    fn simplify_merge(&mut self) -> String {
        let mut changed = String::new();

        // Merge regions with free bidirectional movement
        let mut merge_count = 0;
        let region_indices: Vec<usize> = (0..self.regions.len()).collect();
        
        for &region_idx in &region_indices {
            if self.regions[region_idx].name == "Menu" {
                continue;
            }

            let parent_indices = self.regions[region_idx].parents.clone();
            for &parent_idx in &parent_indices {
                if !self.region_map.contains_key(&self.regions[parent_idx].name) {
                    continue;
                }
                
                // Check if parent is also a child
                let parent_is_child = self.regions[region_idx].connections.iter().any(|c| c.goal_region_idx == parent_idx);
                if !parent_is_child {
                    continue;
                }

                let conns_to_parent_len = self.regions[region_idx].get_connections(parent_idx).len();
                let conns_from_parent_len = self.regions[parent_idx].get_connections(region_idx).len();
                
                if conns_to_parent_len != 1 || conns_from_parent_len != 1 {
                    continue;
                }

                let conns_to_parent_empty = self.regions[region_idx].get_connections(parent_idx)[0].apitems.apitems.is_empty();
                let conns_from_parent_empty = self.regions[parent_idx].get_connections(region_idx)[0].apitems.apitems.is_empty();

                if !conns_to_parent_empty || !conns_from_parent_empty {
                    continue;
                }

                // Merge: Copy donor's connections and update parent connections
                let donor_connections = self.regions[region_idx].connections.clone();
                let donor_parents = self.regions[region_idx].parents.clone();
                let self_idx = self.region_map[&self.regions[parent_idx].name];
                
                // Add connections from donor's parents to self (parent)
                for &donor_parent_idx in &donor_parents {
                    if donor_parent_idx == self_idx {
                        continue;
                    }

                    let connections_to_donor: Vec<Connection> = self.regions[donor_parent_idx]
                        .get_connections(region_idx)
                        .iter()
                        .map(|&c| Connection::new(self_idx, c.apitems.apitems.iter().cloned().collect()))
                        .collect();
                    
                    for conn in connections_to_donor {
                        self.add_connection(donor_parent_idx, conn);
                    }
                }
                
                // Add donor's connections to parent
                for conn in donor_connections {
                    if conn.goal_region_idx == self_idx {
                        continue;
                    }
                    self.add_connection(self_idx, conn);
                }
                
                self.remove_region(region_idx);
                merge_count += 1;
                break;
            }
        }

        if merge_count > 0 {
            changed.push_str(&format!("Merged {} regions\n", merge_count));
            self.reindex();
        }

        changed
    }

    fn simplify_deadendloops(&mut self) -> String {
        let mut changed = String::new();

        // Remove complex loops that don't reach any location
        let mut complexloop_count = 0;
        for region_idx in 0..self.regions.len() {
            if !self.region_map.contains_key(&self.regions[region_idx].name) {
                continue;
            }
            if self.regions[region_idx].location {
                continue;
            }
            if self.regions[region_idx].name == "Menu" {
                continue;
            }

            let mut todo_regions = vec![region_idx];
            let mut reachable_regions = Vec::new();
            let mut reachable_location = false;

            while !todo_regions.is_empty() && !reachable_location {
                let current_idx = todo_regions.pop().unwrap();
                if reachable_regions.contains(&current_idx) {
                    continue;
                }
                reachable_regions.push(current_idx);

                for conn in &self.regions[current_idx].connections {
                    if self.regions[conn.goal_region_idx].location {
                        reachable_location = true;
                        break;
                    }
                    if !reachable_regions.contains(&conn.goal_region_idx) && !todo_regions.contains(&conn.goal_region_idx) {
                        todo_regions.push(conn.goal_region_idx);
                    }
                }
            }

            if !reachable_location {
                for &r_idx in &reachable_regions {
                    self.remove_region(r_idx);
                }
                complexloop_count += reachable_regions.len();
                break;
            }
        }

        if complexloop_count > 0 {
            changed.push_str(&format!("Removed {} complex loop regions\n", complexloop_count));
            self.reindex();
        }

        changed
    }

    fn simplify_dupeconnections(&mut self) -> String {
        let mut changed = String::new();
        // Reduce duplicate connections
        let mut reduce_connections_count = 0;
        for region_idx in 0..self.regions.len() {
            if !self.region_map.contains_key(&self.regions[region_idx].name) {
                continue;
            }

            let mut removed = true;
            while removed {
                removed = false;
                let connections = self.regions[region_idx].connections.clone();
                for i in 0..connections.len() {
                    for j in (i + 1)..connections.len() {
                        if connections[i].goal_region_idx == connections[j].goal_region_idx {
                            if connections[i].apitems.is_subset(&connections[j].apitems) {
                                self.regions[region_idx].connections.remove(j);
                                removed = true;
                                reduce_connections_count += 1;
                                break;
                            } else if connections[j].apitems.is_subset(&connections[i].apitems) {
                                self.regions[region_idx].connections.remove(i);
                                removed = true;
                                reduce_connections_count += 1;
                                break;
                            }
                        }
                    }
                    if removed {
                        break;
                    }
                }
            }
        }

        if reduce_connections_count > 0 {
            changed.push_str(&format!("Reduced {} duplicate connections\n", reduce_connections_count));
        }

        changed
    }

    fn simplify_subrootnodes(&mut self) -> String {
        let mut changed = String::new();

        // Find sub root nodes and remove useless connections to them
        for region_idx in 0..self.regions.len() {
            if !self.region_map.contains_key(&self.regions[region_idx].name) {
                continue;
            }
            if self.regions[region_idx].location {
                continue;
            }
            if self.regions[region_idx].name == "Menu" {
                continue;
            }

            let reachable_regions = self.regions[region_idx].get_reachable_regions(self);

            // Find the rootnode
            let mut rootnode: Option<usize> = None;
            for &reachable_idx in &reachable_regions {
                let mut is_root = false;
                for &parent_idx in &self.regions[reachable_idx].parents {
                    if !reachable_regions.contains(&parent_idx) {
                        is_root = true;
                        break;
                    }
                }

                if is_root {
                    if rootnode.is_some() {
                        rootnode = None;
                        break;
                    }
                    rootnode = Some(reachable_idx);
                }
            }

            if rootnode.is_none() {
                continue;
            }

            let rootnode_idx = rootnode.unwrap();
            let mut removed = 0;

            let parent_indices = self.regions[rootnode_idx].parents.clone();
            for parent_idx in parent_indices {
                if !reachable_regions.contains(&parent_idx) {
                    continue;
                }

                let connections: Vec<Connection> = self.regions[parent_idx]
                    .connections
                    .iter()
                    .filter(|c| c.goal_region_idx == rootnode_idx)
                    .cloned()
                    .collect();

                for _ in &connections {
                    self.regions[parent_idx].remove_connections(rootnode_idx);
                    removed += 1;
                }

                self.regions[rootnode_idx].parents.retain(|&p| p != parent_idx);
            }

            if removed > 0 {
                changed.push_str(&format!("Removed {} connections to subroot {}\n", removed, self.regions[rootnode_idx].name));
                break;
            }
        }

        changed
    }

    fn simplify_tryremoveregion(&mut self) -> String {
        let mut changed = String::new();

        // Remove redundant regions
        let mut original_region_count = self.count();
        let region_indices: Vec<usize> = (0..self.regions.len()).collect();

        for &region_idx in &region_indices {
            if !self.region_map.contains_key(&self.regions[region_idx].name) {
                continue;
            }
            if self.regions[region_idx].location {
                continue;
            }
            if self.regions[region_idx].name == "Menu" {
                continue;
            }

            let all_non_location = self.regions[region_idx].connections.iter().all(|c| !self.regions[c.goal_region_idx].location);
            
            if all_non_location {
                let menu_idx = self.get_region("Menu");
                if let Some(menu_idx) = menu_idx {
                    let reachable = self.regions[menu_idx].get_reachable_regions_ignore(self, Some(region_idx));
                    if reachable.len() + 1 == original_region_count {
                        let region_name = self.regions[region_idx].name.clone();
                        original_region_count -= 1; // we removed one region, so of course next time we expect one less
                        self.remove_region(region_idx);
                        changed.push_str(&format!("Removed {} as it is redundant\n", region_name));
                    }
                }
            }
        }
        if !changed.is_empty() {
            self.reindex();
        }

        changed
    }

    fn simplify_tryremoveconnections(&mut self) -> String {
        let mut changed = String::new();

        // Remove unnecessary connections
        let original_region_count = self.count();
        let region_indices: Vec<usize> = (0..self.regions.len()).collect();

        for &region_idx in &region_indices {
            if !self.region_map.contains_key(&self.regions[region_idx].name) {
                continue;
            }

            let connections = self.regions[region_idx].connections.clone();
            for conn in connections {
                if self.regions[conn.goal_region_idx].location {
                    continue;
                }
                if !conn.apitems.apitems.is_empty() {
                    continue;
                }

                // Remove connection and test reachability
                self.regions[region_idx].remove_connections(conn.goal_region_idx);
                
                let menu_idx = self.get_region("Menu");
                if let Some(menu_idx) = menu_idx {
                    let reachable = self.regions[menu_idx].get_reachable_regions(self);
                    
                    if reachable.len() != original_region_count {
                        // Restore connection
                        self.regions[region_idx].connections.push(conn.clone());
                    } else {
                        // Remove from child's parent list if no other connections
                        let has_other_conns = self.regions[region_idx].connections.iter().any(|c| c.goal_region_idx == conn.goal_region_idx);
                        if !has_other_conns {
                            self.regions[conn.goal_region_idx].parents.retain(|&p| p != region_idx);
                        }
                        changed.push_str(&format!("Removed unnecessary connection from {} to {}\n", 
                            self.regions[region_idx].name, self.regions[conn.goal_region_idx].name));
                    }
                }
            }
        }

        changed
    }

    fn simplify_4(&mut self) -> String {
        let mut changed = String::new();

        let region_indices: Vec<usize> = (0..self.regions.len()).collect();

        for &region_idx in &region_indices {
            if !self.region_map.contains_key(&self.regions[region_idx].name) {
                continue;
            }

            let connections = self.regions[region_idx].connections.clone();
            for conn in connections {
                if !self.regions[conn.goal_region_idx].parents.contains(&region_idx) {
                    continue;
                }

                let connections_to_child = self.regions[conn.goal_region_idx].get_connections(region_idx);
                if connections_to_child.len() != 1 {
                    continue;
                }

                let connections_from_child = self.regions[region_idx].get_connections(conn.goal_region_idx);
                if connections_from_child.len() != 1 {
                    continue;
                }

                let connection_to_child = connections_to_child[0];
                let connection_from_child = connections_from_child[0];

                if connection_to_child.apitems.apitems.is_empty() && !connection_from_child.apitems.apitems.is_empty() {
                    let child_idx = conn.goal_region_idx;
                    let region_name = self.regions[region_idx].name.clone();
                    let child_name = self.regions[child_idx].name.clone();
                    
                    self.regions[region_idx].remove_connections(child_idx);
                    self.regions[child_idx].parents.retain(|&p| p != region_idx);
                    changed.push_str(&format!("Removed useless connection from {} to {}\n", region_name, child_name));
                    break;
                } else if connection_from_child.apitems.apitems.is_empty() && !connection_to_child.apitems.apitems.is_empty() {
                    let child_idx = conn.goal_region_idx;
                    let region_name = self.regions[region_idx].name.clone();
                    let child_name = self.regions[child_idx].name.clone();
                    
                    self.regions[child_idx].remove_connections(region_idx);
                    self.regions[region_idx].parents.retain(|&p| p != child_idx);
                    changed.push_str(&format!("Removed useless connection from {} to {}\n", child_name, region_name));
                    break;
                }
            }
        }
        if !changed.is_empty() {
            self.reindex();
        }

        changed
    }

    fn reindex(&mut self) {
        let mut new_regions: Vec<Region> = Vec::new();
        let mut new_region_map: HashMap<String, usize> = HashMap::new();
        let mut index_map: HashMap<usize, usize> = HashMap::new();

        for region_idx in 0..self.regions.len() {
            if !self.region_map.contains_key(&self.regions[region_idx].name) {
                continue;
            }
            let new_idx = new_regions.len();
            index_map.insert(region_idx, new_idx);
            new_region_map.insert(self.regions[region_idx].name.clone(), new_idx);
            new_regions.push(self.regions[region_idx].clone());
        }

        // Update connections and parents to new indices
        for region in &mut new_regions {
            for conn in &mut region.connections {
                if let Some(&new_idx) = index_map.get(&conn.goal_region_idx) {
                    conn.goal_region_idx = new_idx;
                }
            }
            region.parents = region.parents.iter()
                .filter_map(|&p| index_map.get(&p).copied())
                .collect();
        }

        self.regions = new_regions;
        self.region_map = new_region_map;
    }
}

fn build_graph(item_locs: &Vec<usize>, base_regions: &Vec<BaseRegion>) {
    // Build the Reventure graph
    println!("Building Reventure graph...");

    let mut graph = ReventureGraph::new(LONKS_HOUSE);
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

    // Remove duplicate solutions
    graph.remove_duplicate_solutions();
    println!("Duplicate solutions removed!");
    // plantuml::save_plant_uml(&graph, &format!("graphs/ChangeHistory{}-Level{}", -1, 0));

    println!("Simplifying Graph");

    graph.reindex();
    graph.detect_errors("before simplification");
    let mut changes = 0;
    let mut level = 0;
    while level < 7 {
        println!("Step {}, Simplification Level {}, Graph Size {}", changes, level, graph.count());
        let change = graph.simplify(level);
        if change.is_empty() {
            level += 1;
        } else {
            // graph.detect_errors(format!("Step {}, Simplification Level {}", changes, level).as_str());
            // plantuml::save_plant_uml(&graph, &format!("graphs/ChangeHistory{}-Level{}", changes, level));
            changes += 1;
            level = 0;
        }
    }
    
    println!("Final graph has {} regions!", graph.count());
    let plantuml_filepath = "reventure_graph_rust.plantuml";
    println!("Saving PlantUML graph to {}...", plantuml_filepath);
    plantuml::save_plant_uml(&graph, plantuml_filepath);
    println!("PlantUML graph saved!");

    save_region_graph(&graph, "output.reg");
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
    build_graph(&item_locs, &base_regions);
}