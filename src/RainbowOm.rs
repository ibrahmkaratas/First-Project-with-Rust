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

const FOOD_TANK: f32 = 11790.0;
const FUEL_TANK: f32 = 34770.0;
const AMMO_TANK: f32 = 40120.0;
const TOOLKIT_TANK: f32 = 33700.0;

const FOOD_BURNED_MIN: f32 = 2.44;
const FUEL_BURNED_MIN: f32 = 1.39;
const AMMO_BURNED_MIN: f32 = 2.21;
const TOOLKIT_BURNED_MIN: f32 = 2.67;

 pub fn rainbow_food_expense(){
    let burned_in_hour = FOOD_BURNED_MIN * HOUR ; // 146,4 in hour
    let burned_in_day = burned_in_hour * DAY; // 3513,6 in day
    let burned_in_month = burned_in_day * MONTH; // 105408 in month
    let gatherway_full_tank = FOOD_TANK / burned_in_day; // 3,3555 day with one tank
    let gatherway_in_hour = gatherway_full_tank * DAY; // 80.5 hour with full tank
    let pump_up_times_monthly = MONTH * DAY / gatherway_in_hour; //8,94 times pumped tank
    let filled_tank_by_str_atlas  = FOOD_TANK * FOOD ; // Filled 7,24 star atlas
    let with_fee = burned_in_month * FOOD_FEE;
    let total_supply = pump_up_times_monthly * filled_tank_by_str_atlas + with_fee;
    println!("Rainbow monthly Food expense included fee = {}",total_supply);
 }
  pub fn rainbow_fuel_expense() {
    let burned_in_hour = FUEL_BURNED_MIN * HOUR; // 83,4 in hour
    let burned_in_day = burned_in_hour * DAY; // 2001,6 in day
    let burned_in_month = burned_in_day * MONTH; // 60048 in month
    let gatherway_full_tank = FUEL_TANK / burned_in_day; // 17.37 day with one tank
    let gatherway_in_hour = gatherway_full_tank * DAY; // 416,90 hour with full tank
    let pump_up_times_monthly = MONTH * DAY / gatherway_in_hour; //1,72 times pumped tank
    let filled_tank_by_str_atlas  = FUEL_TANK * FUEL ; // Filled 49,84 star atlas
    let with_fee = burned_in_month * FUEL_FEE;
    let total_supply = pump_up_times_monthly * filled_tank_by_str_atlas + with_fee;
    println!("Rainbow monthly Fuel expense included fee = {}",total_supply);

 }
 pub fn rainbow_ammo_expense() {
    let burned_in_hour = AMMO_BURNED_MIN * HOUR; // 132,6 in hour
    let burned_in_day = burned_in_hour * DAY; // 3182,4 in day
    let burned_in_month = burned_in_day * MONTH; // 95472 in month
    let gatherway_full_tank = AMMO_TANK / burned_in_day; // 12,60  day with one tank
    let gatherway_in_hour = gatherway_full_tank * DAY; // 302,56 hour with full tank
    let pump_up_times_monthly = MONTH * DAY / gatherway_in_hour; // 2,37 times pumped tank
    let filled_tank_by_str_atlas  = AMMO_TANK * AMMO ; // Filled 86,27 star atlas
    let with_fee = burned_in_month * AMMO_FEE;
    let total_supply = pump_up_times_monthly * filled_tank_by_str_atlas + with_fee;
    println!("Rainbow monthly Ammunation expense included fee = {}",total_supply);
}
 pub fn rainbow_toolkit_expense() {
    let burned_in_hour = TOOLKIT_BURNED_MIN * HOUR; // 160,2 in hour
    let burned_in_day = burned_in_hour * DAY; // 3844,8 in day
    let burned_in_month = burned_in_day * MONTH; // 115344 in month
    let gatherway_full_tank = TOOLKIT_TANK / burned_in_day; // 8,76 day with one tank
    let gatherway_in_hour = gatherway_full_tank * DAY; // 210,36 hour with full tank
    let pump_up_times_monthly = MONTH * DAY / gatherway_in_hour; // 3,42 times pumped tank
    let filled_tank_by_str_atlas  = TOOLKIT_TANK * TOOLKIT ; // Filled 58,66 star atlas
    let with_fee = burned_in_month * TOOLKIT_FEE;
    let total_supply = pump_up_times_monthly * filled_tank_by_str_atlas + with_fee;
    println!("Rainbow monthly Toolkit expense included fee = {}",total_supply);

 }
















        
    
