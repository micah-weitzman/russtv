mod sstv;

fn main() {
  let args: Vec<String> = std::env::args().collect();

  let input_file: &String = match args.get(1) {
    Some(filename) => filename,
    _ => panic!("Must give audofile as input")
  };

  let defualt_out = "out.png".to_string();
  let out_file: &String = args.get(2).unwrap_or(&defualt_out);


  match main_decode(input_file, out_file) {
    Err(s) => println!("{}", s),
    Ok(()) => println!("Done.")
  }
}

fn main_decode(infile: &String, outfile: &str) -> Result<(), String> {
  let setup: sstv::SSTVSetup = sstv::SSTVSetup::new(infile);
  let decoder: sstv::SSTVDecoder = setup.decode()?;

  decoder.save_png(outfile)
}