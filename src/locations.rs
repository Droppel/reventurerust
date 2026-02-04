use crate::{BaseRegion};

/// Location region indices
/// These represent the 100 ending locations in Reventure
pub mod locations {
    pub const LOC01: usize = 0;   // "01: It's Dangerous to be Near Tim"
    pub const LOC02: usize = 1;   // "02: Shit Happens"
    pub const LOC03: usize = 2;   // "03: Please Nerf This"
    pub const LOC04: usize = 3;   // "04: Public Enemy"
    pub const LOC05: usize = 4;   // "05: Kingslayer"
    pub const LOC06: usize = 5;   // "06: The Floor is Lava"
    pub const LOC07: usize = 6;   // "07: Go Swimming"
    pub const LOC08: usize = 7;   // "08: Roll & Rock"
    pub const LOC09: usize = 8;   // "09: Customer is Always Right"
    pub const LOC10: usize = 9;   // "10: Gold Rush"
    pub const LOC11: usize = 10;  // "11: Feline Company"
    pub const LOC12: usize = 11;  // "12: Hobbies"
    pub const LOC13: usize = 12;  // "13: Allergic to Cuteness"
    pub const LOC14: usize = 13;  // "14: Dracar-ish"
    pub const LOC15: usize = 14;  // "15: Family Gathering"
    pub const LOC16: usize = 15;  // "16: Monster Hunter"
    pub const LOC17: usize = 16;  // "17: Public Transport Next Time"
    pub const LOC18: usize = 17;  // "18: King of Hearts"
    pub const LOC19: usize = 18;  // "19: Broken Heart"
    pub const LOC20: usize = 19;  // "20: Day Off"
    pub const LOC21: usize = 20;  // "21: You Nailed It"
    pub const LOC22: usize = 21;  // "22: Paperweight"
    pub const LOC23: usize = 22;  // "23: True Beauty is inside"
    pub const LOC24: usize = 23;  // "24: Strawberry"
    pub const LOC25: usize = 24;  // "25: Bully"
    pub const LOC26: usize = 25;  // "26: Greedy Bastard"
    pub const LOC27: usize = 26;  // "27: Airstrike"
    pub const LOC28: usize = 27;  // "28: Don't Try This at Home"
    pub const LOC29: usize = 28;  // "29: The Man in the Steel Mask"
    pub const LOC30: usize = 29;  // "30: Subliminal Message"
    pub const LOC31: usize = 30;  // "31: Collateral Damage"
    pub const LOC32: usize = 31;  // "32: You Monster"
    pub const LOC33: usize = 32;  // "33: Leap of Faith"
    pub const LOC34: usize = 33;  // "34: -1st Floor"
    pub const LOC35: usize = 34;  // "35: Wastewater"
    pub const LOC36: usize = 35;  // "36: Fireproof"
    pub const LOC37: usize = 36;  // "37: Free Hugs"
    pub const LOC38: usize = 37;  // "38: Oh Boy, I'm so Hungry"
    pub const LOC39: usize = 38;  // "39: Everything is Terrible"
    pub const LOC40: usize = 39;  // "40: Sexy Beard"
    pub const LOC41: usize = 40;  // "41: Post-Traumatic Stress Disorder"
    pub const LOC42: usize = 41;  // "42: Sneaky Bastard"
    pub const LOC43: usize = 42;  // "43: Dinner for Two"
    pub const LOC44: usize = 43;  // "44: Bad Leverage"
    pub const LOC45: usize = 44;  // "45: Well Excuuuuse Me, Princess"
    pub const LOC46: usize = 45;  // "46: Extreme Sports"
    pub const LOC47: usize = 46;  // "47: Harakiri"
    pub const LOC48: usize = 47;  // "48: It's my First Day"
    pub const LOC49: usize = 48;  // "49: Victory Royale"
    pub const LOC50: usize = 49;  // "50: P0wned"
    pub const LOC51: usize = 50;  // "51: Politics"
    pub const LOC52: usize = 51;  // "52: I'm Feeling Lucky"
    pub const LOC53: usize = 52;  // "53: Videogames"
    pub const LOC54: usize = 53;  // "54: Paraphilia"
    pub const LOC55: usize = 54;  // "55: Escape Shortcut"
    pub const LOC56: usize = 55;  // "56: Refund Request"
    pub const LOC57: usize = 56;  // "57: Friendzoned"
    pub const LOC58: usize = 57;  // "58: Dark Extreme Sports"
    pub const LOC59: usize = 58;  // "59: Away From Kingdom"
    pub const LOC60: usize = 59;  // "60: Viva La Resistance"
    pub const LOC61: usize = 60;  // "61: Syndicalism"
    pub const LOC62: usize = 61;  // "62: Jackpot"
    pub const LOC63: usize = 62;  // "63: You Don't Mess With Chicken"
    pub const LOC64: usize = 63;  // "64: I Thought It Was A Mimic"
    pub const LOC65: usize = 64;  // "65: Overheal"
    pub const LOC66: usize = 65;  // "66: Finite War"
    pub const LOC67: usize = 66;  // "67: Stay Determined"
    pub const LOC68: usize = 67;  // "68: Otaku Fever"
    pub const LOC69: usize = 68;  // "69: Quick and Dirty"
    pub const LOC70: usize = 69;  // "70: It's a Trap"
    pub const LOC71: usize = 70;  // "71: Sustainable Development"
    pub const LOC72: usize = 71;  // "72: Ecologist"
    pub const LOC73: usize = 72;  // "73: Dark Love"
    pub const LOC74: usize = 73;  // "74: Bittersweet Revenge"
    pub const LOC75: usize = 74;  // "75: Please, Not Again"
    pub const LOC76: usize = 75;  // "76: A Waifu is You"
    pub const LOC77: usize = 76;  // "77: Battle Royale"
    pub const LOC78: usize = 77;  // "78: Silver or Lead"
    pub const LOC79: usize = 78;  // "79: Good Ending"
    pub const LOC80: usize = 79;  // "80: Chicken of Doom"
    pub const LOC81: usize = 80;  // "81: Forever Together"
    pub const LOC82: usize = 81;  // "82: Perfect Crime"
    pub const LOC83: usize = 82;  // "83: We Have to Go Back"
    pub const LOC84: usize = 83;  // "84: Not what you Expected"
    pub const LOC85: usize = 84;  // "85: Hey, Listen"
    pub const LOC86: usize = 85;  // "86: Full House"
    pub const LOC87: usize = 86;  // "87: Crunch Hell"
    pub const LOC88: usize = 87;  // "88: Odyssey"
    pub const LOC89: usize = 88;  // "89: Intestinal Parasites"
    pub const LOC90: usize = 89;  // "90: Try Harder"
    pub const LOC91: usize = 90;  // "91: Jump Around"
    pub const LOC92: usize = 91;  // "92: First Date"
    pub const LOC93: usize = 92;  // "93: Dark Delivery Boy"
    pub const LOC94: usize = 93;  // "94: Influencers"
    pub const LOC95: usize = 94;  // "95: Hypothermia"
    pub const LOC96: usize = 95;  // "96: Pirates"
    pub const LOC97: usize = 96;  // "97: Swimming Into the Sunset"
    pub const LOC98: usize = 97;  // "98: Suspension Points"
    pub const LOC99: usize = 98;  // "99: Delivery Boy"
    pub const LOC100: usize = 99; // "100: The End"
}

