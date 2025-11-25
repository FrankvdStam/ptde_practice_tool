use mem_rs::prelude::*;

struct SendProcess(Process);

unsafe impl Send for SendProcess {}
unsafe impl Sync for SendProcess {}

struct SendPointer(Pointer);
unsafe impl Sync for SendPointer {}

unsafe impl Send for SendPointer {}

pub struct ChrDbg;
impl ChrDbg {
    pub const ALL_NO_STAMINA_CONSUME: usize = 0;
    pub const ALL_NO_MPCONSUME: usize = 1;
    pub const ALL_NO_ARROW_CONSUME: usize = 2;
    pub const PLAYER_HIDE: usize = 3;
    pub const PLAYER_SILENCE: usize = 4;
    pub const ALL_NO_DEAD: usize = 5;
    pub const ALL_NO_DAMAGE: usize = 6;
    pub const ALL_NO_HIT: usize = 7;
    pub const ALL_NO_ATTACK: usize = 8;
    pub const ALL_NO_MOVE: usize = 9;
    pub const ALL_NO_UPDATE_AI: usize = 0xA;
}

pub struct DrawMap;
impl DrawMap {
    pub const DRAW_MAP: usize = 0;
    pub const DRAW_OBJECTS: usize = 0;
    pub const DRAW_CREATURES: usize = 0;
    pub const DRAW_SFX: usize = 0;
}

pub struct CharData1;
impl CharData1 {
    pub const CHAR_MAP_DATA_PTR: usize = 0x28;
    pub const CHR_TYPE: usize = 0x70;
    pub const TEAM_TYPE: usize = 0x74;
    pub const FORCE_PLAY_ANIMATION: usize = 0xFC;
    pub const CHAR_FLAGS_1: usize = 0x1FC;
    pub const PLAY_REGION: usize = 0x284;
    pub const HP: usize = 0x2D4;
    pub const STAMINA: usize = 0x2E4;
    pub const CHAR_FLAGS_2: usize = 0x3C4;
    pub const STORED_ITEM: usize = 0x628;
}

pub struct CharMapData;
impl CharMapData {
    pub const ANIM_DATA_PTR: usize = 0x14;
    pub const CHAR_POS_DATA_PTR: usize = 0x1C;
    pub const CHAR_MAP_FLAGS: usize = 0xC4;
    pub const WARP: usize = 0xC8;
    pub const WARP_X: usize = 0xD0;
    pub const WARP_Y: usize = 0xD4;
    pub const WARP_Z: usize = 0xD8;
    pub const WARP_ANGLE: usize = 0xD4;
}

pub struct AnimData;
impl AnimData {
    pub const PLAY_SPEED: usize = 0x64;
}

pub struct CharPosData;
impl CharPosData {
    pub const POS_ANGLE: usize = 0x4;
    pub const POS_X: usize = 0x10;
    pub const POS_Y: usize = 0x14;
    pub const POS_Z: usize = 0x18;
}

pub struct Ds1 {
    pos_lock_aob: String,
    pos_lock_1_aob_offset: usize,
    pos_lock_2_aob_offset: usize,

    node_graph_aob: String,
    node_graph_aob_offset: usize,

    all_no_magic_qty_consume_aob: String,
    all_no_magic_qty_consume_aob_offset: usize,

    player_no_dead_aob: String,
    player_no_dead_aob_offset: usize,

    player_exterminate_aob: String,
    player_exterminate_aob_offset: usize,

    all_no_stamina_consume_aob: String,
    all_no_stamina_consume_aob_offset: usize,

    //Skipped draw compass for now
    draw_map_aob: String,
    draw_map_aob_offset: usize,

    char_data_1_aob: String,
    char_data_1_aob_offset: usize,
    char_data_1_offset1: usize,
    char_data_1_offset2: usize,
    char_data_1_offset3: usize,

