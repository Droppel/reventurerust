import copy
import typing
import time
import random
# import cProfile

totaljumpincrease = 2
startjump = 4
# totaljumpincrease = 4
# startjump = 2

class APItems:
    def __init__(self):
        self.apitems: typing.Set[str] = set()
    
    def add_apitem(self, item: str):
        if not item in self.apitems:
            self.apitems.add(item)
            return True
        return False
    
    def remove_apitem(self, item: str):
        if item in self.apitems:
            self.apitems.remove(item)
    
    def add_apitems(self, items: typing.List[str]):
        added = []
        for item in items:
            if self.add_apitem(item):
                added.append(item)
        return added

    def remove_apitems(self, items: typing.List[str]):
        self.apitems.difference_update(items)
        # self.apitems = [item for item in self.apitems if not item in items]

    def is_subset(self, other: "APItems"):
        return self.apitems.issubset(other.apitems)
        # return all(item in other.apitems for item in self.apitems)
    
    def is_strict_subset(self, other: "APItems"):
        return self.is_subset(other) and not len(self.apitems) == len(other.apitems)
    
    def add_apitems_from_string(self, items: str):
        if items == "":
            return
        self.add_apitems(items.split(", "))

    def to_string(self):
        return ", ".join(sorted(self.apitems))
        
class APState:
    def __init__(self):
        self.potapitems: typing.List[APItems] = []
        self.reducedstates: typing.Set[str] = set()

    def is_rejected(self, apitems: APItems):
        return apitems.to_string() in self.reducedstates

    def add_apitems(self, apitems: APItems):
        self.potapitems.append(apitems)

    # def reduce_one(self):
    #     for i in range(len(self.potapitems)):
    #         for j in range(i+1, len(self.potapitems)):
    #             if self.potapitems[i].is_subset(self.potapitems[j]):
    #                 self.reducedstates.add(self.potapitems.pop(j).to_string())
    #                 return True
    #             if self.potapitems[j].is_subset(self.potapitems[i]):
    #                 self.reducedstates.add(self.potapitems.pop(i).to_string())
    #                 return True
    #     return False
    
    # def reduce_all(self):
    #     changed = False
    #     while self.reduce_one():
    #         changed = True
    #     return changed
    
    def reduce_all(self):
        new_potapitems: typing.List[APItems] = []
        self.potapitems = list(sorted(self.potapitems, key=lambda x: len(x.apitems)))
        for potapitems in self.potapitems:
            if any(potapitems.apitems.issuperset(used.apitems) for used in new_potapitems):
                self.reducedstates.add(potapitems.to_string())
            else:
                new_potapitems.append(potapitems)
        self.potapitems = new_potapitems
        return len(new_potapitems) != len(self.potapitems)

        # removed = False
        # for i in range(len(self.potapitems)-2, -1, -1):
        #     for j in range(len(self.potapitems)-1, i, -1):
        #         removedI = False
        #         # if self.potapitems[i].is_subset(self.potapitems[j]):
        #         # Inlined for performance
        #         if self.potapitems[i].apitems.issubset(self.potapitems[j].apitems):
        #             self.reducedstates.add(self.potapitems.pop(j).to_string())
        #             removed = True
        #             continue
        #         # if self.potapitems[j].is_subset(self.potapitems[i]):
        #         # Inlined for performance
        #         if self.potapitems[j].apitems.issubset(self.potapitems[i].apitems):
        #             self.reducedstates.add(self.potapitems.pop(i).to_string())
        #             removed = True
        #             removedI = True
        #         if removedI:
        #             break
        return removed


class ReventureState:
    def __init__(self):
        self.state = {}
        # Items
        self.state["has_chicken"] = False
        self.state["has_shovel"] = False
        self.state["has_sword"] = False
        self.state["has_swordelder"] = False
        self.state["has_shield"] = False
        self.state["has_map"] = False
        self.state["has_compass"] = False
        self.state["has_mrhugs"] = False
        self.state["has_lavaTrinket"] = False
        self.state["has_hook"] = False
        self.state["has_princess"] = False
        self.state["has_bomb"] = False
        self.state["has_nuke"] = False
        self.state["has_whistle"] = False
        self.state["has_darkstone"] = False
        self.state["has_burger"] = False
        self.state["sacrificecount"] = 0

        # Events
        self.state["castleBridgeDown"] = False
        self.state["fortressBridgeDown"] = False
        pass

    def copy(self):
        new_state = ReventureState()
        new_state.state = self.state.copy()
        return new_state

    def event(self, event: str):
        return self.state[event]
        
    def get_weight(self):
        weight = 0
        if self.state["has_shovel"]:
            weight += 0.5
        if self.state["has_sword"]:
            weight += 0.5
        if self.state["has_swordelder"]:
            weight += 0.5
        if self.state["has_chicken"]:
            weight += 0.5
        if self.state["has_shield"]:
            weight += 0.5
        if self.state["has_lavaTrinket"]:
            weight += 0.5
        if self.state["has_hook"]:
            weight += 0.5
        if self.state["has_bomb"]:
            weight += 0.5
        if self.state["has_nuke"]:
            weight += 0.5
        if self.state["has_whistle"]:
            weight += 0.5
        if self.state["has_darkstone"]:
            weight += 0.5
        if self.state["has_burger"]:
            weight += 0.5
        return weight

    def get_jump(self, jump_increases: int):
        jump = startjump + jump_increases * 0.5
        return jump - self.get_weight()

CollectionRule = typing.Callable[[ReventureState], bool]
class BaseRegion:
    def __init__(self, name: str):
        self.name = name
        self.connections: typing.List[BaseConnection] = []
        self.jumpconnections: typing.List[JumpConnection] = []
        self.statechange: typing.List[StateChange] = []
        self.locations: typing.List[BaseConnection] = []

    def add_jumpconnection(self, jumpconnection):
        self.jumpconnections.append(jumpconnection)

    def add_connection(self, connection):
        self.connections.append(connection)
    
    def add_statechange(self, statechange):
        self.statechange.append(statechange)

    def add_location(self, location):
        self.locations.append(location)

class BaseConnection:
    def __init__(self, goal_region: BaseRegion, rule: CollectionRule, apitems: typing.List[str] = []):
        self.goal_region = goal_region
        self.rule = rule
        self.apitems = apitems
    
    def can_use(self, state: ReventureState):
        return self.rule(state)

class JumpConnection(BaseConnection):
    def __init__(self, goal_region: BaseRegion, rule: CollectionRule, apitems: typing.List[str] = [], jump_req: float = 0):
        super().__init__(goal_region, rule, apitems)
        self.jump_req = jump_req

    def get_jumpitems_req(self, state: ReventureState):
        weight = state.get_weight()
        return int((self.jump_req + weight - startjump) * 2)

class StateChange:
    def __init__(self, states: typing.List[str], values: typing.List[bool], rule: CollectionRule, apitems: typing.List[str] = []):
        self.rule = rule
        self.apitems = apitems
        self.states = states
        self.values = values
    
    def can_use(self, state: ReventureState):
        return self.rule(state)

class Connection:
    # name equals the name of the goal region
    def __init__(self, child: "Region", apitems: typing.List[str] = []):
        self.region = child
        self.apitems = APItems()
        self.apitems.add_apitems(apitems)

def get_region_name(base_regions: BaseRegion, state: ReventureState):
        name = "_".join([br.name for br in base_regions])
        for event in state.state.keys():
            if state.state[event]:
                name += f"__{event}"
        return name
class Region:
    def __init__(self, base_region: BaseRegion, state: ReventureState, location: bool = False):
        self.base_regions = [base_region]
        self.state = state
        self.location = location
        self.name = get_region_name(self.base_regions, state)

        self.connections: typing.List[Connection] = []
        self.parents: typing.List[Region] = []
        self.apstate: APState = APState()
        self.complexity = 0
        for event in state.state.keys():
            if state.state[event]:
                self.complexity += 1
        
    def add_connection(self, new_connection: Connection):
        exists = False
        if new_connection.region == self:
            return
        for con in self.connections:
            if con.region == new_connection.region and con.apitems.is_subset(new_connection.apitems):
                exists = True
                break
        if exists:
            return
        self.connections.append(new_connection)
        new_connection.region.add_parent(self)

    def get_connections(self, region: "Region"):
        ret: typing.List[Connection]= []
        for conn in self.connections:
            if conn.region == region:
                ret.append(conn)
        return ret
    
    def remove_connections(self, region: "Region"):
        removed = False
        for conn in copy.copy(self.connections):
            if conn.region == region:
                self.connections.remove(conn)
                removed = True
        return removed

    def add_parent(self, parent: "Region"):
        if parent in self.parents:
            return
        if parent == self:
            return
        self.parents.append(parent)

    def merge(self, donor_region: "Region"):
        for parent in copy.copy(donor_region.parents):
            if parent == self:
                continue
            for conn in parent.get_connections(donor_region):
                parent.add_connection(Connection(self, conn.apitems.apitems))
        for connection in donor_region.connections:
            self.add_connection(connection)

    def get_reachable_regions(self):
        reachable_regions = set()
        todo_regions: typing.Set[Region] = {self}
        while len(todo_regions) > 0:
            current_region: Region = todo_regions.pop()
            reachable_regions.add(current_region)
            for conn in current_region.connections:
                if not conn.region in reachable_regions and not conn.region in todo_regions:
                    todo_regions.add(conn.region)
        return reachable_regions

    def get_reachable_regions_ignore(self, ignore = None):
        reachable_regions = set()
        todo_regions: typing.Set[Region] = {self}
        while len(todo_regions) > 0:
            current_region: Region = todo_regions.pop()
            if current_region == ignore:
                continue
            reachable_regions.add(current_region)
            for conn in current_region.connections:
                if not conn.region in reachable_regions and not conn.region in todo_regions:
                    todo_regions.add(conn.region)
        return reachable_regions

