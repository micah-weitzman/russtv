#![allow(dead_code, non_snake_case)]
// """Constants for SSTV specification and each supported mode"""

#[derive(Debug, Clone, PartialEq)]
pub enum ColFmt {
  RGB,
  GBR,
  YUV,
  BW,
}


#[derive(Debug, Clone)]
pub struct Spec {
  pub NAME: String,
  pub COLOR: ColFmt,
  pub LINE_WIDTH: usize,
  pub LINE_COUNT: usize,
  pub SCAN_TIME: f32,
  pub HALF_SCAN_TIME: f32,
  pub SYNC_PULSE: f32,
  pub SYNC_PORCH: f32,
  pub SEP_PULSE: f32,
  pub SEP_PORCH: f32,

  pub CHAN_COUNT: usize,
  pub CHAN_SYNC: usize,
  pub CHAN_TIME: f32,
  pub HALF_CHAN_TIME: f32,

  pub CHAN_OFFSETS: Vec<f32>,

  pub LINE_TIME: f32,
  pub PIXEL_TIME: f32,
  pub HALF_PIXEL_TIME: f32,
  pub WINDOW_FACTOR: f32,

  pub HAS_START_SYNC: bool,
  pub HAS_HALF_SCAN: bool,
  pub HAS_ALT_SCAN: bool,
}

pub struct M1;
pub struct M2;
pub struct S1;
pub struct S2;
pub struct SDX;
pub struct R36;
pub struct R72;

impl M1 {
  pub fn new() -> Spec {
    let sep_pulse: f32 = 0.000572;
    let scan_time: f32 = 0.146432;
    let sync_pulse: f32 = 0.004862;
    let sync_porch: f32 = 0.000572;
    let line_width: usize = 320;
    let chan_time = sep_pulse + scan_time;
    let chan_offsets: Vec<f32> = (0..3).into_iter().map(|i| (sync_pulse + sync_porch) + chan_time * (i) as f32).collect();
    Spec {
      NAME: "Martin 1".to_string(),
      COLOR: ColFmt::GBR,
      LINE_WIDTH: line_width,
      LINE_COUNT: 256,
      SCAN_TIME: scan_time,
      HALF_SCAN_TIME: 0.0,
      SYNC_PULSE: sync_pulse,
      SYNC_PORCH: sync_porch,
      SEP_PULSE: sep_pulse,
      SEP_PORCH: 0.0,

      CHAN_COUNT: 3,
      CHAN_SYNC: 0,
      CHAN_TIME: chan_time,
      HALF_CHAN_TIME: 0.0,

      CHAN_OFFSETS: chan_offsets,

      LINE_TIME: sync_pulse + sync_porch + 3.0 * chan_time,
      PIXEL_TIME: scan_time / line_width as f32,
      HALF_PIXEL_TIME: 0.0,
      WINDOW_FACTOR: 2.34,

      HAS_START_SYNC: false,
      HAS_HALF_SCAN: false,
      HAS_ALT_SCAN: false,
    }
    }
  }



impl M2 {
  pub fn new() -> Spec {
    let sep_pulse: f32 = 0.000572;
    let scan_time: f32 =  0.073216;
    let sync_pulse: f32 = 0.004862;
    let sync_porch: f32 = 0.000572;
    let line_width: usize = 320;
    let chan_time = sep_pulse + scan_time;
    let chan_offsets: Vec<f32> = (0..3).into_iter().map(|i| (sync_pulse + sync_porch) + chan_time * (i) as f32).collect();

    Spec {
      NAME: "Martin 2".to_string(),
      COLOR: ColFmt::GBR,
      LINE_WIDTH: line_width,
      LINE_COUNT: 256,
      SCAN_TIME: scan_time,
      HALF_SCAN_TIME: 0.0,
      SYNC_PULSE: sync_pulse,
      SYNC_PORCH: sync_porch,
      SEP_PULSE: sep_pulse,
      SEP_PORCH: 0.0,

      CHAN_COUNT: 3,
      CHAN_SYNC: 0,
      CHAN_TIME: chan_time,
      HALF_CHAN_TIME: 0.0,

      CHAN_OFFSETS: chan_offsets,

      LINE_TIME: sync_pulse + sync_porch + 3.0 * chan_time,
      PIXEL_TIME: scan_time / line_width as f32,
      HALF_PIXEL_TIME: 0.0,
      WINDOW_FACTOR: 4.68,

      HAS_START_SYNC: false,
      HAS_HALF_SCAN: false,
      HAS_ALT_SCAN: false,
    }
  }
}

