#![allow(unused_variables)]

use crate::sstv::spec;
use crate::sstv::img;


type PixelVec = Vec<Vec<Vec<usize>>>;


pub fn calc_lum(freq: f32) -> usize {
  use std::cmp;
  // Converts SSTV pixel frequency range into 0-255 luminance byte
  let output = ((255.0 / (2300.0 - 1500.0)) * (freq - 1500.0)) as usize;
  return   cmp::min(cmp::max(output, 0), 255);
}


fn peak_fft_freq(data: &[i16], sample_rate: u32) -> f32 {

  use easyfft::prelude::*;
  use spectrum_analyzer::windows::hann_window;

  //"""Finds the peak frequency from a section of audio data"""
  let abs_vals: Vec<f32> = data.into_iter().map(|p| *p as f32).collect();
  let windowed_data = hann_window(abs_vals.as_ref());

  let fft: Vec<u32> = windowed_data.real_fft().iter().map(|v| v.norm() as u32).collect();

  let max: &u32 = fft.iter().max().unwrap();
  let index: usize = fft.iter().position(|element| element == max).unwrap();

  let y1 = if index <= 0 {fft[index]}  else {fft[index-1]} as f32;
  let y3 =  if index + 1 >= fft.len() {fft[index]} else {fft[index+1]} as f32;

  // interpolate max with adjacent values
  let peak =  ((y3 - y1) as f32 / (y3 - y1 + *max as f32) + index as f32) as f32;

  return peak as f32 * sample_rate as f32 / abs_vals.len() as f32;
}

#[derive(Clone)]
pub struct SSTVSetup {
  // mode: Option<spec::Spec>,
  sample_rate: u32,
  samples: Vec<i16>,
}

pub struct SSTVDecoder {
  mode: spec::Spec,
  sample_rate: u32,
  samples: Vec<i16>,
  header_end: usize,
}


// Create an SSTV decoder for decoding audio data
impl SSTVSetup {
  pub fn new(audio_file: &String) -> Self {
    use std::fs::File;
    use std::io::BufReader;
    use rodio::{Decoder, source::Source};

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(audio_file).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();

    let sample_rate = source.sample_rate();
    let channels = source.channels();

    // convert to mono if stereo
    let samples: Vec<i16> = source.step_by(channels as usize).collect();

    SSTVSetup {
      sample_rate,
      samples,
    }
  }


  pub fn decode(&self) -> Result<SSTVDecoder, String> {
    //"""Attempts to decode the audio data as an SSTV signal
    //Returns a PIL image on success, and None if no SSTV signal was found
    //"""

    let header_end = self.find_header()?;
    let mode = self.decode_vis(header_end)?;

    let samples_copy: Vec<i16> = self.samples.iter().map(|e| *e).collect();
    let new_s = SSTVDecoder {
      mode,
      sample_rate: self.sample_rate,
      samples: samples_copy,
      header_end,
    };

    Ok(new_s)
  }



  fn find_header(&self) -> Result<usize, String> {
    //"""Finds the approx sample of the end of the calibration header"""

    let header_size = (spec::HDR_SIZE * self.sample_rate as f32) as usize;
    let window_size = (
      (spec::HDR_WINDOW_SIZE * self.sample_rate as f32) as u64
     ) as usize;

    // Relative sample offsets of the header tones
    let leader_1_sample = 0;
    let leader_1_search = leader_1_sample + window_size;

    let break_sample = (spec::BREAK_OFFSET * self.sample_rate as f32) as usize;
    let break_search = break_sample + window_size;

    let leader_2_sample = (spec::LEADER_OFFSET * self.sample_rate as f32) as usize;
    let leader_2_search = leader_2_sample + window_size;

    let vis_start_sample = (spec::VIS_START_OFFSET * self.sample_rate as f32) as usize;
    let vis_start_search = vis_start_sample + window_size;

    let jump_size = (0.002 * self.sample_rate as f32) as usize;  // check every 2ms

    // The margin of error created here will be negligible when decoding the
    // vis due to each bit having a length of 30ms. We fix this error margin
    // when decoding the image by aligning each sync pulse

    let size = self.samples.iter().count();

    for i in 0..(size - header_size) {
      let current_sample = i * jump_size as usize;
      // Update search progress message
      if current_sample % (jump_size * 256) == 0 {
        // search_msg = "Searching for calibration header... {:.1f}s";
        // let progress = current_sample as u32 / self.sample_rate;
      //   log_message(search_msg.format(progress), recur=True);
      }

      let search_end = current_sample + header_size;
      let search_area = &self.samples[current_sample..search_end];

      let leader_1_area = &search_area[leader_1_sample..leader_1_search];
      let break_area = &search_area[break_sample..break_search];
      let leader_2_area = &search_area[leader_2_sample..leader_2_search];
      let vis_start_area = &search_area[vis_start_sample..vis_start_search];

      // Check they're the correct frequencies
      if (peak_fft_freq(leader_1_area, self.sample_rate) - 1900.0).abs() < 50.0
          && (peak_fft_freq(break_area, self.sample_rate) - 1200.0).abs() < 50.0
          && (peak_fft_freq(leader_2_area, self.sample_rate) - 1900.0).abs() < 50.0
          && (peak_fft_freq(vis_start_area, self.sample_rate) - 1200.0).abs() < 50.0 {

        return Ok(current_sample + header_size);
      }
    }

    return Err("Couldn't find SSTV header in the given audio file".to_string());
  }

