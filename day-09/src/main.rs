use core::panic;

#[derive(Debug, Clone, Copy)]
struct Mem {
    file_id: Option<usize>,
}

fn print_disk(disk: &Vec<Mem>) {
    for mem in disk {
        let char = if let Some(file_id) = &mem.file_id {
            file_id.to_string()
        } else {
            ".".to_owned()
        };
        print!("{}", char);
    }
    print!("\n");
}

fn checksum(disk: &Vec<Mem>) -> u64 {
    let mut checksum: u64 = 0;
    let mut pos: usize = 0;
    for mem in disk {
        if let Some(file_id) = mem.file_id {
            checksum += file_id as u64 * pos as u64;
        }
        pos += 1;
    }
    checksum
}

fn part1(disk: &mut Vec<Mem>) -> Vec<Mem> {
    let mut free_ptr = 0;
    let mut mem_ptr = disk.len() - 1;
    loop {
        let mut out = false;
        while disk[free_ptr].file_id.is_some() {
            if free_ptr == disk.len() - 1 || free_ptr >= mem_ptr {
                out = true;
                break;
            }
            free_ptr += 1;
        }

        while disk[mem_ptr].file_id.is_none() {
            if mem_ptr == 0 || free_ptr >= mem_ptr {
                out = true;
                break;
            }
            mem_ptr -= 1;
        }
        if out {
            break;
        }
        disk[free_ptr] = disk[mem_ptr].clone();
        disk[mem_ptr] = Mem { file_id: None };
        // print_disk(&disk);
    }
    disk.to_owned()
}

fn part2(disk: &mut Vec<Mem>) -> Vec<Mem> {
    let mut file_end = (disk.len() - 1) as i32;
    let mut current_file_id = usize::MAX;
    // dbg!(disk.len());
    loop {
        if file_end <= 0 {
            break;
        }
        // Find end
        loop {
            if file_end < 0 {
                break;
            }
            let current = disk[file_end as usize].file_id;
            if current.is_some_and(|fid| fid != current_file_id) {
                file_end += 1;
                current_file_id = current.unwrap();
                break;
            } else {
                file_end -= 1;
            }
        }

        let mut file_start = file_end - 1;
        loop {
            if file_start < 0 {
                file_start = 0;
                break;
            }
            let current = disk[file_start as usize].file_id;
            if current.is_none() || current.is_some_and(|fid| fid != current_file_id) {
                file_start += 1;
                break;
            } else {
                file_start -= 1;
            }
        }

        let file = &disk[file_start as usize..file_end as usize];
        let file_len = file.len();

        // Now try to find a free junk of length >= file.len()

        let mut free_start = 0;
        let mut free_end = 0;

        // Go through all frees
        loop {
            if free_start >= disk.len() || free_end >= disk.len() {
                break;
            }

            // Find start of free
            loop {
                if free_start >= disk.len() {
                    break;
                }
                let current = disk[free_start].file_id;
                if current.is_some() {
                    free_start += 1;
                } else {
                    break;
                }
            }

            // Find end of free
            free_end = free_start;
            loop {
                if free_end >= disk.len() {
                    break;
                }
                let current = disk[free_end].file_id;
                if current.is_none() {
                    free_end += 1;
                } else {
                    break;
                }
            }

            let free_len = free_end - free_start;

            if free_len >= file_len && free_start < file_start as usize {
                for i in free_start..free_start + file_len {
                    disk[i].file_id = Some(current_file_id);
                }
                for i in file_start..file_end {
                    disk[i as usize].file_id = None;
                }
                break;
            }

            free_start = free_end;
        }

        file_end = file_start;
    }
    disk.to_owned()
}

fn main() {
    // let example = "2333133121414131402";
    // let example = "2333133121414131402";
    let example = std::fs::read_to_string("input").unwrap();

    let mut disk = Vec::<Mem>::new();

    let mut current_file_id = 0;
    for i in (0..example.len()).step_by(2) {
        let used = example[i..i + 1].parse::<usize>().unwrap();

        for _ in 0..used {
            let mem = Mem {
                file_id: Some(current_file_id),
            };
            disk.push(mem);
        }

        // There should be also space declared

        if i == example.len() - 1 {
            break;
        }
        let s = &example[i + 1..i + 2];
        if s.trim().is_empty() {
            break;
        }
        let free = s.parse::<usize>();
        let free = if let Ok(free) = free {
            free
        } else {
            panic!("Invalid str {} {}", s, s.len());
        };
        for _ in 0..free {
            let mem = Mem { file_id: None };
            disk.push(mem);
        }
        current_file_id += 1;
    }

    let part1 = part1(&mut disk.clone());
    let checksum1 = checksum(&part1);
    println!("{}", checksum1);
    let part2 = part2(&mut disk.clone());
    let checksum2 = checksum(&part2);
    println!("{}", checksum2);
}
