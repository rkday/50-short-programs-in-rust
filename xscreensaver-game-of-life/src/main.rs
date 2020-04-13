mod gol;
use gol::GoL;

mod xscreensaver;
use xscreensaver::ScreensaverSetup;

fn main() {
    if let Ok(mut s) = ScreensaverSetup::new() {
        let mut g = GoL::new(s.height_in_cells(), s.width_in_cells());
        let frame_interval = std::time::Duration::from_millis(50);
        loop {
            s.draw_game_of_life(&g);
            std::thread::sleep(frame_interval);
            g = g.next_state();
        }
    } else {
        let mut g = GoL::new(30, 80);
        for _ in 0..1000 {
            g = g.next_state();
        }
        println!("{}", g);
    }
}