class ReventureGraph:
    def __init__(self):
        self.regiondict: typing.Dict[str, Region] = {}
        self.start_region: Region = None
        self.item_locations: typing.List[Region] = []

    def add_region(self, region: Region):
        self.regiondict[region.name] = region

    def remove_region(self, region: Region):
        for parent in region.parents:
            parent.remove_connections(region)
        removedChildren = set()
        for connection in region.connections:
            if connection.region in removedChildren:
                continue
            removedChildren.add(connection.region)
            connection.region.parents.remove(region)
        self.regiondict.pop(region.name)

    def count(self):
        return len(self.regiondict.keys())

    def get_region(self, name: str):
        return self.regiondict.get(name, None)
    
    def detect_errors(self):
        return
        for region in self.regiondict.values():
            if region.name == "Menu":
                continue
            error = ""
            for connection in region.connections:
                if not connection.region.name in self.regiondict:
                    error = f"Zombie connection: {region.name} -> {connection.region.name}"
                if not region in connection.region.parents:
                    error = f"Broken connection: {region.name} -> {connection.region.name}"
            for parent in region.parents:
                if parent == region:
                    error = f"Self parent: {region.name}"
                if not parent.name in self.regiondict:
                    error = f"Zombie parent: {region.name} <- {parent.name}"
            for i in range(len(region.parents)):
                for j in range(i+1, len(region.parents)):
                    if region.parents[i] == region.parents[j]:
                        error = f"Duplicate parent: {region.name} <- {region.parents[i].name}"
            if error != "":
                import traceback
                for line in traceback.format_stack():
                    print(line.strip())
                print(error)                
                import sys
                sys.exit(1)

    def simplify(self, cleanuplevel: int = 0):
        changed = ""
        if cleanuplevel == 0:
            return self.simplify_0()
        if cleanuplevel == 1:
            return self.simplify_1()
        if cleanuplevel == 2:
            return self.simplify_2()
        if cleanuplevel == 3:
            return self.simplify_3()
        if cleanuplevel == 4:
            return self.simplyfy_4()
        return changed

    def simplify_0(self):
        changed = ""
        simpleRemove: typing.List[Region] = []
        oneways: typing.List[Region] = []

        self.detect_errors()
        # Check for needless regions. Never remove Menu or Regions marked as locations
        for region in self.regiondict.values():
            if region.name == "Menu":
                continue
            if region.location:
                continue
            # Remove any regions that have no parents
            if len(region.parents) == 0:
                simpleRemove.append(region)
                continue

            # Remove any regions without connections
            if len(region.connections) == 0:
                simpleRemove.append(region)
                continue

            # Remove regions only connected to a single parent
            remove = True
            parent = region.parents[0]
            for other_parent in region.parents[1:]:
                if other_parent != parent:
                    remove = False
                    break
            if remove:
                for connection in region.connections:
                    if not connection.region == parent:
                        remove = False
                        break
                if remove:
                    simpleRemove.append(region)
                    continue
            
            # One way regions. Only remove if the total amount of apitems is less than 3
            if len(region.parents) == 1 and len(region.connections) == 1:
                connections_from_parent = region.parents[0].get_connections(region)
                if len(connections_from_parent) != 1 and len(region.connections[0].apitems.apitems) + len(connections_from_parent[0].apitems.apitems) > 2:
                    continue
                oneways.append(region)
                continue
        
        for region in simpleRemove:
            self.remove_region(region)
        
        self.detect_errors()

        for region in oneways:
            if len(region.parents) != 1:
                continue
            if len(region.connections) != 1:
                continue
            parent = region.parents[0]
            connection_to_child = region.connections[0]
            connections_from_parent = parent.get_connections(region)
            for conn in connections_from_parent:
                connection_to_child.apitems.add_apitems(conn.apitems.apitems)
                parent.add_connection(connection_to_child)
            self.remove_region(region)
        
        self.detect_errors()

        # Merge regions
        mergecount = 0
        for region in copy.copy(list(self.regiondict.values())):
            if region.name == "Menu":
                continue
            # Free movement between two regions => merge
            for parent in region.parents:
                if not parent in [con.region for con in region.connections]:
                    continue
                if len(region.get_connections(parent)) != 1 or len(parent.get_connections(region)) != 1:
                    continue # Only merge if there is a single connection between the two regions
                if len(region.get_connections(parent)[0].apitems.apitems) != 0 or len(parent.get_connections(region)[0].apitems.apitems) != 0:
                    continue
                # print(f"Merging {region.name} into {parent.name}")
                parent.merge(region)
                self.remove_region(region)
                mergecount += 1
                # self.detect_errors(f"In Merge {region.name}, {parent.name}")
                break
        if mergecount > 0:
            changed += f"Merged {mergecount} regions\n"
        
        self.detect_errors()

        if len(simpleRemove) + len(oneways) > 0:
            changed += f"Removed {len(simpleRemove)} simple regions and {len(oneways)} oneways\n"
            simpleRemove = []
            oneways = []
        self.detect_errors()
        return changed

    def simplify_1(self):
        changed = ""
        complexloop_count = 0
        for region in self.regiondict.values():
            if region.name == "Menu":
                continue
            if region.location:
                continue
            todo_regions = {region}
            reachable_regions = []
            reachable_location = False
            while len(todo_regions) > 0 and not reachable_location:
                current_region = todo_regions.pop()
                reachable_regions.append(current_region)
                for connection in current_region.connections:
                    if connection.region.location:
                        reachable_location = True
                        break
                    if not connection.region in reachable_regions and not connection.region in todo_regions:
                        todo_regions.add(connection.region)
            if not reachable_location:
                for r in reachable_regions:
                    self.remove_region(r)
                complexloop_count += len(reachable_regions)
                break
        if complexloop_count > 0:
            changed += f"Removed {complexloop_count} complex loop regions\n"
        
        # Reduce duplicate connections
        def reduce_connections(region: Region):
            for i in range(len(region.connections)):
                for j in range(i+1, len(region.connections)):
                    if region.connections[i].region == region.connections[j].region:
                        if region.connections[i].apitems.is_subset(region.connections[j].apitems):
                            region.connections.pop(j)
                            return True
                        if region.connections[j].apitems.is_subset(region.connections[i].apitems):
                            region.connections.pop(i)
                            return True
            return False

        reduce_connections_count = 0
        for region in self.regiondict.values():
            if reduce_connections(region):
                reduce_connections_count += 1
        if reduce_connections_count > 0:
            changed += f"Reduced {reduce_connections_count} duplicate connections\n"
        self.detect_errors()
        return changed

    def simplify_2(self):
        changed = ""
        # Find sub root nodes and remove all useless connections to them
        for region in self.regiondict.values():
            if region.name == "Menu":
                continue
            if region.location:
                continue
            # Find single interconnected region that cannot reach the rest of the graph
            reachable_regions = region.get_reachable_regions()

            # Find the entrypoint from the graph
            rootnode: Region = None
            for reachable_region in reachable_regions:
                isroot = False
                for parent in reachable_region.parents:
                    if not parent in reachable_regions:
                        isroot = True
                        break
                if isroot:
                    if rootnode != None: # Multiple roots
                        rootnode = None
                        break
                    rootnode = reachable_region
            if rootnode == None:
                continue

            removed = 0
            for parent in copy.copy(rootnode.parents):
                if not parent in reachable_regions:
                    continue
                removedRoot = False
                for conn in copy.copy(parent.connections):
                    if conn.region == rootnode:
                        removed += 1
                        parent.connections.remove(conn)
                        if not removedRoot:
                            rootnode.parents.remove(parent)
                            removedRoot = True
            if removed > 0:
                changed += f"Removed {removed} connections to subroot {rootnode.name}\n"
                break
        self.detect_errors()
        return changed

    def simplify_3(self):
        changed = ""
        # Remove regions that add nothing of value
        # We do this by checking if a regions is reachable from one of its children
        original_region_count = self.count()
        for region in copy.copy(list(self.regiondict.values())):
            if region.name == "Menu":
                continue
            if region.location:
                continue
            if all([not conn.region.location for conn in region.connections]) and len(self.get_region("Menu").get_reachable_regions_ignore(ignore=region)) + 1 == original_region_count:
                self.remove_region(region)
                changed += f"Removed {region.name} as it is redundant\n"

        # Remove unnecessary connections
        original_region_count = self.count()
        for region in self.regiondict.values():
            for connection in copy.copy(region.connections):
                if connection.region.location:
                    continue
                if len(connection.apitems.apitems) > 0:
                    continue
                region.connections.remove(connection)
                reachable_regions = self.get_region("Menu").get_reachable_regions()
                if len(reachable_regions) != original_region_count:
                    region.connections.append(connection)
                else:
                    if not connection.region in [conn.region for conn in region.connections]:
                        connection.region.parents.remove(region)
                    changed += f"Removed unnecessary connection from {region.name} to {connection.region.name}\n"
        self.detect_errors()
        return changed
        
    def simplyfy_4(self):
        # NOTE: This step is not delayed because it is very expensive. It is only delayed because it uses assumptions
        # that require the previous steps to be finished
        # Remove bidirectional connections where one has no items (A) and one does have apitems (B).
        # The reason this works is based on a few facts:
        #  1. (A) is *strictly* necessary. Otherwise it would have been removed in step 3
        #  2. (A) in a sense A is thus the "parent". If this were not the case it could have been removed.
        #  3. It follows, that (B) is the child and a connection from a child to a parent is useless no matter the apitems it uses
        changed = ""
        for region in self.regiondict.values():
            for connection in copy.copy(region.connections):
                if not (region in connection.region.parents):
                    continue
                connectionstochild = connection.region.get_connections(region)
                if len(connectionstochild) != 1:
                    continue
                connectionsfromchild = region.get_connections(connection.region)
                if len(connectionsfromchild) != 1:
                    continue
                connection_to_child = connectionstochild[0]
                connection_from_child = connectionsfromchild[0]
                if len(connection_to_child.apitems.apitems) == 0 and len(connection_from_child.apitems.apitems) > 0:
                    region.remove_connections(connection.region)
                    connection.region.parents.remove(region)
                    changed += f"Removed useless connection from {region.name} to {connection.region.name}\n"
                    break
                elif len(connection_from_child.apitems.apitems) == 0 and len(connection_to_child.apitems.apitems) > 0:
                    connection.region.remove_connections(region)
                    region.parents.remove(connection.region)
                    changed += f"Removed useless connection from {connection.region.name} to {region.name}\n"
                    break
        self.detect_errors()
        return changed

    def propagate_apstates(self):
        parent_todo_regions = copy.copy(list(self.regiondict.values()))
        parent_todo_regionsDict = {region.name: region for region in parent_todo_regions}
        while len(parent_todo_regions) > 0:
            # print(f"Apstate regions left: {len(parent_todo_regions)}")
            region = parent_todo_regions.pop(0)
            del parent_todo_regionsDict[region.name]
            for connection in region.connections:
                child = connection.region
                prevStateLen = len(child.apstate.potapitems)
                prevStateLengths = [len(child.apstate.potapitems[i].apitems) for i in range(prevStateLen)]
                added = False
                if len(connection.apitems.apitems) > 0:
                    for potapitems in region.apstate.potapitems:
                        addeditems = potapitems.add_apitems(connection.apitems.apitems)
                        if child.apstate.is_rejected(potapitems):
                            potapitems.remove_apitems(addeditems)
                            continue
                        added = True
                        new_potapitems = copy.deepcopy(potapitems)
                        potapitems.remove_apitems(addeditems)
                        child.apstate.potapitems.append(new_potapitems)
                else:
                    for potapitems in region.apstate.potapitems:
                        if child.apstate.is_rejected(potapitems):
                            continue
                        added = True
                        child.apstate.potapitems.append(potapitems)
                if not added:
                    continue
                child.apstate.reduce_all()
                if child.name in parent_todo_regionsDict:
                    continue
                if prevStateLen != len(child.apstate.potapitems):
                    parent_todo_regions.append(child)
                    parent_todo_regionsDict[child.name] = child
                    continue
                change = any(
                    len(child.apstate.potapitems[i].apitems) != prevStateLengths[i]
                    for i in range(len(child.apstate.potapitems))
                )
                if change:
                    parent_todo_regions.append(child)
                    parent_todo_regionsDict[child.name] = child
                    continue

