use reda_spice::simulate::{ngspice::NgSpiceShared, Simulate};

fn main_result() -> Result<(), Box<dyn std::error::Error>>{
    let netlist = r#"
        * RC low-pass filter
        V1 in 0 DC 1
        R1 in out 1k
        C1 out 0 1u
        .IC V(out)=0
        .tran 1u 1m
        .end
    "#;

    let mut ngspice = NgSpiceShared::default()?;
    ngspice.run_tran(netlist)?;
    let vout = ngspice.api.get_vec_real_data("v(out)")?.unwrap();
    println!("{:?}", vout);
    
    Ok(())

}

fn main() {
    if let Err(e) = main_result() {
        eprintln!("{}", e);
    }
}