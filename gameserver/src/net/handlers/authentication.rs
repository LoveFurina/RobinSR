use anyhow::Result;
use proto::*;

use crate::{net::PlayerSession, util};

pub async fn on_player_get_token_cs_req(
    session: &mut PlayerSession,
    _body: &PlayerGetTokenCsReq,
) -> Result<()> {
    session
        .send(
            CMD_PLAYER_GET_TOKEN_SC_RSP,
            PlayerGetTokenScRsp {
                retcode: 0,
                msg: String::from("OK"),
                uid: 1337,
                ..Default::default()
            },
        )
        .await
}

pub async fn on_player_login_cs_req(
    session: &mut PlayerSession,
    body: &PlayerLoginCsReq,
) -> Result<()> {
    session
        .send(
            CMD_PLAYER_LOGIN_SC_RSP,
            PlayerLoginScRsp {
                retcode: 0,
                login_random: body.login_random,
                server_timestamp_ms: util::cur_timestamp_ms(),
                stamina: 240,
                basic_info: Some(PlayerBasicInfo {
                    nickname: String::from("开拓者"),
                    level: 70, //等级
                    exp: 0,
                    world_level: 6, //均衡等级
                    stamina: 240,
                    mcoin: 0,
                    hcoin: 0,
                    scoin: 0,
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .await
}
