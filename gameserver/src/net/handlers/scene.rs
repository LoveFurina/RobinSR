use lazy_static::lazy_static;
use prost::Message;
use tokio::sync::Mutex;

use crate::net::{
    tools::{JsonData, Position},
    tools_res::{PropState, GAME_RESOURCES},
};

use crate::util;

use super::*;

#[derive(Message)]
pub struct Dummy {}

pub async fn on_get_cur_scene_info_cs_req(
    session: &mut PlayerSession,
    _body: &GetCurSceneInfoCsReq,
) -> Result<()> {
    let mut player = JsonData::load().await;
    let entry = player.scene.entry_id;

    let scene = load_scene(session, &mut player, entry, Option::<u32>::None).await;

    let resp = GetCurSceneInfoScRsp {
        retcode: 0,
        scene: if let Ok(scene) = scene {
            Some(scene)
        } else {
            Some(SceneInfo {
                game_mode_type: 1,
                entry_id: player.scene.entry_id,
                plane_id: player.scene.plane_id,
                floor_id: player.scene.floor_id,
                ..Default::default()
            })
        },
    };

    session.send(CMD_GET_CUR_SCENE_INFO_SC_RSP, resp).await?;

    if !player.position.is_empty() {
        session
            .send(
                CMD_SCENE_ENTITY_MOVE_SC_NOTIFY,
                SceneEntityMoveScNotify {
                    entity_id: 0,
                    entry_id: player.scene.entry_id,
                    motion: Some(player.position.to_motion()),
                    ..Default::default()
                },
            )
            .await?;
    };

    Ok(())
}

