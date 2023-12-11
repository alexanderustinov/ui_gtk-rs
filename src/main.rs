use gtk::{glib, prelude::*};

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.me.mine")
        .build();

    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .default_width(600)
        .default_height(600)
        .application(app)
        .title("Mandelbrot")
        .build();

    let drawing_area = gtk::DrawingArea::builder().build();

    drawing_area.set_draw_func(move |_, cr, w, h| {
        cr.set_source_rgb(0., 0., 0.);
        cr.paint().expect("Invalid cairo surface state");
        let max_iterations = 20;
        let gap = 5;
        for c in gap..w - gap {
            let x0 = ((c as f32) / (w as f32 - gap as f32)) * 3.5 - 2.5;
            for r in gap..h - gap {
                let y0 = ((r as f32) / (h as f32 - gap as f32)) * 2.0 - 1.0;
                let mut x = 0.0;
                let mut y = 0.0;
                let mut iteration: u32 = 0;
                while x * x + y * y <= 4.0 && iteration < max_iterations {
                    let xtemp = x * x - y * y + x0;
                    y = 2.0 * x * y + y0;
                    x = xtemp;
                    iteration = iteration + 1;
                }
                cr.set_source_rgba(1., 1., 1., (iteration / max_iterations).into());
                cr.rectangle(c.into(), r.into(), 1., 1.);
                cr.fill();
            }
        }
    });

    window.set_child(Some(&drawing_area));
    window.present();
}