impl S1 {
  pub fn new() -> Spec {
    let sync_pulse: f32 = 0.009000;
    let sep_pulse: f32 = 0.001500;
    let scan_time: f32 =  0.138240;
    let sync_porch: f32 = 0.001500;
    let line_width: usize  = 320;
    let chan_time: f32 = sep_pulse + scan_time;

    let chan_offsets: Vec<f32> = vec![
      sync_pulse + sync_porch + chan_time,
      sync_pulse + sync_porch + chan_time * 2.0,
      sync_pulse + sync_porch
    ];
    Spec {
      NAME: "Scottie 1".to_string(),

      COLOR: ColFmt::GBR,
      LINE_WIDTH: line_width,
      LINE_COUNT: 256,
      SCAN_TIME: scan_time,
      HALF_SCAN_TIME: 0.0,
      SYNC_PULSE: sync_pulse,
      SYNC_PORCH: sync_porch,
      SEP_PULSE: sep_pulse,
      SEP_PORCH: 0.0,

      CHAN_COUNT: 3,
      CHAN_SYNC: 2,
      CHAN_TIME: chan_time,
      HALF_CHAN_TIME: 0.0,

      CHAN_OFFSETS: chan_offsets,

      LINE_TIME: sync_pulse + 3.0 * chan_time,
      PIXEL_TIME: scan_time / line_width as f32,
      HALF_PIXEL_TIME: 0.0,
      WINDOW_FACTOR: 2.48,

      HAS_START_SYNC: true,
      HAS_HALF_SCAN: false,
      HAS_ALT_SCAN: false,
    }
  }
}


impl S2 {
  pub fn new() -> Spec {
    let sync_pulse: f32 = 0.009000;
    let sep_pulse: f32 = 0.001500;
    let scan_time: f32 =  0.088064;
    let sync_porch: f32 = 0.001500;
    let line_width: usize  = 320;
    let chan_time: f32 = sep_pulse + scan_time;

    let chan_offsets: Vec<f32> = vec![
      sync_pulse + sync_porch + chan_time,
      sync_pulse + sync_porch + chan_time * 2.0,
      sync_pulse + sync_porch
    ];
    Spec {
      NAME: "Scottie 2".to_string(),

      COLOR: ColFmt::GBR,
      LINE_WIDTH: line_width,
      LINE_COUNT: 256,
      SCAN_TIME: scan_time,
      HALF_SCAN_TIME: 0.0,
      SYNC_PULSE: sync_pulse,
      SYNC_PORCH: sync_porch,
      SEP_PULSE: sep_pulse,
      SEP_PORCH: 0.0,

      CHAN_COUNT: 3,
      CHAN_SYNC: 2,
      CHAN_TIME: chan_time,
      HALF_CHAN_TIME: 0.0,

      CHAN_OFFSETS: chan_offsets,

      LINE_TIME: sync_pulse + 3.0 * chan_time,
      PIXEL_TIME: scan_time / line_width as f32,
      HALF_PIXEL_TIME: 0.0,
      WINDOW_FACTOR: 3.82,

      HAS_START_SYNC: true,
      HAS_HALF_SCAN: false,
      HAS_ALT_SCAN: false,
    }
  }
}



impl SDX {
  pub fn new() -> Spec {
    let sync_pulse: f32 = 0.009000;
    let sep_pulse: f32 = 0.001500;
    let scan_time: f32 =  0.345600;
    let sync_porch: f32 = 0.001500;
    let line_width: usize  = 320;
    let chan_time: f32 = sep_pulse + scan_time;

    let chan_offsets: Vec<f32> = vec![
      sync_pulse + sync_porch + chan_time,
      sync_pulse + sync_porch + chan_time * 2.0,
      sync_pulse + sync_porch
    ];
    Spec {
      NAME: "Scottie DX".to_string(),

      COLOR: ColFmt::GBR,
      LINE_WIDTH: line_width,
      LINE_COUNT: 256,
      SCAN_TIME: scan_time,
      HALF_SCAN_TIME: 0.0,
      SYNC_PULSE: sync_pulse,
      SYNC_PORCH: sync_porch,
      SEP_PULSE: sep_pulse,
      SEP_PORCH: 0.0,

      CHAN_COUNT: 3,
      CHAN_SYNC: 2,
      CHAN_TIME: chan_time,
      HALF_CHAN_TIME: 0.0,

      CHAN_OFFSETS: chan_offsets,

      LINE_TIME: sync_pulse + 3.0 * chan_time,
      PIXEL_TIME: scan_time / line_width as f32,
      HALF_PIXEL_TIME: 0.0,
      WINDOW_FACTOR: 0.98,

      HAS_START_SYNC: true,
      HAS_HALF_SCAN: false,
      HAS_ALT_SCAN: false,
    }
  }
}