def get_region_in_list(name: str, region_list: typing.List[Region]):
    for reg in region_list:
        if reg.name == name:
            return reg
    return None
    
def create_plant_uml(regions: typing.List[Region]):
    plant_uml = "@startuml\nhide circle\n"
    for region in regions:
        plant_uml += f"class \"{region.name}\"\n"
        for connection in region.connections:
            plant_uml += f"\"{region.name}\" --> \"{connection.region.name}\""
            highestjump = 0
            highestjumpitem = None
            reqitems = copy.copy(connection.apitems.apitems)
            for item in connection.apitems.apitems:
                if not item.startswith("Jump"):
                    continue
                jump = int(item.split("_")[1])
                if jump > highestjump:
                    highestjump = jump
                    if highestjumpitem != None:
                        reqitems.remove(highestjumpitem)
                    highestjumpitem = item
                else:
                    reqitems.remove(item)

            conn_string = ", ".join(reqitems)
            if conn_string != "":
                plant_uml += f" : {conn_string}"
            plant_uml += "\n"
    plant_uml += "@enduml"
    return plant_uml


def write_plantuml(region_graph: ReventureGraph, file_name="reventure_graph.plantuml"):
    plantuml = create_plant_uml(region_graph.regiondict.values())
    with open(file_name, "w") as file:
        file.write(plantuml)

class ItemPlacement():
    def __init__(self, apitem: str, state: str, exclstates: typing.List[str] = None):
        self.apitem = apitem
        self.state = state
        if exclstates == None:
            self.exclstates = [state]
        else:
            self.exclstates = exclstates

