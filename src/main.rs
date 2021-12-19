mod constants;
mod sdl_wrapper;

use constants::*;
use sdl_wrapper::{Event, Keycode, ScreenContextManager};
use std::time::SystemTime;

fn main() {
    // Inicializar context manager de la ventana
    let mut screen = ScreenContextManager::new("Tarea1");

    'main: loop {
        // Tomar segundos ( módulo 256 )
        let secs = (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            * 10) as f64;

        // Cuadrante IV
        screen.set_color(
            (((secs) % 256.0) / 255.0) as f32,
            (((secs + 50.0) % 256.0) / 255.0) as f32,
            (((secs + 50.0) % 256.0) / 255.0) as f32,
        );
        for y in WINDOW_HEIGHT / 2..WINDOW_HEIGHT {
            for x in WINDOW_WIDTH / 2..WINDOW_WIDTH {
                // Dibujar cada pixel con color dependiendo de segundos
                screen.plot_pixel(x, y);
            }
        }

        // Cuadrante III
        screen.set_color(0.0, 0.0, 1.0);
        for y in WINDOW_HEIGHT / 2..WINDOW_HEIGHT {
            for x in 0..WINDOW_WIDTH / 2 {
                // Dibujar azul
                screen.plot_pixel(x, y);
            }
        }

        // Cuadrante I
        screen.set_color(0.0, 1.0, 0.0);
        for y in 0..WINDOW_HEIGHT / 2 {
            for x in WINDOW_WIDTH / 2..WINDOW_WIDTH {
                // Dibujar verde
                screen.plot_pixel(x, y);
            }
        }

        // Cuadrante II
        screen.set_color(1.0, 0.0, 0.0);
        for y in 0..WINDOW_HEIGHT / 2 {
            for x in 0..WINDOW_WIDTH / 2 {
                // Dibujar rojo
                screen.plot_pixel(x, y);
            }
        }

        screen.present();

        // Manejo de eventos
        for event in screen.get_events() {
            match event {
                // Salirse del programa si se cierra la ventana o estripa Esc
                Event::Quit { .. } => break 'main,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => (),
            }
        }
    }
}
