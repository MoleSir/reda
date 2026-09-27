use reda_unit::{u, Current, Resistance, Voltage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MeasurementReport {
    voltage: Voltage,
    current: Current,
    resistance: Resistance,
}

fn main() {
    let v = u!(12.0 V);

    let r = u!(6.0 Ω);

    let i = v / r;

    println!("Voltage: {}", v);
    println!("Resistance: {}", r);
    println!("Current (I = V / R): {}", i);

    let v2 = u!(3.0 V);
    let v_total = v + v2;
    println!("Total voltage (v + v2): {}", v_total);

    let small_current = u!(2.0 mA);
    println!("Small current: {}", small_current);

    let json = r#"
    {
        "voltage": "3.3kV",
        "current": "1.2mA",
        "resistance": "10kΩ"
    }
    "#;

    let report: MeasurementReport = serde_json::from_str(json).unwrap();

    println!("Deserialized:");
    println!("Voltage: {}", report.voltage);
    println!("Current: {}", report.current);
    println!("Resistance: {}", report.resistance);

    let out_json = serde_json::to_string_pretty(&report).unwrap();
    println!("\nRe-serialized JSON:");
    println!("{}", out_json);
}
