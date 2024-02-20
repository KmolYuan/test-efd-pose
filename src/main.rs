use four_bar::{csv, efd, plot::*};

fn main() {
    let w = std::fs::File::open("test2.csv").unwrap();
    let curve = csv::from_reader(w).unwrap();
    // const CURVE2D: &[[f64; 2]] = &[
    //     [1.0, 1.0],
    //     [2.0, 1.5],
    //     [3.0, 2.0],
    //     [3.5, 3.0],
    //     [3.0, 4.0],
    //     [2.5, 5.0],
    //     [2.0, 4.5],
    //     [1.5, 4.0],
    //     [1.0, 3.0],
    //     [1.5, 2.0],
    // ];
    // let curve = CURVE2D.to_vec();
    // let curve = efd::tests::CURVE2D.to_vec();
    let efd = efd::Efd2::from_curve_harmonic(&curve, false, 8);
    let curve_recon = efd.generate_norm(90);
    let (t, _) = efd::get_target_pos(&curve, false);
    let curve = efd.as_geo().inverse().transform(curve);
    let t_norm = efd.generate_norm_by_t(&t);
    fb::Figure::new(None)
        .add_line("EFD Recon.", vec![curve_recon[0]], Style::Circle, GREEN)
        .add_line("", &curve_recon[..85], Style::Line, GREEN)
        .add_line("Target", vec![curve[0]], Style::Circle, BLUE)
        .add_line("", &curve[..], Style::Line, BLUE)
        .add_line("t-θ+π", vec![t_norm[0]], Style::Circle, RED)
        .add_line("", &t_norm[..], Style::DashDottedLine, RED)
        .legend(LegendPos::UL)
        .plot(SVGBackend::new("test.svg", (1600, 1600)))
        .unwrap();
}
