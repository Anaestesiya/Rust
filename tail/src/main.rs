use clap::Parser;
use std::fs::File;
use std::io::SeekFrom;
use std::io::Seek;
use std::io::Read;
use std::io;
use std::io::Write;

const CHUNK_LEN: usize = 2048;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    file: String,

    #[arg(short, long)]
    num: u32,
}

fn main() {
    let args = Args::parse();
    let mut file = File::open(&args.file).unwrap();

    let mut pos = file.seek(SeekFrom::End(0)).unwrap();
    let mut endline_positions : Vec<u64> = Vec::new();
    let mut chunk = [0u8; CHUNK_LEN];


    while pos > 0 && endline_positions.len() <= args.num as usize {
        let read_size: usize = if pos >= CHUNK_LEN as u64 {
            CHUNK_LEN
        }
        else {
            pos as usize
        };
        pos -= read_size as u64;

        file.seek(SeekFrom::Start(pos)).unwrap();
        file.read_exact(&mut chunk[..read_size]).unwrap();

        // search for '\n'
        for i in (0..read_size).rev() {
            if chunk[i] == b'\n' {
                endline_positions.push(pos + i as u64);
                if endline_positions.len() > args.num as usize {
                    break;
                }
            }
        }
    }
    
    endline_positions.reverse();
    //println!("{:?}", endline_positions);
    
    let read_position = if endline_positions.len() >= 1 + args.num as usize {
        endline_positions[endline_positions.len() - 1 - args.num as usize]
    } else {
        0
    };
    println!("{} {}", endline_positions.len(), read_position);
    file.seek(SeekFrom::Start(read_position)).unwrap();


    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).unwrap();
    io::stdout().write_all(&buffer).unwrap();
    //println!("{:?}", buffer);
}