def create_region_graph():
    # Create Location Regions
    loc01 = BaseRegion("01: It\'s Dangerous to be Near Tim")
    loc02 = BaseRegion("02: Shit Happens")
    loc03 = BaseRegion("03: Please Nerf This")
    loc04 = BaseRegion("04: Public Enemy")
    loc05 = BaseRegion("05: Kingslayer")
    loc06 = BaseRegion("06: The Floor is Lava")
    loc07 = BaseRegion("07: Go Swimming")
    loc08 = BaseRegion("08: Roll & Rock")
    loc09 = BaseRegion("09: Customer is Always Right")
    loc10 = BaseRegion("10: Gold Rush")
    loc11 = BaseRegion("11: Feline Company")
    loc12 = BaseRegion("12: Hobbies")
    loc13 = BaseRegion("13: Allergic to Cuteness")
    loc14 = BaseRegion("14: Dracar-ish")
    loc15 = BaseRegion("15: Family Gathering")
    loc16 = BaseRegion("16: Monster Hunter")
    loc17 = BaseRegion("17: Public Transport Next Time")
    loc18 = BaseRegion("18: King of Hearts")
    loc19 = BaseRegion("19: Broken Heart")
    loc20 = BaseRegion("20: Day Off")
    loc21 = BaseRegion("21: You Nailed It")
    loc22 = BaseRegion("22: Paperweight")
    loc23 = BaseRegion("23: True Beauty is inside")
    loc24 = BaseRegion("24: Strawberry")
    loc25 = BaseRegion("25: Bully")
    # loc26 = BaseRegion("26: Greedy Bastard") # Handled Extra
    loc27 = BaseRegion("27: Airstrike")
    loc28 = BaseRegion("28: Don\'t Try This at Home")
    loc29 = BaseRegion("29: The Man in the Steel Mask")
    loc30 = BaseRegion("30: Subliminal Message")
    loc31 = BaseRegion("31: Collateral Damage")
    loc32 = BaseRegion("32: You Monster")
    loc33 = BaseRegion("33: Leap of Faith")
    loc34 = BaseRegion("34: -1st Floor")
    loc35 = BaseRegion("35: Wastewater")
    loc36 = BaseRegion("36: Fireproof")
    loc37 = BaseRegion("37: Free Hugs")
    loc38 = BaseRegion("38: Oh Boy, I\'m so Hungry")
    loc39 = BaseRegion("39: Everything is Terrible")
    loc40 = BaseRegion("40: Sexy Beard")
    loc41 = BaseRegion("41: Post-Traumatic Stress Disorder")
    loc42 = BaseRegion("42: Sneaky Bastard")
    loc43 = BaseRegion("43: Dinner for Two")
    loc44 = BaseRegion("44: Bad Leverage")
    loc45 = BaseRegion("45: Well Excuuuuse Me, Princess")
    loc46 = BaseRegion("46: Extreme Sports")
    loc47 = BaseRegion("47: Harakiri")
    loc48 = BaseRegion("48: It\'s my First Day")
    loc49 = BaseRegion("49: Victory Royale")
    loc50 = BaseRegion("50: P0wned")
    loc51 = BaseRegion("51: Politics")
    loc52 = BaseRegion("52: I\'m Feeling Lucky")
    loc53 = BaseRegion("53: Videogames")
    loc54 = BaseRegion("54: Paraphilia")
    loc55 = BaseRegion("55: Escape Shortcut")
    loc56 = BaseRegion("56: Refund Request")
    loc57 = BaseRegion("57: Friendzoned")
    loc58 = BaseRegion("58: Dark Extreme Sports")
    loc59 = BaseRegion("59: Away From Kingdom")
    loc60 = BaseRegion("60: Viva La Resistance")
    loc61 = BaseRegion("61: Syndicalism")
    loc62 = BaseRegion("62: Jackpot")
    loc63 = BaseRegion("63: You Don\'t Mess With Chicken")
    loc64 = BaseRegion("64: I Thought It Was A Mimic")
    loc65 = BaseRegion("65: Overheal")
    loc66 = BaseRegion("66: Finite War")
    loc67 = BaseRegion("67: Stay Determined")
    loc68 = BaseRegion("68: Otaku Fever")
    loc69 = BaseRegion("69: Quick and Dirty")
    loc70 = BaseRegion("70: It\'s a Trap")
    # loc71 = BaseRegion("71: Sustainable Development") # Handled Extra
    loc72 = BaseRegion("72: Ecologist")
    loc73 = BaseRegion("73: Dark Love")
    loc74 = BaseRegion("74: Bittersweet Revenge")
    loc75 = BaseRegion("75: Please, Not Again")
    loc76 = BaseRegion("76: A Waifu is You")
    loc77 = BaseRegion("77: Battle Royale")
    loc78 = BaseRegion("78: Silver or Lead")
    loc79 = BaseRegion("79: Good Ending")
    loc80 = BaseRegion("80: Chicken of Doom")
    loc81 = BaseRegion("81: Forever Together")
    loc82 = BaseRegion("82: Perfect Crime")
    loc83 = BaseRegion("83: We Have to Go Back")
    loc84 = BaseRegion("84: Not what you Expected")
    loc85 = BaseRegion("85: Hey, Listen")
    loc86 = BaseRegion("86: Full House")
    loc87 = BaseRegion("87: Crunch Hell")
    # set_rule(multiworld.get_location("88: Odyssey", p), lambda state: has_weight(state, p, 4) and state.has_all(["Hook", "Desert Geyser West"], p))
    loc89 = BaseRegion("89: Intestinal Parasites")
    loc90 = BaseRegion("90: Try Harder")
    loc91 = BaseRegion("91: Jump Around")
    loc92 = BaseRegion("92: First Date")
    loc93 = BaseRegion("93: Dark Delivery Boy")
    loc94 = BaseRegion("94: Influencers")
    loc95 = BaseRegion("95: Hypothermia")
    loc96 = BaseRegion("96: Pirates")
    loc97 = BaseRegion("97: Swimming Into the Sunset")
    loc98 = BaseRegion("98: Suspension Points")
    loc99 = BaseRegion("99: Delivery Boy")
    loc100 = BaseRegion("100: The End")

    eventKillJuan = BaseRegion("Event Kill Juan")
    eventKillMiguel = BaseRegion("Event Kill Miguel")
    eventKillJavi = BaseRegion("Event Kill Javi")
    eventKillAlberto = BaseRegion("Event Kill Alberto")
    eventKillDaniel = BaseRegion("Event Kill Daniel")

    # Create regions
    menu = BaseRegion("Menu")
    lonksHouse = BaseRegion("LonksHouse")
    swordChest = BaseRegion("SwordChest")
    elder = BaseRegion("Elder")
    chicken = BaseRegion("Chicken")
    shovel = BaseRegion("Shovel")
    castleFirstFloor = BaseRegion("CastleFirstFloor")
    castleShieldChest = BaseRegion("CastleShieldChest")
    castleMapChest = BaseRegion("CastleMapChest")
    castleRoof = BaseRegion("CastleRoof")
    chimney = BaseRegion("Chimney")
    princessRoom = BaseRegion("PrincessRoom")
    volcanoTopExit = BaseRegion("VolcanoTopExit")
    lavaTrinket = BaseRegion("LavaTrinket")
    volcanoDropStone = BaseRegion("VolcanoDropStone")
    volcanoBridge = BaseRegion("VolcanoBridge")
    belowVolcanoBridge = BaseRegion("BelowVolcanoBridge")
    sewer = BaseRegion("Sewer")
    musicClub = BaseRegion("MusicClub")
    leftOfDragon = BaseRegion("LeftOfDragon")
    rightOfDragon = BaseRegion("RightOfDragon")
    goldRoom = BaseRegion("GoldRoom")
    sewerPipe = BaseRegion("SewerPipe")
    volcanoGeyser = BaseRegion("VolcanoGeyser")
    ultimateDoor = BaseRegion("UltimateDoor")
    castleMinions = BaseRegion("CastleMinions")
    cloud = BaseRegion("Cloud")
    belowCastleBridge = BaseRegion("BelowCastleBridge")
    secretPathMoatWell = BaseRegion("SecretPathMoatWell")
    castleMoat = BaseRegion("CastleMoat")
    barn = BaseRegion("Barn")
    barnSecondFloor = BaseRegion("BarnSecondFloor")
    behindShopBush = BaseRegion("BehindShopBush")
    shop = BaseRegion("Shop")
    shopRoof = BaseRegion("ShopRoof")
    shopLake = BaseRegion("ShopLake")
    ocean = BaseRegion("Ocean")
    nukeStorage = BaseRegion("NukeStorage")
    shopCellar = BaseRegion("ShopCellar")
    parasite = BaseRegion("Parasite")
    hookArea = BaseRegion("HookArea")
    aboveHook = BaseRegion("AboveHook")
    aboveAboveHook = BaseRegion("AboveAboveHook")
    castleCannonToShop = BaseRegion("CastleCannonToShop")
    altar = BaseRegion("Altar")
    bomb = BaseRegion("Bomb")
    fishingBridge = BaseRegion("FishingBridge")
    belowFishingBridge = BaseRegion("BelowFishingBridge")
    fishingRod = BaseRegion("FishingRod")
    mountainLeftOutcrop = BaseRegion("MountainLeftOutcrop")
    mountainTop = BaseRegion("MountainTop")
    strawberry = BaseRegion("Strawberry")
    mountainTreasure = BaseRegion("MountainTreasure")
    levers = BaseRegion("Levers")
    greatWaterfall = BaseRegion("GreatWaterfall")
    greatWaterfallBottom = BaseRegion("GreatWaterfallBottom")
    fortressMoat = BaseRegion("FortressMoat")
    fairyFountain = BaseRegion("FairyFountain")
    fortressBridgeButton = BaseRegion("FortressBridgeButton")
    secretAboveBomb = BaseRegion("SecretAboveBomb")
    waterFalls = BaseRegion("WaterFalls")
    aboveWaterfalls = BaseRegion("AboveWaterfalls")
    whistle = BaseRegion("Whistle")
    whistleAltar = BaseRegion("WhistleAltar")
    belowLeapOfFaith = BaseRegion("BelowLeapOfFaith")
    elevator = BaseRegion("Elevator")
    fortressRoof = BaseRegion("FortressRoof")
    anvil = BaseRegion("Anvil")
    princess = BaseRegion("Princess")
    spikeTrap = BaseRegion("SpikeTrap")
    fireEscape = BaseRegion("FireEscape")
    fortressTreasure = BaseRegion("FortressTreasure")
    rightOfFortress = BaseRegion("RightOfFortress")
    darkstone = BaseRegion("Darkstone")
    desert = BaseRegion("Desert")
    allregions = [lonksHouse, elder, chicken, shovel, castleFirstFloor, castleShieldChest, castleMapChest, castleRoof, princessRoom, volcanoTopExit,
                  lavaTrinket, volcanoDropStone, volcanoBridge, belowVolcanoBridge, sewer, leftOfDragon, rightOfDragon, goldRoom, sewerPipe,
                  volcanoGeyser, ultimateDoor, castleMinions, cloud, belowCastleBridge, secretPathMoatWell, castleMoat, behindShopBush, shop, shopRoof,
                  shopLake, ocean, nukeStorage, hookArea, aboveHook, aboveAboveHook, castleCannonToShop, altar, bomb, fishingBridge,
                  belowFishingBridge, fishingRod, mountainLeftOutcrop, mountainTop, mountainTreasure, levers, greatWaterfall, greatWaterfallBottom, fortressMoat,
                  fairyFountain, fortressBridgeButton, secretAboveBomb, waterFalls, aboveWaterfalls, whistle, whistleAltar, belowLeapOfFaith, elevator, fortressRoof,
                  anvil, princess, fireEscape, fortressTreasure, rightOfFortress]

    start_region = random.choice(allregions)
    start_region = lonksHouse
    print(f"Start Region: {start_region.name}")

    # Generate item randomization
    item_locations = random.sample(allregions, 10)
    item_locations = [swordChest, elder, shovel, bomb, castleShieldChest, princessRoom, lavaTrinket, hookArea, nukeStorage, whistle]
    # Place Items
    item_locations[0].add_statechange(StateChange(["has_sword"], [True],
                                        lambda state: not state.event("has_princess") and not state.event("has_sword")
                                            and not state.event("has_swordelder") and not state.event("has_mrhugs"),
                                          ["Sword Chest"]))
    item_locations[1].add_statechange(StateChange(["has_swordelder"], [True],
                                        lambda state: not state.event("has_princess") and not state.event("has_sword")
                                            and not state.event("has_swordelder") and not state.event("has_mrhugs"),
                                        ["Sword Pedestal"]))
    item_locations[2].add_statechange(StateChange(["has_shovel"], [True],
                                        lambda state: not state.event("has_princess") and not state.event("has_shovel"),
                                        ["Shovel"]))
    item_locations[3].add_statechange(StateChange(["has_bomb"], [True],
                                        lambda state: not state.event("has_princess") and not state.event("has_bomb"),
                                        ["Bomb"]))
    item_locations[4].add_statechange(StateChange(["has_shield"], [True],
                                        lambda state: not state.event("has_princess") and not state.event("has_shield"),
                                        ["Shield"]))
    item_locations[5].add_statechange(StateChange(["has_mrhugs"], [True],
                                        lambda state: not state.event("has_princess") and not state.event("has_mrhugs"),
                                        ["Mister Hugs"]))
    item_locations[6].add_statechange(StateChange(["has_lavaTrinket"], [True],
                                        lambda state: not state.event("has_princess") and not state.event("has_lavaTrinket"),
                                        ["Lava Trinket"]))
    item_locations[7].add_statechange(StateChange(["has_hook"], [True],
                                        lambda state: not state.event("has_princess") and not state.event("has_hook"),
                                        ["Hook"]))
    item_locations[8].add_statechange(StateChange(["has_nuke"], [True],
                                        lambda state: not state.event("has_princess") and not state.event("has_nuke"),
                                        ["Nuke"]))
    item_locations[9].add_statechange(StateChange(["has_whistle"], [True],
                                        lambda state: not state.event("has_princess") and not state.event("has_whistle"),
                                        ["Whistle"]))

    menu.add_connection(BaseConnection(start_region, lambda state: True))
    menu.add_location(BaseConnection(loc59, lambda state: True))

    lonksHouse.add_jumpconnection(JumpConnection(elder, lambda state: not state.event("has_princess"), jump_req=2))
    lonksHouse.add_connection(BaseConnection(castleFirstFloor, lambda state: not state.event("has_princess")))
    lonksHouse.add_connection(BaseConnection(volcanoBridge, lambda state: not state.event("has_princess") and state.event("has_shovel")))
    lonksHouse.add_connection(BaseConnection(fairyFountain, lambda state: not state.event("has_princess"), ["Fairy Portal"]))
    lonksHouse.add_jumpconnection(JumpConnection(swordChest, lambda state: not state.event("has_princess"), jump_req=2))
    lonksHouse.add_location(BaseConnection(loc02, lambda state: not state.event("has_princess"),
                                            ["Faceplant Stone"]))
    lonksHouse.add_location(BaseConnection(loc03, lambda state: not state.event("has_princess")))
    lonksHouse.add_location(BaseConnection(loc04, lambda state: not state.event("has_princess") and state.event("has_sword") or state.event("has_swordelder")))
    lonksHouse.add_location(BaseConnection(loc19, lambda state: not state.event("has_princess") and state.event("has_mrhugs")))
    lonksHouse.add_location(BaseConnection(loc20, lambda state: not state.event("has_princess")))
    lonksHouse.add_location(BaseConnection(loc94, lambda state: state.event("has_princess")))

    swordChest.add_connection(BaseConnection(lonksHouse, lambda state: True))
    
    elder.add_jumpconnection(JumpConnection(chicken, lambda state: True, jump_req=2))
    elder.add_connection(BaseConnection(shovel, lambda state: True))
    elder.add_jumpconnection(JumpConnection(lonksHouse, lambda state: True, jump_req=2))
    elder.add_jumpconnection(JumpConnection(volcanoTopExit, lambda state: True, jump_req=2))
    elder.add_location(BaseConnection(loc01, lambda state: state.event("has_sword") or state.event("has_swordelder"),
                                      ["Elder"]))
    elder.add_location(BaseConnection(loc40, lambda state: state.event("has_mrhugs"),
                                      ["Elder"]))

    chicken.add_connection(BaseConnection(elder, lambda state: True))
    chicken.add_connection(BaseConnection(lonksHouse, lambda state: True))
    chicken.add_statechange(StateChange(["has_chicken"], [True],
                                        lambda state: not state.event("has_princess") and not state.event("has_chicken"),
                                        ["Chicken"]))
    chicken.add_location(BaseConnection(loc63, lambda state: not state.event("has_chicken") and (state.event("has_sword") or state.event("has_swordelder")),
                                        ["Chicken"]))
    chicken.add_location(BaseConnection(loc79, lambda state: not state.event("has_chicken") and state.event("has_mrhugs"),
                                        ["Chicken"]))

    shovel.add_jumpconnection(JumpConnection(elder, lambda state: True, jump_req=3))
    shovel.add_connection(BaseConnection(lonksHouse, lambda state: state.event("has_shovel")))

    castleFirstFloor.add_connection(BaseConnection(lonksHouse, lambda state: not state.event("has_burger") and not state.event("has_princess")))
    castleFirstFloor.add_jumpconnection(JumpConnection(castleShieldChest, lambda state: not state.event("has_burger") and not state.event("has_princess"), jump_req=2))
    castleFirstFloor.add_jumpconnection(JumpConnection(castleMapChest, lambda state: not state.event("has_burger") and not state.event("has_princess"), jump_req=3))
    castleFirstFloor.add_connection(BaseConnection(sewer, lambda state: not state.event("has_burger") and not state.event("has_princess"), ["Open Castle Floor"]))
    castleFirstFloor.add_connection(BaseConnection(castleMinions, lambda state: not state.event("has_burger") and not state.event("has_princess") and state.event("castleBridgeDown")))
    castleFirstFloor.add_statechange(StateChange(["castleBridgeDown"], [True], lambda state: not state.event("has_burger") and not state.event("has_princess") and not state.event("castleBridgeDown") and 
                                                 (state.event("has_sword") or state.event("has_shovel"))))
    castleFirstFloor.add_location(BaseConnection(loc04, lambda state: not state.event("has_burger") and not state.event("has_princess") and state.event("has_sword")))
    castleFirstFloor.add_location(BaseConnection(loc05, lambda state: not state.event("has_burger") and not state.event("has_princess") and state.event("has_sword"),
                                                 ["King"]))
    castleFirstFloor.add_location(BaseConnection(loc18, lambda state: not state.event("has_burger") and not state.event("has_princess") and state.event("has_mrhugs"),
                                                 ["King"]))
    castleFirstFloor.add_location(BaseConnection(loc19, lambda state: not state.event("has_burger") and not state.event("has_princess") and state.event("has_mrhugs")))
    castleFirstFloor.add_location(BaseConnection(loc51, lambda state: not state.event("has_burger") and state.event("has_princess")))
    castleFirstFloor.add_location(BaseConnection(loc60, lambda state: not state.event("has_burger") and not state.event("has_princess") and state.event("has_bomb")))
    castleFirstFloor.add_location(BaseConnection(loc99, lambda state: not state.event("has_princess") and state.event("has_burger")))

    castleShieldChest.add_connection(BaseConnection(castleFirstFloor, lambda state: True))
    
    castleMapChest.add_connection(BaseConnection(castleFirstFloor, lambda state: True))
    castleMapChest.add_jumpconnection(JumpConnection(castleRoof, lambda state: True, jump_req=3))
    # castleMapChest.add_statechange(StateChange(["has_map"], [True],
    #                                            lambda state: not state.event("has_map"),
    #                                            ["map"]))
    # castleMapChest.add_statechange(StateChange(["has_compass"], [True],
    #                                            lambda state: not state.event("has_compass"),
    #                                            ["compass"]))
    
    castleRoof.add_connection(BaseConnection(castleMapChest, lambda state: True))
    castleRoof.add_connection(BaseConnection(princessRoom, lambda state: True))
    castleRoof.add_jumpconnection(JumpConnection(chimney, lambda state: True, jump_req=3))

    chimney.add_location(BaseConnection(loc30, lambda state: True))

    princessRoom.add_jumpconnection(JumpConnection(castleRoof, lambda state: True, jump_req=3))
    princessRoom.add_connection(BaseConnection(castleMinions, lambda state: True))
    princessRoom.add_connection(BaseConnection(anvil, lambda state: True, ["Mirror Portal"]))
    princessRoom.add_location(BaseConnection(loc04, lambda state: state.event("has_sword")))
    princessRoom.add_location(BaseConnection(loc11, lambda state: state.event("has_mrhugs")))
    princessRoom.add_location(BaseConnection(loc19, lambda state: state.event("has_mrhugs")))
    
    volcanoTopExit.add_connection(BaseConnection(elder, lambda state: True))
    volcanoTopExit.add_connection(BaseConnection(lavaTrinket, lambda state: True))
    volcanoTopExit.add_connection(BaseConnection(shopLake, lambda state: True))

    lavaTrinket.add_jumpconnection(JumpConnection(volcanoTopExit, lambda state: True, jump_req=2))
    lavaTrinket.add_connection(BaseConnection(volcanoBridge, lambda state: True))
    
    volcanoDropStone.add_jumpconnection(JumpConnection(volcanoBridge, lambda state: True, jump_req=2))
    volcanoDropStone.add_jumpconnection(JumpConnection(behindShopBush, lambda state: True, jump_req=2))
    volcanoDropStone.add_location(BaseConnection(loc06, lambda state: not state.event("has_princess")))

    volcanoBridge.add_connection(BaseConnection(volcanoDropStone, lambda state: True))
    volcanoBridge.add_connection(BaseConnection(belowVolcanoBridge, lambda state: True))
    volcanoBridge.add_jumpconnection(JumpConnection(lavaTrinket, lambda state: True, jump_req=2))
    volcanoBridge.add_jumpconnection(JumpConnection(sewer, lambda state: True, jump_req=3))
    volcanoBridge.add_connection(BaseConnection(sewer, lambda state: state.event("has_sword") or state.event("has_hook")))

    sewer.add_jumpconnection(JumpConnection(castleFirstFloor, lambda state: True, ["Open Castle Floor"], jump_req=3))
    sewer.add_connection(BaseConnection(volcanoBridge, lambda state: True))
    sewer.add_connection(BaseConnection(belowCastleBridge, lambda state: True))
    sewer.add_connection(BaseConnection(musicClub, lambda state: state.event("has_shovel")))

    musicClub.add_connection(BaseConnection(belowVolcanoBridge, lambda state: True))
    musicClub.add_connection(BaseConnection(sewerPipe, lambda state: state.event("has_shovel")))
    musicClub.add_location(BaseConnection(eventKillDaniel, lambda state: state.event("has_sword")))

    belowVolcanoBridge.add_connection(BaseConnection(leftOfDragon, lambda state: state.event("has_shovel")))
    belowVolcanoBridge.add_connection(BaseConnection(goldRoom, lambda state: True))
    belowVolcanoBridge.add_connection(BaseConnection(parasite, lambda state: state.event("has_shovel") and state.event("has_lavaTrinket")))
    belowVolcanoBridge.add_location(BaseConnection(loc06, lambda state: not state.event("has_princess")))

    goldRoom.add_connection(BaseConnection(rightOfDragon, lambda state: True))
    goldRoom.add_jumpconnection(JumpConnection(sewerPipe, lambda state: True, jump_req=2))

    leftOfDragon.add_connection(BaseConnection(volcanoGeyser, lambda state: state.event("has_shovel")))
    leftOfDragon.add_location(BaseConnection(loc10, lambda state: state.event("has_shovel")))
    leftOfDragon.add_location(BaseConnection(loc14, lambda state: not state.event("has_princess") and not state.event("has_shield") and not state.event("has_lavaTrinket"), ["Dragon"]))
    leftOfDragon.add_location(BaseConnection(loc29, lambda state: not state.event("has_princess") and state.event("has_shield") and not state.event("has_lavaTrinket"), ["Dragon"]))
    leftOfDragon.add_location(BaseConnection(loc36, lambda state: not state.event("has_princess") and not state.event("has_shield") and state.event("has_lavaTrinket"), ["Dragon"]))
    leftOfDragon.add_location(BaseConnection(loc41, lambda state: not state.event("has_princess") and state.event("has_shield") and state.event("has_lavaTrinket"), ["Dragon"]))
    leftOfDragon.add_location(BaseConnection(loc92, lambda state: state.event("has_princess"), ["Dragon"]))

    rightOfDragon.add_connection(BaseConnection(volcanoGeyser, lambda state: True))
    rightOfDragon.add_jumpconnection(JumpConnection(goldRoom, lambda state: True, jump_req=4))
    rightOfDragon.add_location(BaseConnection(loc14, lambda state: True, ["Dragon"]))
    rightOfDragon.add_location(BaseConnection(loc16, lambda state: state.event("has_sword"), ["Dragon"]))
    rightOfDragon.add_location(BaseConnection(loc29, lambda state: state.event("has_shield") and not state.event("has_lavaTrinket"), ["Dragon"]))
    rightOfDragon.add_location(BaseConnection(loc36, lambda state: not state.event("has_shield") and state.event("has_lavaTrinket"), ["Dragon"]))
    rightOfDragon.add_location(BaseConnection(loc41, lambda state: state.event("has_shield") and state.event("has_lavaTrinket"), ["Dragon"]))
    rightOfDragon.add_location(BaseConnection(loc43, lambda state: state.event("has_mrhugs"), ["Dragon"]))
    rightOfDragon.add_location(BaseConnection(loc92, lambda state: state.event("has_princess"), ["Dragon"]))

    sewerPipe.add_connection(BaseConnection(goldRoom, lambda state: True))
    sewerPipe.add_location(BaseConnection(loc35, lambda state: True, ["Sewer Pipe"]))

    volcanoGeyser.add_connection(BaseConnection(leftOfDragon, lambda state: state.event("has_lavaTrinket")))
    volcanoGeyser.add_connection(BaseConnection(castleMinions, lambda state: True, ["Volcano Geyser"]))
    volcanoGeyser.add_jumpconnection(JumpConnection(ultimateDoor, lambda state: True, jump_req=2))
    volcanoGeyser.add_location(BaseConnection(loc06, lambda state: not state.event("has_princess")))

    ultimateDoor.add_connection(BaseConnection(volcanoGeyser, lambda state: True))
    ultimateDoor.add_location(BaseConnection(loc67, lambda state: True))
    ultimateDoor.add_location(BaseConnection(loc100, lambda state: True))

    castleMinions.add_connection(BaseConnection(castleFirstFloor, lambda state: state.event("castleBridgeDown")))
    castleMinions.add_connection(BaseConnection(secretPathMoatWell, lambda state: not state.event("castleBridgeDown")))
    castleMinions.add_connection(BaseConnection(hookArea, lambda state: True))
    castleMinions.add_jumpconnection(JumpConnection(aboveHook, lambda state: True, jump_req=2))
    castleMinions.add_connection(BaseConnection(aboveHook, lambda state: state.event("has_hook")))
    castleMinions.add_connection(BaseConnection(cloud, lambda state: True, ["Vine"]))
    castleMinions.add_location(BaseConnection(loc03, lambda state: True))
    castleMinions.add_location(BaseConnection(loc13, lambda state: state.event("has_mrhugs")))
    castleMinions.add_location(BaseConnection(loc25, lambda state: state.event("has_sword")))
    castleMinions.add_location(BaseConnection(loc95, lambda state: True))

    cloud.add_connection(BaseConnection(castleRoof, lambda state: True))
    cloud.add_location(BaseConnection(loc77, lambda state: True))

    belowCastleBridge.add_jumpconnection(JumpConnection(sewer, lambda state: True, jump_req=2.5))
    belowCastleBridge.add_jumpconnection(JumpConnection(secretPathMoatWell, lambda state: True, jump_req=3))
    belowCastleBridge.add_connection(BaseConnection(castleMoat, lambda state: True))

    secretPathMoatWell.add_connection(BaseConnection(belowCastleBridge, lambda state: True))
    secretPathMoatWell.add_jumpconnection(JumpConnection(castleMinions, lambda state: True, jump_req=3))
    secretPathMoatWell.add_jumpconnection(JumpConnection(bomb, lambda state: True, jump_req=2))

    castleMoat.add_jumpconnection(JumpConnection(belowCastleBridge, lambda state: True, jump_req=2))
    castleMoat.add_connection(BaseConnection(ultimateDoor, lambda state: state.event("has_shovel")))
    castleMoat.add_connection(BaseConnection(barn, lambda state: state.event("has_sword")))
    castleMoat.add_jumpconnection(JumpConnection(fishingBridge, lambda state: True, jump_req=2))
    castleMoat.add_connection(BaseConnection(fishingBridge, lambda state: state.event("has_sword")))
    castleMoat.add_location(BaseConnection(loc95, lambda state: True))
    castleMoat.add_location(BaseConnection(loc07, lambda state: not state.event("has_princess")))

    barn.add_jumpconnection(JumpConnection(barnSecondFloor, lambda state: True, jump_req=2))
    barn.add_location(BaseConnection(loc86, lambda state: state.event("has_princess")))

    barnSecondFloor.add_location(BaseConnection(loc31, lambda state: state.event("has_sword")))

    behindShopBush.add_connection(BaseConnection(volcanoDropStone, lambda state: True))
    behindShopBush.add_connection(BaseConnection(shopLake, lambda state: state.event("has_sword")))

    shop.add_connection(BaseConnection(shopLake, lambda state: True))
    shop.add_jumpconnection(JumpConnection(shopRoof, lambda state: True, jump_req=2))
    shop.add_jumpconnection(JumpConnection(nukeStorage, lambda state: True, jump_req=4))
    shop.add_connection(BaseConnection(nukeStorage, lambda state: state.event("has_hook")))
    shop.add_connection(BaseConnection(shopCellar, lambda state: state.event("has_princess")))
    shop.add_connection(BaseConnection(fortressMoat, lambda state: not state.event("has_princess") and not state.event("has_nuke"), ["Shop Cannon"]))
    shop.add_location(BaseConnection(loc09, lambda state: state.event("has_sword"), ["Shopkeeper"]))
    shop.add_location(BaseConnection(loc17, lambda state: not state.event("has_princess") and not state.event("has_nuke"), ["Shop Cannon"]))
    shop.add_location(BaseConnection(loc27, lambda state: state.event("has_nuke"), ["Shop Cannon"]))
    shop.add_location(BaseConnection(loc37, lambda state: state.event("has_mrhugs"), ["Shopkeeper"]))
    shop.add_location(BaseConnection(loc74, lambda state: state.event("has_sword"), ["Shopkeeper", "Shop Cannon", "Mimic", "Elevator Button"]))
    shop.add_location(BaseConnection(loc74, lambda state: state.event("has_sword"), ["Shopkeeper", "Shop Cannon", "Mimic", "Call Elevator Buttons"]))
    shop.add_location(BaseConnection(loc95, lambda state: True))

    shopRoof.add_connection(BaseConnection(shop, lambda state: True))
    shopRoof.add_jumpconnection(JumpConnection(ocean, lambda state: True, jump_req=3))
    shopRoof.add_connection(BaseConnection(ocean, lambda state: state.event("has_sword")))
    shopRoof.add_location(BaseConnection(loc03, lambda state: True))
    shopRoof.add_location(BaseConnection(loc13, lambda state: state.event("has_mrhugs")))
    shopRoof.add_location(BaseConnection(loc25, lambda state: state.event("has_sword")))
    shopRoof.add_location(BaseConnection(eventKillJuan, lambda state: state.event("has_sword")))

    shopLake.add_jumpconnection(JumpConnection(volcanoTopExit, lambda state: True, jump_req=2))
    shopLake.add_connection(BaseConnection(behindShopBush, lambda state: state.event("has_sword")))
    shopLake.add_connection(BaseConnection(shop, lambda state: True))

    ocean.add_connection(BaseConnection(shopRoof, lambda state: True))
    ocean.add_location(BaseConnection(loc95, lambda state: True))
    ocean.add_location(BaseConnection(loc96, lambda state: True))
    ocean.add_location(BaseConnection(loc97, lambda state: True))

    nukeStorage.add_connection(BaseConnection(shop, lambda state: True))

    shopCellar.add_connection(BaseConnection(shop, lambda state: state.event("has_princess")))
    shopCellar.add_connection(BaseConnection(parasite, lambda state: True))
    shopCellar.add_location(BaseConnection(loc78, lambda state: True))

    parasite.add_location(BaseConnection(loc89, lambda state: True))

    hookArea.add_jumpconnection(JumpConnection(castleMinions, lambda state: True, jump_req=3))
    hookArea.add_connection(BaseConnection(castleMinions, lambda state: state.event("has_hook")))
    
    aboveHook.add_connection(BaseConnection(castleMinions, lambda state: True))
    aboveHook.add_jumpconnection(JumpConnection(aboveAboveHook, lambda state: True, jump_req=3))
    aboveHook.add_connection(BaseConnection(aboveAboveHook, lambda state: state.event("has_hook")))
    aboveHook.add_connection(BaseConnection(bomb, lambda state: True))

    aboveAboveHook.add_connection(BaseConnection(aboveHook, lambda state: True))
    aboveAboveHook.add_jumpconnection(JumpConnection(castleCannonToShop, lambda state: True, jump_req=3))
    aboveAboveHook.add_connection(BaseConnection(castleCannonToShop, lambda state: state.event("has_hook")))
    aboveAboveHook.add_jumpconnection(JumpConnection(altar, lambda state: True, jump_req=2))
    aboveAboveHook.add_connection(BaseConnection(altar, lambda state: state.event("has_hook")))

    castleCannonToShop.add_connection(BaseConnection(aboveAboveHook, lambda state: True))
    castleCannonToShop.add_connection(BaseConnection(shopLake, lambda state: not state.event("has_princess") and not state.event("has_nuke")
                                                     , ["Castle To Shop Cannon"]))
    castleCannonToShop.add_location(BaseConnection(loc17, lambda state: not state.event("has_princess") and not state.event("has_nuke"),
                                                    ["Castle To Shop Cannon"]))
    castleCannonToShop.add_location(BaseConnection(loc56, lambda state: not state.event("has_princess") and state.event("has_nuke"), ["Castle To Shop Cannon"]))

    altar.add_connection(BaseConnection(aboveAboveHook, lambda state: True))
    altar.add_jumpconnection(JumpConnection(mountainLeftOutcrop, lambda state: True, jump_req=2))
    altar.add_jumpconnection(JumpConnection(levers, lambda state: True, jump_req=3))
    altar.add_connection(BaseConnection(levers, lambda state: state.event("has_hook")))
    altar.add_connection(BaseConnection(greatWaterfall, lambda state: True))
    altar.add_location(BaseConnection(loc72, lambda state: state.event("has_princess")))

    bomb.add_jumpconnection(JumpConnection(aboveHook, lambda state: True, jump_req=3))
    bomb.add_connection(BaseConnection(aboveHook, lambda state: state.event("has_hook")))
    bomb.add_connection(BaseConnection(fishingBridge, lambda state: True))
    bomb.add_connection(BaseConnection(secretPathMoatWell, lambda state: True))
    bomb.add_jumpconnection(JumpConnection(secretAboveBomb, lambda state: True, jump_req=3))
    bomb.add_jumpconnection(JumpConnection(greatWaterfall, lambda state: state.event("has_bomb"), jump_req=2))
    bomb.add_location(BaseConnection(loc28, lambda state: state.event("has_bomb")))
    bomb.add_location(BaseConnection(loc32, lambda state: state.event("has_sword"), ["Boulder"]))
    bomb.add_location(BaseConnection(loc54, lambda state: state.event("has_mrhugs"), ["Boulder"]))

    fishingBridge.add_connection(BaseConnection(castleMoat, lambda state: True))
    fishingBridge.add_jumpconnection(JumpConnection(fishingRod, lambda state: True, jump_req=2))
    fishingBridge.add_connection(BaseConnection(belowFishingBridge, lambda state: True))

    belowFishingBridge.add_jumpconnection(JumpConnection(fishingBridge, lambda state: True, jump_req=2))
    belowFishingBridge.add_connection(BaseConnection(waterFalls, lambda state: True))

    fishingRod.add_connection(BaseConnection(fishingBridge, lambda state: True))
    fishingRod.add_jumpconnection(JumpConnection(bomb, lambda state: True, jump_req=2))
    fishingRod.add_location(BaseConnection(loc12, lambda state: not state.event("has_princess"), ["Fishing Rod"]))

    mountainLeftOutcrop.add_connection(BaseConnection(altar, lambda state: True))
    mountainLeftOutcrop.add_jumpconnection(JumpConnection(mountainTop, lambda state: True, jump_req=3))
    mountainLeftOutcrop.add_connection(BaseConnection(mountainTop, lambda state: state.event("has_hook") or state.event("has_sword")))
    mountainLeftOutcrop.add_location(BaseConnection(loc46, lambda state: True))

    mountainTop.add_connection(BaseConnection(mountainLeftOutcrop, lambda state: True))
    mountainTop.add_connection(BaseConnection(mountainTreasure, lambda state: True))
    mountainTop.add_connection(BaseConnection(cloud, lambda state: state.event("has_chicken")))
    mountainTop.add_jumpconnection(JumpConnection(strawberry, lambda state: True, jump_req=3))
    mountainTop.add_location(BaseConnection(eventKillMiguel, lambda state: state.event("has_sword")))

    strawberry.add_location(BaseConnection(loc24, lambda state: True))

    mountainTreasure.add_connection(BaseConnection(belowLeapOfFaith, lambda state: True))
    mountainTreasure.add_location(BaseConnection(loc33, lambda state: not state.event("has_princess")))
    mountainTreasure.add_location(BaseConnection(loc62, lambda state: state.event("has_shovel")))

    levers.add_jumpconnection(JumpConnection(altar, lambda state: True, jump_req=4))
    levers.add_connection(BaseConnection(altar, lambda state: state.event("has_hook")))
    levers.add_jumpconnection(JumpConnection(belowLeapOfFaith, lambda state: True, jump_req=4))
    levers.add_connection(BaseConnection(belowLeapOfFaith, lambda state: state.event("has_hook")))
    levers.add_jumpconnection(JumpConnection(darkstone, lambda state: True, ["Dark Stone Lever Middle"], jump_req=3))
    levers.add_connection(BaseConnection(darkstone, lambda state: state.event("has_hook"), ["Dark Stone Lever Middle"]))
    levers.add_connection(BaseConnection(greatWaterfall, lambda state: True))
    levers.add_location(BaseConnection(loc38, lambda state: not state.event("has_princess"), ["Dark Stone Lever Left"]))
    levers.add_location(BaseConnection(loc44, lambda state: not state.event("has_princess"), ["Dark Stone Lever Right"]))

    darkstone.add_connection(BaseConnection(levers, lambda state: True))
    darkstone.add_statechange(StateChange(["has_darkstone"], [True],
                                        lambda state: not state.event("has_princess") and not state.event("has_darkstone"), ["Darkstone"]))
    darkstone.add_statechange(StateChange(["has_burger"], [True],
                                        lambda state: not state.event("has_princess") and not state.event("has_burger"), ["Burger"]))

    greatWaterfall.add_jumpconnection(JumpConnection(altar, lambda state: True, jump_req=2))
    greatWaterfall.add_connection(BaseConnection(belowFishingBridge, lambda state: True))
    greatWaterfall.add_connection(BaseConnection(bomb, lambda state: state.event("has_bomb")))
    greatWaterfall.add_connection(BaseConnection(greatWaterfallBottom, lambda state: True))
    greatWaterfall.add_connection(BaseConnection(whistle, lambda state: True))
    greatWaterfall.add_connection(BaseConnection(whistleAltar, lambda state: True))

    greatWaterfallBottom.add_connection(BaseConnection(waterFalls, lambda state: True))
    greatWaterfallBottom.add_jumpconnection(JumpConnection(aboveWaterfalls, lambda state: True, jump_req=2))
    greatWaterfallBottom.add_connection(BaseConnection(fortressMoat, lambda state: True))

    secretAboveBomb.add_connection(BaseConnection(bomb, lambda state: True))
    secretAboveBomb.add_connection(BaseConnection(greatWaterfall, lambda state: True))

    waterFalls.add_jumpconnection(JumpConnection(belowFishingBridge, lambda state: True, jump_req=2))
    waterFalls.add_connection(BaseConnection(mountainTop, lambda state: state.event("has_chicken") or (not state.event("has_princess")
                                                                                                       and state.event("has_shovel")), ["Waterfall Geyser"]))
    waterFalls.add_jumpconnection(JumpConnection(aboveWaterfalls, lambda state: True, jump_req=2))
    waterFalls.add_location(BaseConnection(loc08, lambda state: True))
    waterFalls.add_location(BaseConnection(loc82, lambda state: state.event("has_princess")))
    waterFalls.add_location(BaseConnection(loc87, lambda state: True, ["Event Kill Juan", "Event Kill Miguel", "Event Kill Javi", "Event Kill Alberto", "Event Kill Daniel"]))

    aboveWaterfalls.add_connection(BaseConnection(waterFalls, lambda state: True))
    aboveWaterfalls.add_connection(BaseConnection(belowFishingBridge, lambda state: True))
    aboveWaterfalls.add_connection(BaseConnection(fortressMoat, lambda state: True))

    fortressMoat.add_connection(BaseConnection(waterFalls, lambda state: True))
    fortressMoat.add_jumpconnection(JumpConnection(aboveWaterfalls, lambda state: True, jump_req=2))
    fortressMoat.add_connection(BaseConnection(fairyFountain, lambda state: True))
    fortressMoat.add_jumpconnection(JumpConnection(fortressBridgeButton, lambda state: True, jump_req=2))
    fortressMoat.add_jumpconnection(JumpConnection(rightOfFortress, lambda state: True, jump_req=3))
    fortressMoat.add_connection(BaseConnection(rightOfFortress, lambda state: state.event("has_hook")
                                               or state.event("has_shovel") or state.event("has_bomb")))
    fortressMoat.add_location(BaseConnection(loc15, lambda state: True))
    fortressMoat.add_location(BaseConnection(loc21, lambda state: True))
    fortressMoat.add_location(BaseConnection(loc48, lambda state: True))
    fortressMoat.add_location(BaseConnection(loc49, lambda state: state.event("has_sword")))
    fortressMoat.add_location(BaseConnection(loc61, lambda state: True))

    fortressBridgeButton.add_connection(BaseConnection(fortressMoat, lambda state: True))
    fortressBridgeButton.add_connection(BaseConnection(whistleAltar, lambda state: state.event("fortressBridgeDown")))
    fortressBridgeButton.add_statechange(StateChange(["fortressBridgeDown"], [True],
                                                     lambda state: not state.event("fortressBridgeDown")))

    fairyFountain.add_connection(BaseConnection(fortressMoat, lambda state: True))
    fairyFountain.add_connection(BaseConnection(lonksHouse, lambda state: True, ["Fairy Portal"]))
    fairyFountain.add_location(BaseConnection(loc65, lambda state: True))
    fairyFountain.add_location(BaseConnection(loc85, lambda state: state.event("has_sword") or state.event("has_mrhugs")))

    whistle.add_jumpconnection(JumpConnection(greatWaterfall, lambda state: True, jump_req=2))
    whistle.add_connection(BaseConnection(greatWaterfallBottom, lambda state: True))
    whistle.add_connection(BaseConnection(whistleAltar, lambda state: True))
    
    whistleAltar.add_jumpconnection(JumpConnection(greatWaterfall, lambda state: True, jump_req=2))
    whistleAltar.add_connection(BaseConnection(greatWaterfallBottom, lambda state: True))
    whistleAltar.add_jumpconnection(JumpConnection(belowLeapOfFaith, lambda state: True, jump_req=3))
    whistleAltar.add_jumpconnection(JumpConnection(elevator, lambda state: not state.event("has_princess"), jump_req=3))
    whistleAltar.add_connection(BaseConnection(elevator, lambda state: not state.event("has_princess")
                                               and (state.event("has_hook") or state.event("fortressBridgeDown"))))
    whistleAltar.add_jumpconnection(JumpConnection(fortressRoof, lambda state: not state.event("fortressBridgeDown"), jump_req=3))
    whistleAltar.add_jumpconnection(JumpConnection(fortressRoof, lambda state: not state.event("fortressBridgeDown") and state.event("has_hook"), jump_req=2))
    whistleAltar.add_jumpconnection(JumpConnection(whistle, lambda state: True, jump_req=3))
    whistleAltar.add_location(BaseConnection(loc39, lambda state: not state.event("has_princess")))
    whistleAltar.add_location(BaseConnection(loc69, lambda state: state.event("has_sword") and state.event("has_princess")))
    whistleAltar.add_location(BaseConnection(loc73, lambda state: state.event("has_princess") and state.event("has_mrhugs")))
    whistleAltar.add_location(BaseConnection(loc75, lambda state: state.event("has_princess")))
    whistleAltar.add_location(BaseConnection(loc83, lambda state: state.event("has_whistle")))
    whistleAltar.add_location(BaseConnection(loc90, lambda state: state.event("has_princess") and state.event("has_sword")))
    whistleAltar.add_location(BaseConnection(loc93, lambda state: state.event("has_princess") and state.event("has_darkstone")))
    whistleAltar.add_location(BaseConnection(eventKillAlberto, lambda state: state.event("has_sword") and not state.event("fortressBridgeDown")))

    belowLeapOfFaith.add_connection(BaseConnection(levers, lambda state: True))
    belowLeapOfFaith.add_connection(BaseConnection(whistleAltar, lambda state: True))

    elevator.add_connection(BaseConnection(whistleAltar, lambda state: state.event("fortressBridgeDown") and not state.event("has_princess")))
    elevator.add_connection(BaseConnection(anvil, lambda state: True, ["Elevator Button"]))
    elevator.add_connection(BaseConnection(anvil, lambda state: True, ["Call Elevator Buttons"]))
    elevator.add_jumpconnection(JumpConnection(rightOfFortress, lambda state: True, jump_req=4))
    elevator.add_location(BaseConnection(loc34, lambda state: True, ["Elevator Button"]))
    elevator.add_location(BaseConnection(loc34, lambda state: True, ["Call Elevator Button"]))
    elevator.add_location(BaseConnection(loc34, lambda state: state.event("has_princess")))
    elevator.add_location(BaseConnection(loc50, lambda state: state.event("has_princess")))
    elevator.add_location(BaseConnection(loc66, lambda state: state.event("has_darkstone")))
    elevator.add_location(BaseConnection(loc76, lambda state: state.event("has_princess")))
    elevator.add_location(BaseConnection(loc80, lambda state: state.event("has_chicken")))

    fortressRoof.add_jumpconnection(JumpConnection(whistleAltar, lambda state: True, jump_req=4))
    fortressRoof.add_connection(BaseConnection(whistleAltar, lambda state: state.event("fortressBridgeDown")))
    fortressRoof.add_connection(BaseConnection(anvil, lambda state: True))
    fortressRoof.add_connection(BaseConnection(castleMinions, lambda state: not state.event("has_princess") and not state.event("has_nuke"), ["Dark Fortress Cannon"]))
    fortressRoof.add_location(BaseConnection(loc17, lambda state: not state.event("has_princess") and not state.event("has_nuke"), ["Dark Fortress Cannon"]))
    fortressRoof.add_location(BaseConnection(loc42, lambda state: not state.event("has_princess"), ["Princess"]))
    fortressRoof.add_location(BaseConnection(loc52, lambda state: state.event("has_princess"), ["Dark Fortress Cannon"]))
    fortressRoof.add_location(BaseConnection(loc55, lambda state: not state.event("has_chicken") and state.event("has_princess")))
    fortressRoof.add_location(BaseConnection(loc58, lambda state: not state.event("has_chicken") and not state.event("has_princess")))
    fortressRoof.add_location(BaseConnection(loc84, lambda state: not state.event("has_princess") and state.event("has_nuke"), ["Dark Fortress Cannon"]))

    anvil.add_jumpconnection(JumpConnection(fortressRoof, lambda state: True, jump_req=4))
    anvil.add_connection(BaseConnection(fortressRoof, lambda state: state.event("has_hook")))
    anvil.add_connection(BaseConnection(elevator, lambda state: True, ["Elevator Button"]))
    anvil.add_connection(BaseConnection(elevator, lambda state: True, ["Call Elevator Buttons"]))
    anvil.add_jumpconnection(JumpConnection(princess, lambda state: True, jump_req=3))
    anvil.add_connection(BaseConnection(princess, lambda state: state.event("has_hook")))
    anvil.add_connection(BaseConnection(fireEscape, lambda state: state.event("has_princess")))
    anvil.add_connection(BaseConnection(fortressTreasure, lambda state: state.event("has_princess")))
    anvil.add_location(BaseConnection(loc22, lambda state: True, ["Anvil"]))
    anvil.add_location(BaseConnection(loc23, lambda state: True, ["Mimic"]))
    anvil.add_location(BaseConnection(loc53, lambda state: state.event("has_princess")))
    anvil.add_location(BaseConnection(loc98, lambda state: not state.event("has_princess") and state.event("has_burger"), ["Mimic"]))
    
    princess.add_connection(BaseConnection(anvil, lambda state: True))
    princess.add_jumpconnection(JumpConnection(spikeTrap, lambda state: not state.event("has_princess"), jump_req=2))
    princess.add_connection(BaseConnection(spikeTrap, lambda state: not state.event("has_princess") and state.event("has_hook")))
    princess.add_statechange(StateChange(["has_princess", "fortressBridgeDown"], [True, True],
                                        lambda state: not state.event("has_princess"),
                                        ["Princess"]))
    princess.add_location(BaseConnection(loc45, lambda state: state.event("has_princess")))
    princess.add_location(BaseConnection(loc57, lambda state: state.event("has_princess") and state.event("has_mrhugs")))
    princess.add_location(BaseConnection(loc64, lambda state: not state.event("has_princess") and state.event("has_sword")))

    spikeTrap.add_location(BaseConnection(loc70, lambda state: True)) 
    
    fireEscape.add_connection(BaseConnection(elevator, lambda state: True))
    fireEscape.add_jumpconnection(JumpConnection(fortressRoof, lambda state: True, jump_req=2))
    fireEscape.add_connection(BaseConnection(whistleAltar, lambda state: state.event("fortressBridgeDown")))

    fortressTreasure.add_connection(BaseConnection(rightOfFortress, lambda state: True))
    fortressTreasure.add_location(BaseConnection(loc68, lambda state: True))
    fortressTreasure.add_location(BaseConnection(eventKillJavi, lambda state: state.event("has_sword")))

    rightOfFortress.add_jumpconnection(JumpConnection(fortressTreasure, lambda state: True, jump_req=3))
    rightOfFortress.add_connection(BaseConnection(elevator, lambda state: True))
    rightOfFortress.add_connection(BaseConnection(fortressMoat, lambda state: state.event("has_hook")))
    # rightOfFortress.add_connection(BaseConnection(desert, lambda state: state.get_jump() == 1))
    rightOfFortress.add_location(BaseConnection(loc81, lambda state: state.event("has_princess")))

    desert.add_location(BaseConnection(loc91, lambda state: True))

    # Build full graph
    
    starttime = time.time()
    empty_state: ReventureState = ReventureState()
    todo_regions: typing.Set[Region] = set()
    menuregion = Region(menu, empty_state)
    menuregion.apstate.potapitems.append(APItems())
    todo_regions = {menuregion}
    todo_regionsdict = {}
    for region in todo_regions:
        todo_regionsdict[region.name] = region

    region_graph = ReventureGraph()
    region_graph.start_region = start_region
    region_graph.item_locations = item_locations

    while todo_regions:
        # Work through regions
        # print(f"Regioncount: {len(todo_regions)}/{region_graph.count()}")
        region: Region = todo_regions.pop()
        del todo_regionsdict[region.name]
        for base_region in region.base_regions:
            for jumpconnection in base_region.jumpconnections:
                reqjumpincreases = jumpconnection.get_jumpitems_req(region.state)
                if reqjumpincreases > totaljumpincrease: # There are only 6 increases. If we need more, we cannot reach this jumpconnection
                    continue
                name = get_region_name([jumpconnection.goal_region], region.state)
                new_region = region_graph.get_region(name)
                if new_region is None:
                    new_region = todo_regionsdict.get(name, None)
                if new_region is None:
                    new_region = Region(jumpconnection.goal_region, region.state)
                    todo_regions.add(new_region)
                    todo_regionsdict[new_region.name] = new_region
                region.add_connection(Connection(new_region, jumpconnection.apitems + [f"Jump Increase_{i+1}" for i in range(reqjumpincreases)]))

            for base_connection in base_region.connections:
                if not base_connection.can_use(region.state):
                    continue
                name = get_region_name([base_connection.goal_region], region.state)
                new_region = region_graph.get_region(name)
                if new_region is None:
                    new_region = todo_regionsdict.get(name, None)
                if new_region is None:
                    new_region = Region(base_connection.goal_region, region.state)
                    todo_regions.add(new_region)
                    todo_regionsdict[new_region.name] = new_region
                if not region in new_region.parents:
                    region.add_connection(Connection(new_region, base_connection.apitems))
            
            for location in base_region.locations:
                if not location.can_use(region.state):
                    continue
                name = get_region_name([location.goal_region], empty_state)
                new_region = region_graph.get_region(name)
                if new_region is None:
                    new_region = Region(location.goal_region, empty_state, location=True)
                    # No reason to work through locations. So add to done_regions directly
                    region_graph.add_region(new_region)
                region.add_connection(Connection(new_region, location.apitems))

            for statechange in base_region.statechange:
                if not statechange.can_use(region.state):
                    continue
                # Build new state
                new_state = region.state.copy()
                for i in range(len(statechange.states)):
                    new_state.state[statechange.states[i]] = statechange.values[i]

                if (not region.state.event("has_sword")) and new_state.event("has_sword"): # This state can do the Harakiri ending
                    name = get_region_name([loc47], empty_state)
                    new_region = region_graph.get_region(name)
                    if new_region is None:
                        new_region = Region(loc47, empty_state, location=True)
                        region_graph.add_region(new_region)
                    region.add_connection(Connection(new_region, statechange.apitems))
                
                weight = new_state.get_weight()
                reqjumpincreases = weight * 2 - (startjump * 2 - 2)
                if reqjumpincreases <= totaljumpincrease: # There are only 6 increases. If we need more, we cannot reach this statechange
                    name = get_region_name(region.base_regions, new_state)
                    new_region = region_graph.get_region(name)
                    if new_region is None:
                        new_region = todo_regionsdict.get(name, None)
                    if new_region is None:
                        new_region = Region(region.base_regions[0], new_state)
                        todo_regions.add(new_region)
                        todo_regionsdict[new_region.name] = new_region
                    region.add_connection(Connection(new_region, statechange.apitems))


        region_graph.add_region(region)
    print(f"Regioncount: {region_graph.count()}")

    print(f"Time: {time.time() - starttime}")

    # Setup Apstate
    print("Propagating apstates")
    region_graph.propagate_apstates()
    print(f"Time: {time.time() - starttime}")

    # Remove duplicate solutions
    print("Removing duplicate solutions")
    for region in region_graph.regiondict.values():
        if region.name == "Menu":
            continue
        if not region.location:
            continue
        parent_diffed_by_apitems: typing.Dict[str, typing.List[Region]] = {}
        for parent in region.parents:
            connection = parent.get_connections(region)[0] # No merging has happened yet. So there is at most one connection
            for apitems in parent.apstate.potapitems:
                apitems_string = ""
                if len(connection.apitems.apitems) != 0:
                    loc_apitems = copy.deepcopy(apitems)
                    loc_apitems.add_apitems(connection.apitems.apitems)
                    apitems_string = loc_apitems.to_string()
                else:
                    apitems_string = apitems.to_string()
                if not apitems_string in parent_diffed_by_apitems.keys():
                    parent_diffed_by_apitems[apitems_string] = [parent]
                else:
                    parent_diffed_by_apitems[apitems_string].append(parent)
        # Find used apitem sets, whilst removing unnecessary ones
        used_apstates: typing.List[str] = []
        for apstate_str in parent_diffed_by_apitems.keys():
            if len(used_apstates) == 0:
                used_apstates.append(apstate_str)
                continue
            apstate = APItems()
            apstate.add_apitems_from_string(apstate_str)
            new = True
            remove = []
            for used_apstate_str in used_apstates:
                used_apstate = APItems()
                used_apstate.add_apitems_from_string(used_apstate_str)
                if apstate.is_subset(used_apstate):
                    remove.append(used_apstate_str)
                elif used_apstate.is_subset(apstate):
                    new = False
                    break
            for rem in remove:
                used_apstates.remove(rem)
            if new:
                used_apstates.append(apstate_str)

        # Remove all parents not in used_apstates
        toRemove: typing.List[Region] = []
        for parent in region.parents:
            is_used = False
            for potapitems in parent.apstate.potapitems:
                apitems_string = ""
                if len(connection.apitems.apitems) != 0:
                    loc_apitems = copy.deepcopy(potapitems)
                    loc_apitems.add_apitems(connection.apitems.apitems)
                    apitems_string = loc_apitems.to_string()
                else:
                    apitems_string = potapitems.to_string()
                if apitems_string in used_apstates:
                    is_used = True
                    break
            if not is_used:
                toRemove.append(parent)
        for parent in toRemove:
            parent.remove_connections(region)
            region.parents.remove(parent)

        # For each apstate only keep parent with lowest complexity
        for apstate in parent_diffed_by_apitems.keys():
            apstate_parents = parent_diffed_by_apitems[apstate]
            if not apstate in used_apstates: # Not in used apstates, remove all parents
                continue
            # In case of a single parent, nothing happens here, so no need to check
            best_parent = apstate_parents[0]
            best_complexity = best_parent.complexity
            for parent in apstate_parents[1:]:
                if parent.complexity >= best_complexity:
                    parent.remove_connections(region)
                    if parent in region.parents:
                        region.parents.remove(parent)
                else:
                    best_parent.remove_connections(region)
                    if best_parent in region.parents:
                        region.parents.remove(best_parent)
                    best_parent = parent
                    best_complexity = parent.complexity

    print(f"Time: {time.time() - starttime}")
    print("Simplifying graph")

    level = 0
    while level < 5:
        change = region_graph.simplify(cleanuplevel=level)
        # print(f"Regioncount: {region_graph.count()}, Level: {level}")
        if change == "":
            level += 1
        else:
            level = 0
        # print(change)

    print(f"Regioncount: {region_graph.count()}")

    # write to file
    write_plantuml(region_graph)

    total_time = time.time() - starttime
    print(f"Total time: {total_time}")

    return region_graph
    # def calc_logic(root: Region, currentreq):
    #     if root.location:
    #         logicdict[root.name.split(":")[0]] = currentreq
    #     for connection in root.connections:
    #         calc_logic(connection.region, currentreq + ", " + connection.apitems.to_string())
    # Calc and print logic
    # logicdict = {}
    # currentreq = ""
    # calc_logic(region_graph.regiondict["Menu"], currentreq)

    # for i in range(100):
    #     key = f"{i:02d}"
    #     if key in logicdict.keys():
    #         print(f"{key}: {logicdict[key]}")
    #     else:
    #         continue

if __name__ == "__main__":
    create_region_graph()
    # cProfile.run('create_region_graph()', 'restats')