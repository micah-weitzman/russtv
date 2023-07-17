#![allow(unused)]

use std::path::Path;
use std::io::Write;
use std::fs::File;

use crate::sstv::crypt;


pub struct RGB {
  r: u8,
  g: u8,
  b: u8,
}

pub struct Image {
  height: u32,
  width: u32,
  data: Vec<u8>,
}

impl Image {
    pub fn new(height: u32, width: u32) -> Image {
      let size = 3 * height * width;
      let data = vec![0; size as usize];
      Image { height, width, data }
    }

    fn buffer_size(&self) -> u32 {
      3 * self.height * self.width
    }

    fn get_offset(&self, x: u32, y: u32) -> Option<usize> {
      let offset = (y * self.width * 3) + (x * 3);
      if offset < self.buffer_size() {
        Some(offset as usize)
      } else {
        None
      }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<RGB> {
      match self.get_offset(x, y) {
        Some(offset) => {
          let r = self.data[offset];
          let g = self.data[offset + 1];
          let b = self.data[offset + 2];
          Some(RGB {r, g, b})
        },
        None => None
      }
    }

    pub fn set_pixel_usize(&mut self, x: u32, y: u32, color: (usize, usize, usize)) -> bool {
      match self.get_offset(x, y) {
        Some(offset) => {
          self.data[offset] = color.0 as u8;
          self.data[offset + 1] = color.1 as u8;
          self.data[offset + 2] = color.2 as u8;
          true
        },
        None => false
      }
    }

    pub fn write_file(&self, filename: &str) -> std::io::Result<()> {

      let path = Path::new(filename);
      let mut file = File::create(&path)?;
      let header = format!("P6 {} {} 255\n", self.width, self.height);
      file.write(header.as_bytes())?;
      file.write(&self.data)?;

      Ok(())
    }



    fn write_chunk(outfile: &mut File, tag: &[u8], data: &Vec<u8>) -> std::io::Result<()> {
      // Write a PNG chunk to the output file, including length and
      // checksum.
      outfile.write(&(data.len() as u32).to_be_bytes())?;
      outfile.write(tag)?;
      outfile.write(&data)?;
      let mut all_data = tag.to_vec();
      all_data.extend(data);
      let checksum = crypt::crc(&all_data);
      outfile.write(&checksum.to_be_bytes())?;
      Ok(())
    }


    pub fn write_file_png(&self, filename: &str) -> std::io::Result<()> {

      let path: &Path = Path::new(filename);
      let mut file: File = File::create(&path)?;

      // File metadata
      let bit_depth: u8 = 8;
      let color_type: u8 = 2;
      let compression_method: u8 = 0;
      let filter_method: u8 = 0;
      let interlace_method: u8 = 0;

      let png_sig: Vec<u8> = vec![0x89,0x50,0x4E,0x47,0x0D,0x0A,0x1A,0x0A]; // PNG SIGNATURE
      file.write(&png_sig)?;

      // png IHDR
      file.write(&[0x00, 0x00, 0x00, 0x0D])?; // length

      let mut header: Vec<u8> = vec![0x49, 0x48, 0x44, 0x52]; // IHDR bytes
      header.extend(self.width.to_be_bytes());
      header.extend(self.height.to_be_bytes());
      header.extend(&[bit_depth, color_type, 0x00, 0x00, interlace_method]);
      let header_crc = crypt::crc(&header);

      file.write(&header)?;
      file.write(&header_crc.to_be_bytes())?;

      let mut new_slice: Vec<u8> =  vec![];

      let header = [0x49, 0x44, 0x41, 0x54]; // IDAT
      self.data.chunks(3 * self.width as usize).for_each(|slice| {
        new_slice.extend(&[0]);
        new_slice.extend(slice);
      });

      let deflate_data = crypt::encode_data_zlib(&new_slice);
      Image::write_chunk(&mut file, &header, &deflate_data)?;


      // WRITE END
      let end = [0x49,0x45,0x4E,0x44]; // IEND
      let empty: Vec<u8> = vec![];
      Image::write_chunk(&mut file, &end, &empty)?;

      Ok(())
    }
}