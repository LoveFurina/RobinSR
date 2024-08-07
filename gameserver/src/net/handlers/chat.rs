use crate::{
    net::{tools::JsonData, PlayerSession},
    util,
};

use super::*;

pub async fn on_get_friend_list_info_cs_req(
    session: &mut PlayerSession,
    _: &GetFriendListInfoCsReq,
) -> Result<()> {
    session
        .send(
            CMD_GET_FRIEND_LIST_INFO_SC_RSP,
            GetFriendListInfoScRsp {
                retcode: 0,
                friend_info_list: vec![FriendInfo {
                    player_simple_info: Some(PlayerSimpleInfo {
                        level: 70,
                        uid: 1314,
                        nickname: String::from("FireFly"),
                        assist_simple_info_list: vec![AssistSimpleInfo {
                            avatar_id: 1310,
                            level: 80,
                            dressed_skin_id: 0,
                            pos: 0,
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .await
}

// RecvMsgCsReq
pub async fn on_send_msg_cs_req(session: &mut PlayerSession, body: &SendMsgCsReq) -> Result<()> {
    let json = JsonData::load().await;

    tracing::info!("message_text：{}", &body.message_text);

    if let Some((cmd, _args)) = parse_command(&body.message_text) {
        match cmd {
            "sx" => {
                sync_player(session, json).await?;
                session
                    .send(
                        CMD_REVC_MSG_SC_NOTIFY,
                        Ohiiebnlkpb {
                            gpcnecedgof: body.gpcnecedgof.clone(),
                            message_type: body.message_type,
                            message_text: String::from("已刷新为新替换的freesr-data.json数据！"),
                            chat_type: body.chat_type,
                            jlddmekleop: 1337, // from
                            nlhlnacaplk: 1314, // to
                            extra_id: body.extra_id,
                        },
                    )
                    .await?;
            }
            _ => {
                sync_player(session, json).await?;
                session
                    .send(
                        CMD_REVC_MSG_SC_NOTIFY,
                        Ohiiebnlkpb {
                            gpcnecedgof: body.gpcnecedgof.clone(),
                            message_type: body.message_type,
                            message_text: String::from("未知指令！"),
                            chat_type: body.chat_type,
                            jlddmekleop: 1337, // from
                            nlhlnacaplk: 1314, // to
                            extra_id: body.extra_id,
                        },
                    )
                    .await?;
            }
        }
    }

    session
        .send(
            CMD_SEND_MSG_SC_RSP,
            SendMsgScRsp {
                retcode: 0,
                end_time: util::cur_timestamp_ms(),
            },
        )
        .await
}

fn parse_command(command: &str) -> Option<(&str, Vec<&str>)> {
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.is_empty() {
        return Option::None;
    }

    Some((parts[0], parts[1..].to_vec()))
}

async fn sync_player(session: &mut PlayerSession, json: JsonData) -> Result<()> {
    session
        .send(
            CMD_PLAYER_SYNC_SC_NOTIFY,
            Fflpjlakgim {
                emfekeedafc: (1..2000).collect(),
                mjbalomkgdg: (2000..3500).collect(),
                diocdcpkolk: vec![],
                ..Default::default()
            },
        )
        .await?;

    session
        .send(
            CMD_PLAYER_SYNC_SC_NOTIFY,
            Fflpjlakgim {
                egbapniokip: Some(Mgigkpibcid {
                    avatar_list: json
                        .avatars
                        .values()
                        .map(|avatar| avatar.to_avatar_proto(Option::None, vec![]))
                        .collect::<Vec<_>>(),
                }),
                ..Default::default()
            },
        )
        .await?;

    session
        .send(
            CMD_PLAYER_SYNC_SC_NOTIFY,
            Fflpjlakgim {
                relic_list: json.relics.iter().map(|v| v.to_relic_proto()).collect(),
                equipment_list: json
                    .lightcones
                    .iter()
                    .map(|v| v.to_equipment_proto())
                    .collect(),
                ..Default::default()
            },
        )
        .await?;

    session
        .send(
            CMD_PLAYER_SYNC_SC_NOTIFY,
            Fflpjlakgim {
                egbapniokip: Some(Mgigkpibcid {
                    avatar_list: json
                        .avatars
                        .values()
                        .map(|avatar| {
                            avatar.to_avatar_proto(
                                json.lightcones
                                    .iter()
                                    .find(|v| v.equip_avatar == avatar.avatar_id),
                                json.relics
                                    .iter()
                                    .filter(|v| v.equip_avatar == avatar.avatar_id)
                                    .collect(),
                            )
                        })
                        .collect(),
                }),
                ..Default::default()
            },
        )
        .await
}