async fn load_scene(
    _session: &mut PlayerSession,
    json: &mut JsonData,
    entry_id: u32,
    teleport_id: Option<u32>,
) -> Result<SceneInfo> {
    let enterance = GAME_RESOURCES
        .map_entrance
        .get(&entry_id)
        .ok_or_else(|| anyhow::format_err!("Map Entrance Not Found"))?;

    let _plane = GAME_RESOURCES
        .maze_plane
        .get(&enterance.plane_id)
        .ok_or_else(|| anyhow::format_err!("Map Plane Not Found"))?;

    let group_config = GAME_RESOURCES
        .level_group
        .get(&format!("P{}_F{}", enterance.plane_id, enterance.floor_id))
        .ok_or_else(|| anyhow::format_err!("Group Config Not Found"))?;

    let mut position = json.position.clone();
    if let Some(teleport_id) = teleport_id {
        if let Some(teleport) = group_config.teleports.get(&teleport_id) {
            let anchor = group_config
                .group_items
                .get(&teleport.anchor_group_id.unwrap_or_default())
                .and_then(|v| v.anchors.get(&teleport.anchor_id.unwrap_or_default()));
            if let Some(anchor) = anchor {
                position.x = (anchor.pos_x * 1000f64) as i32;
                position.y = (anchor.pos_y * 1000f64) as i32;
                position.z = (anchor.pos_z * 1000f64) as i32;
                position.rot_y = (anchor.rot_y * 1000f64) as i32;
            }
        }
    }

    let mut scene_info = SceneInfo {
        game_mode_type: 1,
        entry_id: json.scene.entry_id,
        plane_id: json.scene.plane_id,
        floor_id: json.scene.floor_id,
        ..Default::default()
    };

    // load player entity
    let mut player_group = SceneGroupInfo {
        state: 1,
        ..Default::default()
    };
    player_group.entity_list.push(SceneEntityInfo {
        inst_id: 0,
        entity_id: 1,
        motion: Some(MotionInfo {
            // pos
            pos: Some(Vector {
                x: json.position.x,
                y: json.position.y,
                z: json.position.z,
            }),
            // rot
            rot: Some(Vector {
                ..Default::default()
            }),
        }),
        actor: Some(SceneActorInfo {
            avatar_type: AvatarType::AvatarFormalType.into(),
            base_avatar_id: *json.lineups.get(&0).unwrap(),
            map_layer: 2,
            uid: 1337,
        }),
        ..Default::default()
    });

    scene_info.scene_group_list.push(player_group);

    let mut _loaded_npc: Vec<u32> = vec![];
    let mut _prop_entity_id = 1_000;
    let mut _npc_entity_id = 20_000;
    let mut _monster_entity_id = 30_000;

    for (group_id, group) in &group_config.group_items {
        let mut group_info = SceneGroupInfo {
            state: 1,
            group_id: *group_id,
            ..Default::default()
        };

        //XXX
        if *group_id == 186 {
            group_info.entity_list.push(SceneEntityInfo {
                inst_id: 300001,
                group_id: *group_id,
                entity_id: 1337,
                prop: Some(ScenePropInfo {
                    prop_id: 808,
                    prop_state: 1,
                    ..Default::default()
                }),
                motion: Some(MotionInfo {
                    // pos
                    pos: Some(Vector {
                        x: json.position.x + 10,
                        y: json.position.y,
                        z: json.position.z + 10,
                    }),
                    // rot
                    rot: Some(Vector { x: 0, y: 0, z: 0 }),
                }),
                ..Default::default()
            });
        }

        // Load Props
        for prop in &group.props {
            let prop_state = if prop.prop_state_list.contains(&PropState::CheckPointEnable) {
                PropState::CheckPointEnable
            } else {
                prop.state.clone()
            };

            _prop_entity_id += 1;

            if _prop_entity_id == 300001 {
                continue;
            }

            let prop_position = Position {
                x: (prop.pos_x * 1000f64) as i32,
                y: (prop.pos_y * 1000f64) as i32,
                z: (prop.pos_z * 1000f64) as i32,
                rot_y: (prop.rot_y * 1000f64) as i32,
            };

            let entity_info = SceneEntityInfo {
                inst_id: prop.id,
                group_id: prop.group_id,
                motion: Some(prop_position.to_motion()),
                prop: Some(ScenePropInfo {
                    prop_id: prop.prop_id,
                    prop_state: prop_state as u32,
                    ..Default::default()
                }),
                entity_id: _prop_entity_id,
                ..Default::default()
            };

            group_info.entity_list.push(entity_info);
        }

        // Load NPCs
        for npc in &group.npcs {
            if _loaded_npc.contains(&(npc.npcid)) || json.avatars.contains_key(&(npc.npcid)) {
                continue;
            }
            _npc_entity_id += 1;
            _loaded_npc.push(npc.npcid);

            let npc_position = Position {
                x: (npc.pos_x * 1000f64) as i32,
                y: (npc.pos_y * 1000f64) as i32,
                z: (npc.pos_z * 1000f64) as i32,
                rot_y: (npc.rot_y * 1000f64) as i32,
            };

            let info = SceneEntityInfo {
                inst_id: npc.id,
                group_id: npc.group_id,
                entity_id: _npc_entity_id,
                motion: Some(npc_position.to_motion()),
                npc: Some(SceneNpcInfo {
                    npc_id: npc.npcid,
                    ..Default::default()
                }),
                ..Default::default()
            };

            group_info.entity_list.push(info);
        }

        // Load Monsters
        for monster in &group.monsters {
            _monster_entity_id += 1;
            let monster_position = Position {
                x: (monster.pos_x * 1000f64) as i32,
                y: (monster.pos_y * 1000f64) as i32,
                z: (monster.pos_z * 1000f64) as i32,
                rot_y: (monster.rot_y * 1000f64) as i32,
            };

            let npc_monster = SceneNpcMonsterInfo {
                monster_id: monster.npcmonster_id,
                event_id: monster.event_id,
                world_level: 6,
                ..Default::default()
            };

            let info = SceneEntityInfo {
                inst_id: monster.id,
                group_id: monster.group_id,
                entity_id: _monster_entity_id,
                motion: Some(monster_position.to_motion()),
                npc_monster: Some(npc_monster),
                ..Default::default()
            };

            group_info.entity_list.push(info);
        }

        scene_info.scene_group_list.push(group_info);
    }

    tracing::info!("scene_info:{:#?}", scene_info);
    Ok(scene_info)
}

lazy_static! {
    static ref NEXT_SCENE_SAVE: Mutex<u64> = Mutex::new(0);
}

pub async fn on_scene_entity_move_cs_req(
    session: &mut PlayerSession,
    request: &SceneEntityMoveCsReq,
) -> Result<()> {
    let mut player = JsonData::load().await;
    let mut timestamp = NEXT_SCENE_SAVE.lock().await;

    if util::cur_timestamp_ms() <= *timestamp {
        session
            .send(CMD_SCENE_ENTITY_MOVE_SC_RSP, Dummy::default())
            .await?;

        return Ok(());
    }

    // save every 5 sec
    *timestamp = util::cur_timestamp_ms() + (5 * 1000);

    for entity in &request.entity_motion_list {
        tracing::info!("entity:{:#?}", entity);

        if entity.entity_id != 1 {
            continue;
        }

        if let Some(motion) = &entity.motion {
            if let Some(pos) = &motion.pos {
                player.position.x = pos.x;
                player.position.y = pos.y;
                player.position.z = pos.z;
            }
            if let Some(rot) = &motion.rot {
                player.position.rot_y = rot.y;
            }
        }
    }

    player.save().await;
    session
        .send(CMD_SCENE_ENTITY_MOVE_SC_RSP, Dummy::default())
        .await
}