/// Event region indices
pub mod events {
    pub const EVENT_KILL_JUAN: usize = 100;     // "Event Kill Juan"
    pub const EVENT_KILL_MIGUEL: usize = 101;   // "Event Kill Miguel"
    pub const EVENT_KILL_JAVI: usize = 102;     // "Event Kill Javi"
    pub const EVENT_KILL_ALBERTO: usize = 103; // "Event Kill Alberto"
    pub const EVENT_KILL_DANIEL: usize = 104;  // "Event Kill Daniel"
}

/// Game region indices
pub mod regions {
    pub const MENU: usize = 105;
    pub const LONKS_HOUSE: usize = 106;
    pub const LONKS_BACKGARDEN: usize = 107;
    pub const LONKS_FRONTGARDEN: usize = 108;
    pub const SWORD_CHEST: usize = 109;
    pub const ELDER: usize = 110;
    pub const CHICKEN: usize = 111;
    pub const SHOVEL: usize = 112;
    pub const CASTLE_FIRST_FLOOR: usize = 113;
    pub const CASTLE_SHIELD_CHEST: usize = 114;
    pub const CASTLE_MAP_CHEST: usize = 115;
    pub const CASTLE_ROOF: usize = 116;
    pub const CHIMNEY: usize = 117;
    pub const PRINCESS_ROOM: usize = 118;
    pub const VOLCANO_TOP_EXIT: usize = 119;
    pub const LAVA_TRINKET: usize = 120;
    pub const VOLCANO_DROP_STONE: usize = 121;
    pub const VOLCANO_BRIDGE: usize = 122;
    pub const BELOW_VOLCANO_BRIDGE: usize = 123;
    pub const SEWER: usize = 124;
    pub const MUSIC_CLUB: usize = 125;
    pub const LEFT_OF_DRAGON: usize = 126;
    pub const RIGHT_OF_DRAGON: usize = 127;
    pub const GOLD_ROOM: usize = 128;
    pub const SEWER_PIPE: usize = 129;
    pub const VOLCANO_GEYSER: usize = 130;
    pub const ULTIMATE_DOOR: usize = 131;
    pub const CASTLE_MINIONS: usize = 132;
    pub const CLOUD: usize = 133;
    pub const BELOW_CASTLE_BRIDGE: usize = 134;
    pub const SECRET_PATH_MOAT_WELL: usize = 135;
    pub const CASTLE_MOAT: usize = 136;
    pub const BARN: usize = 137;
    pub const BARN_SECOND_FLOOR: usize = 138;
    pub const BEHIND_SHOP_BUSH: usize = 139;
    pub const SHOP: usize = 140;
    pub const SHOP_ROOF: usize = 141;
    pub const SHOP_LAKE: usize = 142;
    pub const OCEAN: usize = 143;
    pub const NUKE_STORAGE: usize = 144;
    pub const SHOP_CELLAR: usize = 145;
    pub const PARASITE: usize = 146;
    pub const HOOK_AREA: usize = 147;
    pub const ABOVE_HOOK: usize = 148;
    pub const ABOVE_ABOVE_HOOK: usize = 149;
    pub const CASTLE_CANNON_TO_SHOP: usize = 150;
    pub const ALTAR: usize = 151;
    pub const BOMB: usize = 152;
    pub const FISHING_BRIDGE: usize = 153;
    pub const BELOW_FISHING_BRIDGE: usize = 154;
    pub const FISHING_ROD: usize = 155;
    pub const MOUNTAIN_LEFT_OUTCROP: usize = 156;
    pub const MOUNTAIN_TOP: usize = 157;
    pub const STRAWBERRY: usize = 158;
    pub const MOUNTAIN_TREASURE: usize = 159;
    pub const LEVERS: usize = 160;
    pub const GREAT_WATERFALL: usize = 161;
    pub const GREAT_WATERFALL_BOTTOM: usize = 162;
    pub const FORTRESS_MOAT: usize = 163;
    pub const FAIRY_FOUNTAIN: usize = 164;
    pub const FORTRESS_BRIDGE_BUTTON: usize = 165;
    pub const SECRET_ABOVE_BOMB: usize = 166;
    pub const WATER_FALLS: usize = 167;
    pub const ABOVE_WATERFALLS: usize = 168;
    pub const WHISTLE: usize = 169;
    pub const WHISTLE_ALTAR: usize = 170;
    pub const BELOW_LEAP_OF_FAITH: usize = 171;
    pub const ELEVATOR: usize = 172;
    pub const FORTRESS_ROOF: usize = 173;
    pub const ANVIL: usize = 174;
    pub const PRINCESS: usize = 175;
    pub const SPIKE_TRAP: usize = 176;
    pub const FIRE_ESCAPE: usize = 177;
    pub const FORTRESS_TREASURE: usize = 178;
    pub const RIGHT_OF_FORTRESS: usize = 179;
    pub const DARKSTONE: usize = 180;
    pub const DESERT: usize = 181;
    pub const ALBERTO: usize = 182;
}

