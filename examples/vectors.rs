
use gl_lib::typedef::*;

fn main() {

    let v = V3::new(0.0, 0.0, 1.0); // (dir)

    let a = V3::new(1.0, 0.0, 0.0); //(perp)

    // know to rotate perp1 90 degress around dir
    // go get another vector in same plane

    let v_dot = v.normalize();

    let apara = v_dot * a.dot(&v_dot);
    let aperp = a + apara * -1.0;
    let across = aperp.cross(&v_dot);

    println!("{:?}",  (apara, aperp, across, a.cross(&v_dot)));


    let theta = std::f32::consts::PI;
    let mut b = apara + aperp * theta.cos();
    b = b + across * theta.sin();


    println!("{:.3?}", b);


}
