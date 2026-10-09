import init, { calculate_setup, CarInputs } from "./pkg/calc_carx_dro2.js";

async function run() {
  await init();

  document.getElementById("calcBtn").addEventListener("click", () => {
    const inputs = new CarInputs();
    inputs.total_weight = parseFloat(document.getElementById("weight").value);
    inputs.front_weight_pct = parseFloat(document.getElementById("bias").value);
    inputs.power_hp = parseFloat(document.getElementById("power").value);
    inputs.tyre_width = parseFloat(document.getElementById("width").value);
    inputs.tyre_profile = parseFloat(document.getElementById("profile").value);
    inputs.wheel_diameter = parseFloat(
      document.getElementById("diameter").value,
    );

    const result = calculate_setup(inputs);

    document.getElementById("frontSpring").innerText =
      result.front.spring_rate.toFixed(2);
    document.getElementById("rearSpring").innerText =
      result.rear.spring_rate.toFixed(2);

    inputs.free();
    result.free();
  });
}

run();
