use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CarInputs {
    pub total_weight: f32,     // масса автомобиля(кг)
    pub front_weight_pct: f32, // развесовка(%)
    pub power_hp: f32,         // мощность автомобиля(л.с)
    pub tyre_width: f32,       // ширина покрышек(мм)
    pub tyre_profile: f32,     // профиль шин(%)
    pub wheel_diameter: f32,   // диаметр дисков(дюймы)
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct AxisSetup {
    pub spring_height: f32,        // высота пружин (мм)
    pub spring_rate: f32,          // жесткость пружины (кг/мм)
    pub helper_spring_height: f32, // высота подпружинника (мм)
    pub helper_spring_rate: f32,   // жесткость подпружинника (кг/мм)
    pub sway_bar: f32,             // стабилизатор (кг/мм)
    pub bump: f32,                 // сжатие амортизаторов (Н*м/с)
    pub rebound: f32,              // отбой амортизаторов (Н*м/с)
    pub fast_bump: f32,            // быстрое сжатие амортизаторов (Н*м/с)
    pub fast_rebound: f32,         // быстрый отбой амортизаторов (Н*м/с)
    pub gas_pressure: f32,         // сопротивление газа (кг)
    pub camber: f32,               // развал (градусы)
    pub track_width: f32,          // ширина базы (мм)
    pub toe: f32,                  // схождения (градусы)
    pub spacer: f32,               // проставка (мм)
}

#[wasm_bindgen]
pub struct CarSetup {
    pub front: AxisSetup,
    pub rear: AxisSetup,

    pub caster: f32,           // кастер (градусы)
    pub steering_lock: f32,    // выворот (градусы)
    pub kpi: f32,              // поперечный наклон поворота колеса (градусы)
    pub scrub_radius: f32,     // смещение оси колеса (мм)
    pub ackerman_spacer: f32,  // смещение рулевой сошки (мм)
    pub tyre_pressure: f32,    // давление колес (бар)
    pub brake_balance: f32,    // баланс тормозов перед (%)
    pub brake_torque: f32,     // тормозное усилие (Н*м)
    pub handbrake_torque: f32, // тормозное усилие ручника (Н*м)
}

#[wasm_bindgen]
pub fn calculate_setup(inputs: &CarInputs) -> CarSetup {
    // Переводим проценты развесовки в математическую долю.
    let front_bias = inputs.front_weight_pct / 100.0;

    // Вычисляем статическую массу, давящую на переднюю и заднюю ось в состоянии покоя (кг).
    let total_front_mass = inputs.total_weight * front_bias;
    let total_rear_mass = inputs.total_weight * (1.0 - front_bias);

    // Делим массу осей на 2, чтобы получить чистую нагрузку на одно конкретное колесо (кг).
    let mass_per_front_wheel = total_front_mass / 2.0;
    let mass_per_rear_wheel = total_rear_mass / 2.0;

    // Задаем целевую собственную частоту колебаний подвески.
    let f_front: f32 = 2.2;
    let f_rear: f32 = 1.9;

    let pi = std::f32::consts::PI;

    // Базовая физическая формула жесткости упругого элемента: K = 4 * PI^2 * f^2 * m
    // Деление на 9810.0 переводит результат из Н/м в кг/мм.
    let front_spring_rate = (4.0 * pi.powi(2) * f_front.powi(2) * mass_per_front_wheel) / 9810.0;
    let rear_spring_rate = (4.0 * pi.powi(2) * f_rear.powi(2) * mass_per_rear_wheel) / 9810.0;

    // Собираем переднюю ось
    let front_axis = AxisSetup {
        spring_height: 0.0,
        spring_rate: front_spring_rate,
        helper_spring_height: 0.0,
        helper_spring_rate: 0.0,
        sway_bar: 0.0,
        bump: 0.0,
        rebound: 0.0,
        fast_bump: 0.0,
        fast_rebound: 0.0,
        gas_pressure: 0.0,
        camber: 0.0,
        track_width: 0.0,
        toe: 0.0,
        spacer: 0.0,
    };

    // Собираем заднюю ось
    let rear_axis = AxisSetup {
        spring_height: 0.0,
        spring_rate: rear_spring_rate,
        helper_spring_height: 0.0,
        helper_spring_rate: 0.0,
        sway_bar: 0.0,
        bump: 0.0,
        rebound: 0.0,
        fast_bump: 0.0,
        fast_rebound: 0.0,
        gas_pressure: 0.0,
        camber: 0.0,
        track_width: 0.0,
        toe: 0.0,
        spacer: 0.0,
    };

    CarSetup {
        front: front_axis,
        rear: rear_axis,
        caster: 0.0,
        steering_lock: 0.0,
        kpi: 0.0,
        scrub_radius: 0.0,
        ackerman_spacer: 0.0,
        tyre_pressure: 0.0,
        brake_balance: 0.0,
        brake_torque: 0.0,
        handbrake_torque: 0.0,
    }
}
