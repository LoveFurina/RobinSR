mod authentication;
mod avatar;
mod battle;
mod inventory;
mod lineup;
mod mission;
mod player;
mod scene;
mod tutorial;
mod chat;

use anyhow::Result;
use paste::paste;
use proto::*;
use tokio::io::AsyncWriteExt;

use super::PlayerSession;
use crate::net::NetPacket;

pub use authentication::*;
pub use avatar::*;
pub use battle::*;
pub use inventory::*;
pub use lineup::*;
pub use mission::*;
pub use player::*;
pub use scene::*;
pub use tutorial::*;
pub use chat::*;

#[allow(unused_imports)]
use proto::{
    CmdActivityType::*, CmdAdventureType::*, CmdArchiveType::*, CmdAvatarType::*, CmdBattleType::*,
    CmdChallengeType::*, CmdExpeditionType::*, CmdGachaType::*, CmdItemType::*, CmdLineupType::*,
    CmdMailType::*, CmdMessageType::*, CmdMissionType::*, CmdPlayerType::*, CmdPlotType::*,
    CmdQuestType::*, CmdRaidType::*, CmdRogueCommonType::*, CmdRogueType::*, CmdSceneType::*,
    CmdShopType::*, CmdTutorialType::*, CmdWaypointType::*,
};

macro_rules! dummy {
    ($($cmd:ident),* $(,)*) => {
        paste! {
            impl PlayerSession {
                pub const fn should_send_dummy_rsp(cmd_id: u16) -> bool {
                    match cmd_id {
                        $(
                            x if x == [<Cmd $cmd CsReq>] as u16 => true,
                        )*
                        _ => false,
                    }
                }

                pub async fn send_dummy_response(&mut self, req_id: u16) -> Result<()> {
                    let cmd_type = match req_id {
                        $(
                            x if x == [<Cmd $cmd CsReq>] as u16 => [<Cmd $cmd ScRsp>] as u16,
                        )*
                        _ => return Err(anyhow::anyhow!("Invalid request id {req_id:?}")),
                    };

                    let payload: Vec<u8> = NetPacket {
                        cmd_type,
                        head: Vec::new(),
                        body: Vec::new(),
                    }
                    .into();

                    self.client_socket.write_all(&payload).await?;

                    Ok(())
                }
            }
        }
    };
}

dummy! {
    GetLevelRewardTakenList,
    GetRogueScoreRewardInfo,
    GetGachaInfo,
    QueryProductInfo,
    GetQuestData,
    GetQuestRecord,
    GetRogueHandbookData,
    GetActivityScheduleConfig,
    GetMissionData,
    GetMissionEventData,
    GetChallenge,
    GetCurChallenge,
    GetRogueInfo,
    GetExpeditionData,
    SyncClientResVersion,
    GetLoginActivity,
    GetRaidInfo,
    GetTrialActivityData,
    GetNpcStatus,
    GetSpringRecoverData,
    GetSecretKeyInfo,
    GetVideoVersionKey,
    GetCurBattleInfo,
    PlayerLoginFinish,
    InteractProp
}