/// Total number of base regions (locations + events + game regions)
pub const TOTAL_BASE_REGIONS: usize = regions::ALBERTO + 1;

/// Creates all base regions with their names
pub fn create_all_base_regions() -> Vec<BaseRegion> {
    let mut base_regions = Vec::with_capacity(TOTAL_BASE_REGIONS);
    
    // Location regions (loc01-loc100, excluding loc26, loc71, loc88)
    base_regions.push(BaseRegion::new("01: It's Dangerous to be Near Tim"));
    base_regions.push(BaseRegion::new("02: Shit Happens"));
    base_regions.push(BaseRegion::new("03: Please Nerf This"));
    base_regions.push(BaseRegion::new("04: Public Enemy"));
    base_regions.push(BaseRegion::new("05: Kingslayer"));
    base_regions.push(BaseRegion::new("06: The Floor is Lava"));
    base_regions.push(BaseRegion::new("07: Go Swimming"));
    base_regions.push(BaseRegion::new("08: Roll & Rock"));
    base_regions.push(BaseRegion::new("09: Customer is Always Right"));
    base_regions.push(BaseRegion::new("10: Gold Rush"));
    base_regions.push(BaseRegion::new("11: Feline Company"));
    base_regions.push(BaseRegion::new("12: Hobbies"));
    base_regions.push(BaseRegion::new("13: Allergic to Cuteness"));
    base_regions.push(BaseRegion::new("14: Dracar-ish"));
    base_regions.push(BaseRegion::new("15: Family Gathering"));
    base_regions.push(BaseRegion::new("16: Monster Hunter"));
    base_regions.push(BaseRegion::new("17: Public Transport Next Time"));
    base_regions.push(BaseRegion::new("18: King of Hearts"));
    base_regions.push(BaseRegion::new("19: Broken Heart"));
    base_regions.push(BaseRegion::new("20: Day Off"));
    base_regions.push(BaseRegion::new("21: You Nailed It"));
    base_regions.push(BaseRegion::new("22: Paperweight"));
    base_regions.push(BaseRegion::new("23: True Beauty is inside"));
    base_regions.push(BaseRegion::new("24: Strawberry"));
    base_regions.push(BaseRegion::new("25: Bully"));
    base_regions.push(BaseRegion::new("26: Greedy Bastard"));
    base_regions.push(BaseRegion::new("27: Airstrike"));
    base_regions.push(BaseRegion::new("28: Don't Try This at Home"));
    base_regions.push(BaseRegion::new("29: The Man in the Steel Mask"));
    base_regions.push(BaseRegion::new("30: Subliminal Message"));
    base_regions.push(BaseRegion::new("31: Collateral Damage"));
    base_regions.push(BaseRegion::new("32: You Monster"));
    base_regions.push(BaseRegion::new("33: Leap of Faith"));
    base_regions.push(BaseRegion::new("34: -1st Floor"));
    base_regions.push(BaseRegion::new("35: Wastewater"));
    base_regions.push(BaseRegion::new("36: Fireproof"));
    base_regions.push(BaseRegion::new("37: Free Hugs"));
    base_regions.push(BaseRegion::new("38: Oh Boy, I'm so Hungry"));
    base_regions.push(BaseRegion::new("39: Everything is Terrible"));
    base_regions.push(BaseRegion::new("40: Sexy Beard"));
    base_regions.push(BaseRegion::new("41: Post-Traumatic Stress Disorder"));
    base_regions.push(BaseRegion::new("42: Sneaky Bastard"));
    base_regions.push(BaseRegion::new("43: Dinner for Two"));
    base_regions.push(BaseRegion::new("44: Bad Leverage"));
    base_regions.push(BaseRegion::new("45: Well Excuuuuse Me, Princess"));
    base_regions.push(BaseRegion::new("46: Extreme Sports"));
    base_regions.push(BaseRegion::new("47: Harakiri"));
    base_regions.push(BaseRegion::new("48: It's my First Day"));
    base_regions.push(BaseRegion::new("49: Victory Royale"));
    base_regions.push(BaseRegion::new("50: P0wned"));
    base_regions.push(BaseRegion::new("51: Politics"));
    base_regions.push(BaseRegion::new("52: I'm Feeling Lucky"));
    base_regions.push(BaseRegion::new("53: Videogames"));
    base_regions.push(BaseRegion::new("54: Paraphilia"));
    base_regions.push(BaseRegion::new("55: Escape Shortcut"));
    base_regions.push(BaseRegion::new("56: Refund Request"));
    base_regions.push(BaseRegion::new("57: Friendzoned"));
    base_regions.push(BaseRegion::new("58: Dark Extreme Sports"));
    base_regions.push(BaseRegion::new("59: Away From Kingdom"));
    base_regions.push(BaseRegion::new("60: Viva La Resistance"));
    base_regions.push(BaseRegion::new("61: Syndicalism"));
    base_regions.push(BaseRegion::new("62: Jackpot"));
    base_regions.push(BaseRegion::new("63: You Don't Mess With Chicken"));
    base_regions.push(BaseRegion::new("64: I Thought It Was A Mimic"));
    base_regions.push(BaseRegion::new("65: Overheal"));
    base_regions.push(BaseRegion::new("66: Finite War"));
    base_regions.push(BaseRegion::new("67: Stay Determined"));
    base_regions.push(BaseRegion::new("68: Otaku Fever"));
    base_regions.push(BaseRegion::new("69: Quick and Dirty"));
    base_regions.push(BaseRegion::new("70: It's a Trap"));
    base_regions.push(BaseRegion::new("71: Sustainable Development"));
    base_regions.push(BaseRegion::new("72: Ecologist"));
    base_regions.push(BaseRegion::new("73: Dark Love"));
    base_regions.push(BaseRegion::new("74: Bittersweet Revenge"));
    base_regions.push(BaseRegion::new("75: Please, Not Again"));
    base_regions.push(BaseRegion::new("76: A Waifu is You"));
    base_regions.push(BaseRegion::new("77: Battle Royale"));
    base_regions.push(BaseRegion::new("78: Silver or Lead"));
    base_regions.push(BaseRegion::new("79: Good Ending"));
    base_regions.push(BaseRegion::new("80: Chicken of Doom"));
    base_regions.push(BaseRegion::new("81: Forever Together"));
    base_regions.push(BaseRegion::new("82: Perfect Crime"));
    base_regions.push(BaseRegion::new("83: We Have to Go Back"));
    base_regions.push(BaseRegion::new("84: Not what you Expected"));
    base_regions.push(BaseRegion::new("85: Hey, Listen"));
    base_regions.push(BaseRegion::new("86: Full House"));
    base_regions.push(BaseRegion::new("87: Crunch Hell"));
    base_regions.push(BaseRegion::new("88: Odyssey"));
    base_regions.push(BaseRegion::new("89: Intestinal Parasites"));
    base_regions.push(BaseRegion::new("90: Try Harder"));
    base_regions.push(BaseRegion::new("91: Jump Around"));
    base_regions.push(BaseRegion::new("92: First Date"));
    base_regions.push(BaseRegion::new("93: Dark Delivery Boy"));
    base_regions.push(BaseRegion::new("94: Influencers"));
    base_regions.push(BaseRegion::new("95: Hypothermia"));
    base_regions.push(BaseRegion::new("96: Pirates"));
    base_regions.push(BaseRegion::new("97: Swimming Into the Sunset"));
    base_regions.push(BaseRegion::new("98: Suspension Points"));
    base_regions.push(BaseRegion::new("99: Delivery Boy"));
    base_regions.push(BaseRegion::new("100: The End"));
    
    // Event regions
    base_regions.push(BaseRegion::new("Event Kill Juan"));
    base_regions.push(BaseRegion::new("Event Kill Miguel"));
    base_regions.push(BaseRegion::new("Event Kill Javi"));
    base_regions.push(BaseRegion::new("Event Kill Alberto"));
    base_regions.push(BaseRegion::new("Event Kill Daniel"));
    
    // Game regions
    base_regions.push(BaseRegion::new("Menu"));
    base_regions.push(BaseRegion::new("LonksHouse"));
    base_regions.push(BaseRegion::new("LonksBackGarden"));
    base_regions.push(BaseRegion::new("LonksFrontGarden"));
    base_regions.push(BaseRegion::new("SwordChest"));
    base_regions.push(BaseRegion::new("Elder"));
    base_regions.push(BaseRegion::new("Chicken"));
    base_regions.push(BaseRegion::new("Shovel"));
    base_regions.push(BaseRegion::new("CastleFirstFloor"));
    base_regions.push(BaseRegion::new("CastleShieldChest"));
    base_regions.push(BaseRegion::new("CastleMapChest"));
    base_regions.push(BaseRegion::new("CastleRoof"));
    base_regions.push(BaseRegion::new("Chimney"));
    base_regions.push(BaseRegion::new("PrincessRoom"));
    base_regions.push(BaseRegion::new("VolcanoTopExit"));
    base_regions.push(BaseRegion::new("LavaTrinket"));
    base_regions.push(BaseRegion::new("VolcanoDropStone"));
    base_regions.push(BaseRegion::new("VolcanoBridge"));
    base_regions.push(BaseRegion::new("BelowVolcanoBridge"));
    base_regions.push(BaseRegion::new("Sewer"));
    base_regions.push(BaseRegion::new("MusicClub"));
    base_regions.push(BaseRegion::new("LeftOfDragon"));
    base_regions.push(BaseRegion::new("RightOfDragon"));
    base_regions.push(BaseRegion::new("GoldRoom"));
    base_regions.push(BaseRegion::new("SewerPipe"));
    base_regions.push(BaseRegion::new("VolcanoGeyser"));
    base_regions.push(BaseRegion::new("UltimateDoor"));
    base_regions.push(BaseRegion::new("CastleMinions"));
    base_regions.push(BaseRegion::new("Cloud"));
    base_regions.push(BaseRegion::new("BelowCastleBridge"));
    base_regions.push(BaseRegion::new("SecretPathMoatWell"));
    base_regions.push(BaseRegion::new("CastleMoat"));
    base_regions.push(BaseRegion::new("Barn"));
    base_regions.push(BaseRegion::new("BarnSecondFloor"));
    base_regions.push(BaseRegion::new("BehindShopBush"));
    base_regions.push(BaseRegion::new("Shop"));
    base_regions.push(BaseRegion::new("ShopRoof"));
    base_regions.push(BaseRegion::new("ShopLake"));
    base_regions.push(BaseRegion::new("Ocean"));
    base_regions.push(BaseRegion::new("NukeStorage"));
    base_regions.push(BaseRegion::new("ShopCellar"));
    base_regions.push(BaseRegion::new("Parasite"));
    base_regions.push(BaseRegion::new("HookArea"));
    base_regions.push(BaseRegion::new("AboveHook"));
    base_regions.push(BaseRegion::new("AboveAboveHook"));
    base_regions.push(BaseRegion::new("CastleCannonToShop"));
    base_regions.push(BaseRegion::new("Altar"));
    base_regions.push(BaseRegion::new("Bomb"));
    base_regions.push(BaseRegion::new("FishingBridge"));
    base_regions.push(BaseRegion::new("BelowFishingBridge"));
    base_regions.push(BaseRegion::new("FishingRod"));
    base_regions.push(BaseRegion::new("MountainLeftOutcrop"));
    base_regions.push(BaseRegion::new("MountainTop"));
    base_regions.push(BaseRegion::new("Strawberry"));
    base_regions.push(BaseRegion::new("MountainTreasure"));
    base_regions.push(BaseRegion::new("Levers"));
    base_regions.push(BaseRegion::new("GreatWaterfall"));
    base_regions.push(BaseRegion::new("GreatWaterfallBottom"));
    base_regions.push(BaseRegion::new("FortressMoat"));
    base_regions.push(BaseRegion::new("FairyFountain"));
    base_regions.push(BaseRegion::new("FortressBridgeButton"));
    base_regions.push(BaseRegion::new("SecretAboveBomb"));
    base_regions.push(BaseRegion::new("WaterFalls"));
    base_regions.push(BaseRegion::new("AboveWaterfalls"));
    base_regions.push(BaseRegion::new("Whistle"));
    base_regions.push(BaseRegion::new("WhistleAltar"));
    base_regions.push(BaseRegion::new("BelowLeapOfFaith"));
    base_regions.push(BaseRegion::new("Elevator"));
    base_regions.push(BaseRegion::new("FortressRoof"));
    base_regions.push(BaseRegion::new("Anvil"));
    base_regions.push(BaseRegion::new("Princess"));
    base_regions.push(BaseRegion::new("SpikeTrap"));
    base_regions.push(BaseRegion::new("FireEscape"));
    base_regions.push(BaseRegion::new("FortressTreasure"));
    base_regions.push(BaseRegion::new("RightOfFortress"));
    base_regions.push(BaseRegion::new("Darkstone"));
    base_regions.push(BaseRegion::new("Desert"));
    base_regions.push(BaseRegion::new("Alberto"));
    
    base_regions
}