    process: SendProcess,
    chr_dbg: SendPointer,
    pos_lock: SendPointer,                      // 0x16 0x27
    node_graph: SendPointer,                    // 0x12
    all_no_magic_quantity_consume: SendPointer, // 0x2
    player_no_dead: SendPointer,                // 0x22
    player_exterminate: SendPointer,            // 0x10
    all_no_stamina_consume: SendPointer,        // 0x18
    compass: SendPointer,                       // 0xC 0x15, 0x1E
    chr_data_1: SendPointer,                    // 0x2, 0x0, 0x4, 0x0
    char_map_data: SendPointer,                 // chr_data_1 (aob), 0x2, 0x0, 0x4, 0x0 0x2
    chr_data_2: SendPointer,
    char_pos_data: SendPointer, // 0x1, 0x0, 0x8
    no_stam_consume: bool,
}

impl Ds1 {
    pub fn new() -> Self {
        hudhook::alloc_console().ok();
        let mut ds1struct = Ds1 {

            pos_lock_aob: "F3 0F 11 44 24 08 F3 0F 11 0C 24 F3 0F 11 54 24 04 F3 0F 7E 04 24".to_string(),
            pos_lock_1_aob_offset: 0x16,
            pos_lock_2_aob_offset: 0x27,

            node_graph_aob: "8B 4C 24 5C 8B 11 50 8B 42 34 FF D0 80 BB 90 00 00 00 ?".to_string(),
            node_graph_aob_offset: 0x12,

            all_no_magic_qty_consume_aob: "38 1D ? ? ? ? 0F 94 C1 3A CB".to_string(),
            all_no_magic_qty_consume_aob_offset: 0x2,

            player_no_dead_aob: "53 56 8B F0 8A 9E C4 03 00 00 8B 06 8B 90 A4 00 00 00 C0 EB 05 8B CE 80 E3 01 FF D2 84 C0 ? ? 80 3D ? ? ? ? 00".to_string(),
            player_no_dead_aob_offset: 0x22,

            player_exterminate_aob: "8B 11 8B 82 A4 00 00 00 FF D0 84 C0 ? ? 80 3D ? ? ? ? 00".to_string(),
            player_exterminate_aob_offset: 0x10,

            all_no_stamina_consume_aob: "51 8B 4C 24 08 3B 8A E4 02 00 00 ? ? F6 82 C5 03 00 00 04 ? ? 80 3D ? ? ? ? 00".to_string(),
            all_no_stamina_consume_aob_offset: 0x18,

            draw_map_aob : "80 3D ? ? ? ? 00 A1 ? ? ? ? 8B 48 08 8B 11 56 8B 72 28 B8 00 00 00 80".to_string(),
            draw_map_aob_offset: 0x2,

            char_data_1_aob : "8B 15 ? ? ? ? F3 0F 10 44 24 30 52".to_string(),
            char_data_1_aob_offset : 0x2,
            char_data_1_offset1 : 0x0,
            char_data_1_offset2 : 0x4,
            char_data_1_offset3 : 0x0,







            process: SendProcess(Process::new("DARKSOULS.exe")),
            chr_dbg: SendPointer(Pointer::default()),
            pos_lock: SendPointer(Pointer::default()),
            node_graph: SendPointer(Pointer::default()),
            all_no_magic_quantity_consume: SendPointer(Pointer::default()),
            player_no_dead: SendPointer(Pointer::default()),
            player_exterminate: SendPointer(Pointer::default()),
            all_no_stamina_consume: SendPointer(Pointer::default()),
            compass: SendPointer(Pointer::default()),
            chr_data_1: SendPointer(Pointer::default()),
            char_map_data: SendPointer(Pointer::default()),
            chr_data_2: SendPointer(Pointer::default()),
            char_pos_data: SendPointer(Pointer::default()),
            no_stam_consume: false,
        };
        Self::refresh(&mut ds1struct);
        ds1struct
    }
    // Pointers are declared here
    pub fn refresh(&mut self) -> Result<(), String> {
        if !self.process.0.is_attached() {
            self.process.0.refresh()?;

            self.chr_data_2 = SendPointer(self.process.0.scan_abs(
                "chr_data_2",
                "8B 15 ? ? ? ? F3 0F 10 44 24 30 52",
                2,
                vec![0x0, 0x0, 0x4, 0x0],
            )?);
            self.chr_dbg = SendPointer(self.process.0.scan_abs(
                "all_no_stamina_consume",
                &self.all_no_stamina_consume_aob,
                self.all_no_stamina_consume_aob_offset,
                vec![0x0],
            )?);

            self.chr_data_1 = SendPointer(self.process.0.scan_abs(
                "chr_data_1",
                &self.char_data_1_aob,
                self.char_data_1_aob_offset,
                vec![
                    self.char_data_1_offset1,
                    self.char_data_1_offset2,
                    self.char_data_1_offset3,
                ],
            )?);

            self.char_map_data = SendPointer(self.process.0.scan_abs(
                "chr_map_data",
                &self.char_data_1_aob,
                self.char_data_1_aob_offset,
                vec![
                    self.char_data_1_offset1,
                    self.char_data_1_offset2,
                    self.char_data_1_offset3,
                    CharData1::CHAR_MAP_DATA_PTR,
                ],
            )?);

            self.char_pos_data = SendPointer(self.process.0.scan_abs(
                "chr_pos_data",
                &self.char_data_1_aob,
                self.char_data_1_aob_offset,
                vec![
                    0x0,
                    self.char_data_1_offset1,
                    self.char_data_1_offset2,
                    self.char_data_1_offset3,
                    CharData1::CHAR_MAP_DATA_PTR,
                    CharMapData::CHAR_POS_DATA_PTR,
                ],
            )?);

            self.char_pos_data.0.debug = true;
        } else {
            self.process.0.refresh()?;
        }

        Ok(())
    }