  fn decode_vis(&self, vis_start: usize) -> Result<spec::Spec, String> {
      //"""Decodes the vis from the audio data and returns the SSTV mode"""

      let bit_size = (spec::VIS_BIT_SIZE * self.sample_rate as f32) as usize;
      let mut vis_bits: Vec<usize> = Vec::new();

      for bit_idx in 0..8 {
        let bit_offset = vis_start + bit_idx * bit_size;
        let window_width = bit_offset + bit_size;
        let section = &self.samples[bit_offset..window_width];
        let freq = peak_fft_freq(section, self.sample_rate);
        // 1100 hz = 1, 1300hz = 0
        // println!("Bit {}: {}", bit_idx, freq);
        if freq <= 1200.0 {
          vis_bits.push(1)
        } else {
          vis_bits.push(0)
        }
      }

      // Check for even parity in last bit
      let vis_bit_sum: usize = vis_bits.iter().sum();
      let parity = vis_bit_sum % 2 == 0;
      if !parity {
        return Err("Error decoding VIS header (invalid parity bit)".to_string());
      }

      vis_bits.pop();
      vis_bits.reverse();
      // println!("vis bits: {:?}", vis_bits);
      // LSB first so we must reverse and ignore the parity bit
      let mut vis_value = 0;
      for bit in vis_bits {
        vis_value = (vis_value << 1) | bit;
      }

      match spec::VIS_MAP(vis_value) {
        Ok(mode) => {
          println!("Detected SSTV mode {}", mode.NAME);
          return Ok(mode);
        }
        Err(_) => Err(format!("SSTV mode is unsupported (VIS: {})", vis_value))
      }
  }
}

impl SSTVDecoder {
  #[allow(unused)]
  #[deprecated(since="0.1.0", note="please use `new_method` instead")]
  pub fn save(&self, filename: &str) -> Result<(), String> {
    let vis_end = (self.header_end as f32 + (spec::VIS_BIT_SIZE * 9.0 * self.sample_rate as f32)) as usize;

    let image_data: PixelVec = self.decode_image_data(vis_end)?;
    let img: img::Image = self.draw_image(image_data);
    match img.write_file(filename) {
      Err(..) => Err("Encounter error when writing to file".to_string()),
      Ok(..) => {
        println!("File written");
        Ok(())
      }
    }
  }


  pub fn save_png(&self, filename: &str) -> Result<(), String> {
    let vis_end = (self.header_end as f32 + (spec::VIS_BIT_SIZE * 9.0 * self.sample_rate as f32)) as usize;

    let image_data: PixelVec = self.decode_image_data(vis_end)?;
    let img: img::Image = self.draw_image(image_data);
    match img.write_file_png(filename) {
      Err(..) => Err("Encounter error when writing to file".to_string()),
      Ok(..) => {
        println!("File written");
        Ok(())
      }
    }
  }


  fn align_sync(&self, align_start: usize, start_of_sync:bool) -> Result<usize,()> {
    // """Returns sample where the beginning of the sync pulse was found"""

    // TODO - improve this

    let sync_window = (self.mode.SYNC_PULSE * 1.4 * self.sample_rate as f32) as usize;
    let align_stop = self.samples.len() - sync_window;

    if align_stop <= align_start {
      return Err(());  // Reached end of audio
    }

    let mut curr_sample_ref = align_stop;

    for current_sample in align_start..align_stop {
      let section_end = current_sample + sync_window;
      let search_section = &self.samples[current_sample..section_end];

      if peak_fft_freq(search_section, self.sample_rate) > 1350.0 {
        curr_sample_ref = current_sample;
        break;
      }
    }

    let end_sync = curr_sample_ref as f32 + (sync_window  as f32/ 2.0) as f32;

    if start_of_sync {
      return Ok((end_sync as f32 - (self.mode.SYNC_PULSE * self.sample_rate as f32)) as usize);
    } else {
      return Ok(end_sync as usize);
    }
  }