/// Get the list of all region indices that can be used for item placement
pub fn get_all_game_regions() -> Vec<usize> {
    vec![
        regions::LONKS_HOUSE,
        regions::LONKS_BACKGARDEN,
        regions::LONKS_FRONTGARDEN,
        regions::ELDER,
        regions::CHICKEN,
        regions::SHOVEL,
        regions::CASTLE_FIRST_FLOOR,
        regions::CASTLE_SHIELD_CHEST,
        regions::CASTLE_MAP_CHEST,
        regions::CASTLE_ROOF,
        regions::PRINCESS_ROOM,
        regions::VOLCANO_TOP_EXIT,
        regions::LAVA_TRINKET,
        regions::VOLCANO_DROP_STONE,
        regions::VOLCANO_BRIDGE,
        regions::BELOW_VOLCANO_BRIDGE,
        regions::SEWER,
        regions::LEFT_OF_DRAGON,
        regions::RIGHT_OF_DRAGON,
        regions::GOLD_ROOM,
        regions::SEWER_PIPE,
        regions::VOLCANO_GEYSER,
        regions::ULTIMATE_DOOR,
        regions::CASTLE_MINIONS,
        regions::CLOUD,
        regions::BELOW_CASTLE_BRIDGE,
        regions::SECRET_PATH_MOAT_WELL,
        regions::CASTLE_MOAT,
        regions::BEHIND_SHOP_BUSH,
        regions::SHOP,
        regions::SHOP_ROOF,
        regions::SHOP_LAKE,
        regions::OCEAN,
        regions::NUKE_STORAGE,
        regions::HOOK_AREA,
        regions::ABOVE_HOOK,
        regions::ABOVE_ABOVE_HOOK,
        regions::CASTLE_CANNON_TO_SHOP,
        regions::ALTAR,
        regions::BOMB,
        regions::FISHING_BRIDGE,
        regions::BELOW_FISHING_BRIDGE,
        regions::FISHING_ROD,
        regions::MOUNTAIN_LEFT_OUTCROP,
        regions::MOUNTAIN_TOP,
        regions::MOUNTAIN_TREASURE,
        regions::LEVERS,
        regions::GREAT_WATERFALL,
        regions::GREAT_WATERFALL_BOTTOM,
        regions::FORTRESS_MOAT,
        regions::FAIRY_FOUNTAIN,
        regions::FORTRESS_BRIDGE_BUTTON,
        regions::SECRET_ABOVE_BOMB,
        regions::WATER_FALLS,
        regions::ABOVE_WATERFALLS,
        regions::WHISTLE,
        regions::WHISTLE_ALTAR,
        regions::BELOW_LEAP_OF_FAITH,
        regions::ELEVATOR,
        regions::FORTRESS_ROOF,
        regions::ANVIL,
        regions::PRINCESS,
        regions::FIRE_ESCAPE,
        regions::FORTRESS_TREASURE,
        regions::RIGHT_OF_FORTRESS,
    ]
}

/// Get the default item placement locations
pub fn get_default_item_locations() -> Vec<usize> {
    vec![
        regions::SWORD_CHEST,       // Sword Chest
        regions::ELDER,             // Sword Pedestal
        regions::SHOVEL,            // Shovel
        regions::BOMB,              // Bomb
        regions::CASTLE_SHIELD_CHEST, // Shield
        regions::PRINCESS_ROOM,     // Mister Hugs
        regions::LAVA_TRINKET,      // Lava Trinket
        regions::HOOK_AREA,         // Hook
        regions::NUKE_STORAGE,      // Nuke
        regions::WHISTLE,           // Whistle
    ]
}
