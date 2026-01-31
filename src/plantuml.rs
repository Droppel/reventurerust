use crate::ReventureGraph;

fn create_plant_uml(graph: &ReventureGraph) -> String {
    let mut plant_uml = String::new();
    plant_uml.push_str("@startuml\n");
    plant_uml.push_str("hide circle\n");

    for  region in graph.regions.iter() {
        plant_uml.push_str(&format!("class \"{}\"\n", region.name));
        for connection in &region.connections {
            plant_uml.push_str(&format!(
                "\"{}\" --> \"{}\"",
                region.name, graph.regions[connection.goal_region_idx].name
            ));
            let mut highest_jump = 0;
            let mut highest_jump_item = "".to_string();
            let mut reqitems = connection.apitems.apitems.clone();
            for item in &connection.apitems.apitems {
                if !item.starts_with("Jump") {
                    continue;
                }
                let jump = item.split("_").collect::<Vec<&str>>()[1];
                let jump = jump.parse::<u32>().unwrap();
                if jump > highest_jump {
                    highest_jump = jump;
                    if highest_jump_item.is_empty() {
                        reqitems.retain(|x| x != &highest_jump_item);
                        highest_jump_item = item.clone();
                    } else {
                        reqitems.retain(|x| x != item);
                    }
                }
            }

            let conn_string = Vec::from_iter(reqitems.iter().map(|x| x.as_str())).join(", ");
            if !conn_string.is_empty() {
                plant_uml.push_str(&format!(" : {}", conn_string));
            }
            plant_uml.push_str("\n");
        }
    }

    plant_uml.push_str("@enduml\n");
    return plant_uml;
}

pub fn save_plant_uml(graph: &ReventureGraph, filepath: &str) {
    let plant_uml = create_plant_uml(graph);
    std::fs::write(filepath, plant_uml).expect("Unable to write file");
}