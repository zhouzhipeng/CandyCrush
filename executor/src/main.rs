//! Executor with your game connected to it as a plugin.
use fyrox::engine::executor::Executor;

fn main() {
    let mut executor = Executor::new();
   
    // Dynamic linking with hot reloading.
    #[cfg(feature = "dylib")]
    {
        #[cfg(target_os = "windows")]
        let file_name = "game_dylib.dll";
        #[cfg(target_os = "linux")]
        let file_name = "libgame_dylib.so";
        #[cfg(target_os = "macos")]
        let file_name = "libgame_dylib.dylib";
        executor.add_dynamic_plugin(file_name, true, true).unwrap();
    }

    // Static linking.
    #[cfg(not(feature = "dylib"))]
    {
        use CandyCrush::Game;
        executor.add_plugin(Game::default());
    }  
   
    executor.run()
}