use crate::representations::{RoboCupGameControlData, TeamInfo, RobotInfo};

use std::net::UdpSocket;

const GAMECONTROLLER_HEADER: &[u8; 4] = b"RGme";
const GC_PORT: u16 = 3838;
const GC_RETURN_PORT: u16 = 3939;

struct Reader<'a> {
    slice: &'a [u8],
}

impl<'a> Reader<'a> {
    fn new(slice: &'a [u8]) -> Self {
        Self { slice }
    }

    fn read_u8(&mut self) -> Option<u8> {
        let (val, rest) = self.slice.split_first()?;
        self.slice = rest;
        Some(*val)
    }

    fn read_u16(&mut self) -> Option<u16> {
        if self.slice.len() < 2 { return None; }
        let val = u16::from_le_bytes([self.slice[0], self.slice[1]]);
        self.slice = &self.slice[2..];
        Some(val)
    }

    fn read_i16(&mut self) -> Option<i16> {
        if self.slice.len() < 2 { return None; }
        let val = i16::from_le_bytes([self.slice[0], self.slice[1]]);
        self.slice = &self.slice[2..];
        Some(val)
    }

    fn read_team(&mut self) -> Option<TeamInfo> {
        let team_number = self.read_u8()?;
        let field_player_colour = self.read_u8()?;
        let goalkeeper_colour = self.read_u8()?;
        let goalkeeper = self.read_u8()?;
        let score = self.read_u8()?;
        let penalty_shot = self.read_u8()?;
        let single_shots = self.read_u16()?;
        let message_budget = self.read_u16()?;

        let mut players = Vec::with_capacity(20);
        for _ in 0..20 {
            players.push(RobotInfo {
                penalty: self.read_u8()?,
                secs_till_unpenalised: self.read_u8()?,
                cautions: self.read_u8()?,
            });
        }

        Some(TeamInfo {
            team_number,
            field_player_colour,
            goalkeeper_colour,
            goalkeeper,
            score,
            penalty_shot,
            single_shots,
            message_budget,
            players,
        })
    }
}



pub struct GameControllerHandler {
    socket: UdpSocket,
    buf: [u8; 2048],
    team_num: u8,
    player_num: u8,
}

impl GameControllerHandler{

    pub fn new(team_num: u8, player_num: u8) -> std::io::Result<Self>{
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", GC_PORT))?;
        socket.set_nonblocking(true)?;

        Ok(Self {
            socket,
            buf: [0u8; 2048],
            team_num,
            player_num,
        })
    }

    pub fn parse(bytes: &[u8]) -> Option<RoboCupGameControlData> {
        let mut reader = Reader::new(bytes);

        let header = [
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
            reader.read_u8()?,
        ];
        let version = reader.read_u8()?;
        let packet_number = reader.read_u8()?;
        let players_per_team = reader.read_u8()?;
        let competition_type = reader.read_u8()?;
        let stopped = reader.read_u8()? != 0;
        let game_phase = reader.read_u8()?;
        let state = reader.read_u8()?;
        let set_play = reader.read_u8()?;
        let first_half = reader.read_u8()? != 0;
        let kicking_team = reader.read_u8()?;
        let secs_remaining = reader.read_i16()?;
        let secondary_time = reader.read_i16()?;

        let team1 = reader.read_team()?;
        let team2 = reader.read_team()?;

        Some(RoboCupGameControlData {
            header,
            version,
            packet_number,
            players_per_team,
            competition_type,
            stopped,
            game_phase,
            state,
            set_play,
            first_half,
            kicking_team,
            secs_remaining,
            secondary_time,
            teams: [team1, team2],
        })
    }

    pub fn execute(&mut self, gc: &mut RoboCupGameControlData) -> std::io::Result<()> {
        // Receive packet using pre-allocated buffer
        let (number_of_bytes, src_addr) = match self.socket.recv_from(&mut self.buf) {
            Ok(res) => res,
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // Return early if no packet is available (when set_nonblocking is true)
                return Ok(());
            }
            Err(e) => return Err(e),
        };

        if let Some(gc_data) = GameControllerHandler::parse(&self.buf[..number_of_bytes]){
            //TODO we could check for valid data
            println!(
                "[{src_addr}] Secs Left: {}s | Score: {} - {} | State: {} | {} vs {}",
                gc_data.secs_remaining,
                gc_data.teams[0].score,
                gc_data.teams[1].score,
                gc_data.state,
                gc_data.teams[0].team_number,
                gc_data.teams[1].team_number
            );

            let pose_x = 0.0f32.to_le_bytes();
            let pose_y = 0.0f32.to_le_bytes();
            let pose_theta = 0.0f32.to_le_bytes();
            let ball_age = (-1.0f32).to_le_bytes();
            let ball_x = 0.0f32.to_le_bytes();
            let ball_y = 0.0f32.to_le_bytes();

            let return_data: [u8; 32] = [
                b'R', b'G', b'r', b't',
                4,
                1,
                4,
                0,
                pose_x[0], pose_x[1], pose_x[2], pose_x[3],
                pose_y[0], pose_y[1], pose_y[2], pose_y[3],
                pose_theta[0], pose_theta[1], pose_theta[2], pose_theta[3],
                ball_age[0], ball_age[1], ball_age[2], ball_age[3],
                ball_x[0], ball_x[1], ball_x[2], ball_x[3],
                ball_y[0], ball_y[1], ball_y[2], ball_y[3],
            ];


            let mut target_addr = src_addr;
            target_addr.set_port(GC_RETURN_PORT);
            self.socket.send_to(&return_data, target_addr)?;
        }

        Ok(())
    }

}