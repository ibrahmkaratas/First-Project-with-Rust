use PearceF4::{pearcef4_food_expense, pearcef4_fuel_expense, pearcef4_ammo_expense, pearcef4_toolkit_expense};
use RainbowOm::{rainbow_food_expense, rainbow_fuel_expense, rainbow_ammo_expense, rainbow_toolkit_expense};

mod RainbowOm;
mod PearceF4;

fn main () {
    rainbow_food_expense();
    rainbow_fuel_expense();
    rainbow_ammo_expense();
    rainbow_toolkit_expense();

    pearcef4_food_expense();
    pearcef4_fuel_expense();
    pearcef4_ammo_expense();
    pearcef4_toolkit_expense();
}