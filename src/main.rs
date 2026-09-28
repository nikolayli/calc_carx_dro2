use std::io::{self, Write};

struct CarInputs {
    total_weight: f32,
    front_weight_pct: f32, // Будем вводить в процентах, например 54.0
    power_hp: f32,
    tyre_width: f32,       // Изменил на f32 для удобства ввода
    tyre_profile: f32,     // Изменил на f32
    wheel_diameter: f32,   // Изменил на f32
}

#[derive(Debug)]
struct AxisSetup {
    spring_height: f32,        // высота пружин (мм)
    spring_rate: f32,          // жесткость пружины (кг/мм)
    helper_spring_height: f32, // высота подпружинника (мм)
    helper_spring_rate: f32,   // жесткость подпружинника (кг/мм)
    sway_bar: f32,             // стабилизатор (кг/мм)

    bump: f32,                 // сжатие амортизаторов (Н*м/с)
    rebound: f32,              // отбой амортизаторов (Н*м/с)
    fast_bump: f32,            // быстрое сжатие амортизаторов (Н*м/с)
    fast_rebound: f32,         // быстрый отбой амортизаторов (Н*м/с)
    gas_pressure: f32,         // сопротивление газа (кг)

    camber: f32,               // развал (градусы)
    track_width: f32,          // ширина базы (мм)
    toe: f32,                  // схождения (градусы)
    spacer: f32,               // проставка (мм)
}

#[derive(Debug)]
struct CarSetup {
    front: AxisSetup,
    rear: AxisSetup,

    caster: f32,               // кастер (градусы)
    steering_lock: f32,        // выворот (градусы)
    kpi: f32,                  // поперечный наклон поворота колеса (градусы)
    scrub_radius: f32,         // смещение оси колеса (мм)
    ackerman_spacer: f32,      // смещение рулевой сошки (мм)

    tyre_pressure: f32,        // давление колес (бар)

    brake_balance: f32,        // баланс тормозов перед (%)
    brake_torque: f32,         // тормозное усилие (Н*м)
    handbrake_torque: f32,     // тормозное усилие ручника (Н*м)
}

fn read_input(prompt: &str) -> f32 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");

    input.trim().parse::<f32>().unwrap_or_else(|_| {
        println!("Ошибка: введено не число! Используем 0.0 по умолчанию.");
        0.0
    })
}

fn calculate_setup(inputs: &CarInputs) -> CarSetup {
    // Переводим введенные проценты (например 54.0) в доли (0.54)
    let front_bias = inputs.front_weight_pct / 100.0;

    let front_mass = inputs.total_weight * front_bias;
    let rear_mass = inputs.total_weight * (1.0 - front_bias);

    // --- 1. РАСЧЕТ ПРУЖИН (в кг/мм) ---
    let mut front_spring = (front_mass / 2.0) / 75.0;
    let mut rear_spring = (rear_mass / 2.0) / 85.0;

    // Исправлено: проверяем ШИРИНУ шины (tyre_width) вместо профиля
    if inputs.tyre_width > 245.0 {
        front_spring += 0.5;
        rear_spring += 0.8;
    }

    let front_spring = front_spring.clamp(2.1, 22.4);
    let rear_spring = rear_spring.clamp(1.6, 22.4);

    let front_sway = (front_spring * 1.2).clamp(0.0, 20.4);
    let rear_sway = (rear_spring * 0.8).clamp(0.0, 20.4);

    // --- 2. РАСЧЕТ АМОРТИЗАТОРОВ (Н*м/с) ---
    let front_bump_val = (front_spring * 800.0).clamp(500.0, 20000.0);
    let front_rebound_val = (front_bump_val * 1.6).clamp(500.0, 20000.0);

    let rear_bump_val = (rear_spring * 700.0).clamp(500.0, 20000.0);
    let rear_rebound_val = (rear_bump_val * 1.5).clamp(500.0, 20000.0);

    // --- 3. ДАВЛЕНИЕ В ШИНАХ (бар) ---
    let rear_press = if inputs.power_hp > 500.0 { 1.8 } else { 2.4 };

    // --- 4. СБОРКА ОСЕЙ ---
    let front_axis = AxisSetup {
        spring_height: 210.0,
        spring_rate: front_spring,
        helper_spring_height: 0.0,
        helper_spring_rate: 0.0,
        sway_bar: front_sway,
        bump: front_bump_val,
        rebound: front_rebound_val,
        fast_bump: front_bump_val * 0.75,
        fast_rebound: front_rebound_val * 0.75,
        gas_pressure: 30.0,
        camber: -6.0,
        toe: 0.15,
        track_width: 50.0,
        spacer: 20.0,
    };

    let rear_axis = AxisSetup {
        spring_height: 220.0,
        spring_rate: rear_spring,
        helper_spring_height: 0.0,
        helper_spring_rate: 0.0,
        sway_bar: rear_sway,
        bump: rear_bump_val,
        rebound: rear_rebound_val,
        fast_bump: rear_bump_val * 0.70,
        fast_rebound: rear_rebound_val * 0.70,
        gas_pressure: 25.0,
        camber: -0.5,
        toe: -0.10,
        track_width: 30.0,
        spacer: 10.0,
    };

    CarSetup {
        front: front_axis,
        rear: rear_axis,
        caster: 8.0,
        steering_lock: 65.0,
        kpi: 14.0,
        scrub_radius: 10.0,
        ackerman_spacer: 0.0,
        tyre_pressure: rear_press,
        brake_balance: 65.0,
        brake_torque: 6000.0,
        handbrake_torque: 5500.0,
    }
}

