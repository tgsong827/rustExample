use super::print_current_module_path;

pub fn execute_example() {
    print_current_module_path(module_path!());

    let age = Years(5);
    let age_days = age.to_day();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // println!("Old enough {}", old_enough(&age_days));

    //// new type
    let years = Years(42);
    let years_as_primitive_1: i64 = years.0;
    let Years(years_as_primitive_2) = years;
}

struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_day(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}