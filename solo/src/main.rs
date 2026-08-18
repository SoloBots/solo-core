mod representations;
mod modules;

use representations::{BallPercept, CameraImage, RobotPose, RoboCupGameControlData, TeamInfo, RobotInfo};
use modules::{GameControllerHandler};

#[derive(Default)]
struct Blackboard{
    // TODO add rest of representation here
    gc_data: RoboCupGameControlData,

}


fn main() -> std::io::Result<()> {    
    // initialize the blackboard
    let mut state = Blackboard::default();

    let mut frame_count = 0u64;

    let mut gc_handler = GameControllerHandler::new(4, 2)?;

    loop {
        frame_count += 1;
        gc_handler.execute(&mut state.gc_data);
        

        if frame_count % 33 == 0 {
            println!("[Frame {}]", frame_count);
        }

        // Logging
        // call logging module
    }
}