  fn decode_image_data(&self, image_start: usize) -> Result<PixelVec, String> {
      // """Decodes image from the transmission section of an sstv signal"""

      let window_factor = self.mode.WINDOW_FACTOR;
      let centre_window_time = (self.mode.PIXEL_TIME * window_factor) / 2.0;
      let mut pixel_window = (centre_window_time * 2.0 * self.sample_rate as f32) as usize;

      let height = self.mode.LINE_COUNT;
      let channels = self.mode.CHAN_COUNT;
      let width = self.mode.LINE_WIDTH;
      // Use list comprehension to init list so we can return data early
      let mut image_data: PixelVec = vec![vec![vec![0; width]; channels]; height];


      let mut seq_start = image_start;
      if self.mode.HAS_START_SYNC {
        // Start at the end of the initial sync pulse
        let res = self.align_sync(image_start, false);
        match res {
          Err(..) => return Err("Reached end of audio before image data".to_string()),
          Ok(val) => seq_start = val,
        }
      }

      for line in 0..height {
        if self.mode.CHAN_SYNC > 0 && line == 0 {
          // Align seq_start to the beginning of the previous sync pulse
          let sync_offset = self.mode.CHAN_OFFSETS[self.mode.CHAN_SYNC];
          seq_start -=((sync_offset + self.mode.SCAN_TIME) * self.sample_rate as f32) as usize;
        }

        for chan in 0..channels {
          if chan == self.mode.CHAN_SYNC {
            if line > 0 || chan > 0 {
              // Set base offset to the next line
              seq_start += (self.mode.LINE_TIME * self.sample_rate as f32) as usize;
            }
            // Align to start of sync pulse
            let res = self.align_sync(seq_start, true);
            match res {
              Err(..) => {
                println!("Reached end of audio whilst decoding.");
                return Ok(image_data);
              },
              Ok(start) => seq_start = start,
            }
          }
          let mut pixel_time = self.mode.PIXEL_TIME;
          if self.mode.HAS_HALF_SCAN {
            // Robot mode has half-length second/third scans
            if chan > 0 {
              pixel_time = self.mode.HALF_PIXEL_TIME;
            }

            let centre_window_time = (pixel_time * window_factor) / 2.0;
            pixel_window = (centre_window_time * 2.0 * self.sample_rate as f32) as usize;
          }

          for px in 0..width {
            let chan_offset = self.mode.CHAN_OFFSETS[chan];

            let px_pos = (seq_start as f32 + (chan_offset + px as f32 *
                            pixel_time - centre_window_time) *
                            self.sample_rate as f32) as usize;
            let px_end = px_pos + pixel_window;

            // If we are performing fft past audio length, stop early
            if px_end >= self.samples.len() {
              println!("Reached end of audio whilst decoding.");
              return Ok(image_data);
            }

            let pixel_area = &self.samples[px_pos..px_end];
            let freq = peak_fft_freq(pixel_area, self.sample_rate);

            image_data[line][chan][px] = calc_lum(freq);
          }

          // progress_bar(line, height - 1, "Decoding image...");`
        }
      }
    return Ok(image_data);
  }

  fn draw_image(&self, image_data: PixelVec) -> img::Image {
    //"""Renders the image from the decoded sstv signal"""
    // Let PIL do YUV-RGB conversion for us
    if self.mode.COLOR == spec::ColFmt::YUV {
      let col_mode: &str = "YCbCr";
    } else {
      let col_mode = "RGB";
    }

    let width = self.mode.LINE_WIDTH;
    let height = self.mode.LINE_COUNT;
    let channels = self.mode.CHAN_COUNT;

    let mut image = img::Image::new(height as u32, width as u32);

    println!("Drawing image data...");

    for y in 0..height {
      let odd_line = y % 2;
      for x in 0..width {
        let mut pixel = (0,0,0);
        if channels == 2 {

          if self.mode.HAS_ALT_SCAN {
            if self.mode.COLOR == spec::ColFmt::YUV {
              // R36
              pixel = (image_data[y][0][x],
                        image_data[y-(odd_line-1)][1][x],
                        image_data[y-odd_line][1][x]);
            }
          }

        } else if channels == 3 {
          if self.mode.COLOR == spec::ColFmt::GBR {
            // M1, M2, S1, S2, SDX
            pixel = (image_data[y][2][x],
                      image_data[y][0][x],
                      image_data[y][1][x]);
          } else if self.mode.COLOR == spec::ColFmt::YUV {
            // R72
            pixel = (image_data[y][0][x],
                          image_data[y][2][x],
                          image_data[y][1][x]);
          } else if self.mode.COLOR == spec::ColFmt::RGB {
            pixel = (image_data[y][0][x],
                          image_data[y][1][x],
                          image_data[y][2][x]);
          }

        }
        image.set_pixel_usize(x as u32, y as u32, pixel);
      }
    }


    // if image.mode != RGB {
    //   image = image.convert("RGB");
    // }

    image
  }
}