#[derive(Default, Debug)]
pub struct RobotInfo {
    pub penalty: u8,
    pub secs_till_unpenalised: u8,
    pub cautions: u8,
}

#[derive(Default, Debug)]
pub struct TeamInfo {
    pub team_number: u8,
    pub field_player_colour: u8,
    pub goalkeeper_colour: u8,
    pub goalkeeper: u8,
    pub score: u8,
    pub penalty_shot: u8,
    pub single_shots: u16,
    pub message_budget: u16,
    pub players: Vec<RobotInfo>,
}

#[derive(Default, Debug)]
pub struct RoboCupGameControlData {
    pub header: [u8; 4],
    pub version: u8,
    pub packet_number: u8,
    pub players_per_team: u8,
    pub competition_type: u8,
    pub stopped: bool,
    pub game_phase: u8,
    pub state: u8,
    pub set_play: u8,
    pub first_half: bool,
    pub kicking_team: u8,
    pub secs_remaining: i16,
    pub secondary_time: i16,
    pub teams: [TeamInfo; 2],
}
