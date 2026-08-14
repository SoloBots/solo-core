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

    let mut gc_handler = GameControllerHandler::new(4, 2)?;

    loop {
        gc_handler.execute(&mut state.gc_data);
        // ball_detector.execute(&image, &mut ball_percept)
        //...
        gc_handler.execute(&mut state.gc_data);
    }
}