fn main() {
    println!("=== CALC_CARX_DRO2 v1.0 ===");
    println!("Введите параметры вашего авто для расчета базового сетапа:\n");

    let total_weight = read_input("1. Полный вес машины (кг): ");
    let front_weight_pct = read_input("2. Вес на передней оси (%): ");
    let power_hp = read_input("3. Мощность двигателя (л.с.): ");
    let tyre_width = read_input("4. Ширина покрышек (мм): ");
    let tyre_profile = read_input("5. Профиль покрышек (%): ");
    let wheel_diameter = read_input("6. Диаметр дисков (дюймы): ");

    let user_car = CarInputs {
        total_weight,
        front_weight_pct,
        power_hp,
        tyre_width,
        tyre_profile,
        wheel_diameter,
    };

    let setup = calculate_setup(&user_car);

    println!("\n====== РЕКОМЕНДУЕМЫЕ НАСТРОЙКИ ДЛЯ ИГРЫ ======");
    println!("Пружины:");
    println!("  Передняя ось: Жесткость = {:.2} кг/мм | Стабилизатор = {:.2} кг/мм", setup.front.spring_rate, setup.front.sway_bar);
    println!("  Задняя ось:   Жесткость = {:.2} кг/мм | Стабилизатор = {:.2} кг/мм", setup.rear.spring_rate, setup.rear.sway_bar);
    println!("----------------------------------------------");
    println!("Амортизаторы (Перед / Зад):");
    println!("  Сжатие:         {:.0} / {:.0} Н*м/с", setup.front.bump, setup.rear.bump);
    println!("  Отбой:          {:.0} / {:.0} Н*м/с", setup.front.rebound, setup.rear.rebound);
    println!("  Быстрое сжатие: {:.0} / {:.0} Н*м/с", setup.front.fast_bump, setup.rear.fast_bump);
    println!("----------------------------------------------");
    println!("Геометрия подвески:");
    println!("  Развал: Перед = {:.1} / Зад = {:.1} град.", setup.front.camber, setup.rear.camber);
    println!("  Выворот перед: {:.0} град. | Кастер: {:.1} град.", setup.steering_lock, setup.caster);
    println!("----------------------------------------------");
    println!("Шины и тормоза:");
    println!("  Давление задних колес: {:.1} бар", setup.tyre_pressure);
    println!("  Баланс тормозов перед: {:.0}%", setup.brake_balance);
}
