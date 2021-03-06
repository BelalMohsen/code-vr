use pyro::{Actor, EngineState};
use pyro::gfx::{Camera, CameraProps, ProjectionMode};
use std::sync::{Arc, Mutex};

/// A Player in the CodeVR scene, handles its own movement and editing the CodeVR scene.
pub struct Player {
    // Local State
    health: [i32; 2],
    // Engine State
    camera: Option<Arc<Mutex<Camera>>>,
    engine: Option<EngineState>,
}

/// Local Logic
impl Player {
    pub fn new() -> Player {
        Player {
            health: [100, 100],
            camera: None,
            engine: None,
        }
    }
}

/// Actor Logic
impl Actor for Player {
    // Mount engine state
    fn start(&mut self, mut engine: EngineState) {

        {
            // Add reference to camera
            let mut gfx = engine.gfx.lock().unwrap();
            self.camera = Some(gfx.camera(CameraProps {
                                              projection_mode: ProjectionMode::Perspective,
                                              to: [4., 4., 4.],
                                              from: [0., 0., 0.],
                                              fov: 75.0,
                                          }));
        }

        self.engine = Some(engine);

    }

    // Update Engine State
    fn update(&mut self) {

        if let Some(ref mut engine) = self.engine {

            // Destroy self if we're out of health
            if self.health[0] < 1 {
                //engine.scene.destroy(|actor| true);
            }

            // Check inputs
            if let Some(ref mut camera) = self.camera {
                let cam_x = engine.input_axis(String::from("move_right"));
                let cam_y = engine.input_axis(String::from("move_forward"));
                if let Ok(mut cam) = camera.lock() {
                    cam.rotate([cam_x, cam_y, 0.]);
                }
            }


        }

    }
}