    pub fn get_HP(&self) -> u32 {
        let hp = self.chr_data_2.0.read_u32_rel(Some(0x2d4));
        print!("{:?}", hp);
        return hp;
    }

    pub fn get_Stamina(&self) -> u32 {
        let stam = self.chr_data_2.0.read_u32_rel(Some(0x2e4));
        return stam;
    }


    pub fn get_x_pos(&self) ->f32 {
        let x_pos = self.char_pos_data.0.read_f32_rel(Some(CharPosData::POS_X));
        x_pos
    }

    pub fn get_y_pos(&self) ->f32 {
        let x_pos = self.char_pos_data.0.read_f32_rel(Some(CharPosData::POS_Y));
        x_pos
    }

    pub fn get_z_pos(&self) ->f32 {
        let x_pos = self.char_pos_data.0.read_f32_rel(Some(CharPosData::POS_Z));
        x_pos
    }
    


    pub fn get_no_stam_consume(&mut self) -> bool {
        let no_stamina_consume = self
            .chr_dbg
            .0
            .read_bool_rel(Some(ChrDbg::ALL_NO_STAMINA_CONSUME));
        return no_stamina_consume;
    }

    pub fn get_no_update_ai(&mut self) -> bool {
        let no_update_ai = self.chr_dbg.0.read_bool_rel(Some(ChrDbg::ALL_NO_UPDATE_AI));
        return no_update_ai;
    }

    pub fn set_no_stam_consume(&mut self) -> bool {
        let no_stamina_consume = self.get_no_stam_consume();
        if (no_stamina_consume == false) {
            self.chr_dbg
                .0
                .write_u8_rel(Some(ChrDbg::ALL_NO_STAMINA_CONSUME), 0x1);
        } else {
            self.chr_dbg
                .0
                .write_u8_rel(Some(ChrDbg::ALL_NO_STAMINA_CONSUME), 0x0);
        }
        no_stamina_consume
    }

    pub fn set_no_update_ai(&mut self) -> bool {
        let no_update_ai = self.get_no_update_ai();
        if (no_update_ai == false) {
            self.chr_dbg
                .0
                .write_u8_rel(Some(ChrDbg::ALL_NO_UPDATE_AI), 0x1);
        } else {
            self.chr_dbg
                .0
                .write_u8_rel(Some(ChrDbg::ALL_NO_UPDATE_AI), 0x0);
        }
        no_update_ai
    }
}