impl R36 {
  pub fn new() -> Spec {
    let SCAN_TIME: f32 = 0.088000;
    let LINE_WIDTH: usize  = 320;
    let HALF_SCAN_TIME: f32 = 0.044000;
    let SYNC_PORCH: f32 = 0.003000;
    let SEP_PULSE: f32 = 0.004500;
    let SEP_PORCH: f32 = 0.001500;
    let CHAN_TIME: f32 = SEP_PULSE + SCAN_TIME;
    let SYNC_PULSE: f32 = 0.009000;
    let PIXEL_TIME: f32 = SCAN_TIME / LINE_WIDTH as f32;
    let CHAN_OFFSETS: Vec<f32> = vec! [
      SYNC_PULSE + SYNC_PORCH,
      SYNC_PULSE + SYNC_PORCH + CHAN_TIME + SEP_PORCH
    ];

    let LINE_TIME: f32 = CHAN_OFFSETS[1] + HALF_SCAN_TIME;

    Spec {
      NAME: "Robot 36".to_string(),

      COLOR: ColFmt::YUV,
      LINE_WIDTH,
      LINE_COUNT: 240,
      SCAN_TIME,
      HALF_SCAN_TIME,
      SYNC_PULSE,
      SYNC_PORCH,
      SEP_PULSE,
      SEP_PORCH,

      CHAN_COUNT: 2,
      CHAN_SYNC: 0,
      CHAN_TIME,
      HALF_CHAN_TIME: 0.0,

      CHAN_OFFSETS,

      LINE_TIME,
      PIXEL_TIME,
      HALF_PIXEL_TIME: HALF_SCAN_TIME / LINE_WIDTH as f32,
      WINDOW_FACTOR: 7.70,

      HAS_START_SYNC: false,
      HAS_HALF_SCAN: true,
      HAS_ALT_SCAN: true,
    }
  }
}


impl R72 {
  pub fn new() -> Spec {
    let SCAN_TIME: f32 = 0.138000;
    let LINE_WIDTH: usize  = 320;
    let HALF_SCAN_TIME: f32 = 0.069000;
    let SYNC_PORCH: f32 = 0.003000;
    let SEP_PULSE: f32 = 0.004500;
    let SEP_PORCH: f32 = 0.001500;
    let CHAN_TIME: f32 = SEP_PULSE + SCAN_TIME;
    let SYNC_PULSE: f32 = 0.009000;
    let HALF_CHAN_TIME: f32 = SEP_PULSE + HALF_SCAN_TIME;
    let CHAN_OFFSETS: Vec<f32> = vec! [
      SYNC_PULSE + SYNC_PORCH,
      SYNC_PULSE + SYNC_PORCH + CHAN_TIME + SEP_PORCH,
      SYNC_PULSE + SYNC_PORCH + CHAN_TIME + SEP_PORCH + HALF_CHAN_TIME + SEP_PORCH
    ];

    let LINE_TIME: f32 = CHAN_OFFSETS[2] + HALF_SCAN_TIME;

    Spec {
      NAME: "Robot 72".to_string(),

      COLOR: ColFmt::YUV,
      LINE_WIDTH,
      LINE_COUNT: 240,
      SCAN_TIME,
      HALF_SCAN_TIME,
      SYNC_PULSE,
      SYNC_PORCH,
      SEP_PULSE,
      SEP_PORCH,

      CHAN_COUNT: 3,
      CHAN_SYNC: 0,
      CHAN_TIME,
      HALF_CHAN_TIME,

      CHAN_OFFSETS,

      LINE_TIME,
      PIXEL_TIME: SCAN_TIME / LINE_WIDTH as f32,
      HALF_PIXEL_TIME: HALF_SCAN_TIME / LINE_WIDTH as f32,
      WINDOW_FACTOR: 4.88,

      HAS_START_SYNC: false,
      HAS_HALF_SCAN: true,
      HAS_ALT_SCAN: false,
    }
  }
}



pub fn VIS_MAP(vis: usize) -> Result<Spec, String> {
  match vis {
    8 => Ok(R36::new()),
    12 => Ok(R72::new()),
    40 => Ok(M2::new()),
    44 => Ok(M1::new()),
    56 => Ok(S2::new()),
    60 => Ok(S1::new()),
    76 => Ok(SDX::new()),
    _ => Err("Not found".to_owned())
  }
}

pub const BREAK_OFFSET: f32 = 0.300;
pub const LEADER_OFFSET: f32 = 0.010 + BREAK_OFFSET;
pub const VIS_START_OFFSET: f32 = 0.300 + LEADER_OFFSET;

pub const HDR_SIZE: f32 = 0.030 + VIS_START_OFFSET;
pub const HDR_WINDOW_SIZE: f32 = 0.010;

pub const VIS_BIT_SIZE: f32 = 0.030;
