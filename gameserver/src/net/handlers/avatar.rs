use crate::net::tools::JsonData;

use super::*;

static UNLOCKED_AVATARS: [u32; 62] = [
    8001, 8002, 8003, 8004, 8005, 8006, 1001, 1002, 1003, 1004, 1005, 1006, 1008, 1009, 1013, 1101,
    1102, 1103, 1104, 1105, 1106, 1107, 1108, 1109, 1110, 1111, 1112, 1201, 1202, 1203, 1204, 1205,
    1206, 1207, 1208, 1209, 1210, 1211, 1212, 1213, 1214, 1215, 1217, 1301, 1302, 1303, 1304, 1305,
    1306, 1307, 1308, 1309, 1312, 1315, 1310, 1314, 1315, 1221, 1218, 1220, 1222, 1223,
];

pub async fn on_get_avatar_data_cs_req(
    session: &mut PlayerSession,
    body: &GetAvatarDataCsReq,
) -> Result<()> {
    let json = JsonData::load().await;
    session
        .send(
            CMD_GET_AVATAR_DATA_SC_RSP,
            GetAvatarDataScRsp {
                retcode: 0,
                is_all: body.is_get_all,
                avatar_list: UNLOCKED_AVATARS
                    .iter()
                    .map(|id| {
                        json.avatars
                            .get(id)
                            .map(|v| {
                                v.to_avatar_proto(
                                    json.lightcones.iter().find(|v| v.equip_avatar == *id),
                                    json.relics
                                        .iter()
                                        .filter(|v| v.equip_avatar == *id)
                                        .collect(),
                                )
                            })
                            .unwrap_or(Avatar {
                                base_avatar_id: *id,
                                level: 80,
                                promotion: 6,
                                rank: 6,
                                ..Default::default()
                            })
                    })
                    .collect(),
                ..Default::default()
            },
        )
        .await
}
