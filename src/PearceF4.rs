const FOOD: f32 = 0.0006144;
const FUEL: f32 = 0.0014336;
const AMMO: f32 = 0.0021504;
const TOOLKIT: f32 = 0.0017408;

const FOOD_FEE: f32 = 0.00000141;
const FUEL_FEE: f32 = 0.0000033;
const AMMO_FEE: f32 = 0.00000495;
const TOOLKIT_FEE: f32 = 0.000004;

const MONTH: f32 = 30.0;
const DAY: f32 = 24.0;
const HOUR: f32 = 60.0;

const FOOD_TANK: f32 = 10830.0;
const FUEL_TANK: f32 = 21400.0;
const AMMO_TANK: f32 = 58170.0;
const TOOLKIT_TANK: f32 = 41720.0;

const FOOD_BURNED_MIN: f32 = 1.86;
const FUEL_BURNED_MIN: f32 = 2.32;
const AMMO_BURNED_MIN: f32 = 3.13;
const TOOLKIT_BURNED_MIN: f32 = 3.25;

 pub fn pearcef4_food_expense() {
    let burned_in_hour = FOOD_BURNED_MIN * HOUR; // 111,6 in hour
    let burned_in_day = burned_in_hour * DAY; // 2678,4 in day
    let burned_in_month = burned_in_day * MONTH; // 80352 in month
    let gatherway_full_tank = FOOD_TANK / burned_in_day; // 4,04 day with one tank
    let gatherway_in_hour = gatherway_full_tank * DAY; // 97,04 hour with full tank
    let pump_up_times_monthly = MONTH * DAY / gatherway_in_hour; // 7,41 times pumped tank
    let filled_tank_by_str_atlas  = FOOD_TANK * FOOD ; // Filled 6,65 star atlas
    let with_fee = burned_in_month * FOOD_FEE;
    let total_supply = pump_up_times_monthly * filled_tank_by_str_atlas + with_fee;
    println!("Pearce_F4 monthly Food expense included fee = {}",total_supply);
}
 pub fn pearcef4_fuel_expense() {
    let burned_in_hour = FUEL_BURNED_MIN * HOUR; // 139,2 in hour
    let burned_in_day = burned_in_hour * DAY; // 3340,8 in day
    let burned_in_month = burned_in_day * MONTH; // 100224 in month
    let gatherway_full_tank = FUEL_TANK / burned_in_day; // 6,4 day with one tank
    let gatherway_in_hour = gatherway_full_tank * DAY; // 153,7 hour with full tank
    let pump_up_times_monthly = MONTH * DAY / gatherway_in_hour; // 4,68 times pumped tank
    let filled_tank_by_str_atlas  = FUEL_TANK * FUEL ; // Filled 30,67 star atlas
    let with_fee = burned_in_month * FUEL_FEE;
    let total_supply = pump_up_times_monthly * filled_tank_by_str_atlas + with_fee;
    println!("Pearce_F4 monthly Fuel expense included fee = {}",total_supply);
}
 pub fn pearcef4_ammo_expense() {
    let burned_in_hour = AMMO_BURNED_MIN * HOUR; // 187,8 in hour
    let burned_in_day = burned_in_hour * DAY; // 4507,2 in day
    let burned_in_month = burned_in_day * MONTH; // 135216 in month
    let gatherway_full_tank = AMMO_TANK / burned_in_day; // 12,9 day with one tank
    let gatherway_in_hour = gatherway_full_tank * DAY; // 309,6 hour with full tank
    let pump_up_times_monthly = MONTH * DAY / gatherway_in_hour; // 2,32 times pumped tank
    let filled_tank_by_str_atlas  = AMMO_TANK * AMMO ; // Filled 125,08 star atlas
    let with_fee = burned_in_month * AMMO_FEE;
    let total_supply = pump_up_times_monthly * filled_tank_by_str_atlas + with_fee;
    println!("Pearce_F4 monthly Ammunation expense included fee = {}",total_supply);
}
 pub fn pearcef4_toolkit_expense() {
    let burned_in_hour = TOOLKIT_BURNED_MIN * HOUR; // 195 in hour
    let burned_in_day = burned_in_hour * DAY; // 4680 in day
    let burned_in_month = burned_in_day * MONTH; // 140400 in month
    let gatherway_full_tank = TOOLKIT_TANK / burned_in_day; // 8,9 day with one tank
    let gatherway_in_hour = gatherway_full_tank * DAY; // 213,6 hour with full tank
    let pump_up_times_monthly = MONTH * DAY / gatherway_in_hour; // 3,37 times pumped tank
    let filled_tank_by_str_atlas  = TOOLKIT_TANK * TOOLKIT ; // Filled 72,62 star atlas
    let with_fee = burned_in_month * TOOLKIT_FEE;
    let total_supply = pump_up_times_monthly * filled_tank_by_str_atlas + with_fee;
    println!("Pearce_F4 monthly Toolkit expense included fee = {}",total_supply);
